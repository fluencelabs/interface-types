#![allow(missing_docs)]

use crate::ast::FunctionArg;
use crate::IRecordType;
use crate::IType;
use crate::IValue;

use async_trait::async_trait;

use std::sync::Arc;

pub use it_memory_traits::{Memory, MemoryAccessError, MemoryView};
use it_memory_traits::{MemoryReadable, MemoryWritable};

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

#[async_trait]
pub trait Export: Send {
    fn name(&self) -> &str;
    fn inputs_cardinality(&self) -> usize;
    fn outputs_cardinality(&self) -> usize;
    fn arguments(&self) -> &[FunctionArg];
    fn outputs(&self) -> &[IType];
    //fn call(&self, arguments: &[IValue]) -> Result<Vec<IValue>, ()>;
    async fn call_async(&self, arguments: &[IValue]) -> Result<Vec<IValue>, anyhow::Error>;
}

#[async_trait]
pub trait LocalImport<Store: self::Store>: Send + Sync {
    fn name(&self) -> &str;
    fn inputs_cardinality(&self) -> usize;
    fn outputs_cardinality(&self) -> usize;
    fn arguments(&self) -> &[FunctionArg];
    fn outputs(&self) -> &[IType];
    /*fn call(
        &self,
        store: &mut <Store as self::Store>::ActualStore<'_>,
        arguments: &[IValue],
    ) -> Result<Vec<IValue>, ()>;*/
    async fn call_async(
        &self,
        store: &mut <Store as self::Store>::ActualStore<'_>,
        arguments: &[IValue],
    ) -> Result<Vec<IValue>, anyhow::Error>;
}

#[async_trait]
pub trait LocalImportAsync<Store: self::Store>: Send + LocalImport<Store> {}

pub use it_memory_traits::Store;

pub trait Instance<E, LI, M, MV, S>: Send + Sync
where
    E: Export,
    LI: LocalImport<S>,
    M: Memory<MV, S>,
    MV: MemoryView<S>,
    S: Store,
{
    fn export(&self, export_name: &str) -> Option<&E>;
    fn local_or_import<I: TypedIndex + LocalImportIndex>(&self, index: I) -> Option<&LI>;
    fn memory(&self, index: usize) -> Option<&M>;
    fn memory_view(&self, index: usize) -> Option<MV>;
    fn wit_record_by_id(&self, index: u64) -> Option<&Arc<IRecordType>>;
}

#[async_trait]
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

    /*fn call(&self, _arguments: &[IValue]) -> Result<Vec<IValue>, ()> {
        Err(())
    }*/

    async fn call_async(&self, _arguments: &[IValue]) -> Result<Vec<IValue>, anyhow::Error> {
        Err(anyhow::anyhow!("some error"))
    }
}

#[async_trait]
impl<Store: self::Store> LocalImport<Store> for () {
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
/*
    fn call(
        &self,
        _store: &mut <Store as self::Store>::ActualStore<'_>,
        _arguments: &[IValue],
    ) -> Result<Vec<IValue>, ()> {
        Err(())
    }*/

    async fn call_async(
        &self,
        _store: &mut <Store as it_memory_traits::Store>::ActualStore<'_>,
        _arguments: &[IValue],
    ) -> Result<Vec<IValue>, anyhow::Error> {
        Err(anyhow::anyhow!("some error"))
    }
}

pub(crate) struct EmptyMemoryView;

impl<S: Store> MemoryWritable<S> for EmptyMemoryView {
    fn write_byte(&self, _store: &mut <S as Store>::ActualStore<'_>, _offset: u32, _value: u8) {}

    fn write_bytes(&self, _store: &mut <S as Store>::ActualStore<'_>, _offset: u32, _bytes: &[u8]) {
    }
}

impl<S: Store> MemoryReadable<S> for EmptyMemoryView {
    fn read_byte(&self, _store: &mut <S as Store>::ActualStore<'_>, _offset: u32) -> u8 {
        0
    }

    fn read_array<const COUNT: usize>(
        &self,
        _store: &mut <S as Store>::ActualStore<'_>,
        _offset: u32,
    ) -> [u8; COUNT] {
        [0; COUNT]
    }

    fn read_vec(
        &self,
        _store: &mut <S as Store>::ActualStore<'_>,
        _offset: u32,
        _size: u32,
    ) -> Vec<u8> {
        Vec::default()
    }
}

impl<S: Store> MemoryView<S> for EmptyMemoryView {
    fn check_bounds(
        &self,
        _store: &mut <S as Store>::ActualStore<'_>,
        offset: u32,
        size: u32,
    ) -> Result<(), MemoryAccessError> {
        Err(MemoryAccessError::OutOfBounds {
            size,
            offset,
            memory_size: 0,
        })
    }
}

impl<S: Store> Memory<EmptyMemoryView, S> for () {
    fn view(&self) -> EmptyMemoryView {
        EmptyMemoryView
    }
}

impl<E, LI, M, MV, S> Instance<E, LI, M, MV, S> for ()
where
    E: Export,
    LI: LocalImportAsync<S>,
    M: Memory<MV, S>,
    MV: MemoryView<S>,
    S: Store,
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

    fn wit_record_by_id(&self, _index: u64) -> Option<&Arc<IRecordType>> {
        None
    }
}
