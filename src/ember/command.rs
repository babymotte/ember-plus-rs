use crate::{
    ember::{TagClass, tag},
    glow::command::{Command, CommandType, FieldFlags},
};
use asn1_rs::{FromBer, Integer, Tagged};

impl<'a> Tagged for Command<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 2);
}

impl<'a> FromBer<'a> for CommandType {
    fn from_ber(bytes: &'a [u8]) -> asn1_rs::ParseResult<'a, Self, asn1_rs::Error> {
        let (bytes, int) = Integer::from_ber(bytes)?;
        Ok((bytes, CommandType::try_from(int.as_i32()?)?))
    }
}

impl<'a> FromBer<'a> for FieldFlags {
    fn from_ber(bytes: &'a [u8]) -> asn1_rs::ParseResult<'a, Self, asn1_rs::Error> {
        let (bytes, int) = Integer::from_ber(bytes)?;
        Ok((bytes, FieldFlags::try_from(int.as_i32()?)?))
    }
}
