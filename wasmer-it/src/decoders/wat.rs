//! Parse the WIT textual representation into an [AST](crate::ast).

use crate::IType;
use crate::{ast::*, interpreter::Instruction};

pub use wast::parser::ParseBuffer as Buffer;
use wast::parser::{self, Cursor, Parse, Parser, Peek, Result};
pub use wast::Error;

use std::sync::Arc;

mod keyword {
    pub use wast::{
        custom_keyword,
        kw::{anyref, export, f32, f64, func, i32, i64, import, param, result},
    };

    // New keywords.
    custom_keyword!(implement);
    custom_keyword!(r#type = "type");
    custom_keyword!(record);
    custom_keyword!(field);
    custom_keyword!(it_version);

    // Special symbols
    custom_keyword!(comma = ",");
    custom_keyword!(colon = ":");

    // New types.
    custom_keyword!(s8);
    custom_keyword!(s16);
    custom_keyword!(s32);
    custom_keyword!(s64);
    custom_keyword!(u8);
    custom_keyword!(u16);
    custom_keyword!(u32);
    custom_keyword!(u64);
    custom_keyword!(string);
    custom_keyword!(array);

    // Instructions.
    custom_keyword!(argument_get = "arg.get");
    custom_keyword!(call_core = "call-core");
    custom_keyword!(bool_from_i32 = "bool.from_i32");
    custom_keyword!(s8_from_i32 = "s8.from_i32");
    custom_keyword!(s8_from_i64 = "s8.from_i64");
    custom_keyword!(s16_from_i32 = "s16.from_i32");
    custom_keyword!(s16_from_i64 = "s16.from_i64");
    custom_keyword!(s32_from_i32 = "s32.from_i32");
    custom_keyword!(s32_from_i64 = "s32.from_i64");
    custom_keyword!(s64_from_i32 = "s64.from_i32");
    custom_keyword!(s64_from_i64 = "s64.from_i64");
    custom_keyword!(i32_from_bool = "i32.from_bool");
    custom_keyword!(i32_from_s8 = "i32.from_s8");
    custom_keyword!(i32_from_s16 = "i32.from_s16");
    custom_keyword!(i32_from_s32 = "i32.from_s32");
    custom_keyword!(i32_from_s64 = "i32.from_s64");
    custom_keyword!(i32_push = "i32.push");
    custom_keyword!(i64_from_s8 = "i64.from_s8");
    custom_keyword!(i64_from_s16 = "i64.from_s16");
    custom_keyword!(i64_from_s32 = "i64.from_s32");
    custom_keyword!(i64_from_s64 = "i64.from_s64");
    custom_keyword!(i64_push = "i64.push");
    custom_keyword!(u8_from_i32 = "u8.from_i32");
    custom_keyword!(u8_from_i64 = "u8.from_i64");
    custom_keyword!(u16_from_i32 = "u16.from_i32");
    custom_keyword!(u16_from_i64 = "u16.from_i64");
    custom_keyword!(u32_from_i32 = "u32.from_i32");
    custom_keyword!(u32_from_i64 = "u32.from_i64");
    custom_keyword!(u64_from_i32 = "u64.from_i32");
    custom_keyword!(u64_from_i64 = "u64.from_i64");
    custom_keyword!(i32_from_u8 = "i32.from_u8");
    custom_keyword!(i32_from_u16 = "i32.from_u16");
    custom_keyword!(i32_from_u32 = "i32.from_u32");
    custom_keyword!(i32_from_u64 = "i32.from_u64");
    custom_keyword!(i64_from_u8 = "i64.from_u8");
    custom_keyword!(i64_from_u16 = "i64.from_u16");
    custom_keyword!(i64_from_u32 = "i64.from_u32");
    custom_keyword!(i64_from_u64 = "i64.from_u64");
    custom_keyword!(f32_push = "f32.push");
    custom_keyword!(f64_push = "f64.push");
    custom_keyword!(string_lift_memory = "string.lift_memory");
    custom_keyword!(string_lower_memory = "string.lower_memory");
    custom_keyword!(string_size = "string.size");
    custom_keyword!(byte_array_lift_memory = "byte_array.lift_memory");
    custom_keyword!(byte_array_lower_memory = "byte_array.lower_memory");
    custom_keyword!(byte_array_size = "byte_array.size");
    custom_keyword!(array_lift_memory = "array.lift_memory");
    custom_keyword!(array_lower_memory = "array.lower_memory");
    custom_keyword!(array_size = "array.size");
    custom_keyword!(record_lift = "record.lift");
    custom_keyword!(record_lower = "record.lower");
    custom_keyword!(record_lift_memory = "record.lift_memory");
    custom_keyword!(record_lower_memory = "record.lower_memory");
    custom_keyword!(dup = "dup");
    custom_keyword!(swap2 = "swap2");
}

#[allow(clippy::suspicious_else_formatting)]
impl<'a> Parse<'a> for Instruction {
    #[allow(clippy::cognitive_complexity)]
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut lookahead = parser.lookahead1();

