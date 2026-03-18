use clap::{Parser, error};
use futures::StreamExt;
use libp2p::{
    Multiaddr, PeerId, StreamProtocol,
    bytes::Bytes,
    noise,
    request_response::{self, ProtocolSupport, cbor},
    swarm::{self, NetworkBehaviour, SwarmEvent},
    tcp, yamux,
};
use serde::{Deserialize, Serialize};
use std::{error::Error, time::Duration};
use tokio::{
    fs::File,
    io::{self, AsyncBufReadExt, AsyncReadExt, BufReader, stdin},
    select,
};

#[derive(Parser)]
#[clap(name = "libp2p request response example")]
struct Cli {
    #[arg(long)]
    port: Option<String>,

    #[arg(long)]
    peer: Option<Multiaddr>,
}

#[derive(NetworkBehaviour)]
struct ReqResBehaviour {
    request_response: request_response::cbor::Behaviour<FileRequest, FileResponse>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileRequest(String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileResponse(Vec<u8>);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|_key| ReqResBehaviour {
            request_response: request_response::cbor::Behaviour::new(
                [(
                    StreamProtocol::new("/file-exchange/1"),
                    ProtocolSupport::Full,
                )],
                request_response::Config::default(),
            ),
        })?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(7200)))
        .build();

    let listen_port = cli.port.unwrap_or("0".to_string());
    let mutiaddr = format!("/ip4/0.0.0.0/tcp/{listen_port}");
    swarm.listen_on(mutiaddr.parse()?)?;

    if let Some(peer) = cli.peer {
        swarm.dial(peer)?;
    }

    let mut stdin = BufReader::new(stdin()).lines();

    let mut other_peer_id: Option<PeerId> = None;

    loop {
         select! {

            Ok(Some(line)) = stdin.next_line() => {
                if let Some(peer_id) = other_peer_id {
                    swarm.behaviour_mut().request_response.send_request(
                        &peer_id,
                        FileRequest(line)
                     );
                }
            }
            event = swarm.select_next_some() => match event {
                SwarmEvent::NewListenAddr {
                    address, ..
                } => {
                    println!("Listening on {address}");
                }
                SwarmEvent::ConnectionEstablished {peer_id, ..} =>{
                    other_peer_id = Some(peer_id);
                    println!("Established connection {:?}", peer_id);
                }
                SwarmEvent::Behaviour(ReqResBehaviourEvent::RequestResponse(
                    request_response::Event::Message {
                        message, 
                        .. 
                    }
                )) => match message {
                    request_response::Message::Request {request, channel, .. } => {
                        println!("Request {:?}", request);
                    }
                    request_response::Message::Response { response, ..} => {
                        println!("Response {:?}", response);
                    }
                }

                _ => {},
            }

        }
    }

    Ok(())
}
