use std::{env, io::stdin, str::FromStr};

use pow_proof::{Challenge, ChallengeResponse};

fn main() {
    let args: Vec<_> = env::args().collect();
    let stdin = stdin();
    let mut buffer = String::new();

    let Some(command) = args.get(1) else {
        println!("available commands: solve");
        return
    };

    match command.as_str() {
        "solve" => {
            println!("enter the challenge string");
            stdin.read_line(&mut buffer).unwrap();
            buffer.pop();
            let challenge = Challenge::from_str(&buffer).unwrap();

            buffer.clear();

            println!("enter a message");
            stdin.read_line(&mut buffer).unwrap();
            buffer.pop();
            let msg = buffer.to_owned();

            println!("Doing proof of work... need to find {} leading zeros in sha256 hash", challenge.required_sha256_leading_zeros);

            let mut nonce = 0;
            let challenge_response = loop {
                let cr = ChallengeResponse {
                    nonce,
                    msg: msg.clone(),
                };

                if cr.verify(&challenge) {
                    println!("Found solution at nonce {nonce}");

                    break cr;
                }

                nonce += 1;
            };
            
            println!("Challenge response string: {}", challenge_response.to_string());
        }

        _ => println!("unknown command. trying running with no arguements to see available commands")
    }
}