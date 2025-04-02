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
    pub fn roll(&self, loot_percent: f32) -> Vec<CompactString> { // returns item archetypes
        let mut lpl = loot_percent;
        let mut items = vec![];

        let mut rng = rand::thread_rng();
        let mut total_weight = 0;
        for entry in &self.entries {
            total_weight += entry.weight;
        }
        while lpl >= 100.0 {
            let num = rng.gen_range(0..total_weight);
            let mut current_weight = 0;
            for entry in &self.entries {
                if let Some(item) = &entry.item {
                    current_weight += entry.weight;
                    if num < current_weight {
                        items.push(item.clone());
                        break;
                    }
                }
            }
            lpl -= 100.0;
        }
        if rng.gen_range(0.0..100.0) < lpl {
            let num = rng.gen_range(0..total_weight);
            let mut current_weight = 0;
            for entry in &self.entries {
                if let Some(item) = &entry.item {
                    current_weight += entry.weight;
                    if num < current_weight {
                        items.push(item.clone());
                        break;
                    }
                }
            }
        }


        items
    }
}



#[derive(Clone, Debug)]
pub struct LootTableEntry {
    pub item: Option<CompactString>,
    pub weight: usize,
}
