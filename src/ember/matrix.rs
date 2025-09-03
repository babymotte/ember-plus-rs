use crate::{
    ember::{TagClass, tag},
    glow::matrix::{Connection, Label, Matrix, QualifiedMatrix, Source, Target},
};
use asn1_rs::Tagged;

impl<'a> Tagged for Matrix<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 13);
}

impl<'a> Tagged for Label<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 18);
}

impl<'a> Tagged for Target<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 14);
}

impl<'a> Tagged for Source<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 15);
}

impl<'a> Tagged for Connection<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 16);
}

impl<'a> Tagged for QualifiedMatrix<'a> {
    const TAG: asn1_rs::Tag = tag(TagClass::Application, 17);
}
