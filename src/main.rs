use std::io;
use rand::prelude::*;
mod game;

// desired behavior:
// - input a player 1 name
// - input a player 2 name
// create player struct's
// print character stats
// fight and output the result of each round til one is dead

fn main() -> io::Result<()> {
    let mut rng = thread_rng();

    println!("Enter player 1 name: ");
    let mut p1_name = String::new();
    io::stdin().read_line(&mut p1_name)?;

    println!("Enter player 1 class [Warrior, Wizard]: ");
    let mut p1_class = String::new();
    io::stdin().read_line(&mut p1_class)?;
    println!("{}", p1_class);
    let p1_parsed_class = game::Class::from_str(&p1_class.trim()).unwrap();

    println!("Enter player 2 name: ");
    let mut p2_name = String::new();
    io::stdin().read_line(&mut p2_name)?;

    println!("Enter player 2 class [Warrior, Wizard]: ");
    let mut p2_class = String::new();
    io::stdin().read_line(&mut p2_class)?;
    let p2_parsed_class = game::Class::from_str(&p2_class.trim()).unwrap();

    let mut player1 = game::Player::build(
        p1_name.trim().to_owned(), p1_parsed_class, &mut rng);
    let mut player2 = game::Player::build(
        p2_name.trim().to_owned(), p2_parsed_class, &mut rng);

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

fn print_players(player1: &game::Player, player2: &game::Player) {
    println!("{} vs {}", player1, player2);
}
