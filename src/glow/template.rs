use crate::glow::{
    function::Function,
    matrix::Matrix,
    node::Node,
    parameter::Parameter,
    primitives::{EmberString, Integer32},
};
use asn1_rs::Oid;

pub struct Template<'a> {
    pub number: Integer32<'a>,
    pub element: Option<TemplateElement<'a>>,
    pub description: Option<EmberString<'a>>,
}

pub struct QualifiedTemplate<'a> {
    pub path: Oid<'a>,
    pub element: Option<TemplateElement<'a>>,
    pub description: Option<EmberString<'a>>,
}

pub enum TemplateElement<'a> {
    Parameter(Parameter<'a>),
    Node(Node<'a>),
    Matrix(Matrix<'a>),
    Function(Function<'a>),
}
