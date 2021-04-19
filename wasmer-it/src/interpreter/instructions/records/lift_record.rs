use super::read_from_instance_mem;

use super::value_reader::ValueReader;
use crate::IRecordType;
use crate::IType;
use crate::IValue;
use crate::NEVec;
use crate::{
    errors::{InstructionError, InstructionErrorKind},
    interpreter::Instruction,
};

#[rustfmt::skip]
pub(crate) fn record_lift_memory_impl<'instance, Instance, Export, LocalImport, Memory, MemoryView>(
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
    Instance: crate::interpreter::wasm::structures::Instance<Export, LocalImport, Memory, MemoryView> + 'instance,
{
    let mut values = Vec::with_capacity(record_type.fields.len());

    let size = record_size(record_type);
    let data = read_from_instance_mem(instance, instruction.clone(), offset, size)?;
    let reader = ValueReader::new(data);

    for field in (*record_type.fields).iter() {
        match &field.ty {
            IType::Boolean => values.push(IValue::Boolean(reader.read_u8() == 1)),
            IType::S8 => values.push(IValue::S8(reader.read_i8())),
            IType::S16 => values.push(IValue::S16(reader.read_i16())),
            IType::S32 => values.push(IValue::S32(reader.read_i32())),
            IType::S64 => values.push(IValue::S64(reader.read_i64())),
            IType::I32 => values.push(IValue::I32(reader.read_i32())),
            IType::I64 => values.push(IValue::I64(reader.read_i64())),
            IType::U8 => values.push(IValue::U8(reader.read_u8())),
            IType::U16 => values.push(IValue::U16(reader.read_u16())),
            IType::U32 => values.push(IValue::U32(reader.read_u32())),
            IType::U64 => values.push(IValue::U64(reader.read_u64())),
            IType::F32 => values.push(IValue::F32(reader.read_f32())),
            IType::F64 => values.push(IValue::F64(reader.read_f64())),
            IType::String => values.push(IValue::String(read_string(instance, instruction.clone(), &reader)?)),
            IType::ByteArray => values.push(read_byte_array(instance, instruction.clone(), &reader)?),
            IType::Array(ty) =>  values.push(read_array(instance, instruction.clone(), &reader, &**ty)?),
            IType::Record(record_type_id) => values.push(read_record(instance, instruction.clone(), &reader, *record_type_id)?),
        }
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

fn read_string<'instance, Instance, Export, LocalImport, Memory, MemoryView>(
    instance: &Instance,
    instruction: Instruction,
    reader: &ValueReader,
) -> Result<String, InstructionError>
where
    Export: crate::interpreter::wasm::structures::Export,
    LocalImport: crate::interpreter::wasm::structures::LocalImport,
    Memory: crate::interpreter::wasm::structures::Memory<MemoryView>,
    MemoryView: crate::interpreter::wasm::structures::MemoryView,
    Instance: crate::interpreter::wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>
        + 'instance,
{
    let string_offset = reader.read_u32();
    let string_size = reader.read_u32();

    let string_mem = read_from_instance_mem(
        instance,
        instruction.clone(),
        string_offset as _,
        string_size as _,
    )?;

    let string = String::from_utf8(string_mem).map_err(|e| {
        InstructionError::new(instruction, InstructionErrorKind::CorruptedUTF8String(e))
    })?;

    Ok(string)
}

fn read_byte_array<'instance, Instance, Export, LocalImport, Memory, MemoryView>(
    instance: &Instance,
    instruction: Instruction,
    reader: &ValueReader,
) -> Result<IValue, InstructionError>
where
    Export: crate::interpreter::wasm::structures::Export,
    LocalImport: crate::interpreter::wasm::structures::LocalImport,
    Memory: crate::interpreter::wasm::structures::Memory<MemoryView>,
    MemoryView: crate::interpreter::wasm::structures::MemoryView,
    Instance: crate::interpreter::wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>
        + 'instance,
{
    let offset = reader.read_u32();
    let elements_count = reader.read_u32();

    let array = read_from_instance_mem(instance, instruction, offset as _, elements_count as _)?;
    let byte_array = IValue::ByteArray(array);

    Ok(byte_array)
}

fn read_array<'instance, Instance, Export, LocalImport, Memory, MemoryView>(
    instance: &Instance,
    instruction: Instruction,
    reader: &ValueReader,
    ty: &IType,
) -> Result<IValue, InstructionError>
where
    Export: crate::interpreter::wasm::structures::Export,
    LocalImport: crate::interpreter::wasm::structures::LocalImport,
    Memory: crate::interpreter::wasm::structures::Memory<MemoryView>,
    MemoryView: crate::interpreter::wasm::structures::MemoryView,
    Instance: crate::interpreter::wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>
        + 'instance,
{
    let array_offset = reader.read_u32();
    let elements_count = reader.read_u32();

    super::array_lift_memory_impl(
        instance,
        ty,
        array_offset as _,
        elements_count as _,
        instruction,
    )
}

fn read_record<'instance, Instance, Export, LocalImport, Memory, MemoryView>(
    instance: &Instance,
    instruction: Instruction,
    reader: &ValueReader,
    record_type_id: u64,
) -> Result<IValue, InstructionError>
where
    Export: crate::interpreter::wasm::structures::Export,
    LocalImport: crate::interpreter::wasm::structures::LocalImport,
    Memory: crate::interpreter::wasm::structures::Memory<MemoryView>,
    MemoryView: crate::interpreter::wasm::structures::MemoryView,
    Instance: crate::interpreter::wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>
        + 'instance,
{
    let offset = reader.read_u32();

    let record_type = instance.wit_record_by_id(record_type_id).ok_or_else(|| {
        InstructionError::new(
            instruction.clone(),
            InstructionErrorKind::RecordTypeByNameIsMissing { record_type_id },
        )
    })?;

    record_lift_memory_impl(instance, record_type, offset as _, instruction)
}
