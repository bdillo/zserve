use std::{fs, net::IpAddr, path::PathBuf};

use bytes::Bytes;
use clap::Parser;
use zserve::server::Server;

#[derive(Debug, Parser)]
#[command(about = "Serves a file over HTTPS, discoverable via MDNS")]
struct Args {
    #[arg(short, long)]
    file: PathBuf,
    #[arg(short, long, default_value_t = 8443)]
    port: u16,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();

    env_logger::init();

    // TODO: move to clap args
    let ip_addr: IpAddr = "192.168.1.4".parse().expect("Failed to parse ip addr!");

    let file_contents = Bytes::from(fs::read(args.file).expect("Failed to read file!"));

    let server = Server::new(ip_addr, args.port);

    server
        .run(file_contents)
        .await
        .expect("Failed to run server!");
}
