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
    let mut flag = true;
    while flag {
        for i in 2..7 {
            if k % i != 0 {
                res = res - 1;
                flag = true;
                break;
            }
        }
        flag = false;
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

    loop {
        let mut i = pow10(power_of_ten);
        i = get_next_lower_common_multiple(i);

        let h = get_digits_hash(i);
        let nd_h = number_of_digits(i);

 // TODO      
        loop {
            let mut found: bool = true;

            for k in 2..7 {
                let i2 = i / k;
                let nd_h2 = number_of_digits(i2);

                if nd_h2 != nd_h {
                    // here we need to jump to next power of ten
                    found = false;
                    break;
                }

                let h2 = get_digits_hash(i2);

                if h2 != h {
                    found = false;
                    break;
                }
            }

            if found {
                break;
            }
            i = i + lcm;
            if i % (lcm * 100000) == 0 {
                println!(".")
            }
        }
    }

    println!("the integer is {}", i);
}
