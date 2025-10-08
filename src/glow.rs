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

use rasn::{AsnType, Decode, Decoder, Encode, Encoder, types::ObjectIdentifier};

// =============================
// Primitive aliases
// =============================
pub type EmberString = String;
pub type Integer32 = i32; // INTEGER (-2^31 .. 2^31-1)
pub type Integer64 = i64; // INTEGER (-2^63 .. 2^63-1)

// =============================
// Template
// =============================
#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 24))]
pub struct Template {
    #[rasn(tag(explicit(context, 0)))]
    pub number: Integer32,
    #[rasn(tag(explicit(context, 1)))]
    pub element: Option<TemplateElement>,
    #[rasn(tag(explicit(context, 2)))]
    pub description: Option<EmberString>,
}

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 25))]
pub struct QualifiedTemplate {
    #[rasn(tag(explicit(context, 0)))]
    pub path: ObjectIdentifier,
    #[rasn(tag(explicit(context, 1)))]
    pub element: Option<TemplateElement>,
    #[rasn(tag(explicit(context, 2)))]
    pub description: Option<EmberString>,
}

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
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
#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 1))]
pub struct Parameter {
    #[rasn(tag(explicit(context, 0)))]
    pub number: Integer32,
    #[rasn(tag(explicit(context, 1)))]
    pub contents: Option<ParameterContents>,
    #[rasn(tag(explicit(context, 2)))]
    pub children: Option<ElementCollection>,
}

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 9))]
pub struct QualifiedParameter {
    #[rasn(tag(explicit(context, 0)))]
    pub path: ObjectIdentifier,
    #[rasn(tag(explicit(context, 1)))]
    pub contents: Option<ParameterContents>,
    #[rasn(tag(explicit(context, 2)))]
    pub children: Option<ElementCollection>,
}

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(universal, 17))] // SET
pub struct ParameterContents {
    #[rasn(tag(explicit(context, 0)))]
    pub identifier: Option<EmberString>,
    #[rasn(tag(explicit(context, 1)))]
    pub description: Option<EmberString>,
    #[rasn(tag(explicit(context, 2)))]
    pub value: Option<Value>,
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
    pub template_reference: Option<ObjectIdentifier>,
}

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(choice)]
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

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum MinMax {
    #[rasn(tag(universal, 2))]
    Integer(Integer64),
    #[rasn(tag(universal, 9))]
    Real(f64),
    #[rasn(tag(universal, 5))]
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsnType, Decode, Encode)]
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

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, AsnType, Decode, Encode)]
#[rasn(enumerated, tag(universal, 2))]
pub enum ParameterAccess {
    None = 0,
    #[default]
    Read = 1,
    Write = 2,
    ReadWrite = 3,
}

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 7))]
pub struct StringIntegerPair {
    #[rasn(tag(explicit(context, 0)))]
    pub entry_string: EmberString,
    #[rasn(tag(explicit(context, 1)))]
    pub entry_integer: Integer32,
}

// StringIntegerCollection ::= [APPLICATION 8] IMPLICIT SEQUENCE OF [0] StringIntegerPair
#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 8))]
pub struct StringIntegerCollection(#[rasn(tag(context, 0))] pub Vec<StringIntegerPair>);

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 12))]
pub struct StreamDescription {
    #[rasn(tag(explicit(context, 0)))]
    pub format: StreamFormat,
    #[rasn(tag(explicit(context, 1)))]
    pub offset: Integer32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsnType, Decode, Encode)]
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
#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 2))]
pub struct Command {
    #[rasn(tag(explicit(context, 0)))]
    pub number: CommandType,
    // options is an OPTIONAL CHOICE with context-specific tags on the alternatives.
    pub options: Option<CommandOptions>,
}

