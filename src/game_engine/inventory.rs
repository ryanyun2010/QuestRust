use crate::stat::StatList;
use crate::game_engine::player_abilities::PlayerAbility;
use compact_str::CompactString;
use rustc_hash::FxHashMap;

use crate::{error::PError, error_prolif_allow, game_engine::item::Item, perror, ptry, punwrap, rendering_engine::abstractions::{TextSprite, UIEFull}};

use super::{game::MousePosition, item::ItemType, player_abilities::PlayerAbilityDescriptor, ui::UIESprite};

#[derive(Debug, Clone)]
pub struct ItemOnMouse{
    pub item_id: usize,
    pub slot_belonging: usize
}
#[derive(Debug)]
pub struct Inventory {
    items: FxHashMap<usize, Item>,
    hotbar: Vec<usize>, // slot id of hotbar slots
    cur_hotbar_slot: usize,
    pub slots: Vec<Slot>,
    item_id: usize,
    pub show_inventory: bool,
    item_on_mouse: Option<ItemOnMouse>,
    mouse_position: MousePosition,
    pub items_waiting_to_be_dropped: Vec<usize>,
    chest_slot: Option<usize>,
    helm_slot: Option<usize>,
    boot_slot: Option<usize>,
    pub player_abilities: Vec<PlayerAbility>, // id in vec = ability id
    pub ability_slots: Vec<AbilitySlot>,
    pub hotkey_to_slot: FxHashMap<CompactString, usize>,
    pub ability_slot_clicked: Option<usize>
}


