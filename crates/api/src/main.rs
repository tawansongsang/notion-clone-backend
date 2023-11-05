use database::add;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let result = add(200, 50);
    println!("{}", result);

    // initialize tracing
    // tracing_subscriber::fmt::init();

    let config = api::config::Config::build().unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port.clone()));
    // build our application with a route
    let app = api::app::create_app(config);

    // run our app with hyper, listening globally on port 3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