#[derive(Debug, Clone, Copy, PartialEq, AsnType, Decode, Encode)]
#[rasn(enumerated, tag(universal, 2))]
pub enum CommandType {
    Subscribe = 30,
    Unsubscribe = 31,
    GetDirectory = 32,
    Invoke = 33,
}

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum CommandOptions {
    #[rasn(tag(explicit(context, 1)))]
    DirFieldMask(FieldFlags),
    #[rasn(tag(explicit(context, 2)))]
    Invocation(Invocation),
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, AsnType, Decode, Encode)]
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
#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 3))]
pub struct Node {
    #[rasn(tag(explicit(context, 0)))]
    pub number: Integer32,
    #[rasn(tag(explicit(context, 1)))]
    pub contents: Option<NodeContents>,
    #[rasn(tag(explicit(context, 2)))]
    pub children: Option<ElementCollection>,
}

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 10))]
pub struct QualifiedNode {
    #[rasn(tag(explicit(context, 0)))]
    pub path: ObjectIdentifier,
    #[rasn(tag(explicit(context, 1)))]
    pub contents: Option<NodeContents>,
    #[rasn(tag(explicit(context, 2)))]
    pub children: Option<ElementCollection>,
}

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(universal, 17))]
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
    pub template_reference: Option<ObjectIdentifier>,
}

// =============================
// Matrix & Signals
// =============================
#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
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

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(universal, 17))]
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
    pub template_reference: Option<ObjectIdentifier>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsnType, Decode, Encode)]
#[rasn(enumerated, tag(universal, 2))]
pub enum MatrixType {
    OneToN = 0,
    OneToOne = 1,
    NToN = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsnType, Decode, Encode)]
#[rasn(enumerated, tag(universal, 2))]
pub enum MatrixAddressingMode {
    Linear = 0,
    NonLinear = 1,
}

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum ParametersLocation {
    #[rasn(tag(universal, 13))] // RELATIVE-OID
    BasePath(ObjectIdentifier),
    #[rasn(tag(universal, 2))] // INTEGER
    Inline(Integer32),
}

// LabelCollection ::= SEQUENCE OF [0] Label
#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(universal, 16))]
pub struct LabelCollection(#[rasn(tag(context, 0))] pub Vec<Label>);

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 18))]
pub struct Label {
    #[rasn(tag(explicit(context, 0)))]
    pub base_path: ObjectIdentifier,
    #[rasn(tag(explicit(context, 1)))]
    pub description: EmberString,
}

// TargetCollection ::= SEQUENCE OF [0] Target
#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(universal, 16))]
pub struct TargetCollection(#[rasn(tag(context, 0))] pub Vec<Target>);

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 14))]
pub struct Target(pub Signal);

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(universal, 16))]
pub struct Signal {
    #[rasn(tag(explicit(context, 0)))]
    pub number: Integer32,
    #[rasn(tag(explicit(context, 1)))]
    pub contents: Option<SignalContents>,
}

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(universal, 17))]
pub struct SignalContents {
    #[rasn(tag(explicit(context, 0)))]
    pub identifier: Option<EmberString>,
    #[rasn(tag(explicit(context, 1)))]
    pub is_online: Option<bool>,
    #[rasn(tag(explicit(context, 2)))]
    pub labels_location: Option<ObjectIdentifier>,
}

// SourceCollection ::= SEQUENCE OF [0] Source
#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(universal, 16))]
pub struct SourceCollection(#[rasn(tag(context, 0))] pub Vec<Source>);

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 15))]
pub struct Source(pub Signal);

// ConnectionCollection ::= SEQUENCE OF [0] Connection
#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(universal, 16))]
pub struct ConnectionCollection(#[rasn(tag(context, 0))] pub Vec<Connection>);

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
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

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(universal, 13))]
pub struct PackedNumbers(pub ObjectIdentifier);

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsnType, Decode, Encode)]
#[rasn(enumerated, tag(universal, 2))]
pub enum ConnectionOperation {
    Absolute = 0,
    Connect = 1,
    Disconnect = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsnType, Decode, Encode)]