#[derive(Debug, Clone)] 
pub struct AbilitySlot {
    pub id: usize,
    pub key: CompactString,
    pub cur_ability: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Slot {
    pub item: Option<usize>,
    main_image: Option<UIESprite>,
    item_image: Option<UIESprite>,
    x: usize,
    y: usize,
    accepted_types: Vec<ItemType>
}

        

impl Slot {
    pub fn new(x: usize, y: usize, accepted_types: Vec<ItemType>) -> Self {
        Self {
            item: None,
            accepted_types,
            main_image: Some(UIESprite{
                x: x as f32,
                y: y as f32,
                z: 5.0,
                width: 48.0,
                height: 48.0,
                sprite: CompactString::from("hslot"),            
            }),
            item_image: None,
            x,
            y
        }
    }
    pub fn get_ui(&self) -> Vec<UIESprite>{
        let mut sprites = Vec::new();
        sprites.extend(self.main_image.clone());
        sprites.extend(self.item_image.clone());
        sprites
    }
    pub fn alter_position(&mut self, new_x: usize, new_y: usize) {
        self.x = new_x;
        self.y = new_y;
        if let Some(ii) = self.item_image.as_mut(){
            ii.x = self.x as f32 + 8.0;
            ii.y = self.y as f32 + 8.0;
        }
        if let Some(mi) = self.main_image.as_mut(){ 
            mi.x = self.x as f32;
            mi.y = self.y as f32;
        }
    }
    pub fn set_item(&mut self, item: usize, items: &FxHashMap<usize, Item>) -> Result<(), PError>{
        let i = punwrap!(items.get(&item), NotFound, "no item with id {}", item);
        let mut ok_type = false;
        for a_type in self.accepted_types.iter() {
            if &i.item_type == a_type { ok_type = true};
        }
        if !ok_type {
            return Err(perror!(WrongItemType, "this slot cannot take items of type {:?}", i.item_type));
        }

        self.item = Some(item);
        self.item_image = Some(
            UIESprite {
                x: self.x as f32 + 8.0,
                y: self.y as f32 + 8.0,
                z: 5.2,
                width: 32.0,
                height: 32.0,
                sprite: i.sprite.clone()
            }
        );
        Ok(())
    }
    pub fn remove_item(&mut self) {
        self.item = None;
        self.item_image = None;
    }
}

impl Default for Inventory{
    fn default() -> Self {
        Self {
            hotbar: Vec::new(),
            cur_hotbar_slot: 0,
            helm_slot: None,
            chest_slot: None,
            boot_slot: None,
            items: FxHashMap::default(),
            item_id: 0,
            slots: Vec::new(),
            show_inventory: false,
            item_on_mouse: None,
            mouse_position: MousePosition::default(),
            items_waiting_to_be_dropped: Vec::new(),
            player_abilities: Vec::new(),
            ability_slots: Vec::new(),
            hotkey_to_slot: FxHashMap::default(),
            ability_slot_clicked: None
            
        }
    }
}

impl Inventory {
    pub fn add_item(&mut self, item: Item) -> usize {
        self.items.insert(
            self.item_id, item
        );
        self.item_id += 1;
        self.item_id - 1
    }
    pub fn set_hotbar_slot(&mut self, slot: usize) {
        self.cur_hotbar_slot = slot;
    }
    pub fn set_ability_on_key(&mut self, key: CompactString, ability: Option<usize>) -> Result<(), PError>{
        if let Some(ability_slot) = self.hotkey_to_slot.get(&key) {
            let slot = 
                punwrap!(self.ability_slots.get_mut(*ability_slot), Invalid,  "attempted to set key {} to ability with id {:?} but there is no ability with id {:?}", key, ability, ability);
            slot.cur_ability = ability;
        } else {
            return Err(perror!(Invalid, "attempted to set ability slot with key {} to an ability, however there is no ability slot with key {} held in the hotkey_to_slot map, perhaps you need to add an ability slot with key {} first", key, key, key));
        }
        Ok(())
    }
    pub fn add_ability_slot_for_key(&mut self, key: CompactString) -> Result<(), PError>{
        if self.hotkey_to_slot.contains_key(&key) {
            return Err(perror!(Invalid, "attempted to add ability slot for key {} but one already exists", key));
        } else{
            self.hotkey_to_slot.insert(key.clone(), self.ability_slots.len());
            self.ability_slots.push(AbilitySlot {
                id: self.ability_slots.len(),
                key,
                cur_ability: None
            });
        }
        Ok(())
    }
    pub fn init_ui(&mut self) {
        self.add_hotbar_slot(Slot::new(20, 652, ItemType::all()));
        self.add_hotbar_slot(Slot::new(78, 652, ItemType::all()));
        self.add_hotbar_slot(Slot::new(136, 652, ItemType::all()));
        self.add_hotbar_slot(Slot::new(194, 652, ItemType::all()));
        self.add_hotbar_slot(Slot::new(252, 652, ItemType::all()));
        self.add_slot(Slot::new(520, 200, ItemType::all()));
        self.add_slot(Slot::new(578, 200, ItemType::all()));
        self.add_slot(Slot::new(636, 200, ItemType::all()));
        self.add_slot(Slot::new(694, 200, ItemType::all()));
        self.add_slot(Slot::new(752, 200, ItemType::all()));
        self.add_slot(Slot::new(520, 258, ItemType::all()));
        self.add_slot(Slot::new(578, 258, ItemType::all()));
        self.add_slot(Slot::new(636, 258, ItemType::all()));
        self.add_slot(Slot::new(694, 258, ItemType::all()));
        self.add_slot(Slot::new(752, 258, ItemType::all()));
        self.add_slot(Slot::new(520, 316, ItemType::all()));
        self.add_slot(Slot::new(578, 316, ItemType::all()));
        self.add_slot(Slot::new(636, 316, ItemType::all()));
        self.add_slot(Slot::new(694, 316, ItemType::all()));
        self.add_slot(Slot::new(752, 316, ItemType::all()));

        self.helm_slot = Some(self.add_slot(Slot::new(369,242, vec![ItemType::HelmetArmor])));
        self.chest_slot = Some(self.add_slot(Slot::new(369,300, vec![ItemType::ChestplateArmor])));
        self.boot_slot = Some(self.add_slot(Slot::new(369,358, vec![ItemType::BootsArmor])));
    }
    pub fn add_hotbar_slot(&mut self, slot: Slot) {
        self.hotbar.push(self.slots.len());
        self.slots.push(slot);
    }
    pub fn add_slot(&mut self, slot: Slot) -> usize{
        self.slots.push(slot);
        self.slots.len() - 1
    }
    pub fn get_hotbar_slot(&self, slot: usize) -> Option<&Slot>{
        self.hotbar.get(slot).and_then(
            |x| self.slots.get(*x)
        )
    }
    pub fn get_hotbar_slot_mut(&mut self, slot: usize) -> Option<&mut Slot>{
        self.hotbar.get(slot).and_then(
            |x| self.slots.get_mut(*x)
        )
    }
    pub fn get_combined_stats(&self) -> Result<StatList, PError> {
        let mut stats = StatList::base();
        if let Some(item) = self.get_cur_held_item() {
            stats.to_sum_with(&item.stats);
        }
        if let Some(h) = self.helm_slot {
            let slot = punwrap!(self.get_slot(&h), Invalid, "Helm slot is marked as a non-existent slot {}", h);
            if let Some(i) = slot.item {
                let item = punwrap!(self.get_item(&i), Invalid, "Helm slot refers to non-existent item with id {}", i);
                stats.to_sum_with(&item.stats);
            }
        }

        Ok(stats)
    }

