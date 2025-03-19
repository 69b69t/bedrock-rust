mod util;
mod nether;
mod overworld;

fn main() {
    let seed: u64 = 694201337;
    println!("overworld bedrock floor for seed {seed}");
    for j in -8..8 {
        for i in -8..8 {
            match overworld::is_bedrock(seed, i, -60, j) {
                true => print!("# "),
                false => print!("_ "),
            }
        }
        println!();
    }

    println!("nether bedrock floor for seed {seed}");
    for j in -8..8 {
        for i in -8..8 {
            match nether::is_bedrock(seed, i, 4, j) {
                true => print!("# "),
                false => print!("_ "),
            }
        }
        println!();
    }
}
