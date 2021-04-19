use std::cell::Cell;

pub(super) struct MemoryWriter<'m> {
    memory_view: &'m [Cell<u8>],
    offset: Cell<usize>,
    written_values: Cell<usize>,
}

impl<'m> MemoryWriter<'m> {
    pub(crate) fn new(memory_view: &'m [Cell<u8>], offset: usize) -> Self {
        let offset = Cell::new(offset);
        let written_values = Cell::new(0);

        Self {
            memory_view,
            offset,
            written_values,
        }
    }

    pub(crate) fn write_u8(&self, value: u8) {
        let offset = self.offset.get();
        self.memory_view[offset].set(value);
        self.offset.set(offset + 1);
        self.update_counter();
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
        self.update_counter();
    }

    pub(crate) fn write_array<const N: usize>(&self, values: [u8; N]) {
        let offset = self.offset.get();

        for id in 0..N {
            // don't check for memory overflow here for optimization purposes
            // assuming that caller site work well
            self.memory_view[offset + id].set(values[id]);
        }

        self.offset.set(offset + values.len());
        self.update_counter();
    }

    fn update_counter(&self) {
        let written_values_count = self.written_values.get();
        self.written_values.set(written_values_count + 1);
    }

    pub(crate) fn written_values(&self) -> usize {
        self.written_values.get()
    }
}
