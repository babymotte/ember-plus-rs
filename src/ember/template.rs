use crate::{
    ember::{TagClass, tag},
    glow::template::{QualifiedTemplate, Template},
};
use asn1_rs::Tagged;

impl<'a> Tagged for Template<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 24);
}

impl<'a> Tagged for QualifiedTemplate<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 25);
}
