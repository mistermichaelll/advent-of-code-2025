mod dial;
mod instructions;
use dial::{Dial, DialTurn, get_real_password};
use instructions::get_input_instructions;

fn main() {
    let mut d: Dial = Dial::new(50);
    let instructions: Vec<String> = get_input_instructions("data/input.txt");

    let t: Vec<DialTurn> = instructions
        .iter()
        .map(|v| {
            v.parse::<DialTurn>()
                .expect("could not parse to dial turn.")
        })
        .collect();

    println!("{}", get_real_password(&mut d, t));
}
