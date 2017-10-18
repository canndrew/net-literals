#[macro_use]
extern crate net_literals;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};
use std::str::FromStr;

fn main() {
    assert_eq!(
        ip!("1.2.3.4"),
        IpAddr::from_str("1.2.3.4").unwrap()
    );

    assert_eq!(
        ipv4!("1.2.3.4"),
        Ipv4Addr::from_str("1.2.3.4").unwrap()
    );

    assert_eq!(
        ipv6!("0011:2233:4455:6677:8899:aabb:ccdd:eeff"),
        Ipv6Addr::from_str("0011:2233:4455:6677:8899:aabb:ccdd:eeff").unwrap()
    );

    assert_eq!(
        addr!("2.3.4.5:666"),
        SocketAddr::from_str("2.3.4.5:666").unwrap()
    );

    assert_eq!(
        addrv4!("2.3.4.5:666"),
        SocketAddrV4::from_str("2.3.4.5:666").unwrap()
    );

    assert_eq!(
        addrv6!("[::2345]:666"),
        SocketAddrV6::from_str("[::2345]:666").unwrap()
    );
}

