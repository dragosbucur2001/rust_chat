use s_lib::common::globals::ADDRESS;
use tokio::net::TcpStream;
use tokio::{
    io::{self, split},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listen = TcpListener::bind(ADDRESS)
        .await
        .expect("Cannot bind to address!");

    loop {
        match listen.accept().await {
            Ok((socket, _)) => {
                tokio::spawn(async move { handle_new_connection(socket).await });
            }
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
}

async fn handle_new_connection(socket: TcpStream) {
    let (mut rx, mut wx) = split(socket);

    if io::copy(&mut rx, &mut wx).await.is_err() {
        println!("couldn't send echo");
    } else {
        println!("all good");
    }
}
