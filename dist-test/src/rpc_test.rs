/*
use log::*;

use futures::future::Future;
// Helper
type RpcResult = Promise<(), ::capnp::Error>;

use dist_data::dist_capnp as dist_capnp;

pub use dist_data::dist_capnp::{
    test_interface,
};

#[derive(Debug)]
pub struct TestInterfaceImpl {
    data: String,
}

impl TestInterfaceImpl {
    pub fn new(data: &str, ) -> Self {
        Self { data: data.to_string() }
    }
}

impl test_interface::Server for TestInterfaceImpl {
    fn hello(&mut self, _: test_interface::HelloParams, mut result: test_interface::HelloResults) -> RpcResult {
        result.get().init_value().set_test_string(self.data.as_str());

        trace!("result entered");

        Promise::ok(())
    }
}



#[derive(Clone)]
struct ChannelWrapper {
    pub outbound_sender: crossbeam_channel::Sender<Vec<u8>>,
    pub outbound_receiver: crossbeam_channel::Receiver<Vec<u8>>,

    pub inbound_sender: crossbeam_channel::Sender<Vec<u8 >>,
    pub inbound_receiver: crossbeam_channel::Receiver<Vec<u8>>,

    pub direction: capnp_rpc::rpc_twoparty_capnp::Side,
}

impl ChannelWrapper {
    pub fn new(direction: capnp_rpc::rpc_twoparty_capnp::Side, ) -> Self {
        let inbound = crossbeam_channel::bounded(0);
        let outbound = crossbeam_channel::bounded(0);

        Self {
            outbound_sender: outbound.0,
            outbound_receiver: outbound.1,

            inbound_sender: inbound.0,
            inbound_receiver: inbound.1,

            direction
        }
    }

    pub fn as_server(&self, ) -> Self {
        let mut ret = self.clone();
        ret.direction = capnp_rpc::rpc_twoparty_capnp::Side::Server;
        ret
    }

    pub fn as_client(&self, ) -> Self {
        let mut ret = self.clone();
        ret.direction = capnp_rpc::rpc_twoparty_capnp::Side::Client;
        ret
    }
}

impl Default for ChannelWrapper {
    fn default() -> Self {
        let inbound = crossbeam_channel::bounded(64);
        let outbound = crossbeam_channel::bounded(64);

        Self {
            outbound_sender: outbound.0,
            outbound_receiver: outbound.1,

            inbound_sender: inbound.0,
            inbound_receiver: inbound.1,

            direction: capnp_rpc::rpc_twoparty_capnp::Side::Server,
        }
    }
}

impl std::io::Write for ChannelWrapper
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        info!("Write: {}", buf.len());
        match self.direction {
            capnp_rpc::rpc_twoparty_capnp::Side::Client => { self.inbound_sender.send(buf.to_vec()).unwrap(); },
            capnp_rpc::rpc_twoparty_capnp::Side::Server => { self.outbound_sender.send(buf.to_vec()).unwrap(); },
        }

        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl std::io::Read for ChannelWrapper
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        info!("Read: {}", buf.len());

        let stream = match self.direction {
            capnp_rpc::rpc_twoparty_capnp::Side::Client => { &self.outbound_receiver },
            capnp_rpc::rpc_twoparty_capnp::Side::Server => { &self.inbound_receiver },
        };

        match stream.try_recv() {
            Ok(data) => {
                if data.len() > buf.len() {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Not enough space in provided buffer to read message."));
                }

                buf.iter_mut().zip(data.iter()).for_each(|(place, element)| {
                    *place = *element;
                });

                Ok(data.len())
            },
            Err(_) => {
                // Try fails if its empty, just return
                Ok(0)
            },
        }
    }
}

fn main() {
    env_logger::init();

    use tokio::runtime::current_thread::Runtime;
    use tokio::prelude::*;

    use std::net::ToSocketAddrs;
    let a_addr = "127.0.0.1:1111".to_socket_addrs().unwrap().next().expect("could not parse address");
    let a_socket = ::tokio::net::TcpListener::bind(&a_addr).unwrap();

    let a_server = dist_capnp::test_interface::ToClient::new(TestInterfaceImpl::new("a_server")).into_client::<::capnp_rpc::Server>();

    let done = a_socket.incoming().for_each(move |socket| {
        socket.set_nodelay(true)?;
        let (reader, writer) = socket.split();

        let network =
            capnp_rpc::twoparty::VatNetwork::new(reader, writer,
                                                 capnp_rpc::rpc_twoparty_capnp::Side::Server, Default::default());

        let rpc_system = capnp_rpc::RpcSystem::new(Box::new(network), Some(a_server.clone().client));
        tokio::runtime::current_thread::spawn(rpc_system.map_err(|e| println!("error: {:?}", e)));
        Ok(())
    });

    tokio::runtime::current_thread::block_on_all(done);
}
*/