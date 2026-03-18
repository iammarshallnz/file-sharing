use clap::{Parser, error};
use std::{error::Error, time::Duration};
use libp2p::{
    noise,
    request_response::{self,ProtocolSupport},
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, Multiaddr,PeerId, StreamProtocol
};
use serde::{Deserialize, Serialize};
use futures::StreamExt;
use tokio::{
    io::{self, stdin,AsyncBufReadExt, BufReader, AsyncReadExt},
    select,
    fs::File
};

#[derive(Parser)]
#[clap(name = "libp2p request response example")]
struct Cli {
    #[arg(long)]
    port: Option<String>,


    #[arg(long)]
    peer: Option<Multiaddr>,

}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let cli = Cli::parse();
    println!("Hello, world!");

    Ok(())

}
