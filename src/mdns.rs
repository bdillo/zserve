use mdns_sd::{ServiceDaemon, ServiceInfo};

fn build_service_info(name: &str, port: u16) -> mdns_sd::Result<ServiceInfo> {
    let hostname = &format!("{}.local.", name);

    Ok(ServiceInfo::new("_https._tcp.local.", name, hostname, (), port, None)?.enable_addr_auto())
}

pub(crate) fn run_mdns(name: &str, port: u16) -> mdns_sd::Result<ServiceDaemon> {
    let service_info = build_service_info(name, port)?;

    let mdns = ServiceDaemon::new()?;
    mdns.register(service_info)?;
    Ok(mdns)
}
