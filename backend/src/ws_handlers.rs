use actix::{Actor, Addr, AsyncContext, Handler, Message as ActixMessage, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Mutex;

pub struct State {
    pub clients: Mutex<Vec<Addr<WsActor>>>,
}

pub fn make_state() -> State {
    return State {
        clients: Mutex::new(Vec::new()),
    };
}

// Used to communicate between WS actors
#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Message(pub String);

pub struct WsActor {
    state: Arc<State>,
}

impl Actor for WsActor {
    type Context = ws::WebsocketContext<Self>;
}

impl Handler<Message> for WsActor {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsActor {
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("started: {:p} {:?}", self, ctx.address());
        self.state
            .as_ref()
            .clients
            .lock()
            .unwrap()
            .push(ctx.address());
    }

    fn finished(&mut self, ctx: &mut Self::Context) {
        println!("finished: {:?}", ctx.address());
        let mut clients = self.state.as_ref().clients.lock().unwrap();
        let index = clients.iter().position(|a| *a == ctx.address()).unwrap();
        clients.swap_remove(index);
    }

    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // TODO: Implement heartbeat ?
        // https://agmprojects.com/blog/building-a-rest-and-web-socket-api-with-actix.html
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                println!("ping");
                ctx.pong(&msg)
            }
            Ok(ws::Message::Pong(_)) => {
                println!("pong");
            }
            Ok(ws::Message::Text(text)) => {
                // Broadcast to all clients but ourselves
                let clients = self.state.as_ref().clients.lock().unwrap();
                println!(
                    "Received text: {}, broadcasting to {} clients",
                    text,
                    clients.len()
                );
                for client in clients.iter() {
                    if *client == ctx.address() {
                        continue;
                    }
                    client.do_send(Message(text.to_string()));
                }
                // TODO: Ack message ?
                //ctx.text(format!("{} response from", text))
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub async fn index(
    data: web::Data<State>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    println!("New websocket connection");
    let resp = ws::start(
        WsActor {
            state: data.deref().clone(),
        },
        &req,
        stream,
    );
    resp
}
