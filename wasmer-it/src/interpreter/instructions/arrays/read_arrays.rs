use crate::instr_error;
use crate::interpreter::wasm;
use crate::IValue;
use crate::{
    errors::{InstructionError, InstructionErrorKind},
    interpreter::Instruction,
};

use std::cell::Cell;

macro_rules! def_read_func {
    ($func_name: ident, ($ty:ident, $elements_count:ident), $ctor:expr) => {
        pub(super) fn $func_name<'instance, Instance, Export, LocalImport, Memory, MemoryView>(
            instance: &'instance Instance,
            instruction: Instruction,
            offset: usize,
            $elements_count: usize,
        ) -> Result<IValue, InstructionError>
        where
            Export: wasm::structures::Export + 'instance,
            LocalImport: wasm::structures::LocalImport + 'instance,
            Memory: wasm::structures::Memory<MemoryView> + 'instance,
            MemoryView: wasm::structures::MemoryView,
            Instance: wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>,
        {
            let value_size = std::mem::size_of::<$ty>();

            let ctor_ = $ctor;

            let ivalues = ivalues_from_mem(
                instance,
                instruction,
                offset,
                value_size * $elements_count,
                ctor_,
            )?;

            let ivalue = IValue::Array(ivalues);
            Ok(ivalue)
        }
    };
}

fn ivalues_from_mem<'instance, Instance, Export, LocalImport, Memory, MemoryView>(
    instance: &'instance Instance,
    instruction: Instruction,
    offset: usize,
    size: usize,
    ivalue_ctor: impl FnOnce(&[Cell<u8>]) -> Vec<IValue>,
) -> Result<Vec<IValue>, InstructionError>
where
    Export: wasm::structures::Export + 'instance,
    LocalImport: wasm::structures::LocalImport + 'instance,
    Memory: wasm::structures::Memory<MemoryView> + 'instance,
    MemoryView: wasm::structures::MemoryView + 'instance,
    Instance: wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>,
{
    let memory_index = 0;
    let memory_view = instance
        .memory(memory_index)
        .ok_or_else(|| {
            InstructionError::new(
                instruction.clone(),
                InstructionErrorKind::MemoryIsMissing { memory_index },
            )
        })?
        .view();

    log::trace!("reading {} bytes from offset {}", size, offset);

    let right = offset + size;
    if right < offset || right >= memory_view.len() {
        return instr_error!(
            instruction,
            InstructionErrorKind::MemoryOutOfBoundsAccess {
                index: right,
                length: memory_view.len(),
            }
        );
    }

    let view = &memory_view[offset..offset + size];
    let ivalues = ivalue_ctor(view);
    Ok(ivalues)
}

def_read_func!(read_bool_array, (bool, elements_count), {
    |memory_view: &[Cell<u8>]| {
        let mut result = Vec::with_capacity(elements_count);
        for element_id in 0..elements_count {
            let value = Cell::get(&memory_view[element_id]);
            result.push(IValue::Boolean(value == 1));
        }

        result
    }
});

def_read_func!(read_u8_array, (u8, elements_count), {
    |memory_view: &[Cell<u8>]| {
        let mut result = Vec::with_capacity(elements_count);
        for element_id in 0..elements_count {
            let value = Cell::get(&memory_view[element_id]);
            result.push(IValue::U8(value));
        }

        result
    }
});

def_read_func!(read_s8_array, (i8, elements_count), {
    |memory_view: &[Cell<u8>]| {
        let mut result = Vec::with_capacity(elements_count);
        for element_id in 0..elements_count {
            let value = i8::from_le_bytes([Cell::get(&memory_view[element_id])]);
            result.push(IValue::S8(value));
        }

        result
    }
});

def_read_func!(read_u16_array, (u16, elements_count), {
    |memory_view: &[Cell<u8>]| {
        let mut result = Vec::with_capacity(elements_count);
        for element_id in 0..elements_count {
            let value = u16::from_le_bytes([
                Cell::get(&memory_view[2 * element_id]),
                Cell::get(&memory_view[2 * element_id + 1]),
            ]);
            result.push(IValue::U16(value));
        }

        result
    }
});

def_read_func!(read_s16_array, (i16, elements_count), {
    |memory_view: &[Cell<u8>]| {
        let mut result = Vec::with_capacity(elements_count);
        for element_id in 0..elements_count {
            let value = i16::from_le_bytes([
                Cell::get(&memory_view[2 * element_id]),
                Cell::get(&memory_view[2 * element_id + 1]),
            ]);
            result.push(IValue::S16(value));
        }

        result
    }
});

def_read_func!(read_u32_array, (u32, elements_count), {
    |memory_view: &[Cell<u8>]| {
        let mut result = Vec::with_capacity(elements_count);
        for element_id in 0..elements_count {
            let value = u32::from_le_bytes([
                Cell::get(&memory_view[4 * element_id]),
                Cell::get(&memory_view[4 * element_id + 1]),
                Cell::get(&memory_view[4 * element_id + 2]),
                Cell::get(&memory_view[4 * element_id + 3]),
            ]);
            result.push(IValue::U32(value));
        }

        result
    }
});

