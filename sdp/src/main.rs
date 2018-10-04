extern crate sdp_grpc;

use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::collections::HashSet;
use std::net::Ipv4Addr;
use std::result;
use std::sync::Mutex;

use sdp_grpc::sdpservice::SDPService;

fn main() {
   SDPService::main();
}