        if lookahead.peek::<keyword::argument_get>() {
            parser.parse::<keyword::argument_get>()?;

            Ok(Instruction::ArgumentGet {
                index: parser.parse()?,
            })
        } else if lookahead.peek::<keyword::call_core>() {
            parser.parse::<keyword::call_core>()?;

            Ok(Instruction::CallCore {
                function_index: parser.parse::<u32>()?,
            })
        } else if lookahead.peek::<keyword::s8_from_i32>() {
            parser.parse::<keyword::s8_from_i32>()?;

            Ok(Instruction::S8FromI32)
        } else if lookahead.peek::<keyword::s8_from_i64>() {
            parser.parse::<keyword::s8_from_i64>()?;

            Ok(Instruction::S8FromI64)
        } else if lookahead.peek::<keyword::s16_from_i32>() {
            parser.parse::<keyword::s16_from_i32>()?;

            Ok(Instruction::S16FromI32)
        } else if lookahead.peek::<keyword::s16_from_i64>() {
            parser.parse::<keyword::s16_from_i64>()?;

            Ok(Instruction::S16FromI64)
        } else if lookahead.peek::<keyword::s32_from_i32>() {
            parser.parse::<keyword::s32_from_i32>()?;

            Ok(Instruction::S32FromI32)
        } else if lookahead.peek::<keyword::s32_from_i64>() {
            parser.parse::<keyword::s32_from_i64>()?;

            Ok(Instruction::S32FromI64)
        } else if lookahead.peek::<keyword::s64_from_i32>() {
            parser.parse::<keyword::s64_from_i32>()?;

            Ok(Instruction::S64FromI32)
        } else if lookahead.peek::<keyword::s64_from_i64>() {
            parser.parse::<keyword::s64_from_i64>()?;

            Ok(Instruction::S64FromI64)
        } else if lookahead.peek::<keyword::i32_from_s8>() {
            parser.parse::<keyword::i32_from_s8>()?;

            Ok(Instruction::I32FromS8)
        } else if lookahead.peek::<keyword::i32_from_s16>() {
            parser.parse::<keyword::i32_from_s16>()?;

            Ok(Instruction::I32FromS16)
        } else if lookahead.peek::<keyword::i32_from_s32>() {
            parser.parse::<keyword::i32_from_s32>()?;

            Ok(Instruction::I32FromS32)
        } else if lookahead.peek::<keyword::i32_from_s64>() {
            parser.parse::<keyword::i32_from_s64>()?;

            Ok(Instruction::I32FromS64)
        } else if lookahead.peek::<keyword::i32_push>() {
            parser.parse::<keyword::i32_push>()?;

            Ok(Instruction::PushI32 {
                value: parser.parse()?,
            })
        } else if lookahead.peek::<keyword::i64_from_s8>() {
            parser.parse::<keyword::i64_from_s8>()?;

            Ok(Instruction::I64FromS8)
        } else if lookahead.peek::<keyword::i64_from_s16>() {
            parser.parse::<keyword::i64_from_s16>()?;

            Ok(Instruction::I64FromS16)
        } else if lookahead.peek::<keyword::i64_from_s32>() {
            parser.parse::<keyword::i64_from_s32>()?;

            Ok(Instruction::I64FromS32)
        } else if lookahead.peek::<keyword::i64_from_s64>() {
            parser.parse::<keyword::i64_from_s64>()?;

            Ok(Instruction::I64FromS64)
        } else if lookahead.peek::<keyword::i64_push>() {
            parser.parse::<keyword::i64_push>()?;

            Ok(Instruction::PushI64 {
                value: parser.parse()?,
            })
        } else if lookahead.peek::<keyword::u8_from_i32>() {
            parser.parse::<keyword::u8_from_i32>()?;

            Ok(Instruction::U8FromI32)
        } else if lookahead.peek::<keyword::u8_from_i64>() {
            parser.parse::<keyword::u8_from_i64>()?;

            Ok(Instruction::U8FromI64)
        } else if lookahead.peek::<keyword::u16_from_i32>() {
            parser.parse::<keyword::u16_from_i32>()?;

            Ok(Instruction::U16FromI32)
        } else if lookahead.peek::<keyword::u16_from_i64>() {
            parser.parse::<keyword::u16_from_i64>()?;

            Ok(Instruction::U16FromI64)
        } else if lookahead.peek::<keyword::u32_from_i32>() {
            parser.parse::<keyword::u32_from_i32>()?;

            Ok(Instruction::U32FromI32)
        } else if lookahead.peek::<keyword::u32_from_i64>() {
            parser.parse::<keyword::u32_from_i64>()?;

            Ok(Instruction::U32FromI64)
        } else if lookahead.peek::<keyword::u64_from_i32>() {
            parser.parse::<keyword::u64_from_i32>()?;

            Ok(Instruction::U64FromI32)
        } else if lookahead.peek::<keyword::u64_from_i64>() {
            parser.parse::<keyword::u64_from_i64>()?;

            Ok(Instruction::U64FromI64)
        } else if lookahead.peek::<keyword::i32_from_u8>() {
            parser.parse::<keyword::i32_from_u8>()?;

            Ok(Instruction::I32FromU8)
        } else if lookahead.peek::<keyword::i32_from_u16>() {
            parser.parse::<keyword::i32_from_u16>()?;

            Ok(Instruction::I32FromU16)
        } else if lookahead.peek::<keyword::i32_from_u32>() {
            parser.parse::<keyword::i32_from_u32>()?;

            Ok(Instruction::I32FromU32)
        } else if lookahead.peek::<keyword::i32_from_u64>() {
            parser.parse::<keyword::i32_from_u64>()?;

            Ok(Instruction::I32FromU64)
        } else if lookahead.peek::<keyword::i64_from_u8>() {
            parser.parse::<keyword::i64_from_u8>()?;

            Ok(Instruction::I64FromU8)
        } else if lookahead.peek::<keyword::i64_from_u16>() {
            parser.parse::<keyword::i64_from_u16>()?;

            Ok(Instruction::I64FromU16)
        } else if lookahead.peek::<keyword::i64_from_u32>() {
            parser.parse::<keyword::i64_from_u32>()?;

            Ok(Instruction::I64FromU32)
        } else if lookahead.peek::<keyword::i64_from_u64>() {
            parser.parse::<keyword::i64_from_u64>()?;

            Ok(Instruction::I64FromU64)
        } else if lookahead.peek::<keyword::string_lift_memory>() {
            parser.parse::<keyword::string_lift_memory>()?;

            Ok(Instruction::StringLiftMemory)
        } else if lookahead.peek::<keyword::string_lower_memory>() {
            parser.parse::<keyword::string_lower_memory>()?;

            Ok(Instruction::StringLowerMemory)
        } else if lookahead.peek::<keyword::string_size>() {
            parser.parse::<keyword::string_size>()?;

            Ok(Instruction::StringSize)
        } else if lookahead.peek::<keyword::byte_array_lift_memory>() {
            parser.parse::<keyword::byte_array_lift_memory>()?;

            Ok(Instruction::ByteArrayLiftMemory)
        } else if lookahead.peek::<keyword::byte_array_lower_memory>() {
            parser.parse::<keyword::byte_array_lower_memory>()?;

            Ok(Instruction::ByteArrayLowerMemory)
        } else if lookahead.peek::<keyword::byte_array_size>() {
            parser.parse::<keyword::byte_array_size>()?;

            Ok(Instruction::ByteArraySize)
        } else if lookahead.peek::<keyword::array_lift_memory>() {
            parser.parse::<keyword::array_lift_memory>()?;

            Ok(Instruction::ArrayLiftMemory {
                value_type: parser.parse()?,
            })
        } else if lookahead.peek::<keyword::array_lower_memory>() {
            parser.parse::<keyword::array_lower_memory>()?;

            Ok(Instruction::ArrayLowerMemory {
                value_type: parser.parse()?,
            })
        }
        /*
        else if lookahead.peek::<keyword::record_lift>() {
            parser.parse::<keyword::record_lift>()?;

            Ok(Instruction::RecordLift {
                type_index: parser.parse()?,
            })
        } else if lookahead.peek::<keyword::record_lower>() {
            parser.parse::<keyword::record_lower>()?;

            Ok(Instruction::RecordLower {
                type_index: parser.parse()?,
            })
        }
            */
        else if lookahead.peek::<keyword::record_lift_memory>() {
            parser.parse::<keyword::record_lift_memory>()?;

            Ok(Instruction::RecordLiftMemory {
                record_type_id: parser.parse()?,
            })
        } else if lookahead.peek::<keyword::record_lower_memory>() {
            parser.parse::<keyword::record_lower_memory>()?;

            Ok(Instruction::RecordLowerMemory {
                record_type_id: parser.parse()?,
            })
        } else if lookahead.peek::<keyword::dup>() {
            parser.parse::<keyword::dup>()?;

            Ok(Instruction::Dup)
        } else if lookahead.peek::<keyword::swap2>() {
            parser.parse::<keyword::swap2>()?;

            Ok(Instruction::Swap2)
        } else {
            Err(lookahead.error())
        }
    }
}

