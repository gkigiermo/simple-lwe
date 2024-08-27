mod parameters;
mod lwe;
use parameters::LweParameters;

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
    
    //Assuming a linear regression y = ax + b
    let a = 2u8;
    let b = 12u8;
    println!("Linear regression y = {}x + {} ", a, b);

    let mut ct = lwe::Lwe::new(&params);
    ct.import_from_file("data/data_test");
    // ct.encrypt(message_in_clear, &params);
    // ct.export_to_file("data_test");

    ct.print_ciphertext();


    let mut trivial_ct = lwe::Lwe::new(&params);
    trivial_ct.encrypt_trivial(b, &params);

    ct.small_scalar_mult(a);
    ct.add(&trivial_ct);
    
    trivial_ct.print_ciphertext();

    //Expected result is 2*33 + 12 = 78
    let decrypted_message = ct.decrypt(&params);
    println!("Decrypted message: {}", decrypted_message);

}
