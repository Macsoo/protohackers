use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn is_prime(n: u64) -> bool {
    if n < 2 { return false }
    for i in (2..).take_while(|x| x * x <= n) {
        if n % i == 0 { return false }
    }
    true
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind("192.168.1.200:12730").await?;
    let prime = "{\"method\": \"isPrime\", \"prime\": true }\n".as_bytes();
    let not_prime = "{\"method\": \"isPrime\", \"prime\": false }\n".as_bytes();
    loop {
        let (mut stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            loop {
                let mut json = Vec::new();
                let mut buf = [0; 1];
                loop {
                    let Ok(n) = stream.read(&mut buf).await else { return };
                    if n == 0 { break }
                    if buf[0] == b'\n' { break }
                    json.push(buf[0]);
                }
                let Ok(request) = serde_json::from_slice::<serde_json::Value>(json.as_slice())
                    else { return };

                if Some("isPrime") != request.get("method")
                    .map(|v| v.as_str())
                    .flatten()
                { return };

                let Some(value) = request.get("number") else { return };
                if let serde_json::Value::Number(number) = value {
                    let Some(n) = number.as_u64()
                        .or_else(|| number.as_i64().map(|n| n as u64))
                        .or_else(|| number.as_f64().map(|n| n as u64))
                        else { unreachable!() };

                    stream.write_all(if is_prime(n).await { prime } else { not_prime }).await
                        .expect("Couldn't write.");
                } else { return }
            }
        });
    }
}
