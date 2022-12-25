use crate::interpreter::wasm;
use crate::IRecordType;

use it_lilo::traits::RecordResolvable;
use it_lilo::traits::RecordResolvableError;

use std::marker::PhantomData;

pub struct LiHelper<'i, Instance, Export, LocalImport, Memory, MemoryView, Store>
where
    Export: wasm::structures::Export + 'i,
    LocalImport: wasm::structures::LocalImport<Store> + 'i,
    Memory: wasm::structures::Memory<MemoryView, Store> + 'i,
    MemoryView: wasm::structures::MemoryView<Store> + 'i,
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
    LiHelper<'i, Instance, Export, LocalImport, Memory, MemoryView, Store>
where
    Export: wasm::structures::Export + 'i,
    LocalImport: wasm::structures::LocalImport<Store> + 'i,
    Memory: wasm::structures::Memory<MemoryView, Store> + 'i,
    MemoryView: wasm::structures::MemoryView<Store>,
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

impl<'i, Instance, Export, LocalImport, Memory, MemoryView, Store> RecordResolvable
    for LiHelper<'i, Instance, Export, LocalImport, Memory, MemoryView, Store>
where
    Export: wasm::structures::Export + 'i,
    LocalImport: wasm::structures::LocalImport<Store> + 'i,
    Memory: wasm::structures::Memory<MemoryView, Store> + 'i,
    MemoryView: wasm::structures::MemoryView<Store>,
    Instance: wasm::structures::Instance<Export, LocalImport, Memory, MemoryView, Store>,
    Store: wasm::structures::Store,
{
    fn resolve_record(&self, record_type_id: u64) -> Result<&IRecordType, RecordResolvableError> {
        let record = self
            .instance
            .wit_record_by_id(record_type_id)
            .ok_or(RecordResolvableError::RecordNotFound(record_type_id))?;

        Ok(record)
    }
}
