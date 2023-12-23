use std::str::FromStr;

use crypto::{digest::Digest, sha2::Sha256};

pub fn generate_salt() -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::with_capacity(32);
    for _ in 0..vec.capacity() {
        vec.push(rand::random());
    }

    vec
}

pub fn hash_sha256(data: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();

    hasher.input(data);

    let mut res = vec![0; 32];
    hasher.result(&mut res);

    res
}

pub fn check_leading_bits_equal_zero(vec: &Vec<u8>, mut n: u32) -> bool {
    for b in vec {
        let lz = b.leading_zeros();
        if lz < 8 && lz < n {
            return false;
        }
        if n <= 8 {
            break;
        }
        n -= 8;
    }

    true
}

#[derive(Debug)]
pub struct Challenge {
    pub required_sha256_leading_zeros: u32,
    // always has length of 32
    pub salt: Vec<u8>,
}
impl ToString for Challenge {
    fn to_string(&self) -> String {
        let mut s = hex::encode(&self.required_sha256_leading_zeros.to_be_bytes());
        s.push_str(&hex::encode(&self.salt));

        s
    }
}
impl FromStr for Challenge {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Ok(b) = hex::decode(s) else {
            return Err(());
        };
        if b.len() != 36 {
            return Err(());
        }

        Ok(Self {
            required_sha256_leading_zeros: u32::from_be_bytes(b[..4].try_into().unwrap()),
            salt: b[4..].to_owned(),
        })
    }
}

#[derive(Debug)]
pub struct ChallengeResponse {
    pub nonce: u64,
    pub msg: String,
}
impl ChallengeResponse {
    /// returns true on success
    pub fn verify(&self, challenge: &Challenge) -> bool {
        let mut v = challenge.salt.clone();
        v.append(&mut self.nonce.to_be_bytes().to_vec());

        let h = hash_sha256(&v);
        if check_leading_bits_equal_zero(&h, challenge.required_sha256_leading_zeros) {
            println!("Good sha256 hash: {}", hex::encode(&h));
            true
        } else {
            false
        }
    }
}
impl ToString for ChallengeResponse {
    fn to_string(&self) -> String {
        let mut s = hex::encode(&self.nonce.to_be_bytes());
        s.push_str(&hex::encode(&self.msg));

        s
    }
}
impl FromStr for ChallengeResponse {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Ok(b) = hex::decode(s) else {
            return Err(());
        };

        Ok(Self {
            nonce: u64::from_be_bytes(b[..8].try_into().unwrap()),
            msg: String::from_utf8(b[8..].to_vec()).unwrap()
        })
    }
}
