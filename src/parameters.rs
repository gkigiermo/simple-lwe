use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LweParameters {
    pub plaintext_modulus: u32,
    pub ciphertext_modulus: u32,
    pub delta: u32,
    pub k: usize,
    pub std: f64,
}
impl LweParameters {
    pub fn new(plaintext_modulus: u32, ciphertext_modulus: u32, k: usize, std: f64) -> Self {
        LweParameters {
            plaintext_modulus,
            ciphertext_modulus,
            delta: (ciphertext_modulus / plaintext_modulus),
            k,
            std,
        }
    }

    pub fn print(&self) {
        println!("Plaintext modulus {}", self.plaintext_modulus);
        println!("Ciphertext modulus {}", self.ciphertext_modulus);
        println!("Delta {}", self.delta);
        println!("K {}", self.k);
        println!("Standard deviation {}", self.std);
    }
}