use warp::Filter;
use warp::ws::WebSocket;
use warp::ws::Message;
use tokio::sync::mpsc;


async fn user_connected(ws: WebSocket) {
    use futures_util::StreamExt;

    let (ws_sender, mut ws_receiver) = ws.split();

    let send_channel = create_send_channel(ws_sender);

    let my_id = send_welcome(&send_channel).await;

    log::debug!("new user connected: {}", my_id);

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
 
async fn send_welcome(out: &OutBoundChannel) -> usize {
    unimplemented!()
}

 
#[tokio::main]
async fn main() {
    pretty_env_logger::init();
 
    // added'HTTP' game route, follows by a websocket upgrade
    let game = warp::path("game")
    .and(warp::ws())
    .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(move |socket| {
                    user_connected(socket)
            })
    });

    let status = warp::path!("status").map(move || warp::reply::html("<h1>hello</h1>"));

    let routes = status.or(game);
 
    warp::serve(status).run(([0, 0, 0, 0], 3030)).await;
}