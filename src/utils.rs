use crate::glow::{Integer32, RelativeOid};

#[macro_export]
macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<i32> for $name {
            type Error = crate::error::EmberError;

            fn try_from(v: i32) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as i32 => Ok($name::$vname),)*
                    _ => Err(crate::error::EmberError::S101DecodeError),
                }
            }
        }
    }
}

pub fn join(parent: Option<&RelativeOid>, number: Integer32) -> RelativeOid {
    let mut path = as_path(parent).to_vec();
    path.push(number as u32);
    RelativeOid(path)
}

pub fn as_path(oid: Option<&RelativeOid>) -> &[u32] {
    match oid {
        Some(oid) => &oid.0,
        None => &[],
    }
}

pub fn format_bytes(bytes: &[u8]) -> String {
    format!(
        "[{}]",
        bytes
            .iter()
            .map(|it| format!("0x{it:02x}"))
            .collect::<Vec<String>>()
            .join(", ")
    )
}
