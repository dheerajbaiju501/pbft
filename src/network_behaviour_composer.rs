use crate::behavior::{Pbft, PbftEvent};
use libp2p::mdns::{Mdns, MdnsEvent};
use libp2p::swarm::NetworkBehaviourEventProcess;

use tokio::prelude::{AsyncRead, AsyncWrite};

/*
#[derive(NetworkBehaviour)]
*/
pub struct NetworkBehaviourComposer<TSubstream: AsyncRead + AsyncWrite> {
    mdns: Mdns,
    pub pbft: Pbft<TSubstream>,
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourComposer<TSubstream> {
    pub fn new(mdns: Mdns, pbft: Pbft<TSubstream>) -> Self {
        Self { mdns, pbft }
    }
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<MdnsEvent>
    for NetworkBehaviourComposer<TSubstream>
{
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer_id, address) in list {
                    if !self.pbft.has_peer(&peer_id) {
                        println!("[NetworkBehaviourComposer::inject_event] [MdnsEvent::Discovered] The node has been discovered: {:?}", address);
                        self.pbft.add_peer(&peer_id, &address);
                    }
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer_id, addr) in list {
                    if self.pbft.has_peer(&peer_id) {
                        println!("[NetworkBehaviourComposer::inject_event] [MdnsEvent::Expired] The node has been expired: {:?}", addr);
                        // TODO
                    }
                }
            }
        }
    }
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<PbftEvent>
    for NetworkBehaviourComposer<TSubstream>
{
    fn inject_event(&mut self, event: PbftEvent) {
        println!("inject_event : PbftEvent: {:?}", event);
    }
}
