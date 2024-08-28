extern crate simple_lwe;
use simple_lwe::lwe::Lwe;
use simple_lwe::parameters::LweParameters;

fn main() { 
    // This are over simplified parameters probably not very secure   
    let plaintext_modulus: u32 = 8u32;      // p
    let ciphertext_modulus: u32 = 128;   // q
    let k : usize = 4; // This is the number of mask elements
    let std = 2.412390240121573e-05; 
    let params = LweParameters::new(plaintext_modulus, ciphertext_modulus, k, std);
    params.print();

    let message_in_clear = 33u8;
    println!("\nMessage in clear: {}", message_in_clear);
    
    let mut ct = Lwe::new(&params);
    ct.encrypt(message_in_clear, &params);

    //Store a file with the data encrypted and a file with the secret key
    ct.export_to_file("encrypted_data");

    ct.print_ciphertext();
}
