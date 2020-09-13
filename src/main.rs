#![deny(unsafe_code)]
use macaddr::MacAddr6;
use std::fmt::Debug;
use std::io;
use std::iter;
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use structopt::StructOpt;

const MAC_SIZE: usize = 6;
const MAC_PER_MAGIC: usize = 16;
static HEADER: [u8; 6] = [0xFF; 6];

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

    fn build_magic_packet(&self) -> Vec<u8> {
        let mut packet = Vec::with_capacity(HEADER.len() + MAC_SIZE * MAC_PER_MAGIC);

        let body: Vec<u8> = iter::repeat(self.mac.as_bytes())
            .take(MAC_PER_MAGIC)
            .flatten()
            .cloned()
            .collect();
        packet.extend(HEADER.iter());
        packet.extend(body);
        packet
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
            let hatty = Hatty {
                mac: "18-C0-4D-42-2D-EA".parse().unwrap(),
                dest: listen.parse().unwrap(),
            };
            hatty.send_magic_packet()
        });

        let (amt, _) = socket.recv_from(&mut buf)?;
        handle.join().unwrap()?;
        let rbuf = &buf[..amt];
        assert_eq!(rbuf, b"hello");
        Ok(())
    }

    #[test]
    fn build_magic_packet_test() -> std::io::Result<()> {
        let mac_address: MacAddr6 = "18-C0-4D-42-2D-EA".parse().unwrap();
        let mac_bytes: &[u8] = mac_address.as_bytes();

        let hatty = Hatty {
            mac: mac_address,
            dest: "127.0.0.1:7896".parse().unwrap(),
        };
        let magic_packet = hatty.build_magic_packet();
        assert_eq!(magic_packet.len(), 102);
        let (fs, macs) = magic_packet.split_at(MAC_SIZE);
        assert_eq!(fs, vec![255; MAC_SIZE].as_slice());

        let ms: Vec<u8> = iter::repeat(mac_bytes)
            .take(MAC_PER_MAGIC)
            .flatten()
            .cloned()
            .collect();
        assert_eq!(macs, &ms[..]);
        Ok(())
    }
}
