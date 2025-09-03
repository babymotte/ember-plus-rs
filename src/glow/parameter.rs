use crate::glow::{
    element::ElementCollection,
    primitives::{EmberString, Integer32, Integer64},
};
use asn1_rs::{Boolean, Null, OctetString, Oid, Real, SequenceOf};

pub struct Parameter<'a> {
    pub number: Integer32<'a>,
    pub contents: Option<ParameterContents<'a>>,
    pub children: Option<ElementCollection<'a>>,
}

pub struct QualifiedParameter<'a> {
    pub path: Oid<'a>,
    pub contents: Option<ParameterContents<'a>>,
    pub children: Option<ElementCollection<'a>>,
}

pub struct ParameterContents<'a> {
    pub identifier: Option<EmberString<'a>>,
    pub description: Option<EmberString<'a>>,
    pub value: Option<Value<'a>>,
    pub minimum: Option<MinMax<'a>>,
    pub maximum: Option<MinMax<'a>>,
    pub access: Option<ParameterAccess>,
    pub format: Option<EmberString<'a>>,
    pub enumeration: Option<EmberString<'a>>,
    pub factor: Option<Integer32<'a>>,
    pub is_online: Option<Boolean>,
    pub formula: Option<EmberString<'a>>,
    pub step: Option<Integer32<'a>>,
    pub default: Option<Value<'a>>,
    pub ptype: Option<ParameterType>,
    pub stream_identifier: Option<Integer32<'a>>,
    pub enum_map: Option<StringIntegerCollection<'a>>,
    pub stream_descriptor: Option<StreamDescription<'a>>,
    pub schema_identifiers: Option<EmberString<'a>>,
    pub template_reference: Option<Oid<'a>>,
}

pub enum Value<'a> {
    Integer(Integer64<'a>),
    Real(Real),
    String(EmberString<'a>),
    Boolean(Boolean),
    Octets(OctetString<'a>),
    Null(Null),
}

pub enum MinMax<'a> {
    Integer(Integer64<'a>),
    Real(Real),
    Null(Null),
}

pub enum ParameterType {
    Null = 0,
    Integer = 1,
    Real = 2,
    String = 3,
    Boolean = 4,
    Trigger = 5,
    Enum = 6,
    Octets = 7,
}

pub enum ParameterAccess {
    None = 0,
    Read = 1,
    Write = 2,
    ReadWrite = 3,
}

pub struct StringIntegerPair<'a> {
    pub entry_string: EmberString<'a>,
    pub entry_integer: Integer32<'a>,
}

pub struct StringIntegerCollection<'a>(pub SequenceOf<StringIntegerPair<'a>>);

pub struct StreamDescription<'a> {
    pub format: StreamFormat,
    pub offset: Integer32<'a>,
}

pub enum StreamFormat {
    Uint8 = 0,
    UInt16Be = 2,
    UInt16Le = 3,
    UInt32Be = 4,
    UInt32Le = 5,
    UInt64Be = 6,
    UInt64Le = 7,
    Int8 = 8,
    Int16Be = 10,
    Int16Le = 11,
    Int32Be = 12,
    Int32Le = 13,
    Int64Be = 14,
    Int64Le = 15,
    Float32Be = 20,
    Float32Le = 21,
    Float64Be = 22,
    Float64Le = 23,
}
