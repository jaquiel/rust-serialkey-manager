fn main() {

    use SerialKeyManager::{generate_key, sign_key, verify_key};

    // Generate RSA keys
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    
    // Generate a product key
    let product_key = generate_key();
    println!("Generated Product Key: {}", product_key);
    
    // Sign the product key
    let signature = sign_key(&private_key, &product_key);
    println!("Signature: {}", signature);
    
    // Verify the product key
    let is_valid = verify_key(&public_key, &product_key, &signature);
    println!("Is the product key valid? {}", is_valid);
    
    // Test with an invalid key
    let invalid_key = "ABCDE-FGHIJ-KLMNO-PQRST";
    let is_invalid_key_valid = verify_key(&public_key, invalid_key, &signature);
    println!("Is the invalid product key valid? {}", is_invalid_key_valid);
}
