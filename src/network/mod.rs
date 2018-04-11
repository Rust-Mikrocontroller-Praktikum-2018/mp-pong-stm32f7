use smoltcp;
use smoltcp::iface::EthernetInterface;
use smoltcp::socket::{Socket, SocketSet};
use smoltcp::socket::{UdpPacketMetadata, UdpSocket, UdpSocketBuffer};
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, IpAddress, IpEndpoint, Ipv4Address};

mod packets;
pub use self::packets::BallPacket;
pub use self::packets::GamestatePacket;
pub use self::packets::InputPacket;
pub use self::packets::RacketPacket;
pub use self::packets::WhoamiPacket;
use self::packets::Serializable;

use alloc::Vec;
use board;
use embedded;
use ethernet;
use system_clock;

const PORT: u16 = 2018;

pub struct Network {
    ethernet_interface: EthernetInterface<'static, 'static, ethernet::EthernetDevice>,
    sockets: SocketSet<'static, 'static, 'static>,
    partner_ip_addr: Ipv4Address,
}

impl Network {
    pub fn get_udp_packet(&mut self) -> Result<Option<Vec<u8>>, smoltcp::Error> {
        match self.ethernet_interface.poll(
            &mut self.sockets,
            Instant::from_millis(system_clock::ticks() as i64),
        ) {
            Err(e) => Err(e),
            Ok(socket_changed) => if socket_changed {
                // let mut socket = &mut self.sockets.iter_mut().nth(0).unwrap();
                for mut socket in self.sockets.iter_mut() {
                    return Network::poll_udp_packet(&mut socket);
                }
                Ok(None)
            } else {
                Ok(None)
            },
        }
    }

    fn poll_udp_packet(socket: &mut Socket) -> Result<Option<Vec<u8>>, smoltcp::Error> {
        match socket {
            &mut Socket::Udp(ref mut socket) => match socket.recv() {
                Ok((data, _remote_endpoint)) => Ok(Some(Vec::from(data))),
                Err(err) => Err(err),
            },
            _ => Ok(None),
        }
    }

    pub fn send_udp_packet(&mut self, data: &[u8]) {
        let endpoint = IpEndpoint::new(IpAddress::Ipv4(self.partner_ip_addr), PORT);
        for mut socket in self.sockets.iter_mut() {
            Network::push_udp_packet(&mut socket, endpoint, data);
        }
    }

    fn push_udp_packet(socket: &mut Socket, endpoint: IpEndpoint, data: &[u8]) {
        match socket {
            &mut Socket::Udp(ref mut socket) => {
                let _result = socket.send_slice(data, endpoint); // TODO: Error handling
            }
            _ => {}
        }
    }
}

pub fn init(
    rcc: &mut board::rcc::Rcc,
    syscfg: &mut board::syscfg::Syscfg,
    ethernet_mac: &'static mut board::ethernet_mac::EthernetMac,
    ethernet_dma: &'static mut board::ethernet_dma::EthernetDma,
    gpio: &mut embedded::interfaces::gpio::Gpio,
    ethernet_addr: EthernetAddress,
    ip_addr: Ipv4Address,
    partner_ip_addr: Ipv4Address,
) -> Result<Network, ethernet::Error> {
    // Ethernet init
    let ethernet_interface = ethernet::EthernetDevice::new(
        Default::default(),
        Default::default(),
        rcc,
        syscfg,
        gpio,
        ethernet_mac,
        ethernet_dma,
        ethernet_addr,
    ).map(|device| device.into_interface(ip_addr));
    if let Err(e) = ethernet_interface {
        //hprintln!("ethernet init failed: {:?}", e);
        //return None;
        return Err(e);
    }

    let mut sockets = SocketSet::new(Vec::new());
    let endpoint = IpEndpoint::new(IpAddress::Ipv4(ip_addr), PORT);

    let udp_rx_buffer = UdpSocketBuffer::new(vec![UdpPacketMetadata::EMPTY; 3], vec![0u8; 256]);
    let udp_tx_buffer = UdpSocketBuffer::new(vec![UdpPacketMetadata::EMPTY; 1], vec![0u8; 128]);
    let mut udp_socket = UdpSocket::new(udp_rx_buffer, udp_tx_buffer);
    udp_socket.bind(endpoint).unwrap();
    sockets.add(udp_socket);

    Ok(Network {
        ethernet_interface: ethernet_interface.unwrap(),
        sockets: sockets,
        partner_ip_addr: partner_ip_addr,
    })
}

