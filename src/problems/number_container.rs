use std::collections::{BTreeSet, HashMap};
struct NumberContainers {
    num_to_idx: HashMap<i32, BTreeSet<i32>>,
    container: HashMap<i32, i32>,
}

impl NumberContainers {
    fn new() -> Self {
        Self {
            num_to_idx: HashMap::new(),
            container: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        if let Some(existing_number) = self.container.get(&index) {
            let entry = self
                .num_to_idx
                .entry(*existing_number)
                .or_insert(BTreeSet::new());
            entry.remove(&index);
        }
        self.container.insert(index, number);
        let entry = self.num_to_idx.entry(number).or_insert(BTreeSet::new());
        entry.insert(index);
    }

    fn find(&self, number: i32) -> i32 {
        *self
            .num_to_idx
            .get(&number)
            .unwrap_or(&BTreeSet::new())
            .first()
            .unwrap_or(&-1)
    }
}
