use super::{parse_data_i32, parse_data_k, parse_data_uval, AtreeItemEffect, ClassAtreeEnums, EffectKey, EffectPartKey, EffectType, archer, assassin, mage, shaman, warrior};
use super::{WynnEnum, Class};

#[derive(Default, PartialEq)]
pub struct Spell{
    cost: i32,
    parts: Vec<SpellPart>
}
impl Spell{
    /// Given an Effect (provided by an Atree item) with the type `EffectType::AddSpellProp`, adds the data required. <br>
    /// Note this will panic! if the provided `Effect` does not have the type EffectType::AddSpellProp
    pub (super) fn add_spell_prop<T: ClassAtreeEnums>(&mut self, effect: AtreeItemEffect<T>){
        let mut target_part_idx: usize = 0;
        for (k,v,effect_idx) in effect.iter_data::<i32>(){
            match k{
                EffectKey::Type => assert_eq!(EffectType::AddSpellProp, EffectType::try_from(v).unwrap()),
                EffectKey::Name => (), // why does appspellprop for effigyhit have a name?
                EffectKey::Cost => self.cost+=v,
                EffectKey::TargetPart => {
                    target_part_idx=self.get_target_part_idx(v as u8);
                    if target_part_idx>=self.parts.len(){
                        self.parts.push(SpellPart{id: v as u8, mults: [0;6], hits: 1, final_mult: 100, power: 0})
                    }
                },
                EffectKey::UseAtkspd => todo!(),
                EffectKey::Round => todo!(), // is this important?
                EffectKey::Behavior => todo!(), // Idk what behavior is
                EffectKey::Power => self.parts[target_part_idx].power=v as i8,
                EffectKey::Multipliers => 
                for (i,m) in effect.iter_mults_from_idx(effect_idx, v as usize).enumerate(){
                    self.parts[target_part_idx].mults[i]+=m as i16
                },
                EffectKey::Hits => for (part,num_hits,_) in effect.iter_hits_from_idx(effect_idx, v as usize){
                    let part_id: u32 = part.into();
                    let target_idx = self.get_target_part_idx(part_id as u8);
                    self.parts[target_idx].hits=num_hits as u8
                },
                EffectKey::Parts|EffectKey::Scaling|EffectKey::Bonuses|
                EffectKey::Output|EffectKey::Inputs|EffectKey::Max|EffectKey::Slider|
                EffectKey::SliderName|EffectKey::SliderMax|EffectKey::SliderStep|
                EffectKey::Toggle => panic!("Spell properties should not have key {k}"),
                _ => ()
            }
        }
    }
    pub (super) fn make_spell<T: ClassAtreeEnums>(effect: AtreeItemEffect<T>) -> Self{
        let mut res = Self{cost: 0, parts: Vec::new()};
        for (k,v,effect_idx) in effect.iter_data::<i32>(){
            match k{
                EffectKey::Type => assert_eq!(EffectType::ReplaceSpell, EffectType::try_from(v).unwrap()),
                EffectKey::Name => (),
                EffectKey::Cost => res.cost=v,
                EffectKey::BaseSpell => (),
                EffectKey::TargetPart => todo!(), //error
                EffectKey::Toggle => todo!(), //error
                EffectKey::Slider => todo!(), //error
                EffectKey::SliderName => todo!(), //error
                EffectKey::SliderMax => todo!(), //error
                EffectKey::Max => (), // ?
                EffectKey::UseAtkspd => (), // idk how important this is yet, is important tho
                EffectKey::Behavior => todo!(), // error
                EffectKey::SliderStep => todo!(), //error
                EffectKey::Hide => todo!(),
                EffectKey::Round => todo!(), //is this important? error
                EffectKey::Power => (), // ?
                EffectKey::SliderDefault => todo!(),
                EffectKey::Parts => {
                    let mut add_part: SpellPart = SpellPart{id:0,mults:[0;6],hits:1,final_mult:100,power:0};
                    for (pk,pv,pidx) in effect.iter_parts_from_idx(effect_idx, v as usize){
                        match pk{
                            EffectPartKey::Name => {
                                if add_part.id!=0{
                                    res.parts.push(add_part);
                                    add_part=SpellPart{id:0,mults:[0;6],hits:1,final_mult:100,power:0}
                                }
                                add_part.id=pv as u8;
                            },
                            EffectPartKey::Type => (), // shouldnt matter
                            EffectPartKey::Power => add_part.power=pv as i8,
                            EffectPartKey::Multipliers => for (i,m) in effect.iter_mults_from_idx(pidx, pv as usize).enumerate(){
                                add_part.mults[i]=m as i16
                            },
                            EffectPartKey::Hits => {
                                for (part,num_hits,_) in effect.iter_hits_from_idx(pidx, pv as usize){
                                    let part_id: u32 = part.into();
                                    let target_idx = res.get_target_part_idx(part_id as u8);
                                    res.parts[target_idx].hits=num_hits as u8
                                }
                            },
                        }
                    }
                    if add_part.id!=0{
                        res.parts.push(add_part);
                    }
                },
                EffectKey::Multipliers => todo!(),
                EffectKey::Bonuses => todo!(),
                EffectKey::Output => todo!(),
                EffectKey::Scaling => (), // what does this mean? TODO ERRORS!!!
                EffectKey::Inputs => todo!(),
                EffectKey::Hits => todo!(),
            }
        }
        res
    }
    /// Makes a spell that represents the default melee values
    pub fn melee_default(class: Class) -> Self{
        match class{
            Class::Archer => Self{cost: 0, parts: vec![SpellPart{id:archer::SpellPart::SingleShot as u8,mults:[100,0,0,0,0,0],hits:1,final_mult:100,power:0}]},
            Class::Warrior => Self{cost: 0, parts: vec![SpellPart{id:warrior::SpellPart::Melee as u8,mults:[100,0,0,0,0,0],hits:1,final_mult:100,power:0}]},
            Class::Mage => Self{cost: 0, parts: vec![SpellPart{id:mage::SpellPart::Total as u8,mults:[100,0,0,0,0,0],hits:1,final_mult:100,power:0}]}, // is this right?
            Class::Assassin => Self{cost: 0, parts: vec![SpellPart{id:assassin::SpellPart::Melee as u8,mults:[100,0,0,0,0,0],hits:1,final_mult:100,power:0}]},
            Class::Shaman => Self{cost: 0, parts: vec![SpellPart{id:shaman::SpellPart::SingleBeam as u8,mults:[100,0,0,0,0,0],hits:1,final_mult:100,power:0}]},
        }
    }
    fn get_target_part_idx(&self, target_id: u8) -> usize{
        self.parts.iter().position(|s| s.id==target_id).unwrap_or(self.parts.len())
    }
    pub fn total_mult(&self) -> [f32; 6]{
        let mut temp = [0;6];
        for sp in self.parts.iter(){
            for (i,m) in sp.mults.into_iter().enumerate(){
                temp[i]+=m*sp.hits as i16
            }
        }
        let mut res = [0.0;6];
        for (i,v) in temp.into_iter().enumerate(){
            res[i]=v as f32 / 100f32;
        }
        res
    }
    pub fn cost(&self) -> i32{
        self.cost
    }
}

#[derive(Default, PartialEq)]
pub struct SpellPart{
    id: u8,
    pub mults: [i16; 6],
    pub hits: u8,
    pub final_mult: i32,
    pub power: i8 // heal power
}