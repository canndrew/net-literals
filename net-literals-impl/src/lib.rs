#[macro_use]
extern crate proc_macro_hack;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};
use std::str::FromStr;

fn destring(input: &str) -> &str {
    let mut chars = input.chars();
    if chars.next() != Some('"') || chars.next_back() != Some('"') {
        panic!("Argument must be a string literal");
    }
    &input[1..(input.len() - 1)]
}

proc_macro_expr_impl! {
    pub fn ip_impl(input: &str) -> String {
        let ip = IpAddr::from_str(destring(input)).unwrap();
        match ip {
            IpAddr::V4(ipv4) => {
                let octets = ipv4.octets();
                format!("::std::net::IpAddr::V4(::std::net::Ipv4Addr::new({}, {}, {}, {}))",
                    octets[0], octets[1], octets[2], octets[3])
            },
            IpAddr::V6(ipv6) => {
                let segments = ipv6.segments();
                format!("::std::net::IpAddr::V6(::std::net::Ipv6Addr::new(\
                        {}, {}, {}, {}, {}, {}, {}, {}))",
                    segments[0], segments[1], segments[2], segments[3],
                    segments[4], segments[5], segments[6], segments[7])
            }
        }
    }
}

proc_macro_expr_impl! {
    pub fn ipv4_impl(input: &str) -> String {
        let ipv4 = Ipv4Addr::from_str(destring(input)).unwrap();
        let octets = ipv4.octets();
        format!("::std::net::Ipv4Addr::new({}, {}, {}, {})",
            octets[0], octets[1], octets[2], octets[3])
    }
}

proc_macro_expr_impl! {
    pub fn ipv6_impl(input: &str) -> String {
        let ipv6 = Ipv6Addr::from_str(destring(input)).unwrap();
        let segments = ipv6.segments();
        format!("::std::net::Ipv6Addr::new(\
                {}, {}, {}, {}, {}, {}, {}, {})",
            segments[0], segments[1], segments[2], segments[3],
            segments[4], segments[5], segments[6], segments[7])
    }
}

proc_macro_expr_impl! {
    pub fn addr_impl(input: &str) -> String {
        let addr = SocketAddr::from_str(destring(input)).unwrap();
        match addr {
            SocketAddr::V4(addrv4) => {
                let ipv4 = addrv4.ip();
                let port = addrv4.port();
                let octets = ipv4.octets();
                format!("::std::net::SocketAddr::V4(\
                        ::std::net::SocketAddrV4::new(\
                            ::std::net::Ipv4Addr::new({}, {}, {}, {}), {}))",
                    octets[0], octets[1], octets[2], octets[3], port)
            },
            SocketAddr::V6(addrv6) => {
                let ipv6 = addrv6.ip();
                let port = addrv6.port();
                let segments = ipv6.segments();
                format!("::std::net::SocketAddr::V6(\
                        ::std::net::SocketAddrV6::new(\
                            ::std::net::Ipv6Addr::new(\
                                {}, {}, {}, {}, {}, {}, {}, {}), {}, 0, 0))",
                    segments[0], segments[1], segments[2], segments[3],
                    segments[4], segments[5], segments[6], segments[7],
                    port)
            },
        }
    }
}

proc_macro_expr_impl! {
    pub fn addrv4_impl(input: &str) -> String {
        let addrv4 = SocketAddrV4::from_str(destring(input)).unwrap();
        let ipv4 = addrv4.ip();
        let port = addrv4.port();
        let octets = ipv4.octets();
        format!("::std::net::SocketAddrV4::new(\
                ::std::net::Ipv4Addr::new({}, {}, {}, {}), {})",
            octets[0], octets[1], octets[2], octets[3], port)
    }
}

proc_macro_expr_impl! {
    pub fn addrv6_impl(input: &str) -> String {
        let addrv6 = SocketAddrV6::from_str(destring(input)).unwrap();
        let ipv6 = addrv6.ip();
        let port = addrv6.port();
        let segments = ipv6.segments();
        format!("::std::net::SocketAddrV6::new(\
                ::std::net::Ipv6Addr::new(\
                    {}, {}, {}, {}, {}, {}, {}, {}), {}, 0, 0)",
            segments[0], segments[1], segments[2], segments[3],
            segments[4], segments[5], segments[6], segments[7],
            port)
    }
}

