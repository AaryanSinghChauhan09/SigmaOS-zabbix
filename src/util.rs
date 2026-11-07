//! zabbix model — network tool.

#[derive(Debug, Clone)]
pub struct Host {
    pub name: String,
    pub ip: String,
    pub ports: Vec<u16>,
    pub up: bool,
}

#[derive(Debug, Clone)]
pub struct Alert {
    pub host: String,
    pub message: String,
    pub severity: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub hosts: Vec<Host>,
    pub alerts: Vec<Alert>,
}

impl Default for Model {
    fn default() -> Self { Self { hosts: vec![], alerts: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_host(&mut self, name: &str, ip: &str) -> usize {
        let id = self.hosts.len();
        self.hosts.push(Host { name: name.into(), ip: ip.into(), ports: vec![], up: true });
        id
    }

    pub fn scan(&mut self, name: &str, ports: &[u16]) -> bool {
        if let Some(h) = self.hosts.iter_mut().find(|h| h.name == name) {
            h.ports = ports.to_vec();
            true
        } else { false }
    }

    pub fn alert(&mut self, host: &str, message: &str, severity: &str) {
        self.alerts.push(Alert { host: host.into(), message: message.into(), severity: severity.into() });
    }

    pub fn down(&mut self, name: &str) -> bool {
        if let Some(h) = self.hosts.iter_mut().find(|h| h.name == name) {
            h.up = false;
            true
        } else { false }
    }

    pub fn list(&self) -> &[Host] { &self.hosts }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn host_scan_alert() {
        let mut m = Model::new();
        m.add_host("router", "192.168.1.1");
        assert!(m.scan("router", &[22, 80, 443]));
        m.alert("router", "high load", "warning");
        assert!(m.down("router"));
    }
}
