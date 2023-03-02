use std::collections::HashSet;


fn get_digits_hash( k: u64 ) -> u64 { 

    let mut digits = vec![];

    let mut p = 1u64; 

    let mut k2 = k; 

    while p<k { 
            p = p * 10; 
    }

    p = p/10; 

    loop { 

        let d = k2 / p; // that's my digit ! 
        digits.push( d as u8 );
        k2 = k2 - d*p; 
        p = p/10; 
        if p==0  { 
            break;
        }
    }
    digits.sort();
    
    let mut defhasher = DefaultHasher::new();
    for d in digits {
        d.hash(&mut defhasher); 
        println!( "digit {}", d);
    }
    
    defhasher.finish()
    

}


fn main() {
    println!("Hello, world!");

    let mut _i: u64 = 60;

    println!( "hash 1  {}", get_digits_hash( 123 ) );
    println!( "hash 2 {}", get_digits_hash( 124 ) ); 
    assert!( get_digits_hash( 123 ) == get_digits_hash(321) );
    assert!( get_digits_hash( 112323 ) == get_digits_hash(332211) ); 


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
