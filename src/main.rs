mod die;

use die::Die;

fn main() {
    let mut die = Die::new();

    die.roll();
    die.is_held = true;

    println!("{}", die);
}
