use super::LiLoError;
use super::LiLoResult;
use crate::interpreter::wasm;
use crate::IRecordType;

use it_lilo_utils::memory_reader::MemoryReader;

use std::cell::Cell;
use std::rc::Rc;

pub(crate) struct LiHelper<'i> {
    pub(crate) reader: MemoryReader<'i>,
    pub(crate) record_resolver: RecordResolver<'i>,
}

impl<'instance> LiHelper<'instance> {
    pub(crate) fn new<Instance, Export, LocalImport, Memory, MemoryView>(
        instance: &'instance Instance,
        memory: &'instance [Cell<u8>],
    ) -> LiLoResult<Self>
    where
        Export: wasm::structures::Export + 'instance,
        LocalImport: wasm::structures::LocalImport + 'instance,
        Memory: wasm::structures::Memory<MemoryView> + 'instance,
        MemoryView: wasm::structures::MemoryView,
        Instance: wasm::structures::Instance<Export, LocalImport, Memory, MemoryView>,
    {
        let record_resolver = build_record_resolver(instance)?;
        let reader = MemoryReader::new(memory);

        let helper = Self {
            reader,
            record_resolver,
        };

        Ok(helper)
    }
}

pub(crate) type RecordResolver<'i> = Box<dyn Fn(u64) -> LiLoResult<Rc<IRecordType>> + 'i>;

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
