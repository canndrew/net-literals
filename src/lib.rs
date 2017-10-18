#[macro_use]
extern crate proc_macro_hack;

#[allow(unused_imports)]
#[macro_use]
extern crate net_literals_impl;
#[doc(hidden)]
pub use net_literals_impl::*;

proc_macro_expr_decl! {
    /// Compile-time parsed IpAddr
    ip! => ip_impl
}

proc_macro_expr_decl! {
    /// Compile-time parsed Ipv4Addr
    ipv4! => ipv4_impl
}

proc_macro_expr_decl! {
    /// Compile-time parsed Ipv6Addr
    ipv6! => ipv6_impl
}

proc_macro_expr_decl! {
    /// Compile-time parsed SocketAddr
    addr! => addr_impl
}

proc_macro_expr_decl! {
    /// Compile-time parsed SocketAddrV4
    addrv4! => addrv4_impl
}

proc_macro_expr_decl! {
    /// Compile-time parsed SocketAddrV6
    addrv6! => addrv6_impl
}

