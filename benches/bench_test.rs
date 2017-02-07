#![feature(test)]

extern crate test;
extern crate password_maker_lib;

use test::test::Bencher;
use password_maker_lib::create_package;

#[cfg(not(debug_assertions))]
#[bench]
fn create_package_bench(b: &mut Bencher)
    -> ()
{
    let length = 1000;
    let amount = 1000;
    b.iter(||
        {
            let mut passwords = create_package(&length, &amount);
            //tests are bugged atm, .len() does not return the correct value
            assert_eq!(passwords.len(),amount);
            assert_eq!(passwords.last().unwrap().len(), length);
        });
}
