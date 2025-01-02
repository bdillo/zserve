use std::net::IpAddr;

use mdns_sd::{ServiceDaemon, ServiceInfo};

fn build_service_info(ip_addr: &IpAddr, port: u16) -> mdns_sd::Result<ServiceInfo> {
    // TODO: figure this all out
    // auto_addr?
    ServiceInfo::new(
        "_https._tcp.local.",
        "zserve",
        "zserve.local.",
        ip_addr,
        port,
        None,
    )
}

pub(crate) fn run_mdns(ip_addr: &IpAddr, port: u16) -> mdns_sd::Result<ServiceDaemon> {
    let service_info = build_service_info(ip_addr, port)?;

    let mdns = ServiceDaemon::new()?;
    mdns.register(service_info)?;
    Ok(mdns)
}
