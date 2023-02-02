use ed25519_dalek::{PublicKey, Signature, Verifier};

pub fn verify(signature: &String, timestamp: &String, body: &String, public_key: String) -> bool {
    let public_key = &hex::decode(public_key)
        .and_then(|bytes| Ok(PublicKey::from_bytes(&bytes)))
        .unwrap()
        .unwrap();

    let sig = &hex::decode(&signature)
        .and_then(|byte| Ok(Signature::from_bytes(&byte)))
        .unwrap()
        .unwrap();

    public_key
        .verify(format!("{}{}", timestamp, body).as_bytes(), sig)
        .is_ok()
}
