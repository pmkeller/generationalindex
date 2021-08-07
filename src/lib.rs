pub mod generationalindex {
    #[derive(Copy, Clone, PartialEq)]
    pub struct GenerationalIndex {
        index: usize,
        generation: u64,
    }

    pub struct AllocatorEntry {
        active: bool,
        generation: u64,
    }

    //where we get new GenerationIDs from
    pub struct GenerationalIndexAllocator {
        entries: Vec<AllocatorEntry>,
        available: Vec<usize>,
    }

    impl GenerationalIndex {
        pub fn get_index(&self) -> &usize {
            &self.index
        }

        pub fn get_generation(&self) -> &u64 {
            &self.generation
        }
    }

    impl GenerationalIndexAllocator {
        pub fn new() -> Self {
            GenerationalIndexAllocator {
                entries: Vec::new(),
                available: Vec::new(),
            }
        }

        pub fn allocate(&mut self) -> GenerationalIndex {
            if let Some(index) = self.available.pop() {
                let id_entry = &mut self.entries[index];

                assert!(!id_entry.active);

                id_entry.active = true;
                id_entry.generation += 1;

                return GenerationalIndex {
                    index: index,
                    generation: id_entry.generation,
                };
            }
            self.entries.push(AllocatorEntry {
                active: true,
                generation: 0,
            });

            GenerationalIndex {
                index: self.entries.len() - 1,
                generation: 0,
            }
        }

        pub fn deallocate(&mut self, gen_index: GenerationalIndex) {
            if let Some(alloc_entry) = self.entries.get_mut(gen_index.index) {
                if alloc_entry.active && alloc_entry.generation == gen_index.generation {
                    alloc_entry.active = false;
                    self.available.push(gen_index.index);
                }
            }
        }
    }
}


