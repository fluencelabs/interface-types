use crate::interpreter::wasm;
use crate::interpreter::wasm::structures::FunctionIndex;
use crate::IValue;

use it_lilo_utils::error::MemoryWriteError;
use it_lilo_utils::memory_writer::Heapable;
use it_lilo_utils::WriteResult;

use std::cell::Cell;
use std::marker::PhantomData;

pub struct LoHelper<'i, Instance, Export, LocalImport, Memory, MemoryView>
where
    Export: wasm::structures::Export + 'i,
    LocalImport: wasm::structures::LocalImport + 'i,
    Memory: wasm::structures::Memory<MemoryView> + 'i,
    MemoryView: wasm::structures::MemoryView,
    Instance: wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>,
{
    pub(crate) instance: &'i Instance,
    _phantom_export: PhantomData<Export>,
    _phantom_local_import: PhantomData<LocalImport>,
    _phantom_memory: PhantomData<Memory>,
    _phantom_memory_view: PhantomData<MemoryView>,
}

impl<'i, Instance, Export, LocalImport, Memory, MemoryView>
    LoHelper<'i, Instance, Export, LocalImport, Memory, MemoryView>
where
    Export: wasm::structures::Export + 'i,
    LocalImport: wasm::structures::LocalImport + 'i,
    Memory: wasm::structures::Memory<MemoryView> + 'i,
    MemoryView: wasm::structures::MemoryView,
    Instance: wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>,
{
    pub(crate) fn new(instance: &'i Instance) -> Self {
        Self {
            instance,
            _phantom_export: PhantomData,
            _phantom_local_import: PhantomData,
            _phantom_memory: PhantomData,
            _phantom_memory_view: PhantomData,
        }
    }
}

impl<'i, Instance, Export, LocalImport, Memory, MemoryView> Heapable
    for LoHelper<'i, Instance, Export, LocalImport, Memory, MemoryView>
where
    Export: wasm::structures::Export + 'i,
    LocalImport: wasm::structures::LocalImport + 'i,
    Memory: wasm::structures::Memory<MemoryView> + 'i,
    MemoryView: wasm::structures::MemoryView,
    Instance: wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>,
{
    fn allocate(&self, size: u32, type_tag: u32) -> WriteResult<usize> {
        use crate::interpreter::instructions::ALLOCATE_FUNC_INDEX;
        use crate::interpreter::wasm::structures::TypedIndex;

        let index = FunctionIndex::new(ALLOCATE_FUNC_INDEX as usize);
        let local_or_import = self.instance.local_or_import(index).ok_or(
            MemoryWriteError::AllocateFuncIsMissing {
                function_index: ALLOCATE_FUNC_INDEX,
            },
        )?;

        let inputs = vec![IValue::I32(size as _), IValue::I32(type_tag as _)];
        // TODO: we could check it only once on the module startup or memorize check result
        crate::interpreter::instructions::check_function_signature(
            self.instance,
            local_or_import,
            &inputs,
        )
        .map_err(|_| MemoryWriteError::AllocateFuncIncompatibleSignature)?;

        let outcome = local_or_import
            .call(&inputs)
            .map_err(|_| MemoryWriteError::AllocateCallFailed)?;

        if outcome.len() != 1 {
            return Err(MemoryWriteError::AllocateFuncIncompatibleOutput);
        }

        match outcome[0] {
            IValue::I32(offset) => Ok(offset as _),
            _ => Err(MemoryWriteError::AllocateFuncIncompatibleOutput),
        }
    }

    fn memory_slice(&self, memory_index: usize) -> WriteResult<&[Cell<u8>]> {
        self.instance
            .memory_slice(memory_index)
            .ok_or(MemoryWriteError::MemoryIsMissing { memory_index })
    }
}
