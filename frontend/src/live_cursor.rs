// Mouse position interpolation based on Catmull-Rom splines:
// https://github.com/steveruizok/perfect-cursors
// https://www.mvps.org/directx/articles/catmull/
// http://graphics.cs.cmu.edu/nsp/course/15-462/Fall04/assts/catmullRom.pdf
use crate::utils::performance as get_performance;
use shared::datatypes::{Color, PointerPosition};
use std::cmp::min;
use std::collections::VecDeque;

type Point = (f64, f64);

struct Spline {
    points: Vec<Point>,
}

impl Spline {
    pub fn new() -> Spline {
        Spline { points: Vec::new() }
    }

    fn add_point(&mut self, x: f64, y: f64) {
        self.points.push((x, y));
    }
    fn clear(&mut self) {
        self.points.clear();
    }
    // Index is a number in [0, len(points)]
    fn interpolate_point(&self, index: f64) -> (f64, f64) {
        let i = index.trunc() as usize;
        let l = self.points.len() - 1;
        let p1 = min(i + 1, l);
        let p2 = min(p1 + 1, l);
        let p3 = min(p2 + 1, l);
        let p0 = p1 - 1;
        let t = index - i as f64;
        let tt = t * t;
        let ttt = tt * t;
        // Catmull-Rom spline interpolation
        // https://www.mvps.org/directx/articles/catmull/
        // http://graphics.cs.cmu.edu/nsp/course/15-462/Fall04/assts/catmullRom.pdf
        // (with tau/tension = 0.5)
        let tau = 0.5;
        let q0 = -tau * t + 2.0 * tau * tt - tau * ttt;
        let q1 = 1.0 + (tau - 3.0) * tt + (2.0 - tau) * ttt;
        let q2 = tau * t + (3.0 - 2.0 * tau) * tt + (tau - 2.0) * ttt;
        let q3 = -tau * tt + tau * ttt;
        return (
            self.points[p0].0 * q0
                + self.points[p1].0 * q1
                + self.points[p2].0 * q2
                + self.points[p3].0 * q3,
            self.points[p0].1 * q0
                + self.points[p1].1 * q1
                + self.points[p2].1 * q2
                + self.points[p3].1 * q3,
        );
    }
}

#[derive(Copy, Clone)]
struct Animation {
    // We animate from a given index to the next index over a given timeframe
    start_index: usize,
    // millis from performance.now()
    duration: f64,
    // performance.now() timestamp at which we started to animate this one
    started_at: Option<f64>,
}

pub struct LiveCursor {
    pub color: Color,
    pub current_position: Point,
    performance: web_sys::Performance,
    // timestamp of last call to add_point
    last_add_point: f64,
    spline: Spline,
    anim_queue: VecDeque<Animation>,
}

const MAX_INTERVAL: f64 = 300.0;

impl LiveCursor {
    pub fn new(pos: PointerPosition) -> LiveCursor {
        let performance = get_performance();
        LiveCursor {
            color: pos.color,
            current_position: (pos.x, pos.y),
            last_add_point: performance.now(),
            performance: performance,
            spline: Spline::new(),
            anim_queue: VecDeque::new(),
        }
    }

    pub fn add_point(&mut self, x: f64, y: f64) {
        let now = self.performance.now();
        // If too long has passed since the last cursor update, clear the spline and start again
        // from scratch
        if now - self.last_add_point > MAX_INTERVAL {
            self.spline.clear();
            self.spline.add_point(x, y);
            self.spline.add_point(x, y);
            self.last_add_point = now;
        }
        self.spline.add_point(x, y);

        let duration = MAX_INTERVAL.min(now - self.last_add_point);
        self.last_add_point = now;
        if self.spline.points.len() < 4 {
            self.current_position = (x, y)
        } else {
            // Enqueue an animation
            let animation = Animation {
                start_index: self.spline.points.len() - 3,
                duration: duration,
                started_at: None,
            };
            self.anim_queue.push_back(animation);
        }
    }

    // This should be called on a regular basis (e.g. through setInterval) to animate the cursor
    // position
    pub fn tick(&mut self) {
        let now = self.performance.now();
        // Empty the animation queue until we find the current animation
        loop {
            let anim = self.anim_queue.front_mut();
            match anim {
                Some(anim) => {
                    // This anim hasn't been started => let's start it
                    if anim.started_at.is_none() {
                        anim.started_at = Some(now);
                    }
                    let t = (now - anim.started_at.unwrap()) / anim.duration;
                    if t <= 1.0 {
                        // We are still within this animation => interpolate
                        let point = self.spline.interpolate_point(t + anim.start_index as f64);
                        self.current_position = point;
                        return;
                    } else {
                        // overdue animation => discard and move to next one
                        self.anim_queue.pop_front();
                    }
                }
                None => {
                    if self.spline.points.len() >= 1 {
                        let point = self.spline.points[self.spline.points.len() - 1];
                        self.current_position = (point.0, point.1);
                    }
                    return;
                }
            };
        }
    }
}
