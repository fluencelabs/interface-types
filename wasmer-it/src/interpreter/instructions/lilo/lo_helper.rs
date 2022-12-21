use crate::interpreter::wasm;
use crate::interpreter::wasm::structures::FunctionIndex;
use crate::IValue;

use it_lilo::traits::Allocatable;
use it_lilo::traits::AllocatableError;
use it_lilo::traits::DEFAULT_MEMORY_INDEX;

use std::marker::PhantomData;

pub struct LoHelper<'i, Instance, Export, LocalImport, Memory, MemoryView, Store>
where
    Export: wasm::structures::Export + 'i,
    LocalImport: wasm::structures::LocalImport<Store> + 'i,
    Memory: wasm::structures::Memory<MemoryView> + 'i,
    MemoryView: wasm::structures::MemoryView,
    Instance: wasm::structures::Instance<Export, LocalImport, Memory, MemoryView, Store>,
    Store: wasm::structures::Store,
{
    pub(crate) instance: &'i Instance,
    _export: PhantomData<Export>,
    _local_import: PhantomData<LocalImport>,
    _memory: PhantomData<Memory>,
    _memory_view: PhantomData<MemoryView>,
    _store: PhantomData<Store>,
}

impl<'i, Instance, Export, LocalImport, Memory, MemoryView, Store>
    LoHelper<'i, Instance, Export, LocalImport, Memory, MemoryView, Store>
where
    Export: wasm::structures::Export + 'i,
    LocalImport: wasm::structures::LocalImport<Store> + 'i,
    Memory: wasm::structures::Memory<MemoryView> + 'i,
    MemoryView: wasm::structures::MemoryView,
    Instance: wasm::structures::Instance<Export, LocalImport, Memory, MemoryView, Store>,
    Store: wasm::structures::Store,
{
    pub(crate) fn new(instance: &'i Instance) -> Self {
        Self {
            instance,
            _export: PhantomData,
            _local_import: PhantomData,
            _memory: PhantomData,
            _memory_view: PhantomData,
            _store: PhantomData,
        }
    }
}

impl<'i, Instance, Export, LocalImport, Memory, MemoryView, Store> Allocatable<MemoryView, Store>
    for LoHelper<'i, Instance, Export, LocalImport, Memory, MemoryView, Store>
where
    Export: wasm::structures::Export + 'i,
    LocalImport: wasm::structures::LocalImport<Store> + 'i,
    Memory: wasm::structures::Memory<MemoryView> + 'i,
    MemoryView: wasm::structures::MemoryView,
    Instance: wasm::structures::Instance<Export, LocalImport, Memory, MemoryView, Store>,
    Store: wasm::structures::Store,
{
    fn allocate(
        &mut self,
        store: &mut <Store as wasm::structures::Store>::ActualStore<'_>,
        size: u32,
        type_tag: u32,
    ) -> Result<(u32, MemoryView), AllocatableError> {
        use AllocatableError::*;

        use crate::interpreter::instructions::ALLOCATE_FUNC_INDEX;
        use crate::interpreter::wasm::structures::TypedIndex;

        let index = FunctionIndex::new(ALLOCATE_FUNC_INDEX as usize);
        let local_or_import =
            self.instance
                .local_or_import(index)
                .ok_or(AllocateFuncIsMissing {
                    function_index: ALLOCATE_FUNC_INDEX,
                })?;

        let inputs = vec![IValue::I32(size as _), IValue::I32(type_tag as _)];
        // TODO: we could check it only once on the module startup or memorize check result
        crate::interpreter::instructions::check_function_signature(
            self.instance,
            local_or_import,
            &inputs,
        )
        .map_err(|_| AllocateFuncIncompatibleSignature)?;

        let outcome = local_or_import
            .call(store, &inputs)
            .map_err(|_| AllocateCallFailed)?;

        if outcome.len() != 1 {
            return Err(AllocateFuncIncompatibleOutput);
        }

        match outcome[0] {
            IValue::I32(offset) => {
                let view =
                    self.instance
                        .memory_view(DEFAULT_MEMORY_INDEX)
                        .ok_or(MemoryIsMissing {
                            memory_index: DEFAULT_MEMORY_INDEX,
                        })?;

                Ok((offset as _, view))
            }
            _ => Err(AllocateFuncIncompatibleOutput),
        }
    }
}
