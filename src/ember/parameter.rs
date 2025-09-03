use crate::{
    ember::{TagClass, tag},
    glow::parameter::{
        Parameter, QualifiedParameter, StreamDescription, StringIntegerCollection,
        StringIntegerPair,
    },
};
use asn1_rs::Tagged;

impl<'a> Tagged for Parameter<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 1);
}

impl<'a> Tagged for QualifiedParameter<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 9);
}

impl<'a> Tagged for StringIntegerPair<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 7);
}

impl<'a> Tagged for StringIntegerCollection<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 8);
}

impl<'a> Tagged for StreamDescription<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 12);
}
