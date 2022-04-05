use std::{io,fmt};
use rand::prelude::*;

// desired behavior:
// - input a player 1 name
// - input a player 2 name
// create player struct's
// print character stats
// fight and output the result of each round til one is dead

#[derive(Debug)]
struct Player {
    name: String, // todo: make &str
    hit_points: i8,
    damage_roll: i8,
}

impl Player {
    fn do_damage(&self, other_player: &mut Player, rng: &mut ThreadRng) {
        let damage = rng.gen_range(1..=self.damage_roll);
        other_player.take_damage(damage);
        println!("{} does {} damage to {}", self.name, damage, other_player.name);
    }

    fn take_damage(&mut self, damage: i8) {
        self.hit_points -= damage;
    }

    fn build(name: String, rng: &mut ThreadRng) -> Player {
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

fn main() -> io::Result<()> {
    let mut rng = thread_rng();

    println!("Enter player 1 name: ");
    let mut p1_name = String::new();
    io::stdin().read_line(&mut p1_name)?;

    println!("Enter player 2 name: ");
    let mut p2_name = String::new();
    io::stdin().read_line(&mut p2_name)?;

    let mut player1 = Player::build(p1_name.trim().to_owned(), &mut rng);
    let mut player2 = Player::build(p2_name.trim().to_owned(), &mut rng);

    print_players(&player1, &player2);

    while player1.hit_points > 0 && player2.hit_points > 0 {
        player1.do_damage(&mut player2, &mut rng);
        if player2.hit_points < 1 {
            print_players(&player1, &player2);
            break;
        }
        player2.do_damage(&mut player1, &mut rng);
        print_players(&player1, &player2);
    }

    match player1.hit_points > 0 {
        true => { println!("{} wins!", player1.name) },
        false => { println!("{} wins!", player2.name) },
    }

    Ok(())
}

fn print_players(player1: &Player, player2: &Player) {
    println!("{} vs {}", player1, player2);
}
