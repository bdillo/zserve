use std::{fs, path::PathBuf};

use bytes::Bytes;
use clap::Parser;
use sha2::{Digest, Sha256};
use tracing::{info, level_filters::LevelFilter};
use zserve::server::Server;

#[derive(Debug, Parser)]
#[command(about = "Serves a file over HTTPS, discoverable via MDNS")]
struct Args {
    #[arg(short, long)]
    file: PathBuf,
    #[arg(short, long, default_value_t = 8443)]
    port: u16,
    #[arg(short, long, default_value_t = false)]
    debug: bool,
    #[arg(short, long, default_value = "zserve")]
    name: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();

    let log_level = match args.debug {
        true => LevelFilter::DEBUG,
        false => LevelFilter::INFO,
    };

    tracing_subscriber::fmt().with_max_level(log_level).init();

    let file_contents = Bytes::from(fs::read(&args.file).expect("Failed to read file!"));
    let file_hash = Sha256::digest(&file_contents);

    info!(
        "Serving file {} with SHA256 digest {:02x}",
        args.file.display(),
        file_hash
    );
    info!("Be sure to manually validate this after downloading the file!");

    let server = Server::new(&args.name, args.port);

    server
        .run(file_contents)
        .await
        .expect("Failed to run server!");
}
