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

    let mut ct = Lwe::new(&params);
    //To import data from a file
    ct.import_ciphertext_from_file("data/server_result");
    ct.import_secret_key_from_file("data/encrypted_data");
    
    ct.print_ciphertext();

    //If the data is generated from the examples the expected result is 2*33 + 12 = 78 
    let decrypted_message = ct.decrypt(&params);
    println!("Decrypted message: {}", decrypted_message);

}
