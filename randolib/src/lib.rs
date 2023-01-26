/// Some libraries will expose a prelude module that's meant to be used with a wildcard.
/// This is a convention to allow you to use the important bits easily. Generally you should
/// not use wildcards in other cases. Favor explicit use.
use rand::{distributions::Standard, prelude::*};
use somelib::{error::Error, my_result::MyResult};
use std::{cmp::PartialEq, fmt::Debug, marker::PhantomData};

/// We want `get_random_vec` to be shared amongst all of our `Rando*` types
pub trait GetRandoStuff<T>
where
    // `Standard` is a unit `struct` which implements `Distribution` for common types
    Standard: Distribution<T>,
    T: Debug,
{
    /// This is a declaration and default implementation
    fn get_random_vec(&self, len: usize) -> Vec<T> {
        // Create a thread-local RNG, i.e., one that is `!Send` and `!Sync`
        let mut rng = thread_rng();
        // Here is an example of Rust as a functional language
        // Gen our (max) 32 elements of `T`
        rng.gen::<[T; 32]>()
            // `iter` returns an iterator of &T, `into_iter` returns (owned) T
            .into_iter()
            // take returns `len` or max items
            .take(len)
            // take an iterator and return a collection
            .collect::<Vec<_>>()
    }
}

/// The struct is our main composite type. We can have structs with fields, unit structs
/// and tuple structs `struct RandoX(RandoA)` often called `newtype`s
pub struct RandoA<T>
where
    Standard: Distribution<T>,
    T: Debug,
{
    // Since RandoA has no members, nothing takes the type `T`. PhantomData is the Rust
    // workaround. It's a zero-sized item that 'carries' our generic param.
    phantom_data: PhantomData<T>,
}

impl<T> RandoA<T>
where
    Standard: Distribution<T>,
    T: Debug,
{
    /// We return `Self` by convention, can also return `RandoA`
    ///
    /// This is called an associated function. Functions that take a
    /// `self` parameter are called methods.
    ///
    /// By convention, we use `new(..) -> Self` as a constructor
    pub fn new() -> Self {
        // Implicit return. Note the lack of the `return` keyword and no `;` at the end of the line
        // You can also use `return RandoA { .. };`. Favor the former.
        RandoA {
            phantom_data: PhantomData,
        }
    }

    /// Get a single random `T`
    pub fn get_random_item(&self) -> T {
        // Create a thread-local RNG, i.e., one that is `!Send` and `!Sync`
        let mut rng = thread_rng();
        rng.gen::<T>()
    }
}

/// Since `GetRandoStuff` has a default impl for all of its members, we can use
/// an empty impl here to make `Rando*` `GetRandoStuff`
impl<T> GetRandoStuff<T> for RandoA<T>
where
    Standard: Distribution<T>,
    T: Debug,
{
}

/// Here we're going to maintain state, storing the last random item produced
/// so we can check for consecutive random values.
pub struct RandoB<T>
where
    Standard: Distribution<T>,
    // We need `Clone` to copy `last_item`, `PartialEq` to compare it to our new item
    // and `Debug` to satisfy the bound of `MyResult`
    T: Clone + PartialEq + Debug,
{
    /// Since we won't have a last item until we run `get_random_item`, this
    /// has to be an Option::None when we create our struct
    last_item: Option<T>,
}

impl<T> RandoB<T>
where
    Standard: Distribution<T>,
    T: Clone + PartialEq + Debug,
{
    /// Our ctor
    pub fn new() -> Self {
        // Start with None
        RandoB { last_item: None }
    }

    /// Return a single random `T` or an error if `self.last_item` is the same as our new item
    /// Since we're mutating `self`, we need a mutable reference to it.
    pub fn get_random_item(&mut self) -> MyResult<T, Error> {
        // Create a thread-local RNG, i.e., one that is `!Send` and `!Sync`
        let mut rng = thread_rng();
        let item = rng.gen::<T>();
        if self.last_item.is_none() {
            // This is an explicit return
            return MyResult::Ok(item);
        }
        // Let's take the old item. We could then return it if necessary.
        let last_item = self.last_item.take().unwrap();
        self.last_item = Some(item.clone());
        if last_item == item {
            MyResult::Err(Error::ConsecutiveRandom)
        } else {
            MyResult::Ok(item)
        }
    }
}

/// Since GetRandoStuff has a default impl of all of its members, we can use
/// an empty impl here to make `Rando*` `GetRandoStuff`
impl<T> GetRandoStuff<T> for RandoB<T>
where
    Standard: Distribution<T>,
    T: Clone + PartialEq + Debug,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gens_random_stuff_randoa() {
        let rando = RandoA::<char>::new();

        // `take` returns either the requested or max amount, so this is ok
        let rand_chars = rando.get_random_vec(99);

        // But we only get 32 chars anyway
        assert_eq!(rand_chars.len(), 32);
    }

    #[test]
    fn it_gens_random_stuff_randob() {
        // We need a `mut` `struct` here in order to call `get_random_item`
        let mut rando = RandoB::<char>::new();

        let rand_item = rando.get_random_item();

        // It's impossible for this to error on the first call
        assert!(rand_item.is_ok());
    }
}
