use crate::glow::{
    element::ElementCollection,
    parameter::{ParameterType, Value},
    primitives::{EmberString, Integer32},
};
use asn1_rs::{Boolean, Oid, SequenceOf};

pub struct Function<'a> {
    pub number: Integer32<'a>,
    pub contents: Option<FunctionContents<'a>>,
    pub children: Option<ElementCollection<'a>>,
}

pub struct QualifiedFunction<'a> {
    pub path: Oid<'a>,
    pub contents: Option<FunctionContents<'a>>,
    pub children: Option<ElementCollection<'a>>,
}

pub struct FunctionContents<'a> {
    pub identifier: Option<EmberString<'a>>,
    pub description: Option<EmberString<'a>>,
    pub arguments: Option<TupleDescription<'a>>,
    pub result: Option<TupleDescription<'a>>,
    pub template_reference: Option<Oid<'a>>,
}

pub struct TupleDescription<'a>(pub SequenceOf<TupleItemDescription<'a>>);

pub struct TupleItemDescription<'a> {
    pub ptype: ParameterType,
    pub name: Option<EmberString<'a>>,
}

pub struct Invocation<'a> {
    pub invocation_id: Option<Integer32<'a>>,
    pub arguments: Option<Tuple<'a>>,
}

pub struct Tuple<'a>(pub SequenceOf<Value<'a>>);

pub struct InvocationResult<'a> {
    pub invocation_id: Integer32<'a>,
    pub success: Option<Boolean>,
    pub result: Option<Tuple<'a>>,
}
