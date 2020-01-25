extern crate rand;
use rand::Rng;

extern crate buildmodule;
use buildmodule::greetings::turkish;

fn main() {
    let mut rng =  rand::thread_rng();
    let b:bool = rng.gen();
    println!("{:?}",b);

    println!("English {}, {}",
    buildmodule::greetings::english::hello(),
    buildmodule::greetings::english::goodbye());

    println!("Turkish {}, {}",
    turkish::hello(),
    turkish::goodbye());
}
