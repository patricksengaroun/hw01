use std::time::{SystemTime, UNIX_EPOCH};


fn print_menu(cash: u32) {
    println!("=== PATRICK'S TRADING POST ===");
    println!("Cash Available: ${}", cash);
    println!("1. Oxen      ($40)");
    println!("2. Food      ($1/lb)");
    println!("3. Clothing  ($10)");
    println!("4. Ammo      ($2/box)");
    println!("5. Leave Post");
    println!("===========================");
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


fn main() {
    let mut cash = 400;
    let mut food = 0;


    print_menu(cash);
   

    println!("\n[Testing Hunt]");
    food += hunt_food();
    println!("Total food stash: {} lbs", food);
}
