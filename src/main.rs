use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:3459")
        .await
        .unwrap_or_else(|err| panic!("Problem binding to socket: {err:?}"));
    loop {
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
                writer
                    .write_all(&line.as_bytes())
                    .await
                    .unwrap_or_else(|err| panic!("Problem writing to stream: {err:?}"));
                line.clear()
            }
        });
    }
}
