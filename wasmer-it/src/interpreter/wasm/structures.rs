#![allow(missing_docs)]

use crate::ast::FunctionArg;
use crate::IRecordType;
use crate::IType;
use crate::IValue;
use std::rc::Rc;
use std::{cell::Cell, ops::Deref};

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

pub trait MemoryView: Deref<Target = [Cell<u8>]> {}

pub trait Memory<View>
where
    View: MemoryView,
{
    fn view(&self) -> View;
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
    fn memory_slice(&self, index: usize) -> Option<&[Cell<u8>]>;
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

pub(crate) struct EmptyMemoryView;

impl MemoryView for EmptyMemoryView {}

impl Deref for EmptyMemoryView {
    type Target = [Cell<u8>];

    fn deref(&self) -> &Self::Target {
        &[]
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

    fn memory_slice(&self, _: usize) -> Option<&[Cell<u8>]> {
        None
    }

    fn local_or_import<I: TypedIndex + LocalImportIndex>(&self, _index: I) -> Option<&LI> {
        None
    }

    fn wit_record_by_id(&self, _index: u64) -> Option<&Rc<IRecordType>> {
        None
    }
}
