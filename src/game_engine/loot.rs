use compact_str::CompactString;
use rand::Rng;
#[derive(Clone, Debug)]
pub struct LootTable {
    entries: Vec<LootTableEntry>,
}
impl LootTable{
    pub fn new(entries: Vec<LootTableEntry>) -> Self{
        Self{entries}
    }
    pub fn roll(&self) -> Vec<CompactString> { // returns item archetypes
        let mut total_weight = 0;
        for entry in &self.entries {
            total_weight += entry.weight;
        }
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(0..total_weight);
        let mut current_weight = 0;
        for entry in &self.entries {
            if let Some(item) = &entry.item {
                current_weight += entry.weight;
                if num < current_weight {
                    return vec![item.clone()];
                }
            }
        }
        vec![]
    }
}



#[derive(Clone, Debug)]
pub struct LootTableEntry {
    pub item: Option<CompactString>,
    pub weight: usize,
}
