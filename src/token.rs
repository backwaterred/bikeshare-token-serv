use openssl::rsa::{Rsa, Padding};
use openssl::base64;
use std::fs;
use std::time;

pub fn build(bike_id: String, interval: time::Duration) -> String {

    String::from(format!("TOK={}-{}", bike_id, interval.as_secs()))
}

pub fn encrypt(plain_token: &str) -> String {
    let k_priv = fs::read("id_rsa_pem")
        .expect("unable to read private key file");
    let k_priv = Rsa::private_key_from_pem(&k_priv)
        .expect("keylib unable to import key");

    // from the doc, this can be used to allocate space. What if the token is longer than 2048 bits?
    let mut crypt_token = vec![0; k_priv.size() as usize];
    k_priv.private_encrypt(plain_token.as_bytes(), &mut crypt_token, Padding::PKCS1)
          .unwrap();

    base64::encode_block(&crypt_token)
}

#[cfg(test)]
mod token_tests {
    use openssl::rsa::{Rsa, Padding};
    use openssl::base64;
    use std::fs;
    use std::time;

    use super::*;

    #[test]
    fn test_decryts_ok() {
        let k_pub = fs::read("id_rsa_pem.pub").unwrap();
        let k_pub = Rsa::public_key_from_pem(&k_pub).expect("unable to import key");


        let p_token = build(String::from("INVALID-TEST-BIKEID"),
                                   time::Duration::from_secs(0));

        let c_token = encrypt(&p_token);
        let c_token = base64::decode_block(&c_token).unwrap();

        let mut m_token = vec![0; k_pub.size() as usize];
        let len = k_pub.public_decrypt(&c_token, &mut m_token, Padding::PKCS1).unwrap();
        let m_token = String::from_utf8_lossy(&m_token[..len]);

        assert_eq!(p_token, m_token)
    }
}
