//use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn get_digits_hash(k: u64) -> u64 {
    let mut digits = vec![];

    let mut p = 1u64;

    let mut k2 = k;

    while p < k {
        p = p * 10;
    }

    p = p / 10;

    if p == 0 {
        p = 1;
    }

    loop {
        let d = k2 / p; // that's my digit !
        digits.push(d as u8);
        k2 = k2 - d * p;
        p = p / 10;
        if p == 0 {
            break;
        }
    }
    digits.sort();

    let mut defhasher = DefaultHasher::new();
    for d in digits {
        d.hash(&mut defhasher);
    }

    defhasher.finish()
}


fn pow10(e: u32) -> u64 {
    let mut result: u64 = 1;

    for _k in 0..e {
        result = result * 10u64;
    }

    return result;
}

fn starts_with_one( i:u64 ) -> bool { 

    let mut res = i;
    while res >= 10 { 
        res = res/10;
    }
    res<2
}

fn contains_zero_or_five(i:u64) -> bool {
 // ignore first digit, which is always one
    let mut res = i;
    while res >= 10 {
        let k = res % 10; 
        if k == 0 || k==5 { 
            return true;
        } 
        res = res/10;
    }
    return false;
}

fn contains_at_least_one_even_digit(i:u64) -> bool { 
    // ignore first digit, which is always one
    let mut res = i;
    while res >= 10 {
        let k = res % 10; 
        if k%2 == 0 { 
            return true;
        } 
        res = res/10;
    }
    return false;    
}


fn main() {
    println!("Hello, world!");

    println!("hash 1  {}", get_digits_hash(123));
    println!("hash 2 {}", get_digits_hash(124));
    assert!(get_digits_hash(123) == get_digits_hash(321));
    assert!(get_digits_hash(112323) == get_digits_hash(332211));
    assert!(pow10(1) == 10);
    assert!(pow10(3) == 1000);
    assert!(starts_with_one(12) == true );
    assert!(starts_with_one(21) == false );
    assert!(contains_zero_or_five(10)==true);
    assert!(contains_zero_or_five(132738273827832)==false);
    assert!(contains_zero_or_five(132738273527831)==true);
    assert!(contains_zero_or_five(132738270827839)==true);
    assert!(contains_at_least_one_even_digit(1335)==false);
    assert!(contains_at_least_one_even_digit(1332)==true);
    assert!(contains_at_least_one_even_digit(1239)==true);



    let mut power_of_ten = 3;
    let mut found: bool = true;
    let mut i: u64;

    // x, 2x, 3x, 4, 5x, 6x are written by same digits
    // therefore: 
    // x starts with 1, because otherwise 6x has more digits
    // x ends with 1 or 0: because 5x ends with 0 or five
    // x is divible by 3: because 3x is divisible by three and that is equivalent to sum of digits is divisible by three
    // x contains at least one of { 0,2,4,6,8 }: because 2x is even and even number ends with even digit.


    loop {
        i = pow10(power_of_ten)+2; // plus 2 so that sum of digits is three and therefore so that the number is divisible by three 
         
        

        assert!( i % 3 == 0 );

        loop { 
             
            found = true;

            if !starts_with_one(i) {
                power_of_ten += 1;  
                found = false;
                break;
            }

            if !contains_zero_or_five(i) { 
                i+=3;
                continue;
            }

            if !contains_at_least_one_even_digit(i) { 
                i+=3;
                continue;
            }

            let h = get_digits_hash(i); 

            println!( "{}", i );


            for k in (2..7).rev() {
                let i2 = i * k;

                let h2 = get_digits_hash(i2);

                if h2 != h {
                    found = false;
                    break;
                }
            }

            
            if found { 
                break;
            }
            i += 3;
        }
        if found { 
            break;
        }
    }

    println!("the integer is {}", i);
}
