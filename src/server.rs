use std::{env, io::stdin, str::FromStr};

use pow_proof::{generate_salt, Challenge, ChallengeResponse};

fn main() {
    let args: Vec<_> = env::args().collect();
    let stdin = stdin();
    let mut buffer = String::new();

    let Some(command) = args.get(1) else {
        println!("available commands: create_challenge, verify");
        return
    };

    match command.as_str() {
        "create_challenge" => {
            println!("enter number of leading zeros required in sha256 hash");         
            stdin.read_line(&mut buffer).unwrap();
            buffer.pop();
            let lz = buffer.parse().unwrap();

            let c = Challenge {
                required_sha256_leading_zeros: lz,
                salt: generate_salt()
            };

            println!("challenge: {}", c.to_string());
        }

        "verify" => {
            println!("enter the challenge string");
            stdin.read_line(&mut buffer).unwrap();
            buffer.pop();
            let challenge = Challenge::from_str(&buffer).unwrap();

            buffer.clear();

            println!("enter the challenge response string");
            stdin.read_line(&mut buffer).unwrap();
            buffer.pop();
            let challenge_response = ChallengeResponse::from_str(&buffer).unwrap();

            if !challenge_response.verify(&challenge) {
                println!("Verification failed!!!")
            } else {
                println!("Verification was a success! Message: {}", challenge_response.msg)
            }
        }

        _ => println!("unknown command. trying running with no arguements to see available commands")
    }
}