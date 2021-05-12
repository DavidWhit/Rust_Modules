mod mod_1;
mod mod_2; 
mod utils;

// use mod_1::*; we could do this but then we would have to prefix everything :(
// IE in main mod_1::blah::some_blah_fn();
use mod_1::{blah, blah2, some_function};
// use mod_2::mod_under::{some_function as some_function2};
use mod_2::nested_mod::{some_function as some_function3};
use mod_2::nested_mod::another_mod::{some_function as some_function4};
use mod_2::other::{some_function as some_function5};
use utils::logging::some_logging_fn;

fn main() {
    some_function();
    blah::some_blah_fn();
    blah2::some_blah2_fn();
    // some_function2();
    some_function3();
    some_function4();
    some_function5();
    some_logging_fn();
}
