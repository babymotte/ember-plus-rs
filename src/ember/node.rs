use crate::{
    ember::{TagClass, tag},
    glow::node::{Node, QualifiedNode},
};
use asn1_rs::Tagged;

impl<'a> Tagged for Node<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 3);
}

impl<'a> Tagged for QualifiedNode<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 10);
}
