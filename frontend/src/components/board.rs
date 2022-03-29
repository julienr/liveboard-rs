use super::ws_client::{new_ws_client, WSClient};
use futures::SinkExt;
use log;
use reqwasm::websocket::Message as WsMessage;
use shared::datatypes::{Circle, Color, PointerPosition, SocketMessage};
use std::collections::HashMap;
use std::f64;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;
use yew::{html, html::Scope, Component, Context, Html, NodeRef};

#[derive(Debug)]
pub enum Msg {
    Draw,
    ButtonPressed,
    ButtonReleased(i32, i32),
    MouseMove(i32, i32),
    NewCircle(Circle),
    OtherPointerMoved(PointerPosition),
}

pub struct Board {
    canvas_ref: NodeRef,
    button_pressed: bool,
    circles: Vec<Circle>,
    other_pointers: HashMap<String, PointerPosition>,
    client: WSClient,
    color: Color,
    id: String,
    last_pointer_update: f64,
    performance: web_sys::Performance,
}

impl Component for Board {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let window = web_sys::window().unwrap();
        let crypto = window.crypto().unwrap();
        let scope = ctx.link().clone();
        let client = new_ws_client(move |message: WsMessage| match message {
            WsMessage::Text(value) => {
                // log::info!("String message {}", value);
                let m: SocketMessage = serde_json::from_str(&value).unwrap();
                match m {
                    SocketMessage::Circle(circle) => {
                        log::info!("circle message {:?}", circle);
                        scope.send_message(Msg::NewCircle(circle));
                    }
                    SocketMessage::Pointer(pointer_position) => {
                        log::info!("pointer update {:?}", pointer_position);
                        scope.send_message(Msg::OtherPointerMoved(pointer_position));
                    }
                }
            }
            WsMessage::Bytes(_value) => {
                log::info!("Bytes message");
            }
        });
        ctx.link().send_message(Msg::Draw);
        let mut tmp = [0u8, 0u8, 0u8];
        crypto.get_random_values_with_u8_array(&mut tmp).unwrap();
        let color = Color {
            r: tmp[0],
            g: tmp[1],
            b: tmp[2],
        };
        let performance = window
            .performance()
            .expect("window.performance should be available");
        Self {
            canvas_ref: NodeRef::default(),
            button_pressed: false,
            circles: Vec::new(),
            other_pointers: HashMap::new(),
            client: client,
            color: color,
            id: color.hex_color(),
            last_pointer_update: performance.now(),
            performance: performance,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NewCircle(circle) => {
                self.circles.push(circle);
                ctx.link().send_message(Msg::Draw);
                false
            }
            Msg::Draw => {
                let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
                canvas.set_width(
                    web_sys::window()
                        .unwrap()
                        .inner_width()
                        .unwrap()
                        .as_f64()
                        .unwrap() as u32,
                );
                canvas.set_height(
                    web_sys::window()
                        .unwrap()
                        .inner_height()
                        .unwrap()
                        .as_f64()
                        .unwrap() as u32,
                );
                self.draw_smiley(&canvas);
                self.draw_circles(&canvas);
                self.draw_pointers(&canvas);
                true
            }
            Msg::ButtonPressed => {
                self.button_pressed = true;
                false
            }
            Msg::ButtonReleased(x, y) => {
                if self.button_pressed {
                    let circle = Circle {
                        x: x as f64,
                        y: y as f64,
                        radius: 5.0,
                        color: self.color,
                    };
                    let mut client = self.client.clone();
                    let circle2 = circle.clone();
                    ctx.link().send_future(async move {
                        let m = SocketMessage::Circle(circle2);
                        let jsonval = serde_json::to_string(&m).unwrap();
                        client
                            .sender
                            .send(WsMessage::Text(String::from(jsonval)))
                            .await
                            .unwrap();
                        // TODO: This is not really needed
                        Msg::Draw
                    });
                    self.circles.push(circle);
                    // Trigger a redraw
                    ctx.link().send_message(Msg::Draw);
                }
                self.button_pressed = false;
                false
            }
            Msg::MouseMove(x, y) => {
                let curr_time = self.performance.now();
                if (curr_time - self.last_pointer_update) > 200.0 {
                    self.last_pointer_update = curr_time;
                    let pointer_pos = PointerPosition {
                        id: self.id.clone(),
                        x: x as f64,
                        y: y as f64,
                        color: self.color,
                    };
                    let mut client = self.client.clone();
                    ctx.link().send_future(async move {
                        let m = SocketMessage::Pointer(pointer_pos);
                        let jsonval = serde_json::to_string(&m).unwrap();
                        client
                            .sender
                            .send(WsMessage::Text(String::from(jsonval)))
                            .await
                            .unwrap();
                        Msg::Draw
                    });
                }
                false
            }
            Msg::OtherPointerMoved(pointer_position) => {
                self.other_pointers
                    .insert(pointer_position.id.clone(), pointer_position);
                ctx.link().send_message(Msg::Draw);
                false
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            // Good example with more complex mouse state tracking
            // https://rustwasm.github.io/wasm-bindgen/examples/paint.html
            self.add_canvas_event_listener(
                ctx,
                "mousedown",
                move |_event: web_sys::MouseEvent, scope| {
                    log::info!("MouseDown !");
                    scope.send_future(async { Msg::ButtonPressed })
                },
            );
            self.add_canvas_event_listener(
                ctx,
                "mouseup",
                move |event: web_sys::MouseEvent, scope| {
                    log::info!("MouseUp !");
                    let x = event.offset_x();
                    let y = event.offset_y();
                    scope.send_future(async move { Msg::ButtonReleased(x, y) })
                },
            );
            self.add_canvas_event_listener(
                ctx,
                "mousemove",
                move |event: web_sys::MouseEvent, scope| {
                    let x = event.offset_x();
                    let y = event.offset_y();
                    scope.send_future(async move { Msg::MouseMove(x, y) })
                },
            );
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let style = format!(
            "width: 100%; height: 10px; background-color: {}",
            self.color.hex_color()
        );
        html! {
            <div>
                <div style="position: absolute; bottom: 0; left: 0; margin: 5px;">
                    <p>{ self.circles.len() } { " circles" } </p>
                </div>
                <div { style }></div>
                <canvas
                    ref={self.canvas_ref.clone()}
                    height="500"
                    width="500"
                     />
            </div>
        }
    }
}

impl Board {
    fn get_context(&self, canvas: &HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
        return canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
    }
    fn draw_smiley(&self, canvas: &HtmlCanvasElement) {
        let context = self.get_context(canvas);
        context.begin_path();

        // Draw the outer circle.
        context
            .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        // Draw the mouth.
        context.move_to(110.0, 75.0);
        context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

        // Draw the left eye.
        context.move_to(65.0, 65.0);
        context
            .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        // Draw the right eye.
        context.move_to(95.0, 65.0);
        context
            .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        context.stroke();
    }
    fn draw_circles(&self, canvas: &HtmlCanvasElement) {
        let context = self.get_context(canvas);

        for circle in &self.circles {
            context.set_fill_style(&JsValue::from_str(&circle.color.hex_color()));
            context.begin_path();
            context
                .arc(
                    circle.x,
                    circle.y,
                    circle.radius,
                    0.0,
                    f64::consts::PI * 2.0,
                )
                .unwrap();
            context.fill();
        }
    }

    fn draw_pointers(&self, canvas: &HtmlCanvasElement) {
        const size: f64 = 20.0;
        let context = self.get_context(canvas);
        for (_, pointer_position) in &self.other_pointers {
            context.set_fill_style(&JsValue::from_str(&pointer_position.color.hex_color()));
            context.begin_path();
            context.move_to(pointer_position.x, pointer_position.y);
            context.line_to(pointer_position.x + size, pointer_position.y);
            context.line_to(pointer_position.x, pointer_position.y + size / 2.0);
            context.fill();
        }
    }

    fn add_canvas_event_listener<F>(&self, ctx: &Context<Self>, event: &str, cb: F)
    where
        F: 'static + Fn(web_sys::MouseEvent, &Scope<Board>) -> (),
    {
        let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
        let scope = ctx.link().clone();
        let closure = Closure::wrap(Box::new(move |a| cb(a, &scope)) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
}