struct AtInterface;

impl Peek for AtInterface {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.reserved().map(|(string, _)| string) == Some("@interface")
    }

    fn display() -> &'static str {
        "`@interface`"
    }
}

impl Parse<'_> for AtInterface {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.step(|cursor| {
            if let Some(("@interface", rest)) = cursor.reserved() {
                return Ok((AtInterface, rest));
            }

            Err(cursor.error("expected `@interface`"))
        })
    }
}

#[derive(PartialEq, Debug)]
enum FunctionType {
    Header(Vec<FunctionArg>),
    Output(Vec<IType>),
}

impl Parse<'_> for FunctionType {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.parens(|parser| {
            let mut lookahead = parser.lookahead1();

            if lookahead.peek::<keyword::param>() {
                parser.parse::<keyword::param>()?;

                let mut arguments = vec![];

                while !parser.is_empty() {
                    let arg_name = parser.step(|cursor| {
                        cursor
                            .id()
                            .ok_or_else(|| cursor.error("expecting argument identifier"))
                    })?;

                    if !arg_name.ends_with(':') {
                        parser.step(|cursor| {
                            if let Some((":", rest)) = cursor.reserved() {
                                return Ok(("", rest));
                            }
                            Err(cursor.error("expected : between an argument and a type"))
                        })?;
                    }

                    let arg_type: IType = parser.parse()?;

                    arguments.push(FunctionArg {
                        name: arg_name.trim_end_matches(':').to_string(),
                        ty: arg_type,
                    });
                }

                Ok(FunctionType::Header(arguments))
            } else if lookahead.peek::<keyword::result>() {
                parser.parse::<keyword::result>()?;

                let mut outputs = vec![];

                while !parser.is_empty() {
                    outputs.push(parser.parse()?);
                }

                Ok(FunctionType::Output(outputs))
            } else {
                Err(lookahead.error())
            }
        })
    }
}

