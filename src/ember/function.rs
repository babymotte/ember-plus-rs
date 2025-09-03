use crate::{
    ember::{TagClass, tag},
    glow::function::{Function, InvocationResult, QualifiedFunction, TupleItemDescription},
};
use asn1_rs::Tagged;

impl<'a> Tagged for Function<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 19);
}

impl<'a> Tagged for QualifiedFunction<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 20);
}

impl<'a> Tagged for TupleItemDescription<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 22);
}

impl<'a> Tagged for InvocationResult<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 24);
}
