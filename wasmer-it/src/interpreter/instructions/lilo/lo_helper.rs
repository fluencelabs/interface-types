use super::AllocateFunc;
use super::LiLoResult;

use crate::interpreter::wasm;
use crate::IType;

use it_lilo_utils::memory_writer::MemoryWriter;
use it_lilo_utils::type_code_form_itype;

use std::cell::Cell;

pub(crate) struct LoHelper<'i> {
    pub(crate) writer: MemoryWriter<'i>,
    pub(crate) allocate: AllocateFunc<'i>,
}

impl<'instance> LoHelper<'instance> {
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
        let allocate = super::build_allocate_func(instance)?;
        let writer = MemoryWriter::new(memory);

        let helper = Self { writer, allocate };

        Ok(helper)
    }

    pub(crate) fn write_to_mem(&self, bytes: &[u8]) -> LiLoResult<usize> {
        let alloc_type_code = type_code_form_itype(&IType::U8);
        let offset = (self.allocate)(bytes.len() as _, alloc_type_code as _)?;

        self.writer.write_bytes(offset, bytes)?;

        Ok(offset)
    }
}
