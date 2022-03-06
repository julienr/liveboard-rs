use super::ws_client::{new_ws_client, WSClient};
use futures::SinkExt;
use log;
use reqwasm::websocket::Message as WsMessage;
use shared::datatypes::Circle;
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
    ButtonReleased,
    MouseMove(i32, i32),
}

pub struct Board {
    canvas_ref: NodeRef,
    button_pressed: bool,
    circles: Vec<Circle>,
    client: WSClient,
}

impl Component for Board {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // let scope = ctx.link().clone();
        let client = new_ws_client(move |message: WsMessage| match message {
            WsMessage::Text(value) => {
                log::info!("String message {}", value);
                // scope.send_message(Msg::MessageReceived(value));
            }
            WsMessage::Bytes(_value) => {
                log::info!("Bytes message");
            }
        });
        ctx.link().send_message(Msg::Draw);
        Self {
            canvas_ref: NodeRef::default(),
            button_pressed: false,
            circles: Vec::new(),
            client: client,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Draw => {
                let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
                self.draw_smiley(&canvas);
                self.draw_circles(&canvas);
                log::info!("draw");
                false
            }
            Msg::ButtonPressed => {
                self.button_pressed = true;
                false
            }
            Msg::ButtonReleased => {
                self.button_pressed = false;
                false
            }
            Msg::MouseMove(x, y) => {
                if self.button_pressed {
                    log::info!("MouseMove ! {} {}", x, y);
                    let circle = Circle {
                        x: x as f64,
                        y: y as f64,
                        radius: 5.0,
                    };
                    let mut client = self.client.clone();
                    let circle2 = circle.clone();
                    ctx.link().send_future(async move {
                        let jsonval = serde_json::to_string(&circle2).unwrap();
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
                move |_event: web_sys::MouseEvent, scope| {
                    log::info!("MouseUp !");
                    scope.send_future(async { Msg::ButtonReleased })
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

        context.set_fill_style(&JsValue::from_str("blue"));
        for circle in &self.circles {
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
