use crate::glow::{
    command::Command, function::Function, matrix::Matrix, node::Node, parameter::Parameter,
    template::Template,
};
use asn1_rs::SequenceOf;

pub enum Element<'a> {
    Parameter(Parameter<'a>),
    Node(Node<'a>),
    Command(Command<'a>),
    Matrix(Matrix<'a>),
    Function(Function<'a>),
    Template(Template<'a>),
}

pub struct ElementCollection<'a>(pub SequenceOf<Element<'a>>);
