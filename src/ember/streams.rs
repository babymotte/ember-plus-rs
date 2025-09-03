use crate::{
    ember::{TagClass, tag},
    glow::streams::{StreamCollection, StreamEntry},
};
use asn1_rs::Tagged;

impl<'a> Tagged for StreamEntry<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 5);
}

impl<'a> Tagged for StreamCollection<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 6);
}
