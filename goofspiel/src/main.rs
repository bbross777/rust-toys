use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Goofspiel");
    
    let mut gplayed: [i32; 14] = [0; 14];
    let mut oplayed: [i32; 14] = [0; 14];
    let mut pplayed: [i32; 14] = [0; 14];
    let mut rounds_played = 0;
    let mut pscore = 0;
    let mut oscore = 0;

    'game_loop: loop {
        if rounds_played >= 13 {
            break 'game_loop;
        }
        println!("+========================================================================+");
        println!("Round #{}", rounds_played + 1);
        remaining_cards(&pplayed);
        let target = loop {
            let t = rand::thread_rng().gen_range(1..=13);
            if is_valid(t, &mut gplayed) {
                break t;
            }
        };
        
        println!("\nThe card on the table is {}", target);
        
        let ocard = loop {
            let t = rand::thread_rng().gen_range(1..=13);
            if is_valid(t, &mut oplayed) {
                break t;
            }
        };

        // println!("Opponent plays {}.", ocard);

        let bid = loop {
            println!("Input your bid here: ");
            let mut bid = String::new();

            io::stdin()
                .read_line(&mut bid)
                .expect("Failed to read line.");

            let bid: usize = match bid.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            if bid > 13 || bid < 1 {
                println!("Not a valid card.");
            }
            if is_valid(bid, &mut pplayed) {
                break bid;
            } else {
                println!("Selected card {} has already been used.", bid);
            }
        };
        println!("Opponent bid {}.", ocard);

        match bid.cmp(&ocard) {
            Ordering::Less => {
                println!("You lose this round!");
                oscore += target;
            }
            Ordering::Greater => {
                println!("You win this round!");
                pscore += target;
            },
            Ordering::Equal => {
                println!("Tie!");
            }
        }
        println!("Your Score: {}", pscore);
        println!("Opponent Score: {}", oscore);
        rounds_played += 1;
        println!("+========================================================================+");
        println!();
    }
}

fn remaining_cards(hand: &[i32]) {
    let mut index = 1;
    print!("Your Remaining Cards: ");
    loop {
        if hand[index] == 0 {
            print!("[{}] ", index);
        }
        index += 1;
        if index == hand.len() {
            break;
        }
    }
    println!("");
}

fn is_valid(card: usize, discarded: &mut [i32]) -> bool {
    if discarded[card] == 1 {
        // println!("Selected card {} has already been used.", card);
        return false;
    } else {
        discarded[card] = 1;
        // println!("Card is valid.");
        return true;
    }
}
