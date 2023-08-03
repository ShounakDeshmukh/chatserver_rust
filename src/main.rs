use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:3459")
        .await
        .unwrap_or_else(|err| panic!("Problem binding to socket: {err:?}"));

    let (sender, _reciver) = broadcast::channel::<String>(2);

    loop {
        let sender = sender.clone();

        let mut reciver = sender.subscribe();

        let (mut stream, _socketaddr) = listener
            .accept()
            .await
            .unwrap_or_else(|err| panic!("Problem establising connection: {err:?}"));

        tokio::spawn(async move {
            let (read, mut writer) = stream.split();

            let mut line = String::new();

            let mut reader = BufReader::new(read);

            loop {
                let bytes_read = reader
                    .read_line(&mut line)
                    .await
                    .unwrap_or_else(|err| panic!("Problem reading stream: {err:?}"));

                if bytes_read == 0 {
                    break;
                }

                sender
                    .send(line.clone())
                    .unwrap_or_else(|err| panic!("Problem sending message: {err:?}"));

                let message = reciver
                    .recv()
                    .await
                    .unwrap_or_else(|err| panic!("Problem recieving message: {err:?}"));

                writer
                    .write_all(&message.as_bytes())
                    .await
                    .unwrap_or_else(|err| panic!("Problem writing to stream: {err:?}"));

                line.clear()
            }
        });
    }
}