    pub fn show_inventory(&mut self){
        self.show_inventory = true;
        for i in 0..self.hotbar.len(){
            let slot = self.get_hotbar_slot_mut(i).unwrap();
            slot.alter_position(520 + i * 58, 374);
        }
    }
    pub fn hide_inventory(&mut self) -> Result<(), PError>{
        self.show_inventory = false;
        if self.item_on_mouse.is_some(){
            let iom = self.item_on_mouse.as_ref().unwrap();
            ptry!(self.set_slot_item(iom.slot_belonging, iom.item_id));
            self.item_on_mouse = None;
        }
        for i in 0..self.hotbar.len(){
            let slot = self.get_hotbar_slot_mut(i).unwrap();
            slot.alter_position(20 + i * 58, 652);
        }
        Ok(())
    }
    pub fn remove_item(&mut self, id: usize) -> Result<(), PError> { 
        // DANGEROUS, THIS REMOVES ITEM WITHOUT REMOVING REFS TO IT IN SLOTS, MAKE SURE ITEM ISN'T IN A SLOT BEFORE DOING THIS
        if self.items.remove(&id).is_some() {
            Ok(())
        } else{
            Err(perror!(NotFound, "attempted to remove item with id {} that doesn't exist", id))
        }
    }
    pub fn get_stat_string(&self, list: &StatList) -> String {
        let mut t = String::new();
        for stat in list.into_iter() {
            if stat.0 == "attack_cooldown" {
                let num = (stat.1.map(|x| x.flat).unwrap_or(0.0)/60.0 * 100.0).round() /100.0;
                if num > 0.0 {
                    t.push_str(
                        format!("{}: +{}s \n", stat.0, num).as_str()
                    );
                } else if num < 0.0 {
                    t.push_str(
                        format!("{}: {}s \n", stat.0, num).as_str()
                    );
                }
                if let Some(num2) = stat.1.map(|x| (x.percent * 100.0).round() /100.0){
                    if num2 > 0.0 {
                        t.push_str(
                            format!("{}: +{}% \n", stat.0, num2).as_str()
                        );
                    }else if num2 < 0.0 {
                        t.push_str(
                            format!("{}: {}% \n", stat.0, num2).as_str()
                        );
                    }

                }
                continue;
            }
            if stat.0 == "fire_duration" || stat.0 == "poison_duration" || stat.0 == "lifetime" {
                let num = (stat.1.map(|x| x.flat).unwrap_or(0.0)/60.0 * 100.0).round() /100.0;
                if num > 0.0 {
                    t.push_str(
                        format!("{}: +{}s \n", stat.0, num).as_str()
                    );
                }else if num < 0.0{
                    t.push_str(
                        format!("{}: {}s \n", stat.0, num).as_str()
                    );
                }

                if let Some(num2) = stat.1.map(|x| (x.percent * 100.0).round() /100.0){
                    if num2 > 0.0 {
                        t.push_str(
                            format!("{}: +{}% \n", stat.0, num2).as_str()
                        );
                    }else if num2 < 0.0 {
                        t.push_str(
                            format!("{}: {}% \n", stat.0, num2).as_str()
                        );
                    }
                }
                continue;
            }
            
            if let Some(s) = stat.1{
                let num = (stat.1.map(|x| x.flat).unwrap_or(0.0) * 100.0).round() /100.0;
                if s.flat > 0.0 {
                    t.push_str(
                        format!("{}: +{} \n", stat.0, (s.flat * 100.0).round()/100.0).as_str()
                    );
                }else if s.flat < 0.1 {
                    t.push_str(
                        format!("{}: {} \n", stat.0, (s.flat * 100.0).round()/100.0).as_str()
                    );
                }
                if s.percent > 0.0 {
                    t.push_str(
                        format!("{}: +{}% \n", stat.0, (s.percent * 100.0).round()/100.0).as_str()
                    );
                }else if s.percent < 0.0 {
                    t.push_str(
                        format!("{}: {}% \n", stat.0, (s.percent * 100.0).round()/100.0).as_str()
                    );
                }
            }
        }
        t
    }
    pub fn get_stats_combined_string(&self) -> Result<String, PError> {
        let list = ptry!(self.get_combined_stats());
        let mut t = String::new();
        for stat in list.into_iter() {
            if stat.0 == "attack_cooldown" || stat.0 == "fire_duration" || stat.0 == "poison_duration" || stat.0 == "lifetime" {
                let num = (stat.1.map(|x| x.get_value()).unwrap_or(0.0)/60.0 * 100.0).round() /100.0;
                t.push_str(
                    format!("{}: {}s \n", stat.0, num).as_str()
                );
                continue;
            }
            if stat.0 == "fire_tick_speed" {
                if let Some(s) = stat.1.map(|x| x.get_value()) {
                    let v = (s * 100.0).round()/100.0;
                    let ts = 60.0/super::stat::BASE_FIRE_TICK_DELAY * v;

                    t.push_str(
                        format!("{}: {} — {} t/s \n", stat.0, v, ts).as_str()
                    );
                }
                continue;
            }
            if stat.0 == "poison_tick_speed" {
                if let Some(s) = stat.1.map(|x| x.get_value()) {
                    let v = (s * 100.0).round()/100.0;
                    let ts = 60.0/super::stat::BASE_POISON_TICK_DELAY * v;

                    t.push_str(
                        format!("{}: {} — {} t/s \n", stat.0, v, ts).as_str()
                    );
                }
                continue;
            }
            if stat.1.is_some(){
                t.push_str(
                    format!("{}: {} \n", stat.0, (stat.1.unwrap().get_value() * 10.0).round() /10.0).as_str()
                );
            }
        }
        Ok(t)
    }
    pub fn set_hotbar_slot_item(&mut self, slot: usize, item_id: usize) -> Result<(), PError> {
        let s = punwrap!(self.hotbar.get(slot).and_then(
            |x| self.slots.get_mut(*x)
        ), NotFound, "There is no {}th hotbar slot", slot);
        ptry!(s.set_item(item_id, &self.items), "While setting hotbar slot {} to item {}", slot, item_id);
        Ok(())
    }
    pub fn get_slot(&self, slot: &usize) -> Option<&Slot> {
        self.slots.get(*slot)
    }
    pub fn add_to_slot(&mut self, item: Item) -> Result<(), PError> {
        let it = self.add_item(item);
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if i == self.helm_slot.unwrap_or(usize::MAX) { continue; }
            if i == self.chest_slot.unwrap_or(usize::MAX) { continue; }
            if i == self.boot_slot.unwrap_or(usize::MAX) { continue; }
            if slot.item.is_none() {
                ptry!(slot.set_item(it, &self.items));
                return Ok(());
            }
        }
        Err(perror!(NoSpace, "No space for item"))
    }
    pub fn set_slot_item(&mut self, slot: usize, item_id: usize) -> Result<(), PError> {
        let s = punwrap!(self.slots.get_mut(slot), NotFound, "There is no {}th slot", slot);
        ptry!(s.set_item(item_id, &self.items));
        Ok(())
    }

    pub fn get_ability(&self, id: usize) -> Option<&PlayerAbility> {
        self.player_abilities.get(id)
    }
    pub fn get_ability_mut(&mut self, id: usize) -> Option<&mut PlayerAbility> {
        self.player_abilities.get_mut(id)
    }
    pub fn get_abilities_on_hotkey(&self, key: CompactString) -> Option<usize> {
        self.hotkey_to_slot.get(&key)
            .and_then(|x| self.ability_slots.get(*x))
            .and_then(|x| x.cur_ability)
    }
    pub fn render_ui(&mut self, ability_descriptors: &[PlayerAbilityDescriptor]) -> Result<UIEFull, PError> {
        let mut ui = Vec::new();
        let mut text = Vec::new(); 
        if self.show_inventory {
            ui.push(UIESprite {
                z: 0.0,
                x: 326.0,
                y: 186.5,
                width: 500.0,
                height: 347.0,
                sprite: CompactString::from("inventory")
            });
            ui.push(UIESprite {
                z: -1.0,
                x: 0.0,
                y: 0.0,
                width: 1152.0,
                height: 720.0,
                sprite: CompactString::from("inventory_background")
            });
            for slot in self.slots.iter() {
                ui.extend(slot.get_ui());
                if (slot.x as f32) < self.mouse_position.x_screen && (slot.x as f32 + 48.0) > self.mouse_position.x_screen && (slot.y as f32) < self.mouse_position.y_screen && (slot.y as f32 + 48.0) > self.mouse_position.y_screen{
                    if let Some(i) = slot.item{
                        let item = punwrap!(self.get_item(&i), Invalid, "slot: {:?} is refering a non-existent item with id {}", slot, i);
                        let mut t = format!(
                            "{}\n----------------------------------------\n\n{}\n\n", item.name, item.lore
                        );

                        let stats = &item.stats;
                        t.push_str(&self.get_stat_string(stats));
                        ui.push(
                            UIESprite {
                                x: self.mouse_position.x_screen + 20.0,
                                y: self.mouse_position.y_screen - 165.0, 
                                z: 5.6,
                                width: 220.0,
                                height: 320.0,
                                sprite: CompactString::from("level_editor_menu_background")
                            }
                        );
                        text.push(
                            TextSprite {
                                text: t,
                                font_size: 20.0,
                                x: self.mouse_position.x_screen + 30.0,
                                y: self.mouse_position.y_screen - 150.0,
                                w: 200.0,
                                h: 300.0,
                                color: [1.0,1.0,1.0,1.0],
                                align: wgpu_text::glyph_brush::HorizontalAlign::Left
                            }
                        )
                    }
                }
            }

            ui.push(
                UIESprite {
                    x: (520 + self.cur_hotbar_slot * 58) as f32,
                    y: 374.0,
                    z: 5.1,
                    width: 48.0,
                    height: 48.0,
                    sprite: CompactString::from("slot_highlight")
                }
            );
            

            text.push(
                TextSprite {
                    text: ptry!(self.get_stats_combined_string()),
                    font_size: 25.0,
                    x: 80.0,
                    y: 200.0,
                    w: 500.0,
                    h: 600.0,
                    color: [1.0, 1.0, 1.0, 1.0],
                    align: wgpu_text::glyph_brush::HorizontalAlign::Left
                });


            if let Some(item_on_mouse) = self.item_on_mouse.as_ref(){
                let item = self.get_item(&item_on_mouse.item_id);
                if let Some(item) = item{
                    ui.push(UIESprite {
                        x: self.mouse_position.x_screen - 12.0,
                        y: self.mouse_position.y_screen - 12.0,
                        z: 999.0,
                        width: 24.0,
                        height: 24.0,
                        sprite: item.sprite.clone()
                    })
                }
            }
            for slot in self.ability_slots.iter() {
                ui.push(UIESprite {
                    x: 350.0 + 58.0 * slot.id as f32,
                    y: 590.0,
                    z: 5.2,
                    width: 48.0,
                    height: 48.0,
                    sprite: CompactString::from("hslot")
                });
                text.push(TextSprite {
                    text: slot.key.clone().to_string(),
                    font_size: 30.0,
                    x: 350.0 + 58.0 * slot.id as f32 + 33.0,
                    y: 590.0 + 25.0,
                    w: 15.0,
                    h: 15.0,
                    color: [1.0,1.0,1.0,1.0],
                    align: wgpu_text::glyph_brush::HorizontalAlign::Center,
                });
                let ability = slot.cur_ability.and_then(|x| self.player_abilities.get(x)).and_then(
                        |x| ability_descriptors.get(x.descriptor_id)
                    );
                if let Some(ability) = ability {
                text.push(TextSprite {
                    text: ability.name.clone().to_string(),
                    font_size: 20.0,
                    x: 350.0 + 58.0 * slot.id as f32 + 24.0,
                    y: 590.0 + 12.0,
                    w: 30.0,
                    h: 30.0,
                    color: [1.0,1.0,1.0,1.0],
                    align: wgpu_text::glyph_brush::HorizontalAlign::Center,
                });
                }

            }
            if let Some(aslot_h) = self.ability_slot_clicked {
                let slot = &self.ability_slots[aslot_h];
                ui.push(UIESprite {
                    x: 330.0 + 58.0 * slot.id as f32,
                    y: 450.0,
                    z: 5.2,
                    width: 70.0,
                    height: 120.0,
                    sprite: CompactString::from("level_editor_menu_background")
                });
                for (i, ability) in self.player_abilities.iter().enumerate() {
                    let ability_descriptor = punwrap!(ability_descriptors.get(ability.descriptor_id), Invalid, "ability with id {} refers to non-existent ability descriptor with id {}", i, ability.descriptor_id);
                    text.push( TextSprite {
                        x: 335.0 + 58.0 * slot.id as f32,
                        y: 460.0 + i as f32 * 15.0,
                        text: format!("{}. {}", i, ability_descriptor.name.clone()),
                        font_size: 15.0,
                        w: 60.0,
                        h: 15.0,
                        color: [1.0, 1.0, 1.0, 1.0],
                        align: wgpu_text::glyph_brush::HorizontalAlign::Left
                    });
                }
            }


        }
        else {
            for slot in self.hotbar.iter() {
                ui.extend(punwrap!(self.get_slot(slot), Invalid, "hotbar ids includes slot id {}, but there is no slot with id {}", slot, slot).get_ui());
            }
            ui.push(
                UIESprite{
                    x: 20.0 + 58.0 * self.cur_hotbar_slot as f32,
                    y: 652.0,
                    z: 5.1,
                    width: 48.0,
                    height: 48.0,
                    sprite: CompactString::from("slot_highlight")
                }
            );
            for slot in self.ability_slots.iter() {
                ui.push(UIESprite {
                    x: 350.0 + 58.0 * slot.id as f32,
                    y: 590.0,
                    z: 5.2,
                    width: 48.0,
                    height: 48.0,
                    sprite: CompactString::from("hslot")
                });
                text.push(TextSprite {
                    text: slot.key.clone().to_string(),
                    font_size: 30.0,
                    x: 350.0 + 58.0 * slot.id as f32 + 33.0,
                    y: 590.0 + 25.0,
                    w: 15.0,
                    h: 15.0,
                    color: [1.0,1.0,1.0,1.0],
                    align: wgpu_text::glyph_brush::HorizontalAlign::Center,
                });
                let ability = slot.cur_ability.and_then(|x| self.player_abilities.get(x)).and_then(
                        |x| ability_descriptors.get(x.descriptor_id)
                    );
                if let Some(ability) = ability {
                text.push(TextSprite {
                    text: ability.name.clone().to_string(),
                    font_size: 20.0,
                    x: 350.0 + 58.0 * slot.id as f32 + 24.0,
                    y: 590.0 + 12.0,
                    w: 30.0,
                    h: 30.0,
                    color: [1.0,1.0,1.0,1.0],
                    align: wgpu_text::glyph_brush::HorizontalAlign::Center,
                });
                }
            }
        }
        Ok(UIEFull {
            sprites: ui,
            text,
        })
        
        
    }
    pub fn get_cur_held_item(&self) -> Option<&Item> {
        self.get_hotbar_slot(self.cur_hotbar_slot)
            .and_then(|slot| slot.item)
            .and_then(|item| self.get_item(&item))
    }
    pub fn get_cur_held_item_mut(&mut self) -> Option<&mut Item> {
        self.get_hotbar_slot(self.cur_hotbar_slot)
            .and_then(|slot| slot.item)
            .and_then(|item| self.get_item_mut(&item))
    }
    pub fn get_item(&self, id: &usize) -> Option<&Item> {
        self.items.get(id)
    }
    pub fn get_item_mut(&mut self, id: &usize) -> Option<&mut Item> {
        self.items.get_mut(id)
    }

