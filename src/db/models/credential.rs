#[derive(Debug, Clone)]
pub struct Credential {
    pub encryption_iv: Vec<u8>,
    pub encryption_key: Vec<u8>,
}

impl Credential {
    pub fn new(encryption_iv: Vec<u8>, encryption_key: Vec<u8>) -> Self {
        Credential { encryption_iv, encryption_key }
    }
    

    // output format "encryption_iv_encryption_key"
    // Example: "0430203040506070809010111213141516_2301128381234567890123456789012345"
    pub fn get_credential_string(encryption_iv: &Vec<u8>, encryption_key: &Vec<u8>) -> String {
        let mut result = String::new();

        result.push_str(&encryption_iv.iter().map(|b| format!("{:02x}", b)).collect::<String>());
        result.push('_');
        result.push_str(&encryption_key.iter().map(|b| format!("{:02x}", b)).collect::<String>());

        result
    }

    pub fn new_from_string(credential: &String) -> Credential {
        let parts: Vec<&str> = credential.split('_').collect();
        let encryption_iv = (0..32).step_by(2)
            .map(|i| u8::from_str_radix(&parts[0][i..i+2], 16).unwrap())
            .collect();
        let encryption_key = (0..32).step_by(2)
            .map(|i| u8::from_str_radix(&parts[1][i..i+2], 16).unwrap())
            .collect();

        Credential { encryption_iv, encryption_key }
    }

}