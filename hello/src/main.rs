use randolib::{GetRandoStuff, RandoA, RandoB};
use somelib::error::Error;

/// A `main` fn allows us to compile an executable. This can be async.
/// These can return any type that implements `Termination`
/// Usually these return the unit `()` or `std::result::Result`
fn main() -> Result<(), Error> {
    let rando_a = RandoA::<char>::new();
    let mut rando_b = RandoB::<char>::new();

    println!("RandoA says: {:?}", rando_a.get_random_vec(12));
    println!("RandoA says: {:?}", rando_a.get_random_item());

    loop {
        // Convert `MyResult` into `Result` so we can use the `?` operator
        let item = Into::<Result<_, _>>::into(rando_b.get_random_item())?;
        println!("RandoB says: {:?}", item);
    }
}
