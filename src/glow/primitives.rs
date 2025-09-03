use asn1_rs::{Integer, Oid, Utf8String, oid};

pub const BASE_OID: Oid = oid!(1.3.6.1.4.1.37411.2.1.1.100);

pub type EmberString<'a> = Utf8String<'a>;
pub type Integer32<'a> = Integer<'a>;
pub type Integer64<'a> = Integer<'a>;
