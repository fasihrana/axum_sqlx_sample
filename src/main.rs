mod dbpool;
mod message;

use axum::{
    routing::get,
    Router,
    debug_handler,
    response::IntoResponse
};
use std::net::SocketAddr;

use dbpool::{
    DBPool,
    ModError
};

#[tokio::main]
async fn main() {

    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    let addr = SocketAddr::from(([127,0,0,1],3000));

    tracing::debug!("listening on {}", addr);

    let options = dbpool::ConnectOptions::new("postgres://frana@localhost/msgboard", 5);
    DBPool::connect_options(Some(options));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("Hello, world!");
}

#[debug_handler]
async fn root() -> impl IntoResponse {
    let msg_uuid_fut = DBPool::add_message("hello".to_string()).await;

    match msg_uuid_fut {
        Ok(msg_uuid) => { return msg_uuid.to_string(); },
        Err(_err) => {
            println!("error received");
        },
    }

    return "no uuid".to_string();
}
