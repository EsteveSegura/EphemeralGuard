#[derive(Debug, Clone)]
pub struct Credential {
    pub encryption_iv: Vec<u8>,
    pub encryption_key: Vec<u8>,
}

impl Credential {
    pub fn new(encryption_iv: Vec<u8>, encryption_key: Vec<u8>) -> Self {
        Credential { encryption_iv, encryption_key }
    }
    

    pub fn get_credential_string(encryption_iv: &Vec<u8>, encryption_key: &Vec<u8>) -> String {
        let mut result = String::new();

        result.push_str(&encryption_iv.iter().map(|b| format!("{:02x}", b)).collect::<String>());
        result.push('_');
        result.push_str(&encryption_key.iter().map(|b| format!("{:02x}", b)).collect::<String>());

        result
    }

    pub fn new_from_string(credential: &String) -> Result<Credential, String> {
        let parts: Vec<&str> = credential.split('_').collect();
        
        if parts.len() != 2 {
            return Err(format!("Credential format is incorrect: expected two parts separated by '_', got {}", parts.len()));
        }

        if parts[0].len() < 32 || parts[1].len() < 32 {
            return Err(format!("Each part of the credential must be at least 32 characters long. Got lengths {} and {}", parts[0].len(), parts[1].len()));
        }

        let encryption_iv: Vec<u8> = (0..32).step_by(2)
            .map(|i| u8::from_str_radix(&parts[0][i..i+2], 16))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to parse encryption_iv: {}", e))?;

        let encryption_key: Vec<u8> = (0..32).step_by(2)
            .map(|i| u8::from_str_radix(&parts[1][i..i+2], 16))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to parse encryption_key: {}", e))?;

        Ok(Credential { encryption_iv, encryption_key })
    }

}