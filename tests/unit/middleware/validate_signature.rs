use {
    echo_server::middleware::validate_signature::signature_is_valid,
    ed25519_dalek::{Signer, SigningKey, VerifyingKey},
    rand::rngs::OsRng,
};

/// Setup for tests by creating a public key and returning a signature,
/// timestamp and body
fn setup() -> (VerifyingKey, String, String, String) {
    let mut csprng = OsRng {};
    let keypair: SigningKey = SigningKey::generate(&mut csprng);

    let body = "example_body";
    let timestamp = "1692442800";

    let sig_body = format!("{}.{}.{}", timestamp, body.len(), body);
    let sig = keypair.sign(sig_body.as_bytes());
    let sig_hex = hex::encode(sig.to_bytes());

    (
        keypair.verifying_key(),
        sig_hex,
        timestamp.to_string(),
        body.to_string(),
    )
}

#[tokio::test]
pub async fn valid_signature() {
    let (pub_key, signature, timestamp, body) = setup();

    let res = signature_is_valid(&signature, &timestamp, &body, &pub_key).await;

    // Shouldn't error
    assert!(res.is_ok());

    // Should be valid
    assert!(res.expect("failed to extract result"))
}

#[tokio::test]
pub async fn invalid_signature_not_hex() {
    let (pub_key, _, timestamp, body) = setup();

    let res = signature_is_valid("bad-signature", &timestamp, &body, &pub_key).await;

    // Should error
    assert!(res.is_err());

    let error = res.err().expect("Couldn't unwrap error");
    assert!(error.is_hex());
}

#[tokio::test]
pub async fn invalid_signature_hex() {
    let (pub_key, _, timestamp, body) = setup();

    let res = signature_is_valid(
        // Sig Decoded: invalid-signature
        "696e76616c69642d7369676e6174757265",
        &timestamp,
        &body,
        &pub_key,
    )
    .await;

    // Should error
    assert!(res.is_err());

    let error = res.err().expect("Couldn't unwrap error");
    // Note: should be a from slice error as the signature
    assert!(error.is_ed_25519());
}
