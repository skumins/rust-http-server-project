use std::net::SocketAddr;
use::axum::Router;
use tokio::net::TcpListener;

mod routers;

#[tokio::main]
async fn main() {
    let app: Router = routers::create_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
