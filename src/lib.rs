//! TODO
//! - hardcode repos
//! - implement list
//! - implement go
//! - implement go *
//! - rust doc - see foo examples
//! - unit tests - https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
//! - dynamic config for repos

extern crate failure;

/// This is a summary of the mod
///
/// This is a little more detailed description
pub mod test_mod {
    //! this is inner doc for test_mod
    use failure::Error;

    /// This is some info about the function foo
    ///
    /// # Examples
    /// ```
    /// assert!(ok::test_mod::foo());
    /// ```
    pub fn foo() -> bool {
        println!("Foobar");
        true
    }

    pub fn error_test() -> Result<(), Error> {
        Err(failure::err_msg("an error"))
        //Ok(())
    }
}