pub trait Client {
    fn send_input(&mut self, network: &mut Network, input: &InputPacket);
    fn receive_gamestate(&mut self, network: &mut Network) -> GamestatePacket;
    fn is_server_connected(&mut self, network: &mut Network) -> bool;
    fn send_whoami(&mut self, network: &mut Network);
}

pub trait Server {
    fn receive_input(&mut self, network: &mut Network) -> InputPacket;
    fn send_gamestate(&mut self, network: &mut Network, gamestate: &GamestatePacket); 
    fn is_client_connected(&mut self, network: &mut Network) -> bool;
    fn send_whoami(&mut self, network: &mut Network);

}

pub struct EthServer {
    player_input: InputPacket,
}

impl Server for EthServer {
    fn receive_input(&mut self, network: &mut Network) -> InputPacket {
        let result = network.get_udp_packet();
        match result {
            Ok(value) => match value {
                Some(data) => {
                    if data.len() == InputPacket::len() {
                        self.player_input = InputPacket::deserialize(&data);
                    }
                    // hprintln!("input: {:?} {:?}", data, self.player_input);
                }
                None => {}
            },
            Err(smoltcp::Error::Exhausted) => {}
            Err(e) => {
                hprintln!("Network error: {:?}", e);
            }
        }
        self.player_input
    }
    fn send_gamestate(&mut self, network: &mut Network, gamestate: &GamestatePacket) {
        network.send_udp_packet(&gamestate.serialize());
    }

    fn is_client_connected(&mut self, network: &mut Network) -> bool {
        let result = network.get_udp_packet();
        match result {
            Ok(value) => match value {
                Some(data) => {
                    if data.len() == WhoamiPacket::len() {
                        let whoami = WhoamiPacket::deserialize(&data);
                        if whoami.is_server == false {
                            return true
                        }
                    }
                }
                None => {}
            },
            Err(smoltcp::Error::Exhausted) => {}
            Err(e) => {
                hprintln!("Network error: {:?}", e);
            }
        }
        false
    }

    fn send_whoami(&mut self, network: &mut Network) {
        network.send_udp_packet(&WhoamiPacket{
            is_server: true,
        }.serialize())
    }
}

impl EthServer {
    pub fn new() -> EthServer {
        EthServer {
            player_input: InputPacket::new(),
        }
    }
}

pub struct EthClient {
    gamestate: GamestatePacket,
}

impl Client for EthClient {
    fn send_input(&mut self, network: &mut Network, input: &InputPacket) {
        network.send_udp_packet(&input.serialize());
    }
    fn receive_gamestate(&mut self, network: &mut Network) -> GamestatePacket {
        let result = network.get_udp_packet();
        match result {
            Ok(value) => match value {
                Some(data) => {
                    if data.len() == GamestatePacket::len() {
                        self.gamestate = GamestatePacket::deserialize(&data);
                    }
                    // hprintln!("state: {:?} {:?}", data, self.gamestate);
                }
                None => {}
            },
            Err(smoltcp::Error::Exhausted) => {}
            Err(e) => {
                hprintln!("Network error: {:?}", e);
            }
        }
        self.gamestate
    }
    
    fn is_server_connected(&mut self, network: &mut Network) -> bool {
        let result = network.get_udp_packet();
        match result {
            Ok(value) => match value {
                Some(data) => {
                    if data.len() == WhoamiPacket::len() {
                        
                    }
                    if data.len() == GamestatePacket::len() {
                        return true;
                    }
                }
                None => {}
            },
            Err(smoltcp::Error::Exhausted) => {}
            Err(e) => {
                hprintln!("Network error: {:?}", e);
            }
        }
        false
    }

    fn send_whoami(&mut self, network: &mut Network) {
        network.send_udp_packet(&WhoamiPacket{
            is_server: false,
        }.serialize())
    }
}

impl EthClient {
    pub fn new() -> EthClient {
        EthClient {
            gamestate: GamestatePacket::new(),
        }
    }
}
