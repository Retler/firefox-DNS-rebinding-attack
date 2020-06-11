#![allow(dead_code)]

use packed_struct::prelude::*;
use packed_struct_codegen::PackedStruct;
use std::io::Read;
use std::net::UdpSocket;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

#[derive(Debug, PackedStruct)]
#[packed_struct(bit_numbering = "msb0")]
pub struct DNSHeader {
    #[packed_field(bits = "0..=15", endian = "msb")]
    id: u16,
    #[packed_field(bits = "16")]
    qr: bool,
    // query = false, response = true
    #[packed_field(bits = "17..=20")]
    opcode: u8,
    // U4
    #[packed_field(bits = "21")]
    aa: bool,
    // Authorative answer
    #[packed_field(bits = "22")]
    tc: bool,
    // Message was truncated
    #[packed_field(bits = "23")]
    rd: bool,
    // Recursion Desired
    #[packed_field(bits = "24")]
    ra: bool,
    // Recursion Available
    #[packed_field(bits = "25..=27")]
    z: u8,
    // u3 zero
    #[packed_field(bits = "28..=31")]
    rcode: u8,
    // u4 Reponse code (0 is no error)
    #[packed_field(bits = "32..=47", endian = "msb")]
    qdcount: u16,
    // an unsigned 16 bit integer specifying the number of entries in the question section.
    #[packed_field(bits = "48..=63", endian = "msb")]
    ancount: u16,
    // an unsigned 16 bit integer specifying the number of resource records in the answer section.
    #[packed_field(bits = "64..=79", endian = "msb")]
    nscount: u16,
    // an unsigned 16 bit integer specifying the number of name server resource records in the authority records section.
    #[packed_field(bits = "80..=95", endian = "msb")]
    arcount: u16, // an unsigned 16 bit integer specifying the number of resource records in the additional records section.
}

#[derive(Debug)]
pub struct DNSQuestion {
    qname: String,
    qtype: u16,
    qclass: u16,
}

fn two_u8_to_u16(b0: u8, b1: u8) -> u16 {
    ((b0 as u16) << 8) | b1 as u16
}

impl DNSQuestion {
    // Panics if message isn't formatted correctly
    fn from_bytes(bytes: &[u8]) -> (Self, usize) {
        let vec = bytes.to_vec();
        let mut len_octet = 0;
        let mut labels = String::new();
        while vec[len_octet] != 0 {
            let len = vec[len_octet];
            let str_bytes = vec[len_octet + 1..len_octet + len as usize + 1].to_vec();
            labels.push_str(&String::from_utf8(str_bytes).unwrap());
            labels.push_str(".");
            len_octet += len as usize + 1;
        }

        let qtype = two_u8_to_u16(vec[len_octet + 1], vec[len_octet + 2]);
        let qclass = two_u8_to_u16(vec[len_octet + 3], vec[len_octet + 4]);

        // Remove the last dot
        // labels.pop();
        (
            DNSQuestion {
                qname: labels,
                qtype,
                qclass,
            },
            len_octet + 5,
        )
    }
}

#[derive(Debug, PackedStruct)]
#[packed_struct(bit_numbering = "msb0")]
pub struct StaticResourceRecord {
    #[packed_field(bits = "0..=15", endian = "msb")]
    type_: u16,
    #[packed_field(bits = "16..=31", endian = "msb")]
    class: u16,
    #[packed_field(bits = "32..=63", endian = "msb")]
    ttl: u32,
    #[packed_field(bits = "64..=79", endian = "msb")]
    rdlength: u16,
}

#[derive(Debug)]
struct ResourceRecord<'t> {
    name: String,
    static_data: StaticResourceRecord,
    rdata: &'t [u8; 4],
}

fn read_name(data: &[u8]) -> String {
    let mut ret = String::new();
    let mut i = 0;
    while data[i] != 0 {
        ret.push(data[i] as char);
        i += 1;
    }
    ret
}

impl ResourceRecord<'_> {
    pub fn pack(&mut self) -> Vec<u8> {
        // NAME
        let mut res = vec![];
        res.append(&mut string_to_labels(&self.name));
        // TYPE, CLASS, TTL, RDLENGTH
        res.append(&mut self.static_data.pack().to_vec());
        // RDATA
        res.append(&mut self.rdata.to_vec());
        res
    }

    pub fn unpack(data: &[u8]) -> Self {
        let name = read_name(data);
        ResourceRecord {
            name,
            static_data: StaticResourceRecord {
                type_: 1,
                class: 0,
                ttl: 0,
                rdlength: 0,
            },
            rdata: &[0, 0, 0, 0],
        }
    }
}

fn string_to_labels(s: &str) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];
    for l in s.split('.') {
        let len = l.len();
        res.push(len as u8);
        if len != 0 {
            res.append(&mut l.as_bytes().to_vec())
        }
    }
    res
}

fn get_ip(c: usize) -> [u8; 4] {
    let buffer = 40;
    if c < buffer {
        return [10, 6, 0, 5]
    }
    let n_same_ans = 2; // How many times we should return the same answer
    let new_c = (c - buffer) / n_same_ans;
    let first = std::cmp::min(new_c % 256, 40);
    [10, 5, 0, first as u8]
}

fn main() {
    let should_reset = Arc::new(AtomicBool::new(false));

    let dns_should_reset = Arc::clone(&should_reset);
    let inp_should_reset = Arc::clone(&should_reset);

    let handle_dns = thread::spawn(move || {
        let mut c = 0;
        loop {
            if dns_should_reset.load(Ordering::Relaxed) {
                println!("Reset the IP");
                c = 0;
                dns_should_reset.swap(false, Ordering::Relaxed);
            }
            let return_ip = get_ip(c); // The IP to return in the DNS response
            let socket = UdpSocket::bind("0.0.0.0:53").unwrap();

            let mut buf = [0; 512];
            let (_, src) = socket.recv_from(&mut buf).unwrap();
            let mut msg = DNSHeader::unpack_from_slice(&buf[..12]).unwrap();

            let (q, _) = DNSQuestion::from_bytes(&buf[12..]);

            msg.ancount = 1;
            msg.qdcount = 0;
            msg.arcount = 0;
            msg.ra = true;
            msg.qr = true;
            msg.aa = true;
            let mut res: Vec<u8> = msg.pack().to_vec();

            let mut answer = ResourceRecord {
                name: q.qname,
                static_data: StaticResourceRecord {
                    type_: q.qtype,
                    class: q.qclass,
                    ttl: 1,
                    rdlength: 4,
                },
                rdata: &return_ip,
            };

            res.append(&mut answer.pack());
            println!("c: {}", c);
            println!(
                "Sending ip: {}.{}.{}.{}",
                &return_ip[0], &return_ip[1], &return_ip[2], &return_ip[3]
            );
            socket.send_to(&res, &src).unwrap();
            c += 1;
        }
    });

    println!("Press 'r' to reset");
    let handle_inp = thread::spawn(move || loop {
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let mut buffer = [0; 1];
        handle.read(&mut buffer).unwrap();
        if buffer.contains(&114u8) {
            inp_should_reset.swap(true, Ordering::Relaxed);
            println!("Reset");
        }
    });

    handle_inp.join().unwrap();
    handle_dns.join().unwrap();
}
