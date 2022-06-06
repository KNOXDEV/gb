pub(super) struct TableBuilder<B, const SIZE: usize> {
    table: [B; SIZE],
}

impl<B: Copy, const SIZE: usize> TableBuilder<B, SIZE> {
    pub(super) const fn new(initial: B) -> TableBuilder<B, SIZE> {
        TableBuilder {
            table: [initial; SIZE],
        }
    }

    // i would prefer to use iterators for this but they aren't supported in a const environment yet
    pub(super) const fn bitmask_map(
        mut self,
        bitmask: usize,
        match_value: usize,
        map_item: B,
    ) -> Self {
        let mut count = 0;
        while count < SIZE {
            if count & bitmask == match_value {
                self.table[count] = map_item;
            }
            count = count + 1;
        }
        self
    }

    // how this works is almost indescribable,
    // but its a shorthand for sequences of mappings with the same bitmask
    pub(super) const fn bitmask_map_sequence<const ITEMS: usize>(
        mut self,
        bitmask: usize,
        match_value: usize,
        stride: usize,
        map_item: [B; ITEMS],
    ) -> Self {
        let mut count = 0;
        while count < ITEMS {
            self = self.bitmask_map(bitmask, match_value + stride * ITEMS, map_item[count]);
            count = count + 1;
        }
        self
    }

    pub(super) const fn build(self) -> [B; SIZE] {
        self.table
    }
}
