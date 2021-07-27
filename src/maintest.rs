#[cfg(test)]
mod test {
    use crate::encrypt_decrypt;
    use crate::operationtype::*;
    use crate::playfairconfiguration::*;

    #[test]
    fn when_encrypt_data_then_retuns_expected_cipher_text() {
        let key = "DEF".to_string();
        let plain_text = "ABC";
        let expected_cipher_text = "BDKV";
        let mut config = PlayfairConfiguration::new();
        config.operation_type = OperationType::Encrypt;
        config.key = key;

        let cipher_text = encrypt_decrypt(plain_text, &config);

        assert_eq!(expected_cipher_text, cipher_text);
    }

    #[test]
    fn when_encrypt_data_in_lower_case_then_retuns_expected_cipher_text() {
        let key = "DEF".to_string();
        let plain_text = "abc";
        let expected_cipher_text = "BDKV";
        let mut config = PlayfairConfiguration::new();
        config.operation_type = OperationType::Encrypt;
        config.key = key;

        let cipher_text = encrypt_decrypt(plain_text, &config);

        assert_eq!(expected_cipher_text, cipher_text);
    }

    #[test]
    fn when_encrypt_data_in_mixed_case_then_retuns_expected_cipher_text() {
        let key = "DEF".to_string();
        let plain_text = "aBc";
        let expected_cipher_text = "BDKV";
        let mut config = PlayfairConfiguration::new();
        config.operation_type = OperationType::Encrypt;
        config.key = key;

        let cipher_text = encrypt_decrypt(plain_text, &config);

        assert_eq!(expected_cipher_text, cipher_text);
    }

    #[test]
    fn when_decrypt_data_then_retuns_expected_plain_text() {
        let key = "DEF".to_string();
        let cipher_text = "BDKV";
        let expected_plain_text = "ABCZ";
        let mut config = PlayfairConfiguration::new();
        config.operation_type = OperationType::Decrypt;
        config.key = key;

        let plain_text = encrypt_decrypt(cipher_text, &config);

        assert_eq!(expected_plain_text, plain_text);
    }

    #[test]
    fn when_decrypt_data_from_crypto_book_then_retuns_expected_plain_text() {
        let key = "UNIVERSITY OF LONDON".to_string();
        let cipher_text = "MBOUBTZE";
        let expected_plain_text = "BOREDOMZ";
        let mut config = PlayfairConfiguration::new();
        config.operation_type = OperationType::Decrypt;
        config.key = key;

        let plain_text = encrypt_decrypt(cipher_text, &config);

        assert_eq!(expected_plain_text, plain_text);
    }
}
