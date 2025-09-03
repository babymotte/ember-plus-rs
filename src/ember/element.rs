use crate::{
    ember::{TagClass, tag},
    glow::element::ElementCollection,
};
use asn1_rs::Tagged;

impl<'a> Tagged for ElementCollection<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 4);
}