    pub fn on_key_down(&mut self, key: &str) {
        if key.chars().all(char::is_numeric) {
            let num = key.parse::<usize>().unwrap();
            if num < 6 && num > 0 {
                self.set_hotbar_slot(num - 1);
            }
        }
    }
    pub fn on_mouse_click(&mut self, position: MousePosition, left: bool, right: bool) -> Result<(), PError> {
        if left {
            let mut slot_clicked = None;
            let mut i = 0;
            for slot in self.slots.iter_mut() {
                if slot.x < position.x_screen as usize && slot.x + 48 > position.x_screen as usize && slot.y < position.y_screen as usize && slot.y + 48 > position.y_screen as usize {
                    slot_clicked = Some(slot);
                    break;
                }
                i += 1;
            }
            if let Some(slot) = slot_clicked {
                if slot.item.is_some() {
                    if self.item_on_mouse.is_none() {
                        self.item_on_mouse = Some(
                            ItemOnMouse {
                                slot_belonging: i,
                                item_id: slot.item.unwrap()
                            }
                        );
                        slot.remove_item();
                    }else if self.item_on_mouse.is_some() {
                        let item_clone = slot.item;
                        let res = error_prolif_allow!(slot.set_item(self.item_on_mouse.as_ref().unwrap().item_id, &self.items), WrongItemType);
                        if res.is_ok() {
                            self.item_on_mouse = Some(
                                ItemOnMouse {
                                    slot_belonging: self.item_on_mouse.as_ref().unwrap().slot_belonging,
                                    item_id: item_clone.unwrap()
                                }
                            ); 
                        }
                    }
                }else if self.item_on_mouse.is_some(){
                    let res = error_prolif_allow!(slot.set_item(self.item_on_mouse.as_ref().unwrap().item_id, &self.items), WrongItemType);
                    if res.is_ok() {
                        self.item_on_mouse = None;
                    }
                }
            }

            let mut ability_slot_clicked = None;
            for slot in self.ability_slots.iter() {
                if position.x_screen > 350.0 + 58.0 * slot.id as f32 && position.x_screen < 398.0 + 58.0 * slot.id as f32 && position.y_screen > 590.0 && position.y_screen < 638.0 {
                    ability_slot_clicked = Some(slot.id);
                }
            }

            if let Some(cur) = self.ability_slot_clicked {
                if ability_slot_clicked.is_none() && !(position.x_screen > 350.0 && position.x_screen < 398.0 + 58.0 * cur as f32 && position.y_screen > 590.0 && position.y_screen < 638.0) {
                    self.ability_slot_clicked = None;
                }else if ability_slot_clicked.is_some() {
                    self.ability_slot_clicked = ability_slot_clicked;
                }

                for i in 0..self.player_abilities.len() {
                    if position.x_screen > 330.0 + 58.0 * cur as f32 && position.x_screen < 400.0 + 58.0 * cur as f32 && position.y_screen > 460.0 + i as f32 * 15.0 && position.y_screen < 460.0 + (i+1) as f32 * 15.0 {
                        let slot = punwrap!(self.ability_slots.get_mut(cur), Invalid, "cur ability slot clicked refers to non existent ability slot");
                        slot.cur_ability = Some(i);
                    } 
                }
            }else {
                self.ability_slot_clicked = ability_slot_clicked;
            }
            println!("{:?}", self.ability_slot_clicked);
        }
        Ok(())
    }
    pub fn process_input(&mut self, keys: &FxHashMap<CompactString, bool>){
        if *keys.get("q").unwrap_or(&false) {
            let mut items_dropped = Vec::new();
            for slot in self.slots.iter_mut() {
                if slot.x < self.mouse_position.x_screen as usize && slot.x + 48 > self.mouse_position.x_screen as usize && slot.y < self.mouse_position.y_screen as usize && slot.y + 48 > self.mouse_position.y_screen as usize {
                    if let Some(i) = slot.item {
                        items_dropped.push(i);
                    }
                    slot.remove_item();
                    break;
                }
            } 
            self.items_waiting_to_be_dropped.extend(items_dropped);
        }
    }
    pub fn process_mouse_input(&mut self, position: MousePosition, left: bool, right: bool){
        self.mouse_position = position;
    }
    pub fn update_items_cd(&mut self) -> Result<(), PError> {
        for (_, item) in self.items.iter_mut() {
            if !(item.item_type == ItemType::RangedWeapon) {
                item.time_til_usable -= 1.0; 
            }
        }
        Ok(())
    }
}




