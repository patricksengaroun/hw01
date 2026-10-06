use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};


struct Inventory {
    cash: u32,
    oxen: u32,
    food: u32,
    clothing: u32,
    ammo: u32,
}

fn print_menu(cash: u32) {
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
    println!("💰 Cash:     ${}", inv.cash);
    println!("🐂 Oxen:     {} head", inv.oxen);
    println!("🌾 Food:     {} lbs", inv.food);
    println!("👕 Clothing: {} sets", inv.clothing);
    println!("📦 Ammo:     {} boxes", inv.ammo);
    println!("==========================");
}

fn can_afford(cash: u32, cost: u32) -> bool {
    cash >= cost
}


fn hunt_food() -> u32 {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let food_found = (start % 41) + 10;
    println!("You went hunting and brought back {} lbs of food!", food_found);
    food_found as u32
}


fn read_number() -> u32 {
    print!("Enter choice: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<u32>().unwrap_or(0)
}


fn roll_die() -> u32 {
    std::thread::sleep(std::time::Duration::from_nanos(1));
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let roll = (start % 6) + 1;
    println!("You rolled a: 🎲 {}", roll);
    roll as u32
}


fn roll_many() -> u32 {
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
