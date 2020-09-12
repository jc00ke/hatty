#![deny(unsafe_code)]
use std::fmt::Debug;
use std::net::Ipv4Addr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "wol",
    about = "A little helper to wake up a computer using a magic packet",
    setting = structopt::clap::AppSettings::DeriveDisplayOrder
)]
struct Opt {
    #[structopt(short, long, env, default_value = "0.0.0.0")]
    from: Ipv4Addr,

    #[structopt(short, long, env)]
    mac: String,

    #[structopt(short, long, env, default_value = "255.255.255.255")]
    to: Ipv4Addr,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
