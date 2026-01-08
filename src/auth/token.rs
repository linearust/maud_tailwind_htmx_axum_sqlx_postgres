use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

const TOKEN_LENGTH: usize = 32;

pub fn generate_token() -> String {
    use rand::RngCore;
    let mut rng = rand::rng();
    let mut random_bytes = vec![0u8; TOKEN_LENGTH];
    rng.fill_bytes(&mut random_bytes);
    URL_SAFE_NO_PAD.encode(&random_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_token_length() {
        let token = generate_token();
        assert!(token.len() > 40);
    }

    #[test]
    fn test_generate_token_uniqueness() {
        let token1 = generate_token();
        let token2 = generate_token();
        assert_ne!(token1, token2);
    }
}
