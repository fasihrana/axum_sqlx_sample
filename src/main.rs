mod dbpool;
mod message;

use axum::{
    routing::get,
    Router,
    debug_handler,
    response::IntoResponse,
    extract::State,
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

    let options = dbpool::ConnectOptions::new("postgres://frana@localhost/msgboard".to_string(), 5);
    let dbpool = match DBPool::from_options(&options).await {
        Ok(_dbpool) => { _dbpool },
        Err(err) => {
            println!("{:?}", err);
            std::process::exit(1);
        }
    };

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .with_state(dbpool);

    let addr = SocketAddr::from(([127,0,0,1],3000));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("Hello, world!");
}

#[debug_handler]
async fn root(State(state): State<DBPool>) -> impl IntoResponse {
    let msg_uuid_fut = state.add_message("hello".to_string()).await;

    match msg_uuid_fut {
        Ok(msg_uuid) => { return msg_uuid.to_string(); },
        Err(_err) => {
            println!("error received");
        },
    }

    return "no uuid".to_string();
}
