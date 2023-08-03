use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:3459")
        .await
        .unwrap_or_else(|err| panic!("Problem binding to socket: {err:?}"));

    let (stream, _socketaddr) = listener
        .accept()
        .await
        .unwrap_or_else(|err| panic!("Problem reading stream: {err:?}"));
}
