use crate::glow::{
    element::ElementCollection,
    primitives::{EmberString, Integer32},
};
use asn1_rs::{Boolean, Oid, SequenceOf};

pub struct Matrix<'a> {
    pub number: Integer32<'a>,
    pub contents: Option<MatrixContents<'a>>,
    pub children: Option<ElementCollection<'a>>,
    pub targets: Option<TargetCollection<'a>>,
    pub sources: Option<SourceCollection<'a>>,
    pub connections: Option<ConnectionCollection<'a>>,
}

pub struct MatrixContents<'a> {
    pub identifier: EmberString<'a>,
    pub description: EmberString<'a>,
    pub mtype: MatrixType,
    pub addressing_mode: MatrixAddressingMode,
    pub target_count: Integer32<'a>,
    pub source_count: Integer32<'a>,
    pub maximum_total_connects: Integer32<'a>,
    pub maximum_connects_per_target: Integer32<'a>,
    pub parameters_location: ParametersLocation<'a>,
    pub gain_parameter_number: Integer32<'a>,
    pub labels: LabelCollection<'a>,
    pub schema_identifiers: EmberString<'a>,
    pub template_reference: Oid<'a>,
}

pub enum MatrixType {
    OneToN = 0,
    OneToOne = 1,
    NToN = 2,
}

pub enum MatrixAddressingMode {
    Linear = 0,
    NonLinear = 1,
}

pub enum ParametersLocation<'a> {
    BasePath(Oid<'a>),
    InLine(Integer32<'a>),
}

pub struct LabelCollection<'a>(pub SequenceOf<Label<'a>>);

pub struct Label<'a> {
    pub base_path: Oid<'a>,
    pub description: EmberString<'a>,
}

pub struct TargetCollection<'a>(pub SequenceOf<Target<'a>>);

pub struct Target<'a>(pub Signal<'a>);

pub struct Signal<'a> {
    pub number: Integer32<'a>,
    pub contents: Option<SignalContents<'a>>,
}

pub struct SignalContents<'a> {
    pub identifier: Option<EmberString<'a>>,
    pub is_online: Option<Boolean>,
    pub labels_location: Option<Oid<'a>>,
}

pub struct SourceCollection<'a>(pub SequenceOf<Source<'a>>);

pub struct Source<'a>(pub Signal<'a>);

pub struct ConnectionCollection<'a>(pub SequenceOf<Connection<'a>>);

pub struct Connection<'a> {
    pub target: Integer32<'a>,
    pub sources: Option<PackedNumbers<'a>>,
    pub operation: Option<ConnectionOperation>,
    pub disposition: Option<ConnectionDisposition>,
}

pub type PackedNumbers<'a> = Oid<'a>;

pub enum ConnectionOperation {
    Absolute = 0,
    Connect = 1,
    Disconnect = 2,
}

pub enum ConnectionDisposition {
    Tally = 0,
    Modified = 1,
    Pending = 2,
    Locked = 3,
}

pub struct QualifiedMatrix<'a> {
    pub path: Oid<'a>,
    pub contents: Option<MatrixContents<'a>>,
    pub children: Option<ElementCollection<'a>>,
    pub targets: Option<TargetCollection<'a>>,
    pub sources: Option<SourceCollection<'a>>,
    pub connections: Option<ConnectionCollection<'a>>,
}
