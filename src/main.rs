mod state;

use axum::routing::get;
use socketioxide::{
    extract::{Data, SocketRef, State},
    SocketIo,
};
use tower_http::{cors::CorsLayer};

use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

use crate::state::Message;


#[derive(serde::Serialize)]
struct Messages {
    messages: Vec<state::Message>,
}


async fn on_connect(socket: SocketRef ) {
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
    socket.join("1").ok();
    socket.within("1").emit("consume", format!("connect-{}", socket.id)).ok();

    socket.on(
        "move",
        |socket: SocketRef, Data::<String>(data) ,store: State<state::MessageStore>| async move {
            info!("Received event: {:?} ", data);
            
            store.insert("1" , Message {
                data: data,
                user: socket.id.to_string(),
            }).await;
            let messages = store.get("1").await;
            socket.within("1").emit("consume", Messages {messages}).ok();
        },
    );

    socket.on(
        "emoji",
        |socket: SocketRef, Data::<String>(data)| async move {
            info!("Received event: {:?} ", data);
            socket.within("1").emit("consume", format!("emoji-{}@{}", socket.id, data)).ok();
        },
    );

    socket.on_disconnect(|socket: SocketRef , store: State<state::MessageStore>| async move{
        store.remove("1", socket.id.to_string()).await;
        socket.broadcast().emit("consume", format!("exit-{}" , socket.id)).ok();
    });

}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let messages = state::MessageStore::default();

    let (layer, io) = SocketIo::builder().with_state(messages).build_layer();

    io.ns("/", on_connect);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(io)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive()) 
                .layer(layer)
        );

    info!("Starting server");

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}