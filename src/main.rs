use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind("192.168.1.200:12730").await?;
    loop {
        let (mut stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let n = stream.read(&mut buf).await.expect("Couldn't read");
                if n == 0 { break; }
                stream.write_all(&buf[0..n]).await.expect("Couldn't write");
            }
        });
    }
}