#[rasn(enumerated, tag(universal, 2))]
pub enum ConnectionDisposition {
    Tally = 0,
    Modified = 1,
    Pending = 2,
    Locked = 3,
}

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 17))]
pub struct QualifiedMatrix {
    #[rasn(tag(explicit(context, 0)))]
    pub path: ObjectIdentifier,
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
#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 19))]
pub struct Function {
    #[rasn(tag(explicit(context, 0)))]
    pub number: Integer32,
    #[rasn(tag(explicit(context, 1)))]
    pub contents: Option<FunctionContents>,
    #[rasn(tag(explicit(context, 2)))]
    pub children: Option<ElementCollection>,
}

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 20))]
pub struct QualifiedFunction {
    #[rasn(tag(explicit(context, 0)))]
    pub path: ObjectIdentifier,
    #[rasn(tag(explicit(context, 1)))]
    pub contents: Option<FunctionContents>,
    #[rasn(tag(explicit(context, 2)))]
    pub children: Option<ElementCollection>,
}

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(universal, 17))]
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
    pub template_reference: Option<ObjectIdentifier>,
}

// TupleDescription ::= SEQUENCE OF [0] TupleItemDescription
#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(universal, 16))]
pub struct TupleDescription(#[rasn(tag(context, 0))] pub Vec<TupleItemDescription>);

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 21))]
pub struct TupleItemDescription {
    #[rasn(tag(explicit(context, 0)))]
    pub r#type: ParameterType,
    #[rasn(tag(explicit(context, 1)))]
    pub name: Option<EmberString>,
}

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 22))]
pub struct Invocation {
    #[rasn(tag(explicit(context, 0)))]
    pub invocation_id: Option<Integer32>,
    #[rasn(tag(explicit(context, 1)))]
    pub arguments: Option<Tuple>,
}

// Tuple ::= SEQUENCE OF [0] Value
#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(universal, 16))]
pub struct Tuple(#[rasn(tag(context, 0))] pub Vec<Value>);

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
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

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 4))]
pub struct ElementCollection(#[rasn(tag(context, 0))] pub Vec<Element>);

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum Element {
    Parameter(Parameter),
    Node(Node),
    Command(Command),
    Matrix(Matrix),
    Function(Function),
    Template(Template),
}

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 5))]
pub struct StreamEntry {
    #[rasn(tag(explicit(context, 0)))]
    pub stream_identifier: Integer32,
    #[rasn(tag(explicit(context, 1)))]
    pub stream_value: Value,
}

// StreamCollection ::= [APPLICATION 6] IMPLICIT SEQUENCE OF [0] StreamEntry
#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 6))]
pub struct StreamCollection(#[rasn(tag(context, 0))] pub Vec<StreamEntry>);

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(explicit(application, 0)))] // Root ::= [APPLICATION 0] CHOICE { ... } (explicit by module default)
#[rasn(choice)]
pub enum Root {
    Elements(RootElementCollection),
    Streams(StreamCollection),
    InvocationResult(InvocationResult),
}

// RootElementCollection ::= [APPLICATION 11] IMPLICIT SEQUENCE OF [0] RootElement
#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
#[rasn(tag(application, 11))]
pub struct RootElementCollection(#[rasn(tag(context, 0))] pub Vec<RootElement>);

#[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
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

    pub const GLOW_VERSION_MAJOR: u8 = 2;
    pub const GLOW_VERSION_MINOR: u8 = 50;

    impl Command {
        pub fn get_directory(flags: Option<FieldFlags>) -> Self {
            Command {
                number: CommandType::GetDirectory,
                options: flags.map(|f| CommandOptions::DirFieldMask(f)),
            }
        }
    }

    impl From<Command> for Root {
        fn from(value: Command) -> Self {
            Root::Elements(RootElementCollection(vec![RootElement::Element(
                Element::Command(value),
            )]))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rasn::ber;

    #[test]
    fn serde_roundtrip() {
        let original = Root::Elements(RootElementCollection(vec![RootElement::Element(
            Element::Command(Command {
                number: CommandType::GetDirectory,
                options: Some(CommandOptions::DirFieldMask(FieldFlags::All)),
            }),
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
}
