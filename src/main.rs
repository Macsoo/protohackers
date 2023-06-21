use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind("192.168.1.200:12730").await?;
    loop {
        let (mut stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut prices = Vec::new();
            loop {
                let mut buf = [0; 9];
                if matches!(stream.read_exact(&mut buf).await, Err(_)) { return };
                if buf[0] == b'I' {
                    let timestamp: i32 = (&buf[1..5]).try_into().map(|x| i32::from_be_bytes(x)).unwrap();
                    let price: i32 = (&buf[5..9]).try_into().map(|x| i32::from_be_bytes(x)).unwrap();
                    prices.push((timestamp, price));
                    continue;
                } else if buf[0] == b'Q' {
                    let min: i32 = (&buf[1..5]).try_into().map(|x| i32::from_be_bytes(x)).unwrap();
                    let max: i32 = (&buf[5..9]).try_into().map(|x| i32::from_be_bytes(x)).unwrap();
                    let mut sum = 0i64;
                    let mut n = 0;
                    let mut result = 0;
                    if max >= min {
                        for (timestamp, price) in prices.iter() {
                            if *timestamp > max { continue }
                            if *timestamp < min { continue }
                            sum += *price as i64;
                            n += 1;
                        }
                    }
                    if n > 0 {
                        result = sum / n;
                    }
                    let bytes: [u8; 4] = (result as i32).to_be_bytes();
                    stream.write_all(&bytes).await.expect("Couldn't write.");
                } else { return }
            }
        });
    }
}
