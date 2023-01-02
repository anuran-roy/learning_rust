// Module 1 imports
mod mod1;

use mod2::lifetimes::lifetimes::function_call_sample;

use crate::mod1::arrays_tuples::arrays_tuples::sample as arrays_tuples_sample;
use crate::mod1::exercise::q1::sample as m1q1_sample;
use crate::mod1::exercise::q2::sample as m1q2_sample;
use crate::mod1::firstmod::firstmod::hello;
use crate::mod1::fizzbuzz::fizzbuzz::list_fizz_buzz;
use crate::mod1::generics::generics::sample as generics_sample;
use crate::mod1::struct_impl::struct_impl::sample as struct_sample;
use crate::mod1::views_references::views_references::sample as views_sample;

//Module 2 imports
mod mod2;

use crate::mod2::borrowing::borrowing::sample as borrowing_sample;
use crate::mod2::const_static::const_impl::sample as const_sample;
use crate::mod2::const_static::static_var::static_impl as static_sample;
use crate::mod2::exercise::q1::sample as library_sample;
use crate::mod2::iterators::iterators::sample as iterators_sample;
use crate::mod2::lifetimes::lifetimes::data_structures_sample;
use crate::mod2::ownership::ownership::sample as ownership_sample;
use crate::mod2::scope_shadowing::scope_n_shadowing::sample as scope_shadowing_sample;
use crate::mod2::vectors::vectors::sample as vectors_sample;

fn exec_mod1_samples() {
    views_sample();

    arrays_tuples_sample();
    // Strings
    // 1. &str = Immutable reference to a string slice
    // 2. String = Mutable string buffer

    // Function calling
    list_fizz_buzz(10, 15);
    struct_sample();

    hello();

    generics_sample();
}

fn exec_mod1_ex_samples() {
    println!("{:?}", m1q1_sample(8, 16));
    m1q2_sample();
}

fn exec_mod2_samples() {
    const_sample();
    static_sample();
    scope_shadowing_sample();
    ownership_sample();
    function_call_sample();
    data_structures_sample();
    vectors_sample();
    // borrowing_sample();
}

fn exec_mod2_ex_samples() {
    library_sample();
    iterators_sample();
}

fn main() {
    println!("Hello, world!");

    // exec_mod1_samples();
    // exec_mod1_ex_samples();
    exec_mod2_samples();
    exec_mod2_ex_samples();
}
