use crate::glow::{parameter::Value, primitives::Integer32};
use asn1_rs::SequenceOf;

pub struct StreamEntry<'a> {
    pub stream_identifier: Integer32<'a>,
    pub stream_value: Value<'a>,
}

pub struct StreamCollection<'a>(pub SequenceOf<StreamEntry<'a>>);
