use rand::Rng;

fn main() {
    let mut die_roll = 0;

    while die_roll != 6 {
        print!("Rolling die...");
        die_roll = rand::thread_rng().gen_range(1,7);
        println!("you rolled a {}", die_roll);
    }
    println!("Congratulations!");
}
