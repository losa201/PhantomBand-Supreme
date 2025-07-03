// common/src/lib.rs
pub mod crypto;
pub mod protocol;
pub mod utils;

#[cfg(test)]
mod tests {
    use super::crypto;

    #[test]
    fn test_encryption_decryption() {
        let key = crypto::generate_keypair();
        let original_data = b"Hello, PhantomBand!";

        let encrypted_data = crypto::encrypt(original_data, &key).expect("Encryption failed");
        let decrypted_data = crypto::decrypt(&encrypted_data, &key).expect("Decryption failed");

        assert_eq!(original_data.to_vec(), decrypted_data);
    }

    #[test]
    fn test_encryption_decryption_empty_data() {
        let key = crypto::generate_keypair();
        let original_data = b"";

        let encrypted_data = crypto::encrypt(original_data, &key).expect("Encryption failed");
        let decrypted_data = crypto::decrypt(&encrypted_data, &key).expect("Decryption failed");

        assert_eq!(original_data.to_vec(), decrypted_data);
    }

    #[test]
    fn test_decryption_with_wrong_key() {
        let key1 = crypto::generate_keypair();
        let key2 = crypto::generate_keypair(); // Different key
        let original_data = b"Secret message";

        let encrypted_data = crypto::encrypt(original_data, &key1).expect("Encryption failed");
        let result = crypto::decrypt(&encrypted_data, &key2);

        assert!(result.is_err());
    }
}