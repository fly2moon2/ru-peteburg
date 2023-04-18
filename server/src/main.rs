use warp::Filter;
use warp::ws::WebSocket;
use warp::ws::Message;
use tokio::sync::mpsc;
use crate::app::comm::ServerMessage;
pub mod app; // declared in \types\mod.rs

async fn user_connected(ws: WebSocket, users: Users) {
    use futures_util::StreamExt;

    let (ws_sender, mut ws_receiver) = ws.split();

    let send_channel = create_send_channel(ws_sender);

    let my_id = send_welcome(&send_channel).await;

    log::debug!("new user connected: {}", my_id);
 
    users.write().await.insert(my_id, send_channel);

    while let Some(result) = ws_receiver.next().await {
            let msg = match result {
                    Ok(msg) => msg,
                    Err(e) => {
                            log::warn!("websocket receive error: '{}'", e);
                            break;
                    }
            };

            log::debug!("user sent message: {:?}", msg);
    }

    log::debug!("user disconnected: {}", my_id);
 
    users.write().await.remove(&my_id);
 
    broadcast(ServerMessage::GoodBye(my_id), &users).await;

}

async fn broadcast(msg: ServerMessage, users: &Users) {
        unimplemented!()
}


fn create_send_channel(ws_sender: futures_util::stream::SplitSink<WebSocket, Message>) -> OutBoundChannel  {
    
        use futures_util::StreamExt;
        use futures_util::FutureExt;
        use tokio_stream::wrappers::UnboundedReceiverStream;
     
        let (sender, receiver) = mpsc::unbounded_channel();
        let rx = UnboundedReceiverStream::new(receiver);
     
    
        tokio::task::spawn(rx.forward(ws_sender).map(|result| {
                if let Err(e) = result {
                        log::error!("websocket send error: {}", e);
                }
        }));
     
        sender
}

type OutBoundChannel =
        mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>;
 

use std::sync::atomic::{AtomicUsize, Ordering};
    
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);
 
async fn send_welcome(out: &OutBoundChannel) -> usize {
        // unimplemented!()
        let id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);
 
        let states = ServerMessage::Welcome(id);
 
        send_msg(out, &states).await;
 
        id
} 
  
async fn send_msg(tx: &OutBoundChannel, msg: &ServerMessage) {
    unimplemented!()
}
 
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
 
type Users = Arc<RwLock<HashMap<usize, OutBoundChannel>>>;
 
#[tokio::main]
async fn main() {
    pretty_env_logger::init();

 
        let users = Users::default();

        let users = warp::any().map(move || users.clone());

        // added'HTTP' game route, follows by a websocket upgrade
        let game = warp::path("game")
                .and(warp::ws())
                .and(users)
                .map(|ws: warp::ws::Ws, users| {
                        ws.on_upgrade(move |socket| {
                                user_connected(socket, users)
                        })
                });

    let status = warp::path!("status").map(move || warp::reply::html("<h1>hello</h1>"));

    let routes = status.or(game);
 
    warp::serve(status).run(([0, 0, 0, 0], 3030)).await;
}