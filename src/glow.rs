/*
 *  Copyright (C) 2025 Michael Bachmann
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

pub use ext::*;
use rasn::{AsnType, Decode, Decoder, Encode, Encoder, de::Error, types::SequenceOf};
use serde::{Deserialize, Serialize};

// =============================
// Primitive aliases
// =============================
pub type EmberString = String;
pub type Integer32 = i32; // INTEGER (-2^31 .. 2^31-1)
pub type Integer64 = i64; // INTEGER (-2^63 .. 2^63-1)

// =============================
// RELATIVE-OID
// =============================
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, AsnType)]
#[rasn(tag(universal, 13))]
pub struct RelativeOid(pub SequenceOf<u32>);

// =============================
// Template
// =============================
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 24))]
pub struct Template {
    #[rasn(tag(explicit(context, 0)))]
    pub number: Integer32,
    #[rasn(tag(explicit(context, 1)))]
    pub element: Option<TemplateElement>,
    #[rasn(tag(explicit(context, 2)))]
    pub description: Option<EmberString>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 25))]
pub struct QualifiedTemplate {
    #[rasn(tag(explicit(context, 0)))]
    pub path: RelativeOid,
    #[rasn(tag(explicit(context, 1)))]
    pub element: Option<TemplateElement>,
    #[rasn(tag(explicit(context, 2)))]
    pub description: Option<EmberString>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum TemplateElement {
    Parameter(Parameter),
    Node(Node),
    Matrix(Matrix),
    Function(Function),
}

// =============================
// Parameter
// =============================
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 1))]
pub struct Parameter {
    #[rasn(tag(explicit(context, 0)))]
    pub number: Integer32,
    #[rasn(tag(explicit(context, 1)))]
    pub contents: Option<ParameterContents>,
    #[rasn(tag(explicit(context, 2)))]
    pub children: Option<ElementCollection>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 9))]
pub struct QualifiedParameter {
    #[rasn(tag(explicit(context, 0)))]
    pub path: RelativeOid,
    #[rasn(tag(explicit(context, 1)))]
    pub contents: Option<ParameterContents>,
    #[rasn(tag(explicit(context, 2)))]
    pub children: Option<ElementCollection>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, AsnType, Decode, Encode)]
#[rasn(set, tag(universal, 17))]
pub struct ParameterContents {
    #[rasn(tag(explicit(context, 0)))]
    pub identifier: Option<EmberString>,
    #[rasn(tag(explicit(context, 1)))]
    pub description: Option<EmberString>,
    #[rasn(tag(explicit(context, 2)))]
    #[serde(rename = "value")]
    pub param_value: Option<Value>,
    #[rasn(tag(explicit(context, 3)))]
    pub minimum: Option<MinMax>,
    #[rasn(tag(explicit(context, 4)))]
    pub maximum: Option<MinMax>,
    #[rasn(tag(explicit(context, 5)))]
    pub access: Option<ParameterAccess>,
    #[rasn(tag(explicit(context, 6)))]
    pub format: Option<EmberString>,
    #[rasn(tag(explicit(context, 7)))]
    pub enumeration: Option<EmberString>,
    #[rasn(tag(explicit(context, 8)))]
    pub factor: Option<Integer32>,
    #[rasn(tag(explicit(context, 9)))]
    pub is_online: Option<bool>,
    #[rasn(tag(explicit(context, 10)))]
    pub formula: Option<EmberString>,
    #[rasn(tag(explicit(context, 11)))]
    pub step: Option<Integer32>,
    #[rasn(tag(explicit(context, 12)))]
    pub default: Option<Value>,
    #[rasn(tag(explicit(context, 13)))]
    pub r#type: Option<ParameterType>,
    #[rasn(tag(explicit(context, 14)))]
    pub stream_identifier: Option<Integer32>,
    #[rasn(tag(explicit(context, 15)))]
    pub enum_map: Option<StringIntegerCollection>,
    #[rasn(tag(explicit(context, 16)))]
    pub stream_descriptor: Option<StreamDescription>,
    #[rasn(tag(explicit(context, 17)))]
    pub schema_identifiers: Option<EmberString>,
    #[rasn(tag(explicit(context, 18)))]
    pub template_reference: Option<RelativeOid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(choice)]
#[serde(untagged)]
pub enum Value {
    #[rasn(tag(universal, 2))] // INTEGER
    Integer(Integer64),
    #[rasn(tag(universal, 9))] // REAL
    Real(f64),
    #[rasn(tag(universal, 12))] // UTF8String
    String(EmberString),
    #[rasn(tag(universal, 1))] // BOOLEAN
    Boolean(bool),
    #[rasn(tag(universal, 4))] // OCTET STRING
    Octets(Vec<u8>),
    #[rasn(tag(universal, 5))] // NULL
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum MinMax {
    #[rasn(tag(universal, 2))]
    Integer(Integer64),
    #[rasn(tag(universal, 9))]
    Real(f64),
    #[rasn(tag(universal, 5))]
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, AsnType, Decode, Encode)]
#[rasn(enumerated, tag(universal, 2))]
pub enum ParameterType {
    Null = 0,
    Integer = 1,
    Real = 2,
    String = 3,
    Boolean = 4,
    Trigger = 5,
    Enum = 6,
    Octets = 7,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Default, Copy, PartialEq, Eq, AsnType, Decode, Encode,
)]
#[rasn(enumerated, tag(universal, 2))]
pub enum ParameterAccess {
    None = 0,
    #[default]
    Read = 1,
    Write = 2,
    ReadWrite = 3,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 7))]
pub struct StringIntegerPair {
    #[rasn(tag(explicit(context, 0)))]
    pub entry_string: EmberString,
    #[rasn(tag(explicit(context, 1)))]
    pub entry_integer: Integer32,
}

// StringIntegerCollection ::= [APPLICATION 8] IMPLICIT SEQUENCE OF [0] StringIntegerPair
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 8), delegate)]
pub struct StringIntegerCollection(pub SequenceOf<TaggedStringIntegerPair>);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(0))]
pub struct TaggedStringIntegerPair(pub StringIntegerPair);

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 12))]
pub struct StreamDescription {
    #[rasn(tag(explicit(context, 0)))]
    pub format: StreamFormat,
    #[rasn(tag(explicit(context, 1)))]
    pub offset: Integer32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, AsnType, Decode, Encode)]
#[rasn(enumerated, tag(universal, 2))]
pub enum StreamFormat {
    UnsignedInt8 = 0,
    UnsignedInt16BigEndian = 2,
    UnsignedInt16LittleEndian = 3,
    UnsignedInt32BigEndian = 4,
    UnsignedInt32LittleEndian = 5,
    UnsignedInt64BigEndian = 6,
    UnsignedInt64LittleEndian = 7,
    SignedInt8 = 8,
    SignedInt16BigEndian = 10,
    SignedInt16LittleEndian = 11,
    SignedInt32BigEndian = 12,
    SignedInt32LittleEndian = 13,
    SignedInt64BigEndian = 14,
    SignedInt64LittleEndian = 15,
    IeeeFloat32BigEndian = 20,
    IeeeFloat32LittleEndian = 21,
    IeeeFloat64BigEndian = 22,
    IeeeFloat64LittleEndian = 23,
}

// =============================
// Command
// =============================
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 2))]
pub struct Command {
    #[rasn(tag(explicit(context, 0)))]
    pub number: CommandType,
    // options is an OPTIONAL CHOICE with context-specific tags on the alternatives.
    pub options: Option<CommandOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, AsnType, Decode, Encode)]
#[rasn(enumerated, tag(universal, 2))]
pub enum CommandType {
    Subscribe = 30,
    Unsubscribe = 31,
    GetDirectory = 32,
    Invoke = 33,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum CommandOptions {
    #[rasn(tag(explicit(context, 1)))]
    DirFieldMask(FieldFlags),
    #[rasn(tag(explicit(context, 2)))]
    Invocation(Invocation),
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Default, Copy, PartialEq, Eq, AsnType, Decode, Encode,
)]
#[rasn(enumerated, tag(universal, 2))]
pub enum FieldFlags {
    Sparse = -2,
    All = -1,
    #[default]
    Default = 0,
    Identifier = 1,
    Description = 2,
    Tree = 3,
    Value = 4,
    Connections = 5,
}

// =============================
// Node
// =============================
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 3))]
pub struct Node {
    #[rasn(tag(explicit(context, 0)))]
    pub number: Integer32,
    #[rasn(tag(explicit(context, 1)))]
    pub contents: Option<NodeContents>,
    #[rasn(tag(explicit(context, 2)))]
    pub children: Option<ElementCollection>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 10))]
pub struct QualifiedNode {
    #[rasn(tag(explicit(context, 0)))]
    pub path: RelativeOid,
    #[rasn(tag(explicit(context, 1)))]
    pub contents: Option<NodeContents>,
    #[rasn(tag(explicit(context, 2)))]
    pub children: Option<ElementCollection>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, AsnType, Decode, Encode)]
#[rasn(set, tag(universal, 17))]
pub struct NodeContents {
    #[rasn(tag(explicit(context, 0)))]
    pub identifier: Option<EmberString>,
    #[rasn(tag(explicit(context, 1)))]
    pub description: Option<EmberString>,
    #[rasn(tag(explicit(context, 2)))]
    pub is_root: Option<bool>,
    #[rasn(tag(explicit(context, 3)))]
    pub is_online: Option<bool>,
    #[rasn(tag(explicit(context, 4)))]
    pub schema_identifiers: Option<EmberString>,
    #[rasn(tag(explicit(context, 5)))]
    pub template_reference: Option<RelativeOid>,
}

// =============================
// Matrix & Signals
// =============================
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 13))]
pub struct Matrix {
    #[rasn(tag(explicit(context, 0)))]
    pub number: Integer32,
    #[rasn(tag(explicit(context, 1)))]
    pub contents: Option<MatrixContents>,
    #[rasn(tag(explicit(context, 2)))]
    pub children: Option<ElementCollection>,
    #[rasn(tag(explicit(context, 3)))]
    pub targets: Option<TargetCollection>,
    #[rasn(tag(explicit(context, 4)))]
    pub sources: Option<SourceCollection>,
    #[rasn(tag(explicit(context, 5)))]
    pub connections: Option<ConnectionCollection>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(set, tag(universal, 17))]
pub struct MatrixContents {
    #[rasn(tag(explicit(context, 0)))]
    pub identifier: EmberString,
    #[rasn(tag(explicit(context, 1)))]
    pub description: Option<EmberString>,
    #[rasn(tag(explicit(context, 2)))]
    pub r#type: Option<MatrixType>,
    #[rasn(tag(explicit(context, 3)))]
    pub addressing_mode: Option<MatrixAddressingMode>,
    #[rasn(tag(explicit(context, 4)))]
    pub target_count: Integer32,
    #[rasn(tag(explicit(context, 5)))]
    pub source_count: Integer32,
    #[rasn(tag(explicit(context, 6)))]
    pub maximum_total_connects: Option<Integer32>,
    #[rasn(tag(explicit(context, 7)))]
    pub maximum_connects_per_target: Option<Integer32>,
    #[rasn(tag(explicit(context, 8)))]
    pub parameters_location: Option<ParametersLocation>,
    #[rasn(tag(explicit(context, 9)))]
    pub gain_parameter_number: Option<Integer32>,
    #[rasn(tag(explicit(context, 10)))]
    pub labels: Option<LabelCollection>,
    #[rasn(tag(explicit(context, 11)))]
    pub schema_identifiers: Option<EmberString>,
    #[rasn(tag(explicit(context, 12)))]
    pub template_reference: Option<RelativeOid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, AsnType, Decode, Encode)]
#[rasn(enumerated, tag(universal, 2))]
pub enum MatrixType {
    OneToN = 0,
    OneToOne = 1,
    NToN = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, AsnType, Decode, Encode)]
#[rasn(enumerated, tag(universal, 2))]
pub enum MatrixAddressingMode {
    Linear = 0,
    NonLinear = 1,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum ParametersLocation {
    #[rasn(tag(universal, 13))] // RELATIVE-OID
    BasePath(RelativeOid),
    #[rasn(tag(universal, 2))] // INTEGER
    Inline(Integer32),
}

// LabelCollection ::= SEQUENCE OF [0] Label
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(universal, 16), delegate)]
pub struct LabelCollection(pub SequenceOf<TaggedLabel>);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(0))]
pub struct TaggedLabel(pub Label);

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 18))]
pub struct Label {
    #[rasn(tag(explicit(context, 0)))]
    pub base_path: RelativeOid,
    #[rasn(tag(explicit(context, 1)))]
    pub description: EmberString,
}

// TargetCollection ::= SEQUENCE OF [0] Target
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(universal, 16), delegate)]
pub struct TargetCollection(pub SequenceOf<TaggedTarget>);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(0))]
pub struct TaggedTarget(pub Target);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 14))]
pub struct Target(pub Signal);

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(universal, 16))]
pub struct Signal {
    #[rasn(tag(explicit(context, 0)))]
    pub number: Integer32,
    #[rasn(tag(explicit(context, 1)))]
    pub contents: Option<SignalContents>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, AsnType, Decode, Encode)]
#[rasn(set, tag(universal, 17))]
pub struct SignalContents {
    #[rasn(tag(explicit(context, 0)))]
    pub identifier: Option<EmberString>,
    #[rasn(tag(explicit(context, 1)))]
    pub is_online: Option<bool>,
    #[rasn(tag(explicit(context, 2)))]
    pub labels_location: Option<RelativeOid>,
}

// SourceCollection ::= SEQUENCE OF [0] Source
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(universal, 16), delegate)]
pub struct SourceCollection(pub SequenceOf<TaggedSource>);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(0))]
pub struct TaggedSource(pub Source);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 15))]
pub struct Source(pub Signal);

// ConnectionCollection ::= SEQUENCE OF [0] Connection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(universal, 16), delegate)]
pub struct ConnectionCollection(pub SequenceOf<TaggedConnection>);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(0))]
pub struct TaggedConnection(pub Connection);

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 16))]
pub struct Connection {
    #[rasn(tag(explicit(context, 0)))]
    pub target: Integer32,
    #[rasn(tag(explicit(context, 1)))]
    pub sources: Option<PackedNumbers>,
    #[rasn(tag(explicit(context, 2)))]
    pub operation: Option<ConnectionOperation>,
    #[rasn(tag(explicit(context, 3)))]
    pub disposition: Option<ConnectionDisposition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(universal, 13))]
pub struct PackedNumbers(pub RelativeOid);

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, AsnType, Decode, Encode)]
#[rasn(enumerated, tag(universal, 2))]
pub enum ConnectionOperation {
    Absolute = 0,
    Connect = 1,
    Disconnect = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, AsnType, Decode, Encode)]
#[rasn(enumerated, tag(universal, 2))]
pub enum ConnectionDisposition {
    Tally = 0,
    Modified = 1,
    Pending = 2,
    Locked = 3,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 17))]
pub struct QualifiedMatrix {
    #[rasn(tag(explicit(context, 0)))]
    pub path: RelativeOid,
    #[rasn(tag(explicit(context, 1)))]
    pub contents: Option<MatrixContents>,
    #[rasn(tag(explicit(context, 2)))]
    pub children: Option<ElementCollection>,
    #[rasn(tag(explicit(context, 3)))]
    pub targets: Option<TargetCollection>,
    #[rasn(tag(explicit(context, 4)))]
    pub sources: Option<SourceCollection>,
    #[rasn(tag(explicit(context, 5)))]
    pub connections: Option<ConnectionCollection>,
}

// =============================
// Function
// =============================
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 19))]
pub struct Function {
    #[rasn(tag(explicit(context, 0)))]
    pub number: Integer32,
    #[rasn(tag(explicit(context, 1)))]
    pub contents: Option<FunctionContents>,
    #[rasn(tag(explicit(context, 2)))]
    pub children: Option<ElementCollection>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 20))]
pub struct QualifiedFunction {
    #[rasn(tag(explicit(context, 0)))]
    pub path: RelativeOid,
    #[rasn(tag(explicit(context, 1)))]
    pub contents: Option<FunctionContents>,
    #[rasn(tag(explicit(context, 2)))]
    pub children: Option<ElementCollection>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, AsnType, Decode, Encode)]
#[rasn(set, tag(universal, 17))]
pub struct FunctionContents {
    #[rasn(tag(explicit(context, 0)))]
    pub identifier: Option<EmberString>,
    #[rasn(tag(explicit(context, 1)))]
    pub description: Option<EmberString>,
    #[rasn(tag(explicit(context, 2)))]
    pub arguments: Option<TupleDescription>,
    #[rasn(tag(explicit(context, 3)))]
    pub result: Option<TupleDescription>,
    #[rasn(tag(explicit(context, 4)))]
    pub template_reference: Option<RelativeOid>,
}

// TupleDescription ::= SEQUENCE OF [0] TupleItemDescription
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(universal, 16), delegate)]
pub struct TupleDescription(pub SequenceOf<TaggedTupleItemDescription>);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(0))]
pub struct TaggedTupleItemDescription(pub TupleItemDescription);

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 21))]
pub struct TupleItemDescription {
    #[rasn(tag(explicit(context, 0)))]
    pub r#type: ParameterType,
    #[rasn(tag(explicit(context, 1)))]
    pub name: Option<EmberString>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 22))]
pub struct Invocation {
    #[rasn(tag(explicit(context, 0)))]
    pub invocation_id: Option<Integer32>,
    #[rasn(tag(explicit(context, 1)))]
    pub arguments: Option<Tuple>,
}

// Tuple ::= SEQUENCE OF [0] Value
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(universal, 16), delegate)]
pub struct Tuple(pub SequenceOf<TaggedValue>);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(0))]
pub struct TaggedValue(pub Value);

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 23))]
pub struct InvocationResult {
    #[rasn(tag(explicit(context, 0)))]
    pub invocation_id: Integer32,
    #[rasn(tag(explicit(context, 1)))]
    pub success: Option<bool>,
    #[rasn(tag(explicit(context, 2)))]
    pub result: Option<Tuple>,
}

// =============================
// Element & Root
// =============================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 4), delegate)]
pub struct ElementCollection(pub SequenceOf<TaggedElement>);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(0))]
pub struct TaggedElement(pub Element);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum Element {
    Parameter(Parameter),
    Node(Node),
    Command(Command),
    Matrix(Matrix),
    Function(Function),
    Template(Template),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 5))]
pub struct StreamEntry {
    #[rasn(tag(explicit(context, 0)))]
    pub stream_identifier: Integer32,
    #[rasn(tag(explicit(context, 1)))]
    pub stream_value: Value,
}

// StreamCollection ::= [APPLICATION 6] IMPLICIT SEQUENCE OF [0] StreamEntry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 6), delegate)]
pub struct StreamCollection(pub SequenceOf<TaggedStreamEntry>);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(6))]
pub struct TaggedStreamEntry(pub StreamEntry);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(explicit(application, 0)))] // Root ::= [APPLICATION 0] CHOICE { ... } (explicit by module default)
#[rasn(choice)]
pub enum Root {
    Elements(RootElementCollection),
    Streams(StreamCollection),
    InvocationResult(InvocationResult),
}

// RootElementCollection ::= [APPLICATION 11] IMPLICIT SEQUENCE OF [0] RootElement
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 11), delegate)]
pub struct RootElementCollection(pub SequenceOf<TaggedRootElement>);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(0))]
pub struct TaggedRootElement(pub RootElement);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum RootElement {
    Element(Element),
    QualifiedParameter(QualifiedParameter),
    QualifiedNode(QualifiedNode),
    QualifiedMatrix(QualifiedMatrix),
    QualifiedFunction(QualifiedFunction),
    QualifiedTemplate(QualifiedTemplate),
}

mod ext {

    use super::*;
    use crate::{
        ember::{EmberPacket, MAX_PAYLOAD_LEN},
        error::EmberResult,
        s101::Flags,
        utils::{format_byte_size, join},
    };
    use rasn::{Codec, ber};
    use std::{
        fmt::{self, Debug},
        time::Instant,
    };
    #[cfg(feature = "tracing")]
    use tracing::{error, warn};

    pub const GLOW_VERSION_MAJOR: u8 = 2;
    pub const GLOW_VERSION_MINOR: u8 = 50;

    pub trait ToGlow {
        fn to_glow(&self) -> Option<Root>;
    }

    impl<A: AsRef<[u8]>> ToGlow for A {
        fn to_glow(&self) -> Option<Root> {
            match ber::decode::<Root>(self.as_ref()) {
                Ok(it) => Some(it),
                Err(e) => {
                    #[cfg(feature = "tracing")]
                    error!("Error decoding Glow root element: {e}");
                    None
                }
            }
        }
    }

    impl Command {
        pub fn get_directory(flags: Option<FieldFlags>) -> Self {
            Command {
                number: CommandType::GetDirectory,
                options: flags.map(CommandOptions::DirFieldMask),
            }
        }
    }

    impl From<Command> for Root {
        fn from(value: Command) -> Self {
            Root::Elements(RootElementCollection(vec![TaggedRootElement(
                RootElement::Element(Element::Command(value)),
            )]))
        }
    }

    impl Root {
        pub fn to_packets(&self) -> EmberResult<Vec<EmberPacket>> {
            let payload = ber::encode(self)?;
            let packet_count = packet_count(&payload);
            let mut packets = Vec::with_capacity(packet_count);
            for i in 0..packet_count {
                packets.push(EmberPacket::new(
                    Self::flag(packet_count, i),
                    GLOW_VERSION_MAJOR,
                    GLOW_VERSION_MINOR,
                    payload[i * MAX_PAYLOAD_LEN..((i + 1) * MAX_PAYLOAD_LEN).min(payload.len())]
                        .to_owned(),
                ));
            }
            Ok(packets)
        }

        pub fn from_packets(packets: &[EmberPacket]) -> EmberResult<Root> {
            let reconstructed_payload = packets
                .iter()
                .flat_map(|p| p.payload())
                .copied()
                .collect::<Vec<u8>>();
            #[cfg(feature = "tracing")]
            let start = Instant::now();
            let root = ber::decode::<Root>(&reconstructed_payload)?;
            #[cfg(feature = "tracing")]
            let end = Instant::now();
            if packets.len() >= 500 {
                #[cfg(feature = "tracing")]
                warn!(
                    "Total payload size of multi-packet message: {}; decoding took {:.2} seconds",
                    format_byte_size(reconstructed_payload.len()),
                    (end - start).as_millis() as f32 / 1_000.0
                );
            }
            Ok(root)
        }

        fn flag(packet_count: usize, packet_index: usize) -> Flags {
            if packet_count < 1 {
                Flags::EmptyPacket
            } else if packet_count == 1 {
                Flags::SinglePacket
            } else if packet_index == 0 {
                Flags::MultiPacketFirst
            } else if packet_index == packet_count - 1 {
                Flags::MultiPacketLast
            } else {
                Flags::MultiPacket
            }
        }

        fn command(command: Command) -> Root {
            Root::Elements(RootElementCollection(vec![TaggedRootElement(
                RootElement::Element(Element::Command(command)),
            )]))
        }

        fn element(element: Element) -> Root {
            Root::Elements(RootElementCollection(vec![TaggedRootElement(
                RootElement::Element(element),
            )]))
        }

        fn qualified_node(node: QualifiedNode) -> Root {
            Root::Elements(RootElementCollection(vec![TaggedRootElement(
                RootElement::QualifiedNode(node),
            )]))
        }

        fn qualified_parameter(parameter: QualifiedParameter) -> Root {
            Root::Elements(RootElementCollection(vec![TaggedRootElement(
                RootElement::QualifiedParameter(parameter),
            )]))
        }

        fn qualified_matrix(matrix: QualifiedMatrix) -> Root {
            Root::Elements(RootElementCollection(vec![TaggedRootElement(
                RootElement::QualifiedMatrix(matrix),
            )]))
        }

        fn qualified_template(template: QualifiedTemplate) -> Root {
            Root::Elements(RootElementCollection(vec![TaggedRootElement(
                RootElement::QualifiedTemplate(template),
            )]))
        }

        fn qualified_function(function: QualifiedFunction) -> Root {
            Root::Elements(RootElementCollection(vec![TaggedRootElement(
                RootElement::QualifiedFunction(function),
            )]))
        }
    }

    pub(crate) fn packet_count(payload: &[u8]) -> usize {
        if payload.is_empty() {
            0
        } else {
            ((payload.len() as f32 / MAX_PAYLOAD_LEN as f32).ceil()) as usize
        }
    }

    impl fmt::Display for Root {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}",
                serde_json::to_string_pretty(self).expect("invalid json")
            )
        }
    }

    impl ElementCollection {
        fn command(commad: Command) -> ElementCollection {
            ElementCollection(vec![TaggedElement(Element::Command(commad))])
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum TreeNode {
        Root,
        Node(Node),
        QualifiedNode(QualifiedNode),
        Matrix(Matrix),
        QualifiedMatrix(QualifiedMatrix),
        Parameter(Parameter),
        QualifiedParameter(QualifiedParameter),
        Template(Template),
        QualifiedTemplate(QualifiedTemplate),
    }

    impl TreeNode {
        pub fn id(&self) -> Option<&str> {
            match self {
                TreeNode::Node(node) => node.id(),
                TreeNode::QualifiedNode(qualified_node) => qualified_node.id(),
                TreeNode::Matrix(matrix) => matrix.id(),
                TreeNode::QualifiedMatrix(qualified_matrix) => qualified_matrix.id(),
                TreeNode::Parameter(parameter) => parameter.id(),
                TreeNode::QualifiedParameter(qualified_parameter) => qualified_parameter.id(),
                TreeNode::Root | TreeNode::Template(_) | TreeNode::QualifiedTemplate(_) => None,
            }
        }

        pub fn get_directory(self, parent_path: &RelativeOid) -> Option<(RelativeOid, Root)> {
            let command = Command::get_directory(Some(FieldFlags::All));
            match self {
                TreeNode::Root => Some((RelativeOid::root(), Root::command(command))),
                TreeNode::Node(node) => {
                    let oid = join(parent_path, node.number);
                    Some((
                        oid.clone(),
                        Root::qualified_node(QualifiedNode::command(oid, command)),
                    ))
                }
                TreeNode::QualifiedNode(mut qualified_node) => {
                    qualified_node.contents = None;
                    qualified_node.children = Some(ElementCollection::command(command));
                    let oid = qualified_node.path.clone();
                    Some((oid, Root::qualified_node(qualified_node)))
                }
                TreeNode::Matrix(matrix) => {
                    let oid = join(parent_path, matrix.number);
                    Some((
                        oid.clone(),
                        Root::qualified_matrix(QualifiedMatrix::command(oid, command)),
                    ))
                }
                TreeNode::QualifiedMatrix(mut qualified_matrix) => {
                    qualified_matrix.contents = None;
                    qualified_matrix.children = Some(ElementCollection::command(command));
                    let oid = qualified_matrix.path.clone();
                    Some((oid, Root::qualified_matrix(qualified_matrix)))
                }
                TreeNode::Parameter(parameter) => {
                    let oid = join(parent_path, parameter.number);
                    Some((
                        oid.clone(),
                        Root::qualified_parameter(QualifiedParameter::command(oid, command)),
                    ))
                }
                TreeNode::QualifiedParameter(mut qualified_parameter) => {
                    qualified_parameter.contents = None;
                    qualified_parameter.children = Some(ElementCollection::command(command));
                    let oid = qualified_parameter.path.clone();
                    Some((oid, Root::qualified_parameter(qualified_parameter)))
                }
                TreeNode::Template(_) | TreeNode::QualifiedTemplate(_) => None,
            }
        }

        pub fn oid(&self, parent: &RelativeOid) -> RelativeOid {
            match self {
                TreeNode::Root => RelativeOid::root(),
                TreeNode::Node(node) => join(parent, node.number),
                TreeNode::QualifiedNode(qualified_node) => qualified_node.path.clone(),
                TreeNode::Matrix(matrix) => join(parent, matrix.number),
                TreeNode::QualifiedMatrix(qualified_matrix) => qualified_matrix.path.clone(),
                TreeNode::Parameter(parameter) => join(parent, parameter.number),
                TreeNode::QualifiedParameter(qualified_parameter) => {
                    qualified_parameter.path.clone()
                }
                TreeNode::Template(template) => join(parent, template.number),
                TreeNode::QualifiedTemplate(qualified_template) => qualified_template.path.clone(),
            }
        }

        pub fn is_empty(&self) -> bool {
            match self {
                TreeNode::Root => false,
                TreeNode::Node(node) => node.is_empty(),
                TreeNode::QualifiedNode(qualified_node) => qualified_node.is_empty(),
                TreeNode::Matrix(_) | TreeNode::QualifiedMatrix(_) => true,
                TreeNode::Parameter(parameter) => parameter.is_empty(),
                TreeNode::QualifiedParameter(qualified_parameter) => qualified_parameter.is_empty(),
                TreeNode::Template(_) | TreeNode::QualifiedTemplate(_) => true,
            }
        }

        pub fn is_online(&self) -> bool {
            match self {
                TreeNode::Root => true,
                TreeNode::Node(node) => node.is_online(),
                TreeNode::QualifiedNode(qualified_node) => qualified_node.is_online(),
                TreeNode::Matrix(_) | TreeNode::QualifiedMatrix(_) => true,
                TreeNode::Parameter(parameter) => parameter.is_online(),
                TreeNode::QualifiedParameter(qualified_parameter) => {
                    qualified_parameter.is_online()
                }
                TreeNode::Template(_) | TreeNode::QualifiedTemplate(_) => true,
            }
        }

        pub(crate) fn children(self, parent: &RelativeOid) -> Option<(RelativeOid, Vec<TreeNode>)> {
            match self {
                TreeNode::Node(Node {
                    children: Some(children),
                    number,
                    contents: _,
                }) => Some((
                    join(parent, number),
                    children.0.into_iter().filter_map(|it| it.into()).collect(),
                )),
                TreeNode::QualifiedNode(QualifiedNode {
                    path,
                    contents: _,
                    children: Some(children),
                }) => Some((
                    path,
                    children.0.into_iter().filter_map(|it| it.into()).collect(),
                )),
                TreeNode::Matrix(Matrix {
                    number,
                    contents: _,
                    children: Some(children),
                    targets: _,
                    sources: _,
                    connections: _,
                }) => Some((
                    join(parent, number),
                    children.0.into_iter().filter_map(|it| it.into()).collect(),
                )),
                TreeNode::QualifiedMatrix(QualifiedMatrix {
                    path,
                    contents: _,
                    children: Some(children),
                    targets: _,
                    sources: _,
                    connections: _,
                }) => Some((
                    path,
                    children.0.into_iter().filter_map(|it| it.into()).collect(),
                )),
                TreeNode::Parameter(Parameter {
                    number,
                    children: Some(children),
                    contents: _,
                }) => Some((
                    join(parent, number),
                    children.0.into_iter().filter_map(|it| it.into()).collect(),
                )),
                TreeNode::QualifiedParameter(QualifiedParameter {
                    path,
                    contents: _,
                    children: Some(children),
                }) => Some((
                    path,
                    children.0.into_iter().filter_map(|it| it.into()).collect(),
                )),
                _ => None,
            }
        }

        pub(crate) fn may_have_children(&self) -> bool {
            match self {
                TreeNode::Root => true,
                TreeNode::Node(n) => !n.is_empty(),
                TreeNode::QualifiedNode(n) => !n.is_empty(),
                TreeNode::Parameter(_) | TreeNode::QualifiedParameter(_) => false,
                TreeNode::Matrix(_)
                | TreeNode::QualifiedMatrix(_)
                | TreeNode::Template(_)
                | TreeNode::QualifiedTemplate(_) => todo!(),
            }
        }
    }

    impl fmt::Display for TreeNode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}",
                serde_json::to_string_pretty(self).expect("invalid json")
            )
        }
    }

    impl From<TaggedElement> for Option<TreeNode> {
        fn from(value: TaggedElement) -> Self {
            match value.0 {
                Element::Parameter(parameter) => Some(TreeNode::Parameter(parameter)),
                Element::Node(node) => Some(TreeNode::Node(node)),
                Element::Command(_) => None,
                Element::Matrix(matrix) => Some(TreeNode::Matrix(matrix)),
                Element::Function(_) => None,
                Element::Template(template) => Some(TreeNode::Template(template)),
            }
        }
    }

    impl Node {
        pub fn id(&self) -> Option<&str> {
            self.contents.as_ref().and_then(|c| c.identifier.as_deref())
        }

        pub fn is_empty(&self) -> bool {
            self.children
                .as_ref()
                .map(|c| c.0.is_empty())
                .unwrap_or(true)
                && self.contents.as_ref().map(|c| c.is_empty()).unwrap_or(true)
        }

        fn is_online(&self) -> bool {
            let Some(contents) = &self.contents else {
                return true;
            };
            contents.is_online.unwrap_or(true)
        }
    }

    impl QualifiedNode {
        pub fn id(&self) -> Option<&str> {
            self.contents.as_ref().and_then(|c| c.identifier.as_deref())
        }

        pub fn command(path: RelativeOid, command: Command) -> QualifiedNode {
            QualifiedNode {
                path,
                children: Some(ElementCollection(vec![TaggedElement(Element::Command(
                    command,
                ))])),
                contents: None,
            }
        }

        pub fn is_empty(&self) -> bool {
            self.children
                .as_ref()
                .map(|c| c.0.is_empty())
                .unwrap_or(true)
                && self.contents.as_ref().map(|c| c.is_empty()).unwrap_or(true)
        }

        fn is_online(&self) -> bool {
            let Some(contents) = &self.contents else {
                return true;
            };
            contents.is_online.unwrap_or(true)
        }
    }

    impl NodeContents {
        pub fn is_empty(&self) -> bool {
            self.description.is_none()
                && self.identifier.is_none()
                // omitting this check to be compatible with TinyEmber, which sets isOnline on empty in spite of what the spec says
                // && self.is_online.is_none()
                && self.is_root.is_none()
                && self.schema_identifiers.is_none()
                && self.template_reference.is_none()
        }
    }

    impl Parameter {
        pub fn id(&self) -> Option<&str> {
            self.contents.as_ref().and_then(|c| c.identifier.as_deref())
        }

        pub fn is_empty(&self) -> bool {
            self.children
                .as_ref()
                .map(|c| c.0.is_empty())
                .unwrap_or(true)
                && self.contents.as_ref().map(|c| c.is_empty()).unwrap_or(true)
        }

        fn is_online(&self) -> bool {
            let Some(contents) = &self.contents else {
                return true;
            };
            contents.is_online.unwrap_or(true)
        }

        pub fn value(&self) -> Option<Value> {
            self.contents.clone().and_then(|c| c.param_value.clone())
        }
    }

    impl QualifiedParameter {
        pub fn id(&self) -> Option<&str> {
            self.contents.as_ref().and_then(|c| c.identifier.as_deref())
        }

        pub fn command(path: RelativeOid, command: Command) -> QualifiedParameter {
            QualifiedParameter {
                path,
                children: Some(ElementCollection(vec![TaggedElement(Element::Command(
                    command,
                ))])),
                contents: None,
            }
        }

        pub fn is_empty(&self) -> bool {
            self.children
                .as_ref()
                .map(|c| c.0.is_empty())
                .unwrap_or(true)
                && self.contents.as_ref().map(|c| c.is_empty()).unwrap_or(true)
        }

        fn is_online(&self) -> bool {
            let Some(contents) = &self.contents else {
                return true;
            };
            contents.is_online.unwrap_or(true)
        }

        pub fn value(&self) -> Option<Value> {
            self.contents.clone().and_then(|c| c.param_value.clone())
        }
    }

    impl ParameterContents {
        pub fn is_empty(&self) -> bool {
            self.access.is_none()
                && self.default.is_none()
                && self.description.is_none()
                && self.enum_map.is_none()
                && self.enumeration.is_none()
                && self.factor.is_none()
                && self.format.is_none()
                && self.formula.is_none()
                && self.identifier.is_none()
            // omitting this check to be compatible with TinyEmber, which sets isOnline on empty in spite of what the spec says
            // && self.is_online.is_none()
        }
    }

    impl Signal {
        fn is_online(&self) -> bool {
            let Some(contents) = &self.contents else {
                return true;
            };
            contents.is_online.unwrap_or(true)
        }
    }

    impl Matrix {
        pub fn id(&self) -> Option<&str> {
            self.contents.as_ref().map(|c| c.identifier.as_str())
        }
    }

    impl QualifiedMatrix {
        pub fn id(&self) -> Option<&str> {
            self.contents.as_ref().map(|c| c.identifier.as_str())
        }

        pub fn command(path: RelativeOid, command: Command) -> QualifiedMatrix {
            QualifiedMatrix {
                path,
                children: Some(ElementCollection(vec![TaggedElement(Element::Command(
                    command,
                ))])),
                contents: None,
                connections: None,
                sources: None,
                targets: None,
            }
        }
    }

    impl QualifiedFunction {
        pub fn command(path: RelativeOid, command: Command) -> QualifiedFunction {
            QualifiedFunction {
                path,
                children: Some(ElementCollection(vec![TaggedElement(Element::Command(
                    command,
                ))])),
                contents: None,
            }
        }
    }

    impl Encode for RelativeOid {
        fn encode_with_tag_and_constraints<'b, E: Encoder<'b>>(
            &self,
            encoder: &mut E,
            tag: rasn::prelude::Tag,
            constraints: rasn::prelude::Constraints,
            identifier: rasn::prelude::Identifier,
        ) -> Result<(), E::Error> {
            let mut content = Vec::new();
            for &arc in &self.0 {
                // base-128 encode each arc
                let mut buf = [0u8; 5];
                let mut i = buf.len();
                let mut v = arc as u64;
                loop {
                    i -= 1;
                    buf[i] = (v & 0x7f) as u8;
                    v >>= 7;
                    if v == 0 {
                        break;
                    }
                }
                // set continuation bits for all but the last
                for j in i..buf.len() - 1 {
                    buf[j] |= 0x80;
                }
                content.extend_from_slice(&buf[i..]);
            }
            // write a primitive with RELATIVE-OID tag
            encoder.encode_octet_string(tag, constraints, &content, identifier)?;
            Ok(())
        }
    }

    impl RelativeOid {
        pub fn parent(&self) -> RelativeOid {
            if self.0.is_empty() {
                self.clone()
            } else {
                RelativeOid(self.0[..self.0.len() - 1].to_owned())
            }
        }

        pub(crate) fn root() -> RelativeOid {
            RelativeOid(vec![])
        }
    }

    impl Decode for RelativeOid {
        fn decode_with_tag_and_constraints<D: Decoder>(
            decoder: &mut D,
            tag: rasn::prelude::Tag,
            constraints: rasn::prelude::Constraints,
        ) -> Result<Self, D::Error> {
            let bytes: Vec<u8> = decoder.decode_octet_string(tag, constraints)?; // raw content octets
            // parse base-128 arcs
            let mut arcs = Vec::new();
            let mut cur: u32 = 0;
            for &b in &bytes {
                cur = (cur << 7) | u32::from(b & 0x7f);
                if (b & 0x80) == 0 {
                    arcs.push(cur);
                    cur = 0;
                }
            }
            if (bytes.last().copied().unwrap_or(0) & 0x80) != 0 {
                return Err(D::Error::custom(
                    "unterminated RELATIVE-OID arc",
                    Codec::Ber,
                ));
            }
            Ok(RelativeOid(arcs))
        }
    }

    impl fmt::Display for RelativeOid {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                ".{}",
                self.0
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(".")
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::s101::{Flags, NonEscapingS101Frame};
    use rasn::ber;
    use std::fs;

    #[test]
    fn serde_roundtrip() {
        let original = Root::Elements(RootElementCollection(vec![TaggedRootElement(
            RootElement::Element(Element::Command(Command {
                number: CommandType::GetDirectory,
                options: Some(CommandOptions::DirFieldMask(FieldFlags::All)),
            })),
        )]));
        let encoded = ber::encode(&original).unwrap();
        let decoded = ber::decode(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn get_dir_is_encoded_correctly() {
        let expected: Vec<u8> = vec![
            0x60, 0x10, 0x6b, 0x0e, 0xa0, 0x0c, 0x62, 0x0a, 0xa0, 0x3, 0x2, 0x1, 0x20, 0xa1, 0x3,
            0x2, 0x1, 0xff,
        ];
        let root = Root::from(Command::get_directory(Some(FieldFlags::All)));
        let encoded = ber::encode(&root).unwrap();
        assert_eq!(expected, encoded);
    }

    #[test]
    fn get_dir_is_decoded_correctly() {
        let input: Vec<u8> = vec![
            0x60, 0x17, 0x6b, 0x15, 0xa0, 0x13, 0x62, 0x11, 0xa0, 0x3, 0x2, 0x1, 0x20, 0xa1, 0xa,
            0x2, 0x8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ];
        let decoded = ber::decode::<Root>(&input).unwrap();
        let expected = Root::from(Command::get_directory(Some(FieldFlags::All)));
        assert_eq!(expected, decoded);
    }

    #[test]
    fn to_single_packet_works() {
        let original = Root::from(Command::get_directory(Some(FieldFlags::All)));
        let packets = original.to_packets().unwrap();
        assert_eq!(1, packets.len());
        assert_eq!(Flags::SinglePacket, packets[0].flag());
        let mut buf = [0u8; 1290];
        for p in packets.clone() {
            let frame = NonEscapingS101Frame::EmberPacket(p);
            frame.encode(&mut buf);
        }
        let reconstructed = Root::from_packets(&packets).unwrap();
        assert_eq!(original, reconstructed);
    }

    #[test]
    fn to_two_packets_works() {
        let original = xl_root();
        let packets = original.to_packets().unwrap();
        assert_eq!(2, packets.len());
        assert_eq!(Flags::MultiPacketFirst, packets[0].flag());
        assert_eq!(Flags::MultiPacketLast, packets[1].flag());
        let mut buf = [0u8; 1290];
        for p in packets.clone() {
            let frame = NonEscapingS101Frame::EmberPacket(p);
            frame.encode(&mut buf);
        }
        let reconstructed = Root::from_packets(&packets).unwrap();
        assert_eq!(original, reconstructed);
    }

    #[test]
    fn to_multi_packets_works() {
        let original = xxl_root();
        let packets = original.to_packets().unwrap();
        assert_eq!(4, packets.len());
        assert_eq!(Flags::MultiPacketFirst, packets[0].flag());
        assert_eq!(Flags::MultiPacket, packets[1].flag());
        assert_eq!(Flags::MultiPacket, packets[2].flag());
        assert_eq!(Flags::MultiPacketLast, packets[3].flag());
        let mut buf = [0u8; 1290];
        for p in packets.clone() {
            let frame = NonEscapingS101Frame::EmberPacket(p);
            frame.encode(&mut buf);
        }
        let reconstructed = Root::from_packets(&packets).unwrap();
        assert_eq!(original, reconstructed);
    }

    #[test]
    fn packet_count_is_calculated_correctly() {
        let payload = [];
        assert_eq!(0, packet_count(&payload));

        let payload = [0; 1];
        assert_eq!(1, packet_count(&payload));

        let payload = [0; 10];
        assert_eq!(1, packet_count(&payload));

        let payload = [0; 100];
        assert_eq!(1, packet_count(&payload));

        let payload = [0; 1000];
        assert_eq!(1, packet_count(&payload));

        let payload = [0; 1024];
        assert_eq!(1, packet_count(&payload));

        let payload = [0; 1025];
        assert_eq!(2, packet_count(&payload));

        let payload = [0; 2047];
        assert_eq!(2, packet_count(&payload));

        let payload = [0; 2048];
        assert_eq!(2, packet_count(&payload));

        let payload = [0; 2049];
        assert_eq!(3, packet_count(&payload));

        let payload = [0; 3071];
        assert_eq!(3, packet_count(&payload));

        let payload = [0; 3072];
        assert_eq!(3, packet_count(&payload));

        let payload = [0; 3073];
        assert_eq!(4, packet_count(&payload));
    }

    #[test]
    fn relative_oid_is_encoded_correctly() {
        let oid = RelativeOid(vec![1, 2, 3]);
        let encoded = ber::encode(&oid).unwrap();
        eprintln!(
            "oid: [{}]",
            encoded
                .iter()
                .map(|it| format!("0x{it:02x}"))
                .collect::<Vec<String>>()
                .join(", ")
        );
        let expected = vec![0x0d, 0x03, 0x01, 0x02, 0x03];
        assert_eq!(expected, encoded);
    }

    #[test]
    fn big_relative_oid_is_encoded_correctly() {
        let oid = RelativeOid(vec![16383]);
        let encoded = ber::encode(&oid).unwrap();
        eprintln!(
            "oid: [{}]",
            encoded
                .iter()
                .map(|it| format!("0x{it:02x}"))
                .collect::<Vec<String>>()
                .join(", ")
        );
        let expected = vec![0x0d, 0x02, 0xff, 0x7f];
        assert_eq!(expected, encoded);
    }

    #[test]
    fn parameters_are_decoded_correctly() {
        let input = [
            0x60, 0x62, 0x6b, 0x60, 0xa0, 0x2e, 0x69, 0x2c, 0xa0, 0x05, 0x0d, 0x03, 0x01, 0x01,
            0x01, 0xa1, 0x23, 0x31, 0x21, 0xa0, 0x04, 0x0c, 0x02, 0x50, 0x31, 0xa1, 0x04, 0x0c,
            0x02, 0x50, 0x31, 0xa5, 0x03, 0x02, 0x01, 0x03, 0xa4, 0x04, 0x02, 0x02, 0x03, 0xe8,
            0xa3, 0x03, 0x02, 0x01, 0x00, 0xa2, 0x03, 0x02, 0x01, 0x00, 0xa0, 0x2e, 0x69, 0x2c,
            0xa0, 0x05, 0x0d, 0x03, 0x01, 0x01, 0x02, 0xa1, 0x23, 0x31, 0x21, 0xa0, 0x04, 0x0c,
            0x02, 0x50, 0x32, 0xa1, 0x04, 0x0c, 0x02, 0x50, 0x32, 0xa5, 0x03, 0x02, 0x01, 0x03,
            0xa4, 0x04, 0x02, 0x02, 0x03, 0xe8, 0xa3, 0x03, 0x02, 0x01, 0x00, 0xa2, 0x03, 0x02,
            0x01, 0x00,
        ];
        let decoded = ber::decode::<Root>(&input).unwrap();
        let expected = Root::Elements(RootElementCollection(vec![
            TaggedRootElement(RootElement::QualifiedParameter(QualifiedParameter {
                path: RelativeOid(vec![1, 1, 1]),
                children: None,
                contents: Some(ParameterContents {
                    identifier: Some("P1".to_owned()),
                    description: Some("P1".to_owned()),
                    param_value: Some(Value::Integer(0)),
                    minimum: Some(MinMax::Integer(0)),
                    maximum: Some(MinMax::Integer(1000)),
                    access: Some(ParameterAccess::ReadWrite),
                    format: None,
                    enumeration: None,
                    factor: None,
                    is_online: None,
                    formula: None,
                    step: None,
                    default: None,
                    r#type: None,
                    stream_identifier: None,
                    enum_map: None,
                    stream_descriptor: None,
                    schema_identifiers: None,
                    template_reference: None,
                }),
            })),
            TaggedRootElement(RootElement::QualifiedParameter(QualifiedParameter {
                path: RelativeOid(vec![1, 1, 2]),
                children: None,
                contents: Some(ParameterContents {
                    identifier: Some("P2".to_owned()),
                    description: Some("P2".to_owned()),
                    param_value: Some(Value::Integer(0)),
                    minimum: Some(MinMax::Integer(0)),
                    maximum: Some(MinMax::Integer(1000)),
                    access: Some(ParameterAccess::ReadWrite),
                    format: None,
                    enumeration: None,
                    factor: None,
                    is_online: None,
                    formula: None,
                    step: None,
                    default: None,
                    r#type: None,
                    stream_identifier: None,
                    enum_map: None,
                    stream_descriptor: None,
                    schema_identifiers: None,
                    template_reference: None,
                }),
            })),
        ]));

        assert_eq!(expected, decoded);
    }

    #[test]
    fn parameter_is_decoded_correctly() {
        let input = vec![
            0x60, 0x32, 0x6b, 0x30, 0xa0, 0x2e, 0x69, 0x2c, 0xa0, 0x5, 0xd, 0x3, 0x1, 0x1, 0x1,
            0xa1, 0x23, 0x31, 0x21, 0xa0, 0x4, 0xc, 0x2, 0x50, 0x31, 0xa1, 0x4, 0xc, 0x2, 0x50,
            0x31, 0xa5, 0x3, 0x2, 0x1, 0x3, 0xa4, 0x4, 0x2, 0x2, 0x3, 0xe8, 0xa3, 0x3, 0x2, 0x1,
            0x0, 0xa2, 0x3, 0x2, 0x1, 0x0,
        ];

        let decoded = ber::decode::<Root>(&input).unwrap();

        let expected = Root::Elements(RootElementCollection(vec![TaggedRootElement(
            RootElement::QualifiedParameter(QualifiedParameter {
                path: RelativeOid(vec![1, 1, 1]),
                children: None,
                contents: Some(ParameterContents {
                    identifier: Some("P1".to_owned()),
                    description: Some("P1".to_owned()),
                    param_value: Some(Value::Integer(0)),
                    minimum: Some(MinMax::Integer(0)),
                    maximum: Some(MinMax::Integer(1000)),
                    access: Some(ParameterAccess::ReadWrite),
                    format: None,
                    enumeration: None,
                    factor: None,
                    is_online: None,
                    formula: None,
                    step: None,
                    default: None,
                    r#type: None,
                    stream_identifier: None,
                    enum_map: None,
                    stream_descriptor: None,
                    schema_identifiers: None,
                    template_reference: None,
                }),
            }),
        )]));

        assert_eq!(expected, decoded);
    }

    #[test]
    fn parameter_is_encoded_correctly() {
        let root = Root::Elements(RootElementCollection(vec![TaggedRootElement(
            RootElement::QualifiedParameter(QualifiedParameter {
                path: RelativeOid(vec![1, 1, 1]),
                children: None,
                contents: Some(ParameterContents {
                    identifier: Some("P1".to_owned()),
                    description: Some("P1".to_owned()),
                    param_value: Some(Value::Integer(0)),
                    minimum: Some(MinMax::Integer(0)),
                    maximum: Some(MinMax::Integer(0)),
                    access: Some(ParameterAccess::ReadWrite),
                    format: None,
                    enumeration: None,
                    factor: None,
                    is_online: None,
                    formula: None,
                    step: None,
                    default: None,
                    r#type: None,
                    stream_identifier: None,
                    enum_map: None,
                    stream_descriptor: None,
                    schema_identifiers: None,
                    template_reference: None,
                }),
            }),
        )]));

        let encoded = ber::encode(&root).unwrap();
        let decoded = ber::decode::<Root>(&encoded).unwrap();

        assert_eq!(root, decoded);
    }

    #[test]
    fn parameters_are_encoded_correctly() {
        let root = Root::Elements(RootElementCollection(vec![
            TaggedRootElement(RootElement::QualifiedParameter(QualifiedParameter {
                path: RelativeOid(vec![1, 1, 1]),
                children: None,
                contents: Some(ParameterContents {
                    identifier: Some("P1".to_owned()),
                    description: Some("P1".to_owned()),
                    param_value: Some(Value::Integer(0)),
                    minimum: Some(MinMax::Integer(0)),
                    maximum: Some(MinMax::Integer(1000)),
                    access: Some(ParameterAccess::ReadWrite),
                    format: None,
                    enumeration: None,
                    factor: None,
                    is_online: None,
                    formula: None,
                    step: None,
                    default: None,
                    r#type: None,
                    stream_identifier: None,
                    enum_map: None,
                    stream_descriptor: None,
                    schema_identifiers: None,
                    template_reference: None,
                }),
            })),
            TaggedRootElement(RootElement::QualifiedParameter(QualifiedParameter {
                path: RelativeOid(vec![1, 1, 2]),
                children: None,
                contents: Some(ParameterContents {
                    identifier: Some("P2".to_owned()),
                    description: Some("P2".to_owned()),
                    param_value: Some(Value::Integer(0)),
                    minimum: Some(MinMax::Integer(0)),
                    maximum: Some(MinMax::Integer(1000)),
                    access: Some(ParameterAccess::ReadWrite),
                    format: None,
                    enumeration: None,
                    factor: None,
                    is_online: None,
                    formula: None,
                    step: None,
                    default: None,
                    r#type: None,
                    stream_identifier: None,
                    enum_map: None,
                    stream_descriptor: None,
                    schema_identifiers: None,
                    template_reference: None,
                }),
            })),
        ]));

        let encoded = ber::encode(&root).unwrap();
        let decoded = ber::decode::<Root>(&encoded).unwrap();

        assert_eq!(root, decoded);
    }

    #[test]
    fn node_is_decoded_correctly() {
        let expected = Root::Elements(RootElementCollection(vec![TaggedRootElement(
            RootElement::Element(Element::Node(Node {
                number: 1,
                contents: Some(NodeContents {
                    identifier: Some("Device".into()),
                    description: Some("Device".into()),
                    is_root: None,
                    is_online: Some(true),
                    schema_identifiers: None,
                    template_reference: None,
                }),
                children: None,
            })),
        )]));
        let input = vec![
            0x60, 0x28, 0x6b, 0x26, 0xa0, 0x24, 0x63, 0x22, 0xa0, 0x3, 0x2, 0x1, 0x1, 0xa1, 0x1b,
            0x31, 0x19, 0xa0, 0x8, 0xc, 0x6, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0xa1, 0x8, 0xc,
            0x6, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0xa3, 0x3, 0x1, 0x1, 0xff,
        ];
        let decoded = ber::decode::<Root>(&input).unwrap();

        assert_eq!(expected, decoded);
    }

    #[test]
    fn node_is_encoded_correctly() {
        let node = Root::Elements(RootElementCollection(vec![TaggedRootElement(
            RootElement::Element(Element::Node(Node {
                number: 1,
                contents: Some(NodeContents {
                    identifier: Some("Device".into()),
                    description: Some("Device".into()),
                    is_root: None,
                    is_online: Some(true),
                    schema_identifiers: None,
                    template_reference: None,
                }),
                children: None,
            })),
        )]));
        let expected = vec![
            0x60, 0x28, 0x6b, 0x26, 0xa0, 0x24, 0x63, 0x22, 0xa0, 0x3, 0x2, 0x1, 0x1, 0xa1, 0x1b,
            0x31, 0x19, 0xa0, 0x8, 0xc, 0x6, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0xa1, 0x8, 0xc,
            0x6, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0xa3, 0x3, 0x1, 0x1, 0xff,
        ];
        let encoded = ber::encode(&node).unwrap();

        assert_eq!(expected, encoded);
    }

    #[test]
    fn big_message_is_decoded_in_reasonable_time() {
        let data = std::fs::read("./large.EmBER").unwrap();
        let start = std::time::Instant::now();
        let _ = ber::decode::<Root>(&data).unwrap();
        eprintln!("EmBER+ BER decode took {:?}", start.elapsed());
        assert!(start.elapsed().as_millis() < 100);
    }

    #[test]
    fn examples_are_decoded_correctly() {
        let _root = ber::decode::<Root>(&fs::read("./test/DHD_Example1.EmBER").unwrap()).unwrap();
        // eprintln!("{}", _root);
        let _root = ber::decode::<Root>(&fs::read("./test/DHD_Example2.EmBER").unwrap()).unwrap();
        // eprintln!("{}", _root);
        let _root = ber::decode::<Root>(&fs::read("./test/RAVENNAnet.EmBER").unwrap()).unwrap();
        // eprintln!("{}", _root);
        let _root = ber::decode::<Root>(&fs::read("./test/sapphire.EmBER").unwrap()).unwrap();
        // eprintln!("{}", _root);
    }

    fn xl_root() -> Root {
        let command = Command::get_directory(Some(FieldFlags::All));
        let element = wrapped_element(75, Element::Command(command));
        Root::Elements(RootElementCollection(vec![TaggedRootElement(
            RootElement::Element(element),
        )]))
    }

    fn xxl_root() -> Root {
        let command = Command::get_directory(Some(FieldFlags::All));
        let element = wrapped_element(150, Element::Command(command));
        Root::Elements(RootElementCollection(vec![TaggedRootElement(
            RootElement::Element(element),
        )]))
    }

    fn wrapped_element(size: usize, content: Element) -> Element {
        let mut element = content;
        for i in 0..size {
            element = Element::Node(Node {
                number: i as i32,
                contents: None,
                children: Some(ElementCollection(vec![TaggedElement(element)])),
            })
        }
        element
    }
}
