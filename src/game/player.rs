use rand::prelude::*;
use std::fmt;
use super::{Class,Item};

pub struct Player {
    pub name: String, // todo: make &str
    pub hit_points: i8,
    damage_roll: i8,
    class: Class,
    pub items: Vec<Item>,
}

impl Player {
    pub fn do_damage(&self, other_player: &mut Player, rng: &mut ThreadRng) {
        let damage = rng.gen_range(1..=self.damage_roll);
        other_player.take_damage(damage);
        let item_name = &self.items.first().unwrap().name;
        println!("{} hits with {} and does {} damage to {}", self.name, item_name, damage, other_player.name);
    }

    fn take_damage(&mut self, damage: i8) {
        self.hit_points -= damage;
    }

    pub fn build(name: String, class: Class, rng: &mut ThreadRng) -> Player {
        let hit_points = Class::hit_points(&class, rng).unwrap();
        Player {
            name,
            class,
            hit_points,
            damage_roll: rng.gen_range(1..=3),
            items: vec![],
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}[{:?}] ({}hp)", self.name, self.class, self.hit_points)
    }
}