extern crate rand;

use rand::{thread_rng,Rng};

pub fn create_package(length: &usize, amount: &usize)
    -> Vec<String>
{
    let mut passwords:Vec<String> = Vec::with_capacity(*amount);
    let mut rng = thread_rng();
    let mut buffer = String::with_capacity(*length);
    
    let allowed = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890+-!?=.,;:'*^";
    'ultra: loop
    {
        'outer: loop
        {
            buffer.push(allowed.chars().nth(rng.gen_range(0usize, allowed.len())).unwrap() );
            if buffer.len() >= *length
            {break 'outer;}
        }
        passwords.push(buffer);
        buffer = String::with_capacity(*length);
        if passwords.len() >= *amount
        {break 'ultra;}
    }
    passwords
}

pub fn printout_password(length: &usize, amount: &usize)
{
    // change the output stream from println to whatever the caller specifies.
    let mut rng = thread_rng();
    let allowed = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890+-!?=.,;:'*^";
    let mut current_len:usize;
    let mut current_amount:usize = 0;
    let mut buffer = String::with_capacity(*length);
    'ultra: loop
    {
        current_len = 0;
        'outer: loop
        {
            buffer.push(allowed.chars().nth(rng.gen_range(0usize, allowed.len())).unwrap() );
            current_len += 1;
            if current_len >= *length
            {break 'outer;}
        }
        println!("{}", buffer);
        current_amount += 1;
        buffer.clear();
        if current_amount >= *amount
        {break 'ultra;}
    }
}


mod test
{
    #[test]
    fn bounds_test()
    {
        let length = 1000;
        let amount = 1000;
        let mut passwords = super::create_package(&length, &amount);
        //tests are bugged atm, .len() does not return the correct value
        assert_eq!(passwords.len(), amount);
        assert_eq!(passwords.last().unwrap().len(), length);
    }
}
