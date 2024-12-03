// i think realistically
// it should be LootTable contains a vector of loot table entrys
// each entry has an Item + a Rarity just as a number between 0 and 1
// Then the loot table goes through each entry and checks if you got it
// LootTableRarity seems overengineered





use crate::entities::Item;
#[derive(Clone, Debug)]
pub struct Loot {
    tables: Vec<LootTable>,
}
impl Loot{
    pub fn new(tables: Vec<LootTable>) -> Self {
        Self {
            tables: tables,
        }
    }
}

#[derive(Clone, Debug)]
pub struct LootTable {
    entries: Vec<LootTableEntry>,
    total_weight: usize,
    rarity: LootTableRarity
}

impl LootTable {
    pub fn new(&mut self, entries: Vec<LootTableEntry>, rarity: LootTableRarity) -> Self {
        let mut total_weight = 0;
        for entry in 0..self.entries.len()-1 {
            total_weight += self.entries[entry].weight;
            self.entries[entry].initialize_range(total_weight);
        }
        Self {
            entries: entries,
            total_weight: total_weight,
            rarity: rarity
        }
    }
    pub fn roll(&self) -> Option<Item> {
        let mut rand: f32 = rand::random();
        let roll: usize = (rand*(self.total_weight as f32)).floor() as usize;
        for entry in 0..self.entries.len()-1 {
            if roll <= self.entries[entry].weight_range {
                return self.entries[entry].item.clone()
            }
        }
        None
    }
}

#[derive(Clone, Debug)]
pub enum LootTableRarity {
    Common, //Over 20%
    Uncommon, //Over 5%
    Rare, //Over 1%
    Epic, //Over 0.1%
    Legendary, //Less than 0.1%
}

#[derive(Clone, Debug)]
pub struct LootTableEntry {
    item: Option<Item>,
    weight: usize,
    weight_range: usize
}

impl LootTableEntry {
    pub fn new(item: Option<Item>, weight: usize) -> Self {
        Self {
            item: item,
            weight: weight,
            weight_range: 0,
        }
    }
    pub fn initialize_range(&mut self, weight_range: usize) {
        self.weight_range = weight_range;
    }
}