use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::collections::HashSet;
use std::net::Ipv4Addr;
use std::result;
use std::sync::Mutex;


/// All error information is propagated to the diagnostics argument, but we signal that an error
/// occurred by returning Err(()).
pub type Result<T> = result::Result<T, ()>;

pub struct SDPService {
    registry: Mutex<HashMap<String, HashSet<Ipv4Addr>>>
}

impl SDPService {
    pub fn put(&self, service_key: String, ip: Ipv4Addr) -> Result<String> {
        let mut reg = self.registry.lock().unwrap();
        let peers = match reg.entry(service_key.clone()) {
            Vacant(entry) => entry.insert(HashSet::new()),
            Occupied(entry) => entry.into_mut(),
        };
        peers.insert(ip);
        Ok(service_key)
    }

    pub fn delete(&self, service_key: String, ip: &Ipv4Addr) -> Result<String> {
        let mut reg = self.registry.lock().unwrap();
        match reg.entry(service_key.clone()) {
            Occupied(entry) => {
                let peers = entry.into_mut();
                peers.remove(ip);
            }
            _ => (),
        };
        Ok(service_key)
    }

    fn print(&self) {
        let reg = self.registry.lock().unwrap();
        for (service_key, peers) in reg.iter() {
            for peer in peers.iter() {
                println!("Key={key}, Value={val}", key = service_key, val = peer);
            }
        }
    }

    pub fn main() {
        let reg = HashMap::new();
        let sdp = SDPService {registry: Mutex::new(reg)};
        sdp.put(String::from("raft"), Ipv4Addr::new(0, 0, 0, 0));
        sdp.put(String::from("raft"), Ipv4Addr::new(1, 1, 1, 1));
        sdp.print();
    }
}

