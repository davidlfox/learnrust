use rand::prelude::*;
use std::fmt;

pub struct Player {
    pub name: String, // todo: make &str
    pub hit_points: i8,
    damage_roll: i8,
}

impl Player {
    pub fn do_damage(&self, other_player: &mut Player, rng: &mut ThreadRng) {
        let damage = rng.gen_range(1..=self.damage_roll);
        other_player.take_damage(damage);
        println!("{} does {} damage to {}", self.name, damage, other_player.name);
    }

    fn take_damage(&mut self, damage: i8) {
        self.hit_points -= damage;
    }

    pub fn build(name: String, rng: &mut ThreadRng) -> Player {
        Player { 
            name: name,
            hit_points: rng.gen_range(1..=4) + 10,
            damage_roll: rng.gen_range(1..=3),
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({}hp)", self.name, self.hit_points)
    }
}