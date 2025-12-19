mod dial;
mod instructions;
use dial::{Dial, DialTurn, get_new_password_version};
use instructions::parse_instructions;

fn main() {
    let mut d: Dial = Dial::new(50);
    let instructions: Vec<DialTurn> = parse_instructions("data/input.txt");

    println!("{}", get_new_password_version(&mut d, instructions));
}
