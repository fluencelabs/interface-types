#![allow(missing_docs)]

use crate::ast::FunctionArg;
use crate::IRecordType;
use crate::IType;
use crate::IValue;
use std::rc::Rc;

pub use it_traits::{Memory, MemoryView, SequentialReader, SequentialWriter};

pub trait TypedIndex: Copy + Clone {
    fn new(index: usize) -> Self;
    fn index(&self) -> usize;
}

macro_rules! typed_index {
    ($type:ident) => {
        #[derive(Copy, Clone)]
        pub struct $type(usize);

        impl TypedIndex for $type {
            fn new(index: usize) -> Self {
                Self(index)
            }

            fn index(&self) -> usize {
                self.0
            }
        }
    };
}

typed_index!(FunctionIndex);
typed_index!(LocalFunctionIndex);
typed_index!(ImportFunctionIndex);

pub trait LocalImportIndex {
    type Local: TypedIndex;
    type Import: TypedIndex;
}

impl LocalImportIndex for FunctionIndex {
    type Local = LocalFunctionIndex;
    type Import = ImportFunctionIndex;
}

pub trait Export {
    fn name(&self) -> &str;
    fn inputs_cardinality(&self) -> usize;
    fn outputs_cardinality(&self) -> usize;
    fn arguments(&self) -> &[FunctionArg];
    fn outputs(&self) -> &[IType];
    fn call(&self, arguments: &[IValue]) -> Result<Vec<IValue>, ()>;
}

pub trait LocalImport {
    fn name(&self) -> &str;
    fn inputs_cardinality(&self) -> usize;
    fn outputs_cardinality(&self) -> usize;
    fn arguments(&self) -> &[FunctionArg];
    fn outputs(&self) -> &[IType];
    fn call(&self, arguments: &[IValue]) -> Result<Vec<IValue>, ()>;
}

pub trait Instance<E, LI, M, MV>
where
    E: Export,
    LI: LocalImport,
    M: Memory<MV>,
    MV: MemoryView,
{
    fn export(&self, export_name: &str) -> Option<&E>;
    fn local_or_import<I: TypedIndex + LocalImportIndex>(&self, index: I) -> Option<&LI>;
    fn memory(&self, index: usize) -> Option<&M>;
    fn memory_view(&self, index: usize) -> Option<MV>;
    fn wit_record_by_id(&self, index: u64) -> Option<&Rc<IRecordType>>;
}

impl Export for () {
    fn name(&self) -> &str {
        ""
    }

    fn inputs_cardinality(&self) -> usize {
        0
    }

    fn outputs_cardinality(&self) -> usize {
        0
    }

    fn arguments(&self) -> &[FunctionArg] {
        &[]
    }

    fn outputs(&self) -> &[IType] {
        &[]
    }

    fn call(&self, _arguments: &[IValue]) -> Result<Vec<IValue>, ()> {
        Err(())
    }
}

impl LocalImport for () {
    fn name(&self) -> &str {
        ""
    }

    fn inputs_cardinality(&self) -> usize {
        0
    }

    fn outputs_cardinality(&self) -> usize {
        0
    }

    fn arguments(&self) -> &[FunctionArg] {
        &[]
    }

    fn outputs(&self) -> &[IType] {
        &[]
    }

    fn call(&self, _arguments: &[IValue]) -> Result<Vec<IValue>, ()> {
        Err(())
    }
}

struct EmptySeqWriter();
struct EmptySeqReader();

impl SequentialWriter for EmptySeqWriter {
    fn start_offset(&self) -> usize {
        0
    }

    fn write_u8(&self, _value: u8) {}

    fn write_u32(&self, _value: u32) {}

    fn write_bytes(&self, _bytes: &[u8]) {}
}

impl SequentialReader for EmptySeqReader {
    fn read_bool(&self) -> bool {
        todo!()
    }

    fn read_u8(&self) -> u8 {
        todo!()
    }

    fn read_i8(&self) -> i8 {
        todo!()
    }

    fn read_u16(&self) -> u16 {
        todo!()
    }

    fn read_i16(&self) -> i16 {
        todo!()
    }

    fn read_u32(&self) -> u32 {
        todo!()
    }

    fn read_i32(&self) -> i32 {
        todo!()
    }

    fn read_f32(&self) -> f32 {
        todo!()
    }

    fn read_u64(&self) -> u64 {
        todo!()
    }

    fn read_i64(&self) -> i64 {
        todo!()
    }

    fn read_f64(&self) -> f64 {
        todo!()
    }
}

pub(crate) struct EmptyMemoryView;

impl MemoryView for EmptyMemoryView {
    fn sequential_writer<'s>(
        &'s self,
        _offset: usize,
        _size: usize,
    ) -> Box<dyn SequentialWriter + 's> {
        Box::new(EmptySeqWriter())
    }

    fn sequential_reader<'s>(
        &'s self,
        _offset: usize,
        _size: usize,
    ) -> Box<dyn SequentialReader + 's> {
        Box::new(EmptySeqReader())
    }
}

impl Memory<EmptyMemoryView> for () {
    fn view(&self) -> EmptyMemoryView {
        EmptyMemoryView
    }
}

impl<E, LI, M, MV> Instance<E, LI, M, MV> for ()
where
    E: Export,
    LI: LocalImport,
    M: Memory<MV>,
    MV: MemoryView,
{
    fn export(&self, _export_name: &str) -> Option<&E> {
        None
    }

    fn memory(&self, _: usize) -> Option<&M> {
        None
    }

    fn memory_view(&self, _index: usize) -> Option<MV> {
        None
    }

    fn local_or_import<I: TypedIndex + LocalImportIndex>(&self, _index: I) -> Option<&LI> {
        None
    }

    fn wit_record_by_id(&self, _index: u64) -> Option<&Rc<IRecordType>> {
        None
    }
}
