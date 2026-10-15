use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};


const START_GOLD: u64 = 100;
const FOOD_PRICE: u64 = 5;
const MAX_SEED: u64 = 9999;

const OXEN_PRICE: u64 = 40;
const CLOTHING_PRICE: u64 = 10;
const AMMO_PRICE: u64 = 2;


struct Inventory {
    cash: u64,
    oxen: u64,
    food: u64,
    clothing: u64,
    ammo: u64,
    day: u64,
    roll_count: u64,
}

fn print_menu(cash: u64) {
    println!("=== PATRICK'S TRADING POST ===");
    println!("Cash Available: ${}", cash);
    println!("1. Oxen      ($40)");
    println!("2. Food      ($1/lb)");
    println!("3. Clothing  ($10)");
    println!("4. Ammo      ($2/box)");
    println!("5. Hunt For Food");
    println!("6. Roll Multiple Dice");
    println!("7. Check Inventory Status");
    println!("8. Leave Post");
    println!("===========================");
}

fn print_status(inv: &Inventory) {
    println!("\n==========================");
    println!("      CURRENT STATUS      ");
    println!("==========================");
    println!("Cash:     ${}", inv.cash);
    println!("Oxen:     {} head", inv.oxen);
    println!("Food:     {} lbs", inv.food);
    println!("Clothing: {} sets", inv.clothing);
    println!("Ammo:     {} boxes", inv.ammo);
    println!("==========================");
}

fn can_afford(cash: u64, cost: u64) -> bool {
    cash >= cost
}

fn hunt_food() -> u64 {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let food_found = ((start % 41) + 10) as u64;
    println!("You went hunting and brought back {} lbs of food!", food_found);
    food_found
}

fn read_number_bounded(min: u64, max: u64) -> u64 {
    loop {
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim().parse::<u64>() {
            Ok(num) if num >= min && num <= max => return num,
            _ => {
                print!("Invalid choice. Please enter a number between {} and {}: ", min, max);
            }
        }
    }
}

fn read_number() -> u64 {
    print!("Enter choice: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<u64>().unwrap_or(0)
}


fn roll_die() -> u64 {
    std::thread::sleep(std::time::Duration::from_nanos(1));
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let roll = ((start % 6) + 1) as u64;
    println!("You rolled a: 🎲 {}", roll);
    roll
}


fn roll_many() -> u64 {
    print!("How many dice would you like to roll?: ");
    let count = read_number();
    let mut total = 0;


    for _ in 0..count {
        total += roll_die();
    }


    println!("Total combined score: {}", total);
    total
}


fn main() {
    let mut player_inv = Inventory {
        cash: 400,
        oxen: 0,
        food: 0,
        clothing: 0,
        ammo: 0,
	day: 0,
	roll_count: 0,
    };


    loop {
        print_menu(player_inv.cash);
        print!("Enter choice: ");
        let choice = read_number();

        match choice {
            1 => {
                if can_afford(player_inv.cash, 40) {
                    player_inv.cash -= 40;
                    player_inv.oxen += 1;
                    println!("Successfully purchased 1 Ox!");
                } else {
                    println!("You can't afford that!");
                }
            }
            2 => {
                print!("How many pounds of food? ($1/lb): ");
                let amount = read_number();
                if can_afford(player_inv.cash, amount) {
                    player_inv.cash -= amount;
                    player_inv.food += amount;
                    println!("Successfully purchased {} lbs of food!", amount);
                } else {
                    println!("You can't afford that much food!");
                }
            }
            3 => {
                if can_afford(player_inv.cash, 10) {
                    player_inv.cash -= 10;
                    player_inv.clothing += 1;
                    println!("Successfully purchased 1 Clothing set!");
                } else {
                    println!("You can't afford that!");
                }
            }
            4 => {
                if can_afford(player_inv.cash, 2) {
                    player_inv.cash -= 2;
                    player_inv.ammo += 1;
                    println!("Successfully purchased 1 box of Ammo!");
                } else {
                    println!("You can't afford that!");
                }
            }
            5 => {
                let found = hunt_food();
                player_inv.food += found;
            }
            6 => {
                roll_many();
            }
            7 => {
                print_status(&player_inv);
            }
            8 => {
                println!("Goodbye, traveler! Good luck on the trail.");
                break;
            }
            _ => println!("Invalid option. Please choose 1-8."),
        }
    }
}