def_read_func!(read_f32_array, (f32, elements_count), {
    |memory_view: &[Cell<u8>]| {
        let mut result = Vec::with_capacity(elements_count);
        for element_id in 0..elements_count {
            let value = f32::from_le_bytes([
                Cell::get(&memory_view[4 * element_id]),
                Cell::get(&memory_view[4 * element_id + 1]),
                Cell::get(&memory_view[4 * element_id + 2]),
                Cell::get(&memory_view[4 * element_id + 3]),
            ]);
            result.push(IValue::F32(value));
        }

        result
    }
});

def_read_func!(read_s32_array, (i32, elements_count), {
    |memory_view: &[Cell<u8>]| {
        let mut result = Vec::with_capacity(elements_count);
        for element_id in 0..elements_count {
            let value = i32::from_le_bytes([
                Cell::get(&memory_view[4 * element_id]),
                Cell::get(&memory_view[4 * element_id + 1]),
                Cell::get(&memory_view[4 * element_id + 2]),
                Cell::get(&memory_view[4 * element_id + 3]),
            ]);
            result.push(IValue::S32(value));
        }

        result
    }
});

def_read_func!(read_i32_array, (i32, elements_count), {
    |memory_view: &[Cell<u8>]| {
        let mut result = Vec::with_capacity(elements_count);
        for element_id in 0..elements_count {
            let value = i32::from_le_bytes([
                Cell::get(&memory_view[4 * element_id]),
                Cell::get(&memory_view[4 * element_id + 1]),
                Cell::get(&memory_view[4 * element_id + 2]),
                Cell::get(&memory_view[4 * element_id + 3]),
            ]);
            result.push(IValue::I32(value));
        }

        result
    }
});

def_read_func!(read_u64_array, (u64, elements_count), {
    |memory_view: &[Cell<u8>]| {
        let mut result = Vec::with_capacity(elements_count);
        for element_id in 0..elements_count {
            let value = u64::from_le_bytes([
                Cell::get(&memory_view[4 * element_id]),
                Cell::get(&memory_view[4 * element_id + 1]),
                Cell::get(&memory_view[4 * element_id + 2]),
                Cell::get(&memory_view[4 * element_id + 3]),
                Cell::get(&memory_view[4 * element_id + 4]),
                Cell::get(&memory_view[4 * element_id + 5]),
                Cell::get(&memory_view[4 * element_id + 6]),
                Cell::get(&memory_view[4 * element_id + 7]),
            ]);
            result.push(IValue::U64(value));
        }

        result
    }
});

def_read_func!(read_f64_array, (f64, elements_count), {
    |memory_view: &[Cell<u8>]| {
        let mut result = Vec::with_capacity(elements_count);
        for element_id in 0..elements_count {
            let value = f64::from_le_bytes([
                Cell::get(&memory_view[4 * element_id]),
                Cell::get(&memory_view[4 * element_id + 1]),
                Cell::get(&memory_view[4 * element_id + 2]),
                Cell::get(&memory_view[4 * element_id + 3]),
                Cell::get(&memory_view[4 * element_id + 4]),
                Cell::get(&memory_view[4 * element_id + 5]),
                Cell::get(&memory_view[4 * element_id + 6]),
                Cell::get(&memory_view[4 * element_id + 7]),
            ]);
            result.push(IValue::F64(value));
        }

        result
    }
});

def_read_func!(read_s64_array, (i64, elements_count), {
    |memory_view: &[Cell<u8>]| {
        let mut result = Vec::with_capacity(elements_count);
        for element_id in 0..elements_count {
            let value = i64::from_le_bytes([
                Cell::get(&memory_view[4 * element_id]),
                Cell::get(&memory_view[4 * element_id + 1]),
                Cell::get(&memory_view[4 * element_id + 2]),
                Cell::get(&memory_view[4 * element_id + 3]),
                Cell::get(&memory_view[4 * element_id + 4]),
                Cell::get(&memory_view[4 * element_id + 5]),
                Cell::get(&memory_view[4 * element_id + 6]),
                Cell::get(&memory_view[4 * element_id + 7]),
            ]);
            result.push(IValue::S64(value));
        }

        result
    }
});

def_read_func!(read_i64_array, (i64, elements_count), {
    |memory_view: &[Cell<u8>]| {
        let mut result = Vec::with_capacity(elements_count);
        for element_id in 0..elements_count {
            let value = i64::from_le_bytes([
                Cell::get(&memory_view[4 * element_id]),
                Cell::get(&memory_view[4 * element_id + 1]),
                Cell::get(&memory_view[4 * element_id + 2]),
                Cell::get(&memory_view[4 * element_id + 3]),
                Cell::get(&memory_view[4 * element_id + 4]),
                Cell::get(&memory_view[4 * element_id + 5]),
                Cell::get(&memory_view[4 * element_id + 6]),
                Cell::get(&memory_view[4 * element_id + 7]),
            ]);
            result.push(IValue::I64(value));
        }

        result
    }
});

