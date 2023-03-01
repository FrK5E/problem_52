use std::collections::HashSet;

fn get_digits_hash( k: u64 ) { 

    let mut p = 1u64; 

    let mut k2 = k; 

    while p<k { 
            p = p * 10; 
    }

    p = p/10; 

    loop { 

        let d = k2 / p; // that's my digit ! 
        println!( "this is digit: {} " , d );
        k2 = k2 - d*p; 
        p = p/10; 
        if p==0  { 
            break;
        }
    }

}


fn main() {
    println!("Hello, world!");

    let mut _i: u64 = 60;

    get_digits_hash( 123 ); 

    return;


/*     loop { 

        let h = get_digits_hash( i );

        let mut found : bool = true; 

        for k in 2..7 { 
            let i2 = i / k; 
            let h2 = get_digits_hash( i2 ); 

            if h2 != h { 
                found = false;
                break; 
            }
        }

        if found { 
            break; 
        }
        i = i+60;

    }

    println!( "the integer is {}", i );
 */}
