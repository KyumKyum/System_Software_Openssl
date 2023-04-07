use openssl::sha;
use std::time::Instant;
//use std::env;

fn main() {
    let data = b"homework";
    let mut binary = "".to_string();
    let mut itr: usize = 0;

    //* Variable for check execution time */
    binary = generate_hash(data, binary);

    println!("{}", binary); //* Print hashed data */
    println!();

    while itr < 20 {
        println!();
        let start = Instant::now();
        itr = find_collision(itr);
        let end = start.elapsed();
        println!("Time Elapsed: {:?}", end);
        println!();
        itr += 1;
    }
}

pub fn generate_hash(data: &[u8], mut binary: String) -> String {
    let mut hash = sha::sha256(data);
    //* Basic Function: Generate SHA256 hash, convert into binary. */
    //*Enable Double Hash  - Uncomment this*/
    hash = sha::sha256(&hash);

    for x in hash {
        binary += &format!("{:08b }", x);
    }

    // let double_hash = sha::sha256(&binary.as_bytes());

    // binary = "".to_string();

    // for x in double_hash {
    //     binary += &format!("{:08b }", x);
    // }
    return binary;
}

pub fn find_collision(num: usize) -> usize {
    let mut nonce: u32 = 0;
    let mut hash_binary: String = "".to_string();
    let mut buffer = [0u8; 8];
    let mut itr: usize = 0;
    let mut found: bool = true;

    println!(
        "CASE {}: Find collision - hash start with {} zeros in binary...",
        num + 1,
        num + 1
    );

    loop {
        //*Reset */
        hash_binary = "".to_string();
        itr = 0;
        found = true;
        let tgt_string: String = format!("homework{}", nonce);

        let mut compare: &[u8] = &mut buffer;

        compare = tgt_string.as_bytes();

        hash_binary = generate_hash(compare, hash_binary);

        while itr <= num {
            if hash_binary.as_bytes()[itr] as char != '0' {
                found = false;
                break;
            }
            itr += 1;
        }
        if found == true {
            //* Collision found: print nounce and end the loop */
            println!("\nCollision Found: Nonce -> {}", nonce);
            print!("String: {}, Hashed: {}        ", tgt_string, hash_binary);
            break;
        }
        //*Else: add nonce and repeat */
        //print!("{}", nonce);
        nonce += 1;
    }

    return num;
}

// pub fn generate_compare_string(range: Range<i32>) -> String {
//     let z_string = "00000000000000000000";
//     let compare = z_string[range].to_string();

//     println!("{}", compare);
//     compare
// }
