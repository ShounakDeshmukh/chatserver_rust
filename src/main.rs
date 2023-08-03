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

    let (sender, _reciver) = broadcast::channel(2);

    loop {
        let sender = sender.clone();

        let mut reciver = sender.subscribe();

        let (mut stream, socketaddr) = listener
            .accept()
            .await
            .unwrap_or_else(|err| panic!("Problem establising connection: {err:?}"));

        tokio::spawn(async move {
            let (read, mut writer) = stream.split();

            let mut line = String::new();

            let mut reader = BufReader::new(read);

            loop {
                tokio::select! {

                bytes_read=reader.read_line(&mut line) => {

                    if bytes_read.unwrap() == 0 {
                        break;
                    }
                    sender.send((line.clone(),socketaddr)).unwrap_or_else(|err| panic!("Problem sending message: {err:?}"));

                    line.clear()
                    }

                 response= reciver.recv()=>{

                    let (message, partner_addr) = response.unwrap();
                    let user =socketaddr.port().to_string() + " says: " + &message;
                    if socketaddr != partner_addr {
                    writer
                    .write_all(&user.as_bytes())
                    .await
                    .unwrap_or_else(|err| panic!("Problem writing to stream: {err:?}"));

                    }
                }

                }
            }
        });
    }
}
