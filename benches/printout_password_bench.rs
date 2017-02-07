#![feature(test)]

extern crate test;
extern crate password_maker_lib;

use test::test::Bencher;
use password_maker_lib::printout_password;

#[cfg(not(debug_assertions))]
#[bench]
#[ignore]
fn printout_password_bench(b: &mut Bencher)
    -> ()
{
    let length = 10000;
    let amount = 75000;
    b.iter(||
        {
            printout_password(&length, &amount);
        });
}
