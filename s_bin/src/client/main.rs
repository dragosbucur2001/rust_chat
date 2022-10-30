use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use s_lib::common::globals::ADDRESS;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpStream::connect(ADDRESS).await.expect("Cannot connect to server");
    let (mut rd, mut wr) = io::split(socket);

    // Write data in the background
    tokio::spawn(async move {
        wr.write_all(b"hello\r\n").await?;
        wr.write_all(b"world\r\n").await?;
        wr.write(b"").await?;
        wr.shutdown().await?;

        // Sometimes, the rust type inferencer needs
        // a little help
        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];

    loop {
        let n = rd.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        println!("GOT {:?}", &buf[..n]);
    }

    Ok(())
}
