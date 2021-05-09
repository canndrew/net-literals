extern crate proc_macro;

use {
    std::{
        str::FromStr,
        net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    },
    syn::LitStr,
    proc_macro::TokenStream,
    quote::quote,
    proc_macro_error::{abort, proc_macro_error},
};

#[proc_macro_error]
#[proc_macro]
pub fn ip(tokens: TokenStream) -> TokenStream {
    let lit = syn::parse_macro_input!(tokens as LitStr);
    let span = lit.span();
    let s = lit.value();
    let ip = match IpAddr::from_str(&s) {
        Ok(ip) => ip,
        Err(err) => {
            abort!(span, "{}", err);
        },
    };
    let ret = match ip {
        IpAddr::V4(ipv4) => {
            let [b0, b1, b2, b3] = ipv4.octets();
            quote! {
                ::std::net::IpAddr::V4(::std::net::Ipv4Addr::new(#b0, #b1, #b2, #b3))
            }
        },
        IpAddr::V6(ipv6) => {
            let [s0, s1, s2, s3, s4, s5, s6, s7] = ipv6.segments();
            quote! {
                ::std::net::IpAddr::V6(::std::net::Ipv6Addr::new(#s0, #s1, #s2, #s3, #s4, #s5, #s6, #s7))
            }
        },
    };
    ret.into()
}

#[proc_macro_error]
#[proc_macro]
pub fn ipv4(tokens: TokenStream) -> TokenStream {
    let lit = syn::parse_macro_input!(tokens as LitStr);
    let span = lit.span();
    let s = lit.value();
    let ipv4 = match Ipv4Addr::from_str(&s) {
        Ok(ipv4) => ipv4,
        Err(err) => {
            abort!(span, "{}", err);
        },
    };
    let [b0, b1, b2, b3] = ipv4.octets();
    let ret = quote! {
        ::std::net::Ipv4Addr::new(#b0, #b1, #b2, #b3)
    };
    ret.into()
}

#[proc_macro_error]
#[proc_macro]
pub fn ipv6(tokens: TokenStream) -> TokenStream {
    let lit = syn::parse_macro_input!(tokens as LitStr);
    let span = lit.span();
    let s = lit.value();
    let ipv6 = match Ipv6Addr::from_str(&s) {
        Ok(ipv6) => ipv6,
        Err(err) => {
            abort!(span, "{}", err);
        },
    };
    let [s0, s1, s2, s3, s4, s5, s6, s7] = ipv6.segments();
    let ret = quote! {
        ::std::net::Ipv6Addr::new(#s0, #s1, #s2, #s3, #s4, #s5, #s6, #s7)
    };
    ret.into()
}

#[proc_macro_error]
#[proc_macro]
pub fn addr(tokens: TokenStream) -> TokenStream {
    let lit = syn::parse_macro_input!(tokens as LitStr);
    let span = lit.span();
    let s = lit.value();
    let addr = match SocketAddr::from_str(&s) {
        Ok(addr) => addr,
        Err(err) => {
            abort!(span, "{}", err);
        },
    };
    let ret = match addr {
        SocketAddr::V4(addrv4) => {
            let [b0, b1, b2, b3] = addrv4.ip().octets();
            let port = addrv4.port();
            quote! {
                ::std::net::SocketAddr::V4(::std::net::SocketAddrV4::new(
                    ::std::net::Ipv4Addr::new(#b0, #b1, #b2, #b3),
                    #port,
                ))
            }
        },
        SocketAddr::V6(addrv6) => {
            let [s0, s1, s2, s3, s4, s5, s6, s7] = addrv6.ip().segments();
            let port = addrv6.port();
            let flowinfo = addrv6.flowinfo();
            let scope_id = addrv6.scope_id();
            quote! {
                ::std::net::SocketAddr::V6(::std::net::SocketAddrV6::new(
                    ::std::net::Ipv6Addr::new(#s0, #s1, #s2, #s3, #s4, #s5, #s6, #s7),
                    #port,
                    #flowinfo,
                    #scope_id,
                ))
            }
        },
    };
    ret.into()
}

#[proc_macro_error]
#[proc_macro]
pub fn addrv4(tokens: TokenStream) -> TokenStream {
    let lit = syn::parse_macro_input!(tokens as LitStr);
    let span = lit.span();
    let s = lit.value();
    let addrv4 = match SocketAddrV4::from_str(&s) {
        Ok(addrv4) => addrv4,
        Err(err) => {
            abort!(span, "{}", err);
        },
    };
    let [b0, b1, b2, b3] = addrv4.ip().octets();
    let port = addrv4.port();
    let ret = quote! {
        ::std::net::SocketAddrV4::new(
            ::std::net::Ipv4Addr::new(#b0, #b1, #b2, #b3),
            #port,
        )
    };
    ret.into()
}

#[proc_macro_error]
#[proc_macro]
pub fn addrv6(tokens: TokenStream) -> TokenStream {
    let lit = syn::parse_macro_input!(tokens as LitStr);
    let span = lit.span();
    let s = lit.value();
    let addrv6 = match SocketAddrV6::from_str(&s) {
        Ok(addrv6) => addrv6,
        Err(err) => {
            abort!(span, "{}", err);
        },
    };
    let [s0, s1, s2, s3, s4, s5, s6, s7] = addrv6.ip().segments();
    let port = addrv6.port();
    let flowinfo = addrv6.flowinfo();
    let scope_id = addrv6.scope_id();
    let ret = quote! {
        ::std::net::SocketAddrV6::new(
            ::std::net::Ipv6Addr::new(#s0, #s1, #s2, #s3, #s4, #s5, #s6, #s7),
            #port,
            #flowinfo,
            #scope_id,
        )
    };
    ret.into()
}
