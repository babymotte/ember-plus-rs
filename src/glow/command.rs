use crate::{back_to_enum, glow::function::Invocation};

pub struct Command<'a> {
    pub number: CommandType,
    pub options: Option<CommandOptions<'a>>,
}

pub enum CommandOptions<'a> {
    DirFieldMask(FieldFlags),
    Invocation(Invocation<'a>),
}

back_to_enum! {
pub enum CommandType {
    Subscribe = 30,
    Unsubscribe = 31,
    GetDirectory = 32,
    Invoke = 33,
}}

back_to_enum! {
pub enum FieldFlags {
    Spare = -2,
    All = -1,
    Default = 0,
    Identifier = 1,
    Description = 2,
    Tree = 3,
    Value = 4,
    Connections = 5,
}}
