use crate::glow::{
    element::ElementCollection,
    primitives::{EmberString, Integer32},
};
use asn1_rs::{Boolean, Oid};

pub struct Node<'a> {
    pub number: Integer32<'a>,
    pub contents: Option<NodeContents<'a>>,
    pub children: Option<ElementCollection<'a>>,
}

pub struct QualifiedNode<'a> {
    pub path: Oid<'a>,
    pub contents: Option<NodeContents<'a>>,
    pub children: Option<ElementCollection<'a>>,
}

pub struct NodeContents<'a> {
    pub identifier: Option<EmberString<'a>>,
    pub description: Option<EmberString<'a>>,
    pub is_root: Option<Boolean>,
    pub is_online: Option<Boolean>,
    pub schema_identifiers: Option<EmberString<'a>>,
    pub template_reference: Option<Oid<'a>>,
}
