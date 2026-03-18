use clap::Parser;
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
    
}

fn main() {
    println!("Hello, world!");
}
