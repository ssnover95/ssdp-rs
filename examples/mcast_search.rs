extern crate ssdp;

use ssdp::header::{HeaderMut, Man, MX, ST};
use ssdp::message::{SearchRequest};

fn main() {
    // Create Our Search Request
    let mut request = SearchRequest::new();
    
    // Set Our Desired Headers (Not Verified By The Library)
    request.set(Man); request.set(MX(5)); request.set(ST::All);
    
    // Iterate Over Streaming Responses
    for response in request.multicast().unwrap() {
        println!("{:?}\n\n", i);
    }
}