use packed_struct::prelude::*;

#[derive(Debug, PrimitiveEnum_u4)]
pub enum OPCODE {
    STANDARD = 0,
    INVERSE = 1,
    STATUS = 2,
}

#[derive(Debug, PrimitiveEnum_u16)]
pub enum TYPE {
    A = 1,
    NS = 2,
    MD = 3,
    MF = 4,
    CNAME = 5,
    SOA = 6,
    MB = 7,
    MG = 8,
    MR = 9,
    NULL = 10,
    WKS = 11,
    PTR = 12,
    HINFO = 13,
    MINFO = 14,
    MX = 15,
    TXT = 16,
}

#[derive(Debug, PrimitiveEnum_u16)]
pub enum QTYPE {
    AXFR = 252,
    MAILB = 253,
    MAILA = 254,
    ALL = 255,
}

#[derive(Debug, PrimitiveEnum_u16)]
pub enum CLASS {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
}

#[derive(Debug, PrimitiveEnum_u16)]
pub enum QCLASS {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
    ANY = 255,
}
