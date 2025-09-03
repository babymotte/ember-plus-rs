use asn1_rs::SequenceOf;

use crate::glow::{
    element::Element,
    function::{InvocationResult, QualifiedFunction},
    matrix::QualifiedMatrix,
    node::QualifiedNode,
    parameter::QualifiedParameter,
    streams::StreamCollection,
    template::QualifiedTemplate,
};

pub enum Root<'a> {
    Elements(RootElementCollection<'a>),
    Streams(StreamCollection<'a>),
    InvocationResult(InvocationResult<'a>),
}

pub struct RootElementCollection<'a>(pub SequenceOf<RootElement<'a>>);

pub enum RootElement<'a> {
    Element(Element<'a>),
    QualifiedParameter(QualifiedParameter<'a>),
    QualifiedNode(QualifiedNode<'a>),
    QualifiedMatrix(QualifiedMatrix<'a>),
    QualifiedFunction(QualifiedFunction<'a>),
    QualifiedTemplate(QualifiedTemplate<'a>),
}
