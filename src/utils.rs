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
            type Error = $crate::error::EmberError;

            fn try_from(v: i32) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as i32 => Ok($name::$vname),)*
                    _ => Err($crate::error::EmberError::S101DecodeError("unknown enum variant".into())),
                }
            }
        }
    }
}

pub fn join(parent: &RelativeOid, number: Integer32) -> RelativeOid {
    let mut path = parent.0.clone();
    path.push(number as u32);
    RelativeOid(path)
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

pub fn format_byte_size(bytes: usize) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{:.0} {}", size, UNITS[unit])
    } else {
        format!("{:.1} {}", size, UNITS[unit])
    }
}