#[derive(PartialEq, Debug)]
struct Version(pub String);

impl Parse<'_> for Version {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.parse::<keyword::it_version>()?;
        let version = parser.parse()?;

        Ok(Version(version))
    }
}

#[derive(PartialEq, Debug)]
enum Interface<'a> {
    Version(Version),
    Type(Type),
    Import(Import<'a>),
    Adapter(Adapter),
    Export(Export<'a>),
    Implementation(Implementation),
}

impl<'a> Parse<'a> for Interface<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parens(|parser| {
            let mut lookahead = parser.lookahead1();

            if lookahead.peek::<AtInterface>() {
                parser.parse::<AtInterface>()?;

                let mut lookahead = parser.lookahead1();

                if lookahead.peek::<keyword::it_version>() {
                    Ok(Interface::Version(parser.parse()?))
                } else if lookahead.peek::<keyword::r#type>() {
                    Ok(Interface::Type(parser.parse()?))
                } else if lookahead.peek::<keyword::import>() {
                    Ok(Interface::Import(parser.parse()?))
                } else if lookahead.peek::<keyword::func>() {
                    Ok(Interface::Adapter(parser.parse()?))
                } else if lookahead.peek::<keyword::export>() {
                    Ok(Interface::Export(parser.parse()?))
                } else if lookahead.peek::<keyword::implement>() {
                    Ok(Interface::Implementation(parser.parse()?))
                } else {
                    Err(lookahead.error())
                }
            } else {
                Err(lookahead.error())
            }
        })
    }
}

