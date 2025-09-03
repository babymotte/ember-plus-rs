use crate::{
    ember::{TagClass, tag},
    glow::root::{Root, RootElementCollection},
};
use asn1_rs::Tagged;

impl<'a> Tagged for Root<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 0);
}

impl<'a> Tagged for RootElementCollection<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 11);
}
