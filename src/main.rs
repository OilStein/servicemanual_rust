mod machines;
mod utils;
use crate::machines::Machine;

fn main() {
    let m = Machine::new("name".to_string(), 2000, "EV".to_string());

    println!("{:?}", m);

    let _ma = Machine::read_csv_to_db();
}
