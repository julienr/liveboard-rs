use log;
use std::f64;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use yew::{html, Component, Context, Html, NodeRef};

#[derive(Debug)]
pub enum Msg {
    Draw,
}

pub struct Board {
    canvas_ref: NodeRef,
}

impl Component for Board {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::Draw);
        Self {
            canvas_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Draw => {
                let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
                self.draw(&canvas);
                false
            }
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            // Good example with more complex mouse state tracking
            // https://rustwasm.github.io/wasm-bindgen/examples/paint.html
            let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                log::info!("MouseDown !");
            }) as Box<dyn FnMut(_)>);
            canvas
                .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div style="margin: 10px; width: 500px; height: 500px;">
                <canvas
                    ref={self.canvas_ref.clone()}
                    height="500"
                    width="500"
                    style="border: 1px dashed gray; height:100%; width: 100%;" />
            </div>
        }
    }
}

impl Board {
    fn draw(&self, canvas: &HtmlCanvasElement) {
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
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
}