impl<'a> Parse<'a> for Type {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<keyword::r#type>()?;

        let ty = parser.parens(|parser| {
            let mut lookahead = parser.lookahead1();

            if lookahead.peek::<keyword::func>() {
                parser.parse::<keyword::func>()?;

                let mut arguments = vec![];
                let mut output_types = vec![];

                while !parser.is_empty() {
                    let function_type = parser.parse::<FunctionType>()?;

                    match function_type {
                        FunctionType::Header(mut func_arguments) => {
                            arguments.append(&mut func_arguments);
                        }
                        FunctionType::Output(mut outputs) => output_types.append(&mut outputs),
                    }
                }

                Ok(Type::Function {
                    arguments: Arc::new(arguments),
                    output_types: Arc::new(output_types),
                })
            } else if lookahead.peek::<keyword::record>() {
                Ok(Type::Record(Arc::new(parser.parse()?)))
            } else {
                Err(lookahead.error())
            }
        })?;

        Ok(ty)
    }
}

impl<'a> Parse<'a> for Import<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<keyword::import>()?;

        let namespace = parser.parse()?;
        let name = parser.parse()?;

        let function_type = parser.parens(|parser| {
            parser.parse::<keyword::func>()?;

            parser.parens(|parser| {
                parser.parse::<keyword::r#type>()?;

                parser.parse()
            })
        })?;

        Ok(Import {
            namespace,
            name,
            function_type,
        })
    }
}

impl<'a> Parse<'a> for Export<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<keyword::export>()?;

        let name = parser.parse()?;

        let function_type = parser.parens(|parser| {
            parser.parse::<keyword::func>()?;

            parser.parse()
        })?;

        Ok(Export {
            name,
            function_type,
        })
    }
}

impl<'a> Parse<'a> for Implementation {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<keyword::implement>()?;

        let core_function_type = parser.parens(|parser| {
            parser.parse::<keyword::func>()?;

            parser.parse()
        })?;

        let adapter_function_type = parser.parens(|parser| {
            parser.parse::<keyword::func>()?;

            parser.parse()
        })?;

        Ok(Implementation {
            core_function_type,
            adapter_function_type,
        })
    }
}

impl<'a> Parse<'a> for Adapter {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<keyword::func>()?;

        let function_type = parser.parens(|parser| {
            parser.parse::<keyword::r#type>()?;

            parser.parse()
        })?;

        let mut instructions = vec![];

        while !parser.is_empty() {
            instructions.push(parser.parse()?);
        }

        Ok(Adapter {
            function_type,
            instructions,
        })
    }
}

impl<'a> Parse<'a> for Interfaces<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut version: Option<semver::Version> = None;
        let mut types = vec![];
        let mut imports = vec![];
        let mut adapters = vec![];
        let mut exports = vec![];
        let mut implementations = vec![];

        while !parser.is_empty() {
            let interface = parser.parse::<Interface>()?;

            match interface {
                Interface::Version(version_str) => {
                    try_handle_version(&version_str, &mut version, &parser)?
                }
                Interface::Type(ty) => types.push(ty),
                Interface::Import(import) => imports.push(import),
                Interface::Adapter(adapter) => adapters.push(adapter),
                Interface::Export(export) => exports.push(export),
                Interface::Implementation(implementation) => implementations.push(implementation),
            }
        }

        let version = version.ok_or_else(|| {
            wast::Error::new(parser.cur_span(), String::from("version must be specified, probably this module was built with old version of fce, please rebuild it"))
        })?;

        let interfaces = Interfaces {
            version,
            types,
            imports,
            adapters,
            exports,
            implementations,
        };

        Ok(interfaces)
    }
}

fn try_handle_version(
    sdk_version: &Version,
    version: &mut Option<semver::Version>,
    parser: &Parser<'_>,
) -> Result<()> {
    use std::str::FromStr;

    if version.is_some() {
        return Err(wast::Error::new(
            parser.cur_span(),
            String::from("only one version directive is possible"),
        ));
    }

    let parsed_version = semver::Version::from_str(&sdk_version.0)
        .map_err(|e| wast::Error::new(parser.cur_span(), format!("version is corrupted: {}", e)))?;
    *version = Some(parsed_version);

    Ok(())
}

