use rand::seq::IteratorRandom;

fn main() {
    let mut rng = rand::thread_rng();

    let noun = include_str!("../data/noun")
        .lines()
        .choose(&mut rng)
        .expect("Could not get noun from collection!");

    let adjective = include_str!("../data/adjective")
        .lines()
        .choose(&mut rng)
        .expect("Could not get noun from collection!");

    println!("{}-{}", adjective, noun);
}
