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
    
    //Assuming a linear regression y = ax + b
    let a = 2u8;
    let b = 12u8;
    println!("Linear regression y = {}x + {} ", a, b);

    let mut ct = Lwe::new(&params);
    //To import data from a file
    ct.import_ciphertext_from_file("data/encrypted_data");    
    ct.print_ciphertext();


    let mut trivial_ct = Lwe::new(&params);
    trivial_ct.encrypt_trivial(b, &params);

    ct.small_scalar_mult(a);
    ct.add(&trivial_ct);


    ct.export_ciphertext_to_file("server_result");

}
