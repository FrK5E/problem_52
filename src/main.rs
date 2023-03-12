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

fn number_of_digits(k: u64) -> u32 {
    let a = (k as f64).log10().floor();
    (a as u32) + 1
}

fn pow10(e: u32) -> u64 {
    let mut result: u64 = 1;

    for _k in 0..e {
        result = result * 10u64;
    }

    return result;
}

fn get_next_lower_common_multiple(k: u64) -> u64 {
    let mut res = k;
    let mut repeat_flag = true;
    while repeat_flag {
        repeat_flag = false;
        for i in 2..7 {
            if res % i != 0 {
                res = res - 1;
                repeat_flag = true;
                break;
            }
        }
        
    }
    return res;
}

fn main() {
    println!("Hello, world!");

    println!("hash 1  {}", get_digits_hash(123));
    println!("hash 2 {}", get_digits_hash(124));
    assert!(get_digits_hash(123) == get_digits_hash(321));
    assert!(get_digits_hash(112323) == get_digits_hash(332211));
    assert!(number_of_digits(112323) == number_of_digits(332211));
    assert!(pow10(1) == 10);
    assert!(pow10(3) == 1000);
    assert!(get_next_lower_common_multiple(100) == 60);

    let lcm: u64 = 60;
    let mut power_of_ten = 3;
    let mut found: bool = true;
    let mut i: u64;

    loop {
        i = pow10(power_of_ten);
        i = get_next_lower_common_multiple(i);

        loop {
            let h = get_digits_hash(i);
            let nd_h = number_of_digits(i);
            found = true;
            let mut next_power_of_ten: bool = false;

            for k in (2..7).rev() {
                let i2 = i / k;
                let nd_h2 = number_of_digits(i2);

                if nd_h2 < nd_h {
                    // here we need to jump to next power of ten
                    found = false;
                    next_power_of_ten = true;
                    break;
                }

                let h2 = get_digits_hash(i2);

                if h2 != h {
                    found = false;
                    break;
                }
            }

            if next_power_of_ten {
                power_of_ten += 1;
                break;
            }
            if found { 
                break;
            }
            i -= lcm;
        }
        if found { 
            break;
        }
    }

    println!("the integer is {}", i);
}
