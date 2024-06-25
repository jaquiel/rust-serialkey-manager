use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
use rand::Rng;
use base64::{encode, decode};
use sha2::{Sha256, Digest};
use std::str;

pub struct SerialKeyManager;

pub fn generate_key() -> String {    
    let rng = rand::thread_rng();
    let segments: Vec<String> = (0..4)
        .map(|_| {
            rng.clone().sample_iter(&rand::distributions::Alphanumeric)
                .take(5)
                .map(char::from)
                .collect()
        })
        .collect();
    
    segments.join("-")
            .to_uppercase()
}

pub fn sign_key(private_key: &RsaPrivateKey, product_key: &str) -> String {
    let padding = PaddingScheme::PKCS1v15Sign {
        hash: Some(rsa::Hash::SHA2_256),
    };
    let mut hasher = Sha256::new();
    hasher.update(product_key.as_bytes());
    let hashed = hasher.finalize();
    let signature = private_key.sign(padding, &hashed).expect("failed to sign");
    encode(signature)
}

pub fn verify_key(public_key: &RsaPublicKey, product_key: &str, signature: &str) -> bool {
    let padding = PaddingScheme::PKCS1v15Sign {
        hash: Some(rsa::Hash::SHA2_256),
    };
    let mut hasher = Sha256::new();
    hasher.update(product_key.as_bytes());
    let hashed = hasher.finalize();
    let decoded_signature = decode(signature).expect("failed to decode base64 signature");
    public_key.verify(padding, &hashed, &decoded_signature).is_ok()
}

#[cfg(test)]
mod tests {
    use rand::rngs::OsRng;
    use super::*;

    #[test]
    fn test_generate_key() {
        let key = generate_key();
        println!("Generated Key: {}", key);
        assert_eq!(key.len(), 23); // 4 segments of 5 chars each + 3 hyphens
        for segment in key.split('-') {
            assert_eq!(segment.len(), 5);
        }
    }

    #[test]
    fn test_sign_and_verify_key() {
        let mut rng = OsRng;
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate private key");
        let public_key = RsaPublicKey::from(&private_key);
        
        let product_key = generate_key();
        let signature = sign_key(&private_key, &product_key);

        assert!(verify_key(&public_key, &product_key, &signature));

        // Test with an incorrect product key
        let wrong_product_key = generate_key();
        assert!(!verify_key(&public_key, &wrong_product_key, &signature));
        
        // Test with an incorrect signature
        let wrong_signature = sign_key(&private_key, &wrong_product_key);
        assert!(!verify_key(&public_key, &product_key, &wrong_signature));
    }
}