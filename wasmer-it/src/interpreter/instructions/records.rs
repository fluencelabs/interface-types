use super::read_from_instance_mem;
use super::write_to_instance_mem;

use crate::instr_error;
use crate::interpreter::instructions::{is_record_fields_compatible_to_type, to_native};
use crate::IRecordType;
use crate::IType;
use crate::IValue;
use crate::NEVec;
use crate::{
    errors::{InstructionError, InstructionErrorKind},
    interpreter::Instruction,
};

use std::convert::TryInto;

pub(super) fn record_lift_memory_<'instance, Instance, Export, LocalImport, Memory, MemoryView>(
    instance: &'instance Instance,
    record_type: &IRecordType,
    offset: usize,
    instruction: Instruction,
) -> Result<IValue, InstructionError>
where
    Export: crate::interpreter::wasm::structures::Export,
    LocalImport: crate::interpreter::wasm::structures::LocalImport,
    Memory: crate::interpreter::wasm::structures::Memory<MemoryView>,
    MemoryView: crate::interpreter::wasm::structures::MemoryView,
    Instance: crate::interpreter::wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>
        + 'instance,
{
    let length = record_type.fields.len();
    let mut values = Vec::with_capacity(length);
    let size = record_size(record_type);
    let data = read_from_instance_mem(instance, instruction.clone(), offset, size)?;

    let mut field_id = 0;
    for field in (*record_type.fields).iter() {
        let value = data[field_id];
        match &field.ty {
            IType::Boolean => {
                values.push(IValue::Boolean(value as _));
            }
            IType::S8 => {
                values.push(IValue::S8(value as _));
            }
            IType::S16 => {
                values.push(IValue::S16(value as _));
            }
            IType::S32 => {
                values.push(IValue::S32(value as _));
            }
            IType::S64 => {
                values.push(IValue::S64(value as _));
            }
            IType::I32 => {
                values.push(IValue::I32(value as _));
            }
            IType::I64 => {
                values.push(IValue::I64(value as _));
            }
            IType::U8 => {
                values.push(IValue::U8(value as _));
            }
            IType::U16 => {
                values.push(IValue::U16(value as _));
            }
            IType::U32 => {
                values.push(IValue::U32(value as _));
            }
            IType::U64 => {
                values.push(IValue::U64(value as _));
            }
            IType::F32 => {
                values.push(IValue::F32(value as _));
            }
            IType::F64 => values.push(IValue::F64(f64::from_bits(value))),
            IType::String => {
                let string_offset = value;
                field_id += 1;
                let string_size = data[field_id];

                if string_size != 0 {
                    let string_mem = read_from_instance_mem(
                        instance,
                        instruction.clone(),
                        string_offset as _,
                        string_size as _,
                    )?;

                    // TODO: check
                    let string = String::from_utf8(string_mem).unwrap();
                    values.push(IValue::String(string));
                } else {
                    values.push(IValue::String(String::new()));
                }
            }
            IType::Array(ty) => {
                let array_offset = value;
                field_id += 1;
                let array_size = data[field_id];

                if array_size != 0 {
                    let array = super::array_lift_memory_impl(
                        instance,
                        &**ty,
                        array_offset as _,
                        array_size as _,
                        instruction.clone(),
                    )?;
                    values.push(array);
                } else {
                    values.push(IValue::Array(vec![]));
                }
            }
            IType::Record(record_type_id) => {
                let offset = value;

                let record_type = instance.wit_record_by_id(*record_type_id).ok_or_else(|| {
                    InstructionError::new(
                        instruction.clone(),
                        InstructionErrorKind::RecordTypeByNameIsMissing {
                            record_type_id: *record_type_id,
                        },
                    )
                })?;

                values.push(record_lift_memory_(
                    instance,
                    record_type,
                    offset as _,
                    instruction.clone(),
                )?)
            }
        }
        field_id += 1;
    }

    Ok(IValue::Record(
        NEVec::new(values.into_iter().collect())
            .expect("Record must have at least one field, zero given"),
    ))
}

/// Returns record size in bytes.
fn record_size(record_type: &IRecordType) -> usize {
    let mut record_size = 0;

    for field_type in record_type.fields.iter() {
        record_size += match field_type.ty {
            IType::Boolean | IType::S8 | IType::U8 => 1,
            IType::S16 | IType::U16 => 2,
            IType::S32
            | IType::U32
            | IType::I32
            | IType::F32
            | IType::String
            | IType::ByteArray
            | IType::Array(_)
            | IType::Record(_) => 32,
            IType::S64 | IType::U64 | IType::I64 | IType::F64 => 64,
        };
    }

    record_size
}

pub(crate) fn record_lift_memory<Instance, Export, LocalImport, Memory, MemoryView>(
    record_type_id: u64,
    instruction: Instruction,
) -> crate::interpreter::ExecutableInstruction<Instance, Export, LocalImport, Memory, MemoryView>
where
    Export: crate::interpreter::wasm::structures::Export,
    LocalImport: crate::interpreter::wasm::structures::LocalImport,
    Memory: crate::interpreter::wasm::structures::Memory<MemoryView>,
    MemoryView: crate::interpreter::wasm::structures::MemoryView,
    Instance:
        crate::interpreter::wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>,
{
    #[allow(unused_imports)]
    use crate::interpreter::stack::Stackable;
    Box::new({
        move |runtime| -> _ {
            let inputs = runtime.stack.pop(1).ok_or_else(|| {
                InstructionError::new(
                    instruction.clone(),
                    InstructionErrorKind::StackIsTooSmall { needed: 1 },
                )
            })?;

            let offset: usize = to_native::<i32>(&inputs[0], instruction.clone())?
                .try_into()
                .map_err(|e| (e, "offset").into())
                .map_err(|k| InstructionError::new(instruction.clone(), k))?;

            // TODO: size = 0
            let instance = &runtime.wasm_instance;
            let record_type = instance.wit_record_by_id(record_type_id).ok_or_else(|| {
                InstructionError::new(
                    instruction.clone(),
                    InstructionErrorKind::RecordTypeByNameIsMissing { record_type_id },
                )
            })?;

            log::trace!(
                "record.lift_memory: record {:?} resolved for type id {}",
                record_type,
                record_type_id
            );

            let record =
                record_lift_memory_(&**instance, record_type, offset, instruction.clone())?;

            log::debug!("record.lift_memory: pushing {:?} on the stack", record);
            runtime.stack.push(record);

            Ok(())
        }
    })
}

pub(super) fn record_lower_memory_<Instance, Export, LocalImport, Memory, MemoryView>(
    instance: &mut Instance,
    instruction: Instruction,
    values: NEVec<IValue>,
) -> Result<i32, InstructionError>
where
    Export: crate::interpreter::wasm::structures::Export,
    LocalImport: crate::interpreter::wasm::structures::LocalImport,
    Memory: crate::interpreter::wasm::structures::Memory<MemoryView>,
    MemoryView: crate::interpreter::wasm::structures::MemoryView,
    Instance:
        crate::interpreter::wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>,
{
    let mut result: Vec<u8> = Vec::with_capacity(values.len());

    for value in values.into_vec() {
        match value {
            IValue::Boolean(value) => result.push(value as _),
            IValue::S8(value) => result.push(value as _),
            IValue::S16(value) => result.extend_from_slice(&value.to_le_bytes()),
            IValue::S32(value) => result.extend_from_slice(&value.to_le_bytes()),
            IValue::S64(value) => result.extend_from_slice(&value.to_le_bytes()),
            IValue::U8(value) => result.push(value),
            IValue::U16(value) => result.extend_from_slice(&value.to_le_bytes()),
            IValue::U32(value) => result.extend_from_slice(&value.to_le_bytes()),
            IValue::U64(value) => result.extend_from_slice(&value.to_le_bytes()),
            IValue::I32(value) => result.extend_from_slice(&value.to_le_bytes()),
            IValue::I64(value) => result.extend_from_slice(&value.to_le_bytes()),
            IValue::F32(value) => result.extend_from_slice(&value.to_le_bytes()),
            IValue::F64(value) => result.extend_from_slice(&value.to_le_bytes()),
            IValue::String(value) => {
                let string_pointer =
                    write_to_instance_mem(instance, instruction.clone(), value.as_bytes())?;

                result.extend_from_slice(&string_pointer.to_le_bytes());
                result.extend_from_slice(&value.len().to_le_bytes());
            }
            IValue::ByteArray(value) => {
                let array_pointer = write_to_instance_mem(instance, instruction.clone(), &value)?;

                result.extend_from_slice(&array_pointer.to_le_bytes());
                result.extend_from_slice(&value.len().to_le_bytes());
            }

            IValue::Array(values) => {
                let (offset, size) =
                    super::array_lower_memory_impl(instance, instruction.clone(), values)?;

                result.extend_from_slice(&offset.to_le_bytes());
                result.extend_from_slice(&size.to_le_bytes());
            }

            IValue::Record(values) => {
                let record_ptr = record_lower_memory_(instance, instruction.clone(), values)?;

                result.extend_from_slice(&record_ptr.to_le_bytes());
            }
        }
    }

    let result_pointer = write_to_instance_mem(instance, instruction, &result)?;

    Ok(result_pointer as _)
}

pub(crate) fn record_lower_memory<Instance, Export, LocalImport, Memory, MemoryView>(
    record_type_id: u64,
    instruction: Instruction,
) -> crate::interpreter::ExecutableInstruction<Instance, Export, LocalImport, Memory, MemoryView>
where
    Export: crate::interpreter::wasm::structures::Export,
    LocalImport: crate::interpreter::wasm::structures::LocalImport,
    Memory: crate::interpreter::wasm::structures::Memory<MemoryView>,
    MemoryView: crate::interpreter::wasm::structures::MemoryView,
    Instance:
        crate::interpreter::wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>,
{
    #[allow(unused_imports)]
    use crate::interpreter::stack::Stackable;
    Box::new({
        move |runtime| -> _ {
            let instance = &mut runtime.wasm_instance;

            match runtime.stack.pop1() {
                Some(IValue::Record(record_fields)) => {
                    is_record_fields_compatible_to_type(
                        &**instance,
                        record_type_id,
                        &record_fields,
                        instruction.clone(),
                    )?;

                    log::debug!("record.lower_memory: obtained {:?} values on the stack for record type = {}", record_fields, record_type_id);

                    let offset =
                        record_lower_memory_(*instance, instruction.clone(), record_fields)?;

                    log::debug!("record.lower_memory: pushing {} on the stack", offset);
                    runtime.stack.push(IValue::I32(offset));

                    Ok(())
                }
                Some(value) => instr_error!(
                    instruction.clone(),
                    InstructionErrorKind::InvalidValueOnTheStack {
                        expected_type: IType::Record(record_type_id),
                        received_value: value,
                    }
                ),
                None => instr_error!(
                    instruction.clone(),
                    InstructionErrorKind::StackIsTooSmall { needed: 1 }
                ),
            }
        }
    })
}
