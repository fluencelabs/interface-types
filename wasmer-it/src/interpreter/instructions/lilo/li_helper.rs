use super::LiLoResult;
use super::RecordResolver;
use crate::interpreter::wasm;

use it_lilo_utils::memory_reader::MemoryReader;

use std::cell::Cell;

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
        let record_resolver = super::build_record_resolver(instance)?;
        let reader = MemoryReader::new(memory);

        let helper = Self {
            reader,
            record_resolver,
        };

        Ok(helper)
    }
}