// use std::{borrow::Borrow, cell::RefCell};

// use super::{item::Item, world::{World}};

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct ItemContainerPointer {
//     element_id: RefCell<Option<usize>>,
//     container_type: Option<Vec<ItemType>>,
// }
// impl ItemContainerPointer {
//     pub fn new(container_type: Option<Vec<ItemType>>) -> Self {
//         Self {
//             container_type,
//             element_id: RefCell::new(None)
//         }
//     }
//     pub fn is_init(&self) -> bool {
//         self.element_id.borrow().is_some()
//     }
// }
// //We just store a single item in each container.
// //Containers, not to be confused with chests (if we implement them), or items (which are single, dynamic, inventory slots).
// #[derive(Clone, Debug)]
// pub struct ItemContainer {
//     // element_id: usize,
//     container_type: Option<Vec<ItemType>>,
//     contained_item: Option<Item>,
// }
// impl ItemContainer {
//     pub fn new(container_type: Option<Vec<ItemType>>) -> Self {
//         Self {
//             container_type,
//             contained_item: None
//         }
//     }
//     pub fn tansfer_item(mut self, mut other: ItemContainer) -> ItemContainer {
//         other.contained_item = self.contained_item;
//         self.contained_item = None;
//         println!("Self: {:?}\nOther: {:?}", self.contained_item, other.contained_item);
//         other
//     }
//     pub fn drop(mut self) {
//         //Spawn entity shit, will be used for resizing inventory and closing GUIs.
//     }
// }
// #[derive(Clone, Debug)]
// pub enum Hotbar {
//     Normal([ItemContainerPointer; 6]),
//     Extended([[ItemContainerPointer; 3]; 3]),
// }
// impl World {
//     pub fn item_container_init(&self, mut container: &ItemContainerPointer) -> usize {
//         self.item_containers.borrow_mut().insert(self.element_id, ItemContainer::new(container.container_type.clone()));
//         *container.element_id.borrow_mut() = Some(self.element_id);
//         self.element_id
//     }
//     pub fn player_init(&mut self) {
//         for i in 0..6 {
//             for j in 0..6 {
//                 self.element_id=self.item_container_init(&self.player.borrow_mut().inventory[i][j])+1;
//             }
//         }
//     }
// }
