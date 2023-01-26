use std::fmt::{Debug, Formatter};
use std::process::{ExitCode, Termination};

/// Partially recreate `std::result::Result` to show how Rust `enum`s / ADTs work
/// Name this `MyResult` as `std::result::Result` is imported automatically
pub enum MyResult<T, E>
where
    T: Debug, // Since we want to be able to implement `Debug` below, our `T` and `E` should be `Debug`
    E: Debug, // As we want most things to be `Debug` it's common for generics to be constrained to `Debug`
{
    /// Our variants:
    /// This is basically a tuple struct
    Ok(T),
    Err(E),
    // These can take concrete types as well
    // `ErrCode(i64),`
    //
    // We can also have unit structs
    // `Invalid,`
    //
    // We can also have structs with fields
    // `Err { reason: String },`
}

/// This is the default (non-`trait`-based) implementation. This
/// shares some similarities with classes in other languages.
/// There can be multiple of these, but by convention usually zero or one.
impl<T, E> MyResult<T, E>
where
    T: Debug,
    E: Debug,
{
    /// We don't need to handle `T` or `E` here because we don't care about the contents
    /// Here we take `self` by reference `&`
    pub fn is_ok(&self) -> bool {
        match self {
            MyResult::Ok(_) => true, // The `_` means we want to ignore the value
            _ => false,              // The lone `_` here is a catch-all
        }
    }

    pub fn is_err(&self) -> bool {
        match self {
            MyResult::Err(_) => true,
            _other => false, // We can have a named catch-all (prefixing it with `_` silences the unused_variables warning)
        }
    }

    /// Here we take `self` by value and return `T` or panic
    /// In this case taking `self` by value `move`s `self` into this function / scope
    pub fn unwrap(self) -> T {
        match self {
            MyResult::Ok(val) => val,
            _ => panic!("attempting to unwrap a nonexistent value"), // Terminate the program with a message
        }
    }

    /// Write the scaffold for returning the error but finish it later
    pub fn unwrap_err(self) -> E {
        match self {
            MyResult::Err(_err) => todo!(), // Incomplete, but will compile
            _ => unimplemented!(),          // Same as `todo`
        }
    }
}

/// Here we manually implement `Debug` which will give us a string rep
/// of our enum. Favor using
/// `#[derive(Debug)]`
/// `enum MyResult {..}`
/// unless there is an explicit reason to write this yourself.
/// Unlike the default impl above, we use the `impl A for B` syntax
impl<T, E> Debug for MyResult<T, E>
where
    T: Debug,
    E: Debug,
{
    /// We generally don't need to leave doc comments like this for `trait` implementations
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // If we did not constrain `T` and `E` to `Debug`, we could not format them with `{:?}` here
            MyResult::Ok(val) => write!(f, "Ok({:?})", val),
            MyResult::Err(err) => write!(f, "Err({:?})", err),
        }
    }
}

/// We don't normally have to implement this, but we're doing it here so `MyResult`
/// can be used as a return type for `main`.
impl<T, E> Termination for MyResult<T, E>
where
    T: Debug,
    E: Debug,
{
    fn report(self) -> ExitCode {
        match self {
            MyResult::Ok(_) => ExitCode::SUCCESS,
            MyResult::Err(_) => ExitCode::FAILURE,
        }
    }
}

/// Implement `From<Result>` for `MyResult` so we can use the `?` operator
impl<T, E> From<MyResult<T, E>> for Result<T, E>
where
    T: Debug,
    E: Debug,
{
    fn from(value: MyResult<T, E>) -> Self {
        match value {
            MyResult::Ok(val) => Ok(val),
            MyResult::Err(err) => Err(err),
        }
    }
}

/// Marker traits have empty implementations
/// Can be moved across thread boundaries
unsafe impl<T, E> Send for MyResult<T, E> {}

/// Can have shared references between thread boundaries, requires `Send`
unsafe impl<T, E> Sync for MyResult<T, E> {}

/// Customary to put a test module in source files for *unit* tests
#[cfg(test)]
mod tests {
    /// Import everything from our parent module
    use super::*;

    /// This will get picked up by `cargo test`
    #[test]
    /// Special case, we expect this to panic
    #[should_panic(expected = "attempting to unwrap a nonexistent value")]
    fn it_should_panic() {
        // We want this to panic, so we don't care about `T` or `E`
        // but we *do* have to explicitly define them since there is nothing
        // for the compiler to go on about what these types should be.
        // In this case, we make them both the *unit* type `()`
        let result = MyResult::Err::<(), ()>(()); // The `::<_>` here is called *turbofish*
        result.unwrap();
    }
}
