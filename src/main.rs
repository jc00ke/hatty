#![deny(unsafe_code)]
use macaddr::MacAddr6;
use std::fmt::Debug;
use std::io;
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "wol",
    about = "A little helper to wake up a computer using a magic packet",
    setting = structopt::clap::AppSettings::DeriveDisplayOrder
)]
struct Opt {
    #[structopt(short, long, env)]
    mac: MacAddr6,

    #[structopt(short, long, env, default_value = "255.255.255.255")]
    to: Ipv4Addr,
}

#[derive(Debug)]
struct Hatty {
    mac: MacAddr6,
    dest: SocketAddr,
}

impl Hatty {
    pub fn new(mac: MacAddr6, dest: SocketAddr) -> Self {
        Self { mac, dest }
    }

    pub fn send_magic_packet(&self) -> std::io::Result<()> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        match socket.send_to(b"hello", self.dest) {
            Ok(nbytes) if nbytes == 5 => Ok(()),
            Ok(nbytes) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Expected 5 bytes, got {}", nbytes),
            )),
            Err(e) => Err(e),
        }
    }
}

impl From<Opt> for Hatty {
    fn from(o: Opt) -> Self {
        Self {
            dest: SocketAddr::new(o.to.into(), 9),
            mac: o.mac,
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    let hatty: Hatty = opt.into();
    println!("{:?}", hatty);
    hatty.send_magic_packet().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn listen_for_magic_packet_test() -> std::io::Result<()> {
        let listen = "127.0.0.1:34356";
        let socket = UdpSocket::bind(listen)?;
        socket.set_read_timeout(Some(std::time::Duration::from_secs(1)))?;

        let mut buf = [0; 102];

        let handle = std::thread::spawn(move || {
            let hatty = Hatty::new(
                "18-C0-4D-42-2D-EA".parse().unwrap(),
                listen.parse().unwrap(),
            );
            hatty.send_magic_packet()
        });

        let (amt, _) = socket.recv_from(&mut buf)?;
        handle.join().unwrap()?;
        let rbuf = &buf[..amt];
        assert_eq!(rbuf, b"hello");
        Ok(())
    }
    //fn extend_mac_test() {
    //let mac = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06];

    //let extended_mac = Hatty::extend_mac(&mac);

    //assert_eq!(extended_mac.len(), MAC_PER_MAGIC * MAC_SIZE);
    //assert_eq!(&extended_mac[(MAC_PER_MAGIC - 1) * MAC_SIZE..], &mac[..]);
    //}

    //#[test]
    //fn mac_to_byte_test() {
    //let mac = "01:02:03:04:05:06";
    //let result = super::WolPacket::mac_to_byte(mac, ':');

    //assert_eq!(result, vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
    //}

    //#[test]
    //#[should_panic]
    //fn mac_to_byte_invalid_chars_test() {
    //let mac = "ZZ:02:03:04:05:06";
    //super::WolPacket::mac_to_byte(mac, ':');
    //}

    //#[test]
    //#[should_panic]
    //fn mac_to_byte_invalid_separator_test() {
    //let mac = "01002:03:04:05:06";
    //super::WolPacket::mac_to_byte(mac, ':');
    //}

    //#[test]
    //fn create_packet_bytes_test() {
    //let bytes = super::WolPacket::create_packet_bytes(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);

    //assert_eq!(
    //bytes.len(),
    //super::MAC_SIZE * super::MAC_PER_MAGIC + super::HEADER.len()
    //);
    //assert!(bytes.iter().all(|&x| x == 0xFF));
    //}
}
