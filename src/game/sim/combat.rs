use crate::game::sim::event_handling::SimMessageEvent;
use crate::game::ItemId;
use crate::game::{sim::dungeon_components::TextType, StatBonus};
use bevy::prelude::*;
use bevy::reflect::Reflect;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Component, Default, Copy, Clone, Reflect, Serialize, Deserialize, Debug)]
pub struct Combatant {
    pub health: i32,
    pub max_health: i32,
    pub proficiency: i32,
    pub damage_res: i32,
    pub damage_bonus: i32,
    pub negative_feedback: i32,
}

impl std::fmt::Display for Combatant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}/{}/{}/{}",
            self.health, self.max_health, self.proficiency, self.damage_res, self.damage_bonus
        )
    }
}

impl Combatant {
    pub fn boost(mut self, stats: StatBonus) {
        self.max_health += stats.max_health;
        self.proficiency += stats.proficiency;
        self.damage_res += stats.damage_res;
        self.damage_bonus += stats.damage_bonus;
    }
}

#[derive(Default, PartialEq, Eq, Copy, Clone)]
pub enum CombatState {
    #[default]
    Init,
    InProgress,
    EnemyDead,
    HeroDead,
    Ended,
}

#[derive(Default, Resource)]
pub struct Hero {
    pub combat_stats: Combatant,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
pub enum EnemyId {
    #[default]
    None,
    Rat,
    GoblinBrat,
    GoblinShieldbearer,
    GoblinSwordsman,
    OrcWarrior,
    Skeleton,
    Zombie,
    OgreNecromancer,
}

#[derive(Clone, Deserialize, Serialize, Debug, Resource)]
pub struct Enemy {
    pub enemy_id: EnemyId,
    pub combat_stats: Combatant,
    pub name: String,
    pub enter_combat_text: TextType,
    pub drop_table: DropTable,
}

impl std::fmt::Display for Enemy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, stats: {}", self.name, self.combat_stats)
    }
}

#[derive(Default, Clone, Deserialize, Serialize, Debug)]
pub struct DropTable {
    pub items: Vec<ItemId>,
    pub chances: Vec<u32>,
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            enemy_id: EnemyId::None,
            combat_stats: Default::default(),
            name: "Empty enemy".to_string(),
            enter_combat_text: TextType::EnterRat,
            drop_table: DropTable::default(),
        }
    }
}

pub fn process_combat(
    events: &mut EventWriter<SimMessageEvent>,
    mut monster: &mut Combatant,
    mut hero: &mut Combatant,
    cmbt_state: &mut CombatState,
) {
    const DICE: i32 = 12;
    let mut rng = rand::thread_rng();
    let monster_roll = rng.gen_range(0..DICE) + monster.proficiency - monster.negative_feedback;
    let hero_roll = rng.gen_range(0..DICE) + hero.proficiency - hero.negative_feedback;

    if monster_roll > hero_roll {
        let diff = ((monster_roll - hero_roll) as f32 / 2.0).round() as i32;
        let damage = (monster.damage_bonus - hero.damage_res + diff).clamp(1, 500);
        hero.health -= damage;
        events.send(SimMessageEvent(TextType::CombatHeroHit));
        monster.negative_feedback += 1;
        hero.negative_feedback = 0;
        debug!("Hero hit for {}: HP at {}.", damage, hero.health);
    } else if hero_roll > monster_roll {
        let diff = ((hero_roll - monster_roll) as f32 / 2.0).round() as i32;
        let damage = (hero.damage_bonus + diff - monster.damage_res).clamp(1, 500);
        monster.health -= damage;
        events.send(SimMessageEvent(TextType::CombatEnemyHit));
        debug!("Monster hit for {}: HP at {}.", damage, monster.health);
        monster.negative_feedback = 0;
        hero.negative_feedback += 1;
    } else {
        events.send(SimMessageEvent(TextType::CombatNoResolution));
    }

    if hero.health < 1 {
        *cmbt_state = CombatState::HeroDead;
    } else if monster.health < 1 {
        *cmbt_state = CombatState::EnemyDead;
    }
}
