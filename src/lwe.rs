use crate::parameters;
use parameters::LweParameters;
use rand::Rng;
use rand_distr::{Normal, Distribution};
use std::io::Write;
use std::io::Read;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Lwe{
    pub ciphertext: Vec<u128>,
    secret_key: Vec<u32>,
    lwe_size: usize,
}

impl Lwe {
    
    pub fn new(params: &LweParameters) -> Self {
        let mut lwe = Lwe {
            ciphertext: Vec::new(),
            secret_key: Vec::new(),
            lwe_size: params.k + 1,
        };
        lwe.generate_secret_key(params);
        lwe
    }

    fn generate_secret_key(&mut self, params: &LweParameters) {
        let mut rng = rand::thread_rng();
        self.secret_key = (0..params.k).map(|_| rng.gen_range(0..2)).collect();
    }
    fn generate_mask(&mut self, params: &LweParameters) {
        let mut rng = rand::thread_rng();
        self.ciphertext = (0..params.k).map(|_| rng.gen_range(0..u128::MAX)).collect();
    }
    fn generate_trivial_mask(&mut self, params: &LweParameters) {
        self.ciphertext = (0..params.k).map(|_| 0u128).collect();
    }

    pub fn encrypt(&mut self, message: u8, params: &LweParameters) {
        
        self.generate_mask(params);
        let normal = Normal::new(0.0,params.std).unwrap();
        let noise: f64 = normal.sample(&mut rand::thread_rng()) as f64;
        let scaled_noise: u128 = noise.abs().round() as u128;

        let mut body: u128 = 0u128;
        for i in 0..params.k {
            body += self.ciphertext[i]*(self.secret_key[i] as u128);
        }
        body += (params.delta as u128)*(message as u128);
        if noise > 0.0 {
            body += scaled_noise;
        } else {
            body -= scaled_noise;
        }
        self.ciphertext.push(body);
    }

    pub fn decrypt(&self, params: &LweParameters) -> u128 {
        let mut applied_mask: u128 = 0u128;
        for i in 0..params.k {
            applied_mask += self.ciphertext[i]*(self.secret_key[i] as u128);
        }

        let mut decrypted_message = self.ciphertext[params.k] - applied_mask;
        let log2_delta = (params.delta as f64).log2() as u128;
        let round_bit = (decrypted_message >> (log2_delta - 1)) & 1;
        decrypted_message = ((decrypted_message as u128)>>log2_delta) + round_bit as u128 ;
        decrypted_message
    }

    pub fn print_ciphertext(&self) {
        println!("Ciphertext:");
        for i in 0..self.lwe_size {
            print!("{} ", self.ciphertext[i]);
        }
        print!("\n");
        
        self.print_secret_key()
    }

    // Just for debugging
    fn print_secret_key(&self) {
        println!("Secret Key:");
        for i in 0..self.lwe_size - 1 {
            print!("{} ", self.secret_key[i]);
        }
        print!("\n");
    }

    pub fn encrypt_trivial(&mut self, message: u8, params: &LweParameters) {
        
        self.generate_trivial_mask(params);
        let body = (params.delta as u128)*(message as u128);
        self.ciphertext.push(body);

    }

    // Here we should push an add in the VM for each element of the ciphertext
    pub fn add(&mut self, lwe: &Lwe) {
        for i in 0..self.lwe_size {
            self.ciphertext[i] += lwe.ciphertext[i];
        }
    }

    // Here we should push a multiplication in the VM for each element of the ciphertext
    pub fn small_scalar_mult(&mut self, scalar: u8) {     
        for i in 0..self.lwe_size {
            self.ciphertext[i] *= scalar as u128;
        }  
    }
    pub fn export_ciphertext_to_file(&self, filename: &str) {
        let ct_filename = filename.to_owned() + ".ct";
        let mut ct_file = std::fs::File::create(ct_filename).unwrap();
        for i in 0..self.lwe_size {
            ct_file.write_all(self.ciphertext[i].to_le_bytes().as_ref()).unwrap();
        }
    }

    pub fn export_to_file(&self, filename: &str) {
        self.export_ciphertext_to_file(filename);

        let sk_filename = filename.to_owned() + ".sk";
        let mut ct_file = std::fs::File::create(sk_filename).unwrap();
        for i in 0..self.lwe_size - 1 {
            ct_file.write_all(self.secret_key[i].to_le_bytes().as_ref()).unwrap();
        }
    }
    
    pub fn import_ciphertext_from_file(&mut self, filename: &str) {
        let ct_filename = filename.to_owned() + ".ct";
        let mut ct_file = std::fs::File::open(ct_filename).unwrap();
        for _i in 0..self.lwe_size {
            let mut buffer = [0u8; 16];
            ct_file.read_exact(&mut buffer).unwrap();
            self.ciphertext.push(u128::from_le_bytes(buffer));
        }
    }

    pub fn import_secret_key_from_file(&mut self, filename: &str) {
        let sk_filename = filename.to_owned() + ".sk";
        let mut ct_file = std::fs::File::open(sk_filename).unwrap();
        self.secret_key.clear();
        for _i in 0..self.lwe_size - 1 {
            let mut buffer = [0u8; 4];
            ct_file.read_exact(&mut buffer).unwrap();
            self.secret_key.push(u32::from_le_bytes(buffer));
        }
    }

    pub fn import_from_file(&mut self, filename: &str) {
        self.import_ciphertext_from_file(filename);
        self.import_secret_key_from_file(filename);
    }


}


