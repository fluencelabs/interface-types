use std::cell::Cell;

pub(super) struct MemoryWriter<'m> {
    memory_view: &'m [Cell<u8>],
    offset: Cell<usize>,
}

impl<'m> MemoryWriter<'m> {
    pub(crate) fn new(memory_view: &'m [Cell<u8>], offset: usize) -> Self {
        let offset = Cell::new(offset);

        Self {
            memory_view,
            offset,
        }
    }

    pub(crate) fn write_u8(&self, value: u8) {
        let offset = self.offset.get();
        self.memory_view[offset].set(value);
        self.offset.set(offset + 1);
    }

    #[allow(dead_code)]
    pub(crate) fn write_slice(&self, values: &[u8]) {
        let offset = self.offset.get();

        for (id, value) in values.iter().enumerate() {
            // don't check for memory overflow here for optimization purposes
            // assuming that caller site work well
            self.memory_view[offset + id].set(*value);
        }

        self.offset.set(offset + values.len());
    }

    pub(crate) fn write_array<const N: usize>(&self, values: [u8; N]) {
        let offset = self.offset.get();

        for id in 0..N {
            // don't check for memory overflow here for optimization purposes
            // assuming that caller site work well
            self.memory_view[offset + id].set(values[id]);
        }

        self.offset.set(offset + values.len());
    }
}
