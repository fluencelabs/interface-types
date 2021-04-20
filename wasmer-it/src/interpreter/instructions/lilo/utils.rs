use super::LiLoError;
use super::LiLoResult;
use crate::interpreter::instructions::ALLOCATE_FUNC_INDEX;
use crate::interpreter::wasm;
use crate::interpreter::wasm::structures::{FunctionIndex, TypedIndex};

use crate::IRecordType;
use crate::IValue;

use std::rc::Rc;

pub(crate) type AllocateFunc<'i> = Box<dyn Fn(usize, usize) -> LiLoResult<usize> + 'i>;
pub(crate) type RecordResolver<'i> = Box<dyn Fn(u64) -> LiLoResult<Rc<IRecordType>> + 'i>;

pub(super) fn build_allocate_func<'instance, Instance, Export, LocalImport, Memory, MemoryView>(
    instance: &'instance Instance,
) -> LiLoResult<AllocateFunc<'instance>>
where
    Export: wasm::structures::Export + 'instance,
    LocalImport: wasm::structures::LocalImport + 'instance,
    Memory: wasm::structures::Memory<MemoryView> + 'instance,
    MemoryView: wasm::structures::MemoryView,
    Instance: wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>,
{
    let closure = move |size: usize, ty: usize| {
        let index = FunctionIndex::new(ALLOCATE_FUNC_INDEX as usize);
        let local_or_import =
            instance
                .local_or_import(index)
                .ok_or(LiLoError::AllocateFuncIsMissing {
                    function_index: ALLOCATE_FUNC_INDEX,
                })?;

        let inputs = vec![IValue::I32(size as _), IValue::I32(ty as _)];
        // TODO: we could check it only once on the module startup or memorize check result
        crate::interpreter::instructions::check_function_signature(
            instance,
            local_or_import,
            &inputs,
        )
        .map_err(|_| LiLoError::AllocateFuncIncompatibleSignature)?;

        let outcome = local_or_import
            .call(&inputs)
            .map_err(|_| LiLoError::AllocateCallFailed)?;

        if outcome.len() != 1 {
            return Err(LiLoError::AllocateFuncIncompatibleOutput);
        }

        match outcome[0] {
            IValue::I32(offset) => Ok(offset as _),
            _ => Err(LiLoError::AllocateFuncIncompatibleOutput),
        }
    };

    Ok(Box::new(closure))
}

pub(super) fn build_record_resolver<'instance, Instance, Export, LocalImport, Memory, MemoryView>(
    instance: &'instance Instance,
) -> LiLoResult<RecordResolver<'instance>>
where
    Export: wasm::structures::Export + 'instance,
    LocalImport: wasm::structures::LocalImport + 'instance,
    Memory: wasm::structures::Memory<MemoryView> + 'instance,
    MemoryView: wasm::structures::MemoryView,
    Instance: wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>,
{
    let resolver = move |record_type_id: u64| {
        let record = instance
            .wit_record_by_id(record_type_id)
            .ok_or(LiLoError::RecordTypeByNameIsMissing { record_type_id })?;

        Ok(record.clone())
    };

    Ok(Box::new(resolver))
}
