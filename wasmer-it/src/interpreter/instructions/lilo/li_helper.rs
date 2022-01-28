use crate::interpreter::wasm;
use crate::IRecordType;

use it_lilo::traits::RecordResolvable;
use it_lilo::traits::RecordResolvableError;

use std::marker::PhantomData;

pub struct LiHelper<'i, Instance, Export, LocalImport, Memory, SequentialMemoryView>
where
    Export: wasm::structures::Export + 'i,
    LocalImport: wasm::structures::LocalImport + 'i,
    Memory: wasm::structures::Memory<SequentialMemoryView> + 'i,
    SequentialMemoryView: (for<'a> wasm::structures::SequentialMemoryView<'a>) + 'i,
    Instance: wasm::structures::Instance<Export, LocalImport, Memory, SequentialMemoryView>,
{
    pub(crate) instance: &'i Instance,
    _export: PhantomData<Export>,
    _local_import: PhantomData<LocalImport>,
    _memory: PhantomData<Memory>,
    _memory_view: PhantomData<SequentialMemoryView>,
}

impl<'i, Instance, Export, LocalImport, Memory, SequentialMemoryView>
    LiHelper<'i, Instance, Export, LocalImport, Memory, SequentialMemoryView>
where
    Export: wasm::structures::Export + 'i,
    LocalImport: wasm::structures::LocalImport + 'i,
    Memory: wasm::structures::Memory<SequentialMemoryView> + 'i,
    SequentialMemoryView: (for<'a> wasm::structures::SequentialMemoryView<'a>),
    Instance: wasm::structures::Instance<Export, LocalImport, Memory, SequentialMemoryView>,
{
    pub(crate) fn new(instance: &'i Instance) -> Self {
        Self {
            instance,
            _export: PhantomData,
            _local_import: PhantomData,
            _memory: PhantomData,
            _memory_view: PhantomData,
        }
    }
}

impl<'i, Instance, Export, LocalImport, Memory, SequentialMemoryView> RecordResolvable
    for LiHelper<'i, Instance, Export, LocalImport, Memory, SequentialMemoryView>
where
    Export: wasm::structures::Export + 'i,
    LocalImport: wasm::structures::LocalImport + 'i,
    Memory: wasm::structures::Memory<SequentialMemoryView> + 'i,
    SequentialMemoryView: (for<'a> wasm::structures::SequentialMemoryView<'a>),
    Instance: wasm::structures::Instance<Export, LocalImport, Memory, SequentialMemoryView>,
{
    fn resolve_record(&self, record_type_id: u64) -> Result<&IRecordType, RecordResolvableError> {
        let record = self
            .instance
            .wit_record_by_id(record_type_id)
            .ok_or(RecordResolvableError::RecordNotFound(record_type_id))?;

        Ok(record)
    }
}
