use crate::db;
use actix::{Actor, Addr, AsyncContext, Handler, Message as ActixMessage, StreamHandler};
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use shared::datatypes::{Shape, SocketMessage};
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Mutex;

pub struct State {
    pub clients: Mutex<Vec<Addr<WsActor>>>,
}

pub fn make_state() -> State {
    State {
        clients: Mutex::new(Vec::new()),
    }
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Message(pub String);

pub struct WsActor {
    board_id: i32,
    state: Arc<State>,
    db_state: Arc<db::State>,
}

// Broadcast to all clients but ourselves
fn broadcast(state: &State, ctx: &mut <WsActor as Actor>::Context, msg: &str) {
    let clients = state.clients.lock().unwrap();
    log::debug!(
        "Received text: {}, broadcasting to {} clients",
        msg,
        clients.len()
    );
    for client in clients.iter() {
        if *client == ctx.address() {
            continue;
        }
        client.do_send(Message(msg.to_string()));
    }
}

async fn parse_and_persist(client: db::Client, msg: &str, board_id: i32) {
    // Parse and decide if needs to be persisted
    let m: SocketMessage = serde_json::from_str(msg).unwrap();
    match m {
        SocketMessage::Circle(circle) => {
            log::info!("Persisting circle for {:?}", board_id);
            db::create_shape(&client, Shape::Circle(circle), board_id)
                .await
                .unwrap();
        }
        _ => {}
    }
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
                let text2 = text.clone();
                let pool = self.db_state.pool.clone();
                let board_id = self.board_id;
                let fut = async move {
                    let client = pool.get().await.unwrap();
                    parse_and_persist(client, &text, board_id).await;
                };
                let fut = actix::fut::wrap_future::<_, Self>(fut);
                ctx.spawn(fut);
                broadcast(self.state.as_ref(), ctx, &text2);
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

#[get("/boards/{id}/ws")]
pub async fn ws_for_board(
    ws_data: web::Data<State>,
    db_data: web::Data<db::State>,
    path: web::Path<(i32,)>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    println!("New websocket connection");
    let board_id = path.0;
    let resp = ws::start(
        WsActor {
            board_id,
            state: ws_data.deref().clone(),
            db_state: db_data.deref().clone(),
        },
        &req,
        stream,
    );
    resp
}