use super::read_from_instance_mem;
use safe_transmute::guard::AllOrNothingGuard;
use safe_transmute::transmute_many;

const WASM_POINTER_SIZE: usize = 4;

pub(super) fn read_string_array<'instance, Instance, Export, LocalImport, Memory, MemoryView>(
    instance: &'instance Instance,
    instruction: Instruction,
    offset: usize,
    elements_count: usize,
) -> Result<IValue, InstructionError>
where
    Export: crate::interpreter::wasm::structures::Export,
    LocalImport: crate::interpreter::wasm::structures::LocalImport,
    Memory: crate::interpreter::wasm::structures::Memory<MemoryView>,
    MemoryView: crate::interpreter::wasm::structures::MemoryView,
    Instance: crate::interpreter::wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>
        + 'instance,
{
    let data = read_from_instance_mem(
        instance,
        instruction.clone(),
        offset,
        WASM_POINTER_SIZE * elements_count,
    )?;
    let data = transmute_many::<u32, AllOrNothingGuard>(&data).unwrap();

    if data.is_empty() {
        return Ok(IValue::Array(vec![]));
    }

    let mut result = Vec::with_capacity(data.len() / 2);
    let mut data = data.iter();

    while let Some(string_offset) = data.next() {
        let string_size = data.next().ok_or_else(|| {
            InstructionError::new(
                instruction.clone(),
                InstructionErrorKind::CorruptedArray(String::from(
                    "serialized array must contain even count of elements",
                )),
            )
        })?;

        let string_mem = read_from_instance_mem(
            instance,
            instruction.clone(),
            *string_offset as _,
            *string_size as _,
        )?;

        // TODO: check
        let string = String::from_utf8(string_mem).unwrap();
        result.push(IValue::String(string));
    }

    let result = IValue::Array(result);

    Ok(result)
}

pub(super) fn read_record_array<'instance, Instance, Export, LocalImport, Memory, MemoryView>(
    instance: &'instance Instance,
    instruction: Instruction,
    record_type_id: u64,
    offset: usize,
    elements_count: usize,
) -> Result<IValue, InstructionError>
where
    Export: crate::interpreter::wasm::structures::Export,
    LocalImport: crate::interpreter::wasm::structures::LocalImport,
    Memory: crate::interpreter::wasm::structures::Memory<MemoryView>,
    MemoryView: crate::interpreter::wasm::structures::MemoryView,
    Instance: crate::interpreter::wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>
        + 'instance,
{
    let record_type = instance.wit_record_by_id(record_type_id).ok_or_else(|| {
        InstructionError::new(
            instruction.clone(),
            InstructionErrorKind::RecordTypeByNameIsMissing { record_type_id },
        )
    })?;

    let data = read_from_instance_mem(
        instance,
        instruction.clone(),
        offset,
        WASM_POINTER_SIZE * elements_count,
    )?;
    let data = transmute_many::<u32, AllOrNothingGuard>(&data).unwrap();

    let mut result = Vec::with_capacity(data.len());

    for record_offset in data {
        result.push(super::record_lift_memory_(
            instance,
            record_type,
            *record_offset as _,
            instruction.clone(),
        )?);
    }

    let result = IValue::Array(result);
    Ok(result)
}

pub(super) fn read_array_array<'instance, Instance, Export, LocalImport, Memory, MemoryView>(
    instance: &'instance Instance,
    instruction: Instruction,
    ty: &crate::IType,
    offset: usize,
    elements_count: usize,
) -> Result<IValue, InstructionError>
where
    Export: crate::interpreter::wasm::structures::Export,
    LocalImport: crate::interpreter::wasm::structures::LocalImport,
    Memory: crate::interpreter::wasm::structures::Memory<MemoryView>,
    MemoryView: crate::interpreter::wasm::structures::MemoryView,
    Instance: crate::interpreter::wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>
        + 'instance,
{
    let data = read_from_instance_mem(
        instance,
        instruction.clone(),
        offset,
        WASM_POINTER_SIZE * elements_count,
    )?;
    let data = transmute_many::<u32, AllOrNothingGuard>(&data).unwrap();

    if data.is_empty() {
        return Ok(IValue::Array(vec![]));
    }

    let mut result = Vec::with_capacity(data.len() / 2);
    let mut data = data.iter();

    while let Some(array_offset) = data.next() {
        let array_size = data.next().ok_or_else(|| {
            InstructionError::new(
                instruction.clone(),
                InstructionErrorKind::CorruptedArray(String::from(
                    "serialized array must contain even count of elements",
                )),
            )
        })?;

        let value = match ty {
            crate::IType::ByteArray => {
                let value = read_from_instance_mem(
                    instance,
                    instruction.clone(),
                    *array_offset as _,
                    *array_size as _,
                )?;
                IValue::ByteArray(value)
            }
            _ => super::array_lift_memory_impl(
                instance,
                &*ty,
                *array_offset as _,
                *array_size as _,
                instruction.clone(),
            )?,
        };

        result.push(value);
    }

    let result = IValue::Array(result);
    Ok(result)
}
