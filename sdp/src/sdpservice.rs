use bincode::serialize;
use memmap::MmapMut;
use scheduled_executor::ThreadPoolExecutor;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::result;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;


/// All error information is propagated to the diagnostics argument, but we signal that an error
/// occurred by returning Err(()).
pub type Result<T> = result::Result<T, ()>;

pub struct SDPService {
    registry_in_mem: Mutex<HashMap<String, HashSet<Ipv4Addr>>>,
    path: PathBuf,
}

impl SDPService {
    pub fn put(&self, service_key: String, ip: Ipv4Addr) -> Result<String> {
        let mut reg = self.registry_in_mem.lock().unwrap();
        let peers = match reg.entry(service_key.clone()) {
            Vacant(entry) => entry.insert(HashSet::new()),
            Occupied(entry) => entry.into_mut(),
        };
        peers.insert(ip);
        Ok(service_key)
    }

    pub fn delete(&self, service_key: String, ip: &Ipv4Addr) -> Result<String> {
        let mut reg = self.registry_in_mem.lock().unwrap();
        match reg.entry(service_key.clone()) {
            Occupied(entry) => {
                let peers = entry.into_mut();
                peers.remove(ip);
            }
            _ => (),
        };
        Ok(service_key)
    }

    fn flush(&self) {
        let mut reg = self.registry_in_mem.lock().unwrap();
        let registry_on_disk = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
        let mut mmap = unsafe { MmapMut::map_mut(&file)? };
        let encoded: Vec<u8> = serialize(&req).unwrap();
        (&mut mmap[..]).write_all(&encoded)?;
        mmap.flush()?;
    }

    fn print(&self) {
        let reg = self.registry_in_mem.lock().unwrap();
        for (service_key, peers) in reg.iter() {
            for peer in peers.iter() {
                println!("Key={key}, Value={val}", key = service_key, val = peer);
            }
        }
    }

    pub fn main() {
        let reg = HashMap::new();
        let sdp = SDPService { registry_in_mem: Mutex::new(reg), path: RelativePathBuf::new() };
        sdp.put(String::from("raft"), Ipv4Addr::new(0, 0, 0, 0));
        sdp.put(String::from("raft"), Ipv4Addr::new(1, 1, 1, 1));
        sdp.print();
        
        //todo extract to method
        let executor = ThreadPoolExecutor::new(1).expect("Thread pool creation failed");


        executor.schedule_fixed_rate(
            Duration::from_secs(0),  // No wait for scheduling the first task
            Duration::from_secs(360),  // and schedule every following task at 6 mins intervals
            move |_| {
                sdp.flush()
            },
        );

        loop {
            thread::park();
        }
    }
}