/// Parse a WIT definition in its textual format, and produces an
/// [AST](crate::ast) with the [`Interfaces`](crate::ast::Interfaces)
/// structure upon succesful.
///
/// # Examples
///
/// ```rust
/// use wasmer_interface_types::{
///     ast::{Adapter, Export, Implementation, Import, Interfaces, Type},
///     decoders::wat::{parse, Buffer},
///     interpreter::Instruction,
///     types::IType,
/// };
///
/// let input = Buffer::new(
///     r#"(@interface type (func (param i32) (result s8)))
///
/// (@interface import "ns" "foo" (func (type 0)))
///
/// (@interface func (type 0) arg.get 42)
///
/// (@interface export "bar" (func 0))
///
/// (@interface implement (func 0) (func 1))"#,
/// )
/// .unwrap();
/// let output = Interfaces {
///     types: vec![Type::Function {
///         inputs: vec![IType::I32],
///         outputs: vec![IType::S8],
///     }],
///     imports: vec![Import {
///         namespace: "ns",
///         name: "foo",
///         function_type: 0,
///     }],
///     adapters: vec![Adapter {
///         function_type: 0,
///         instructions: vec![Instruction::ArgumentGet { index: 42 }],
///     }],
///     exports: vec![Export {
///         name: "bar",
///         function_type: 0,
///     }],
///     implementations: vec![Implementation {
///         core_function_type: 0,
///         adapter_function_type: 1,
///     }],
/// };
///
/// assert_eq!(parse(&input).unwrap(), output);
/// ```
pub fn parse<'input>(input: &'input Buffer) -> Result<Interfaces<'input>> {
    parser::parse::<Interfaces>(&input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wast::parser;

    fn buffer(input: &str) -> Buffer {
        Buffer::new(input).expect("Failed to build the parser buffer.")
    }

    #[test]
    fn test_interface_type() {
        let inputs = vec![
            "s8",
            "s16",
            "s32",
            "s64",
            "u8",
            "u16",
            "u32",
            "u64",
            "f32",
            "f64",
            "string",
            "anyref",
            "i32",
            "i64",
            "record (field string)",
        ];
        let outputs = vec![
            IType::S8,
            IType::S16,
            IType::S32,
            IType::S64,
            IType::U8,
            IType::U16,
            IType::U32,
            IType::U64,
            IType::F32,
            IType::F64,
            IType::String,
            IType::Anyref,
            IType::I32,
            IType::I64,
            IType::Record(RecordType {
                fields: vec1![IType::String],
            }),
        ];

        assert_eq!(inputs.len(), outputs.len());

        for (input, output) in inputs.iter().zip(outputs.iter()) {
            assert_eq!(&parser::parse::<IType>(&buffer(input)).unwrap(), output);
        }
    }

    #[test]
    fn test_record_type() {
        let inputs = vec![
            "record (field string)",
            "record (field string) (field i32)",
            "record (field string) (field record (field i32) (field i32)) (field f64)",
        ];
        let outputs = vec![
            RecordType {
                fields: vec1![IType::String],
            },
            RecordType {
                fields: vec1![IType::String, IType::I32],
            },
            RecordType {
                fields: vec1![
                    IType::String,
                    IType::Record(RecordType {
                        fields: vec1![IType::I32, IType::I32],
                    }),
                    IType::F64,
                ],
            },
        ];

        assert_eq!(inputs.len(), outputs.len());

        for (input, output) in inputs.iter().zip(outputs.iter()) {
            assert_eq!(
                &parser::parse::<RecordType>(&buffer(input)).unwrap(),
                output
            );
        }
    }

    #[test]
    fn test_instructions() {
        let inputs = vec![
            "arg.get 7",
            "call-core 7",
            "s8.from_i32",
            "s8.from_i64",
            "s16.from_i32",
            "s16.from_i64",
            "s32.from_i32",
            "s32.from_i64",
            "s64.from_i32",
            "s64.from_i64",
            "i32.from_s8",
            "i32.from_s16",
            "i32.from_s32",
            "i32.from_s64",
            "i64.from_s8",
            "i64.from_s16",
            "i64.from_s32",
            "i64.from_s64",
            "u8.from_i32",
            "u8.from_i64",
            "u16.from_i32",
            "u16.from_i64",
            "u32.from_i32",
            "u32.from_i64",
            "u64.from_i32",
            "u64.from_i64",
            "i32.from_u8",
            "i32.from_u16",
            "i32.from_u32",
            "i32.from_u64",
            "i64.from_u8",
            "i64.from_u16",
            "i64.from_u32",
            "i64.from_u64",
            "string.lift_memory",
            "string.lower_memory",
            "string.size",
            "record.lift 42",
            "record.lower 42",
        ];
        let outputs = vec![
            Instruction::ArgumentGet { index: 7 },
            Instruction::CallCore { function_index: 7 },
            Instruction::S8FromI32,
            Instruction::S8FromI64,
            Instruction::S16FromI32,
            Instruction::S16FromI64,
            Instruction::S32FromI32,
            Instruction::S32FromI64,
            Instruction::S64FromI32,
            Instruction::S64FromI64,
            Instruction::I32FromS8,
            Instruction::I32FromS16,
            Instruction::I32FromS32,
            Instruction::I32FromS64,
            Instruction::I64FromS8,
            Instruction::I64FromS16,
            Instruction::I64FromS32,
            Instruction::I64FromS64,
            Instruction::U8FromI32,
            Instruction::U8FromI64,
            Instruction::U16FromI32,
            Instruction::U16FromI64,
            Instruction::U32FromI32,
            Instruction::U32FromI64,
            Instruction::U64FromI32,
            Instruction::U64FromI64,
            Instruction::I32FromU8,
            Instruction::I32FromU16,
            Instruction::I32FromU32,
            Instruction::I32FromU64,
            Instruction::I64FromU8,
            Instruction::I64FromU16,
            Instruction::I64FromU32,
            Instruction::I64FromU64,
            Instruction::StringLiftMemory,
            Instruction::StringLowerMemory,
            Instruction::StringSize,
            /*
            Instruction::RecordLift { type_index: 42 },
            Instruction::RecordLower { type_index: 42 },
             */
        ];

        assert_eq!(inputs.len(), outputs.len());

        for (input, output) in inputs.iter().zip(outputs.iter()) {
            assert_eq!(
                &parser::parse::<Instruction>(&buffer(input)).unwrap(),
                output
            );
        }
    }

    #[test]
    fn test_param_empty() {
        let input = buffer("(param)");
        let output = FunctionType::InputTypes(vec![]);

        assert_eq!(parser::parse::<FunctionType>(&input).unwrap(), output);
    }

    #[test]
    fn test_param() {
        let input = buffer("(param i32 string)");
        let output = FunctionType::InputTypes(vec![IType::I32, IType::String]);

        assert_eq!(parser::parse::<FunctionType>(&input).unwrap(), output);
    }

    #[test]
    fn test_result_empty() {
        let input = buffer("(result)");
        let output = FunctionType::Output(vec![]);

        assert_eq!(parser::parse::<FunctionType>(&input).unwrap(), output);
    }

    #[test]
    fn test_result() {
        let input = buffer("(result i32 string)");
        let output = FunctionType::Output(vec![IType::I32, IType::String]);

        assert_eq!(parser::parse::<FunctionType>(&input).unwrap(), output);
    }

    #[test]
    fn test_type_function() {
        let input = buffer(r#"(@interface type (func (param i32 i32) (result i32)))"#);
        let output = Interface::Type(Type::Function {
            inputs: vec![IType::I32, IType::I32],
            outputs: vec![IType::I32],
        });

        assert_eq!(parser::parse::<Interface>(&input).unwrap(), output);
    }

    #[test]
    fn test_type_record() {
        let input = buffer(r#"(@interface type (record (field string) (field i32)))"#);
        let output = Interface::Type(Type::Record(RecordType {
            fields: vec1![IType::String, IType::I32],
        }));

        assert_eq!(parser::parse::<Interface>(&input).unwrap(), output);
    }

    #[test]
    fn test_export() {
        let input = buffer(r#"(@interface export "foo" (func 0))"#);
        let output = Interface::Export(Export {
            name: "foo",
            function_type: 0,
        });

        assert_eq!(parser::parse::<Interface>(&input).unwrap(), output);
    }

    #[test]
    fn test_export_escaped_name() {
        let input = buffer(r#"(@interface export "fo\"o" (func 0))"#);
        let output = Interface::Export(Export {
            name: r#"fo"o"#,
            function_type: 0,
        });

        assert_eq!(parser::parse::<Interface>(&input).unwrap(), output);
    }

    #[test]
    fn test_import() {
        let input = buffer(r#"(@interface import "ns" "foo" (func (type 0)))"#);
        let output = Interface::Import(Import {
            namespace: "ns",
            name: "foo",
            function_type: 0,
        });

        assert_eq!(parser::parse::<Interface>(&input).unwrap(), output);
    }

    #[test]
    fn test_adapter() {
        let input = buffer(r#"(@interface func (type 0) arg.get 42)"#);
        let output = Interface::Adapter(Adapter {
            function_type: 0,
            instructions: vec![Instruction::ArgumentGet { index: 42 }],
        });

        assert_eq!(parser::parse::<Interface>(&input).unwrap(), output);
    }

    #[test]
    fn test_implementation() {
        let input = buffer(r#"(@interface implement (func 0) (func 1))"#);
        let output = Interface::Implementation(Implementation {
            core_function_type: 0,
            adapter_function_type: 1,
        });

        assert_eq!(parser::parse::<Interface>(&input).unwrap(), output);
    }

    #[test]
    fn test_interfaces() {
        let input = buffer(
            r#"(@interface type (func (param i32) (result s8)))

(@interface import "ns" "foo" (func (type 0)))

(@interface func (type 0) arg.get 42)

(@interface export "bar" (func 0))

(@interface implement (func 0) (func 1))"#,
        );
        let output = Interfaces {
            types: vec![Type::Function {
                inputs: vec![IType::I32],
                outputs: vec![IType::S8],
            }],
            imports: vec![Import {
                namespace: "ns",
                name: "foo",
                function_type: 0,
            }],
            adapters: vec![Adapter {
                function_type: 0,
                instructions: vec![Instruction::ArgumentGet { index: 42 }],
            }],
            exports: vec![Export {
                name: "bar",
                function_type: 0,
            }],
            implementations: vec![Implementation {
                core_function_type: 0,
                adapter_function_type: 1,
            }],
        };

        assert_eq!(parser::parse::<Interfaces>(&input).unwrap(), output);
    }
}
