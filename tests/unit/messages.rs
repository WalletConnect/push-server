use echo_server::{
    blob::{DecryptedPayloadBlob, ENCRYPTED_FLAG},
    handlers::push_message::MessagePayload,
};

const EXAMPLE_TOPIC: &str = "example-topic";

// base64 encoded json string
const EXAMPLE_CLEARTEXT_ENCODED_BLOB: &str = "eyJ0aXRsZSI6IllvdSBoYXZlIGEgc2lnbiByZXF1ZXN0IiwiYm9keSI6ImV4YW1wbGUtZGFwcCBoYXMgc2VudCB5b3UgYSByZXF1ZXN0IHRvIHNpZ24gYSBtZXNzYWdlIn0=";

// json string
const EXAMPLE_CLEARTEXT_BLOB_TITLE: &str = "You have a sign request";
const EXAMPLE_CLEARTEXT_BLOB_BODY: &str = "example-dapp has sent you a request to sign a message";
const EXAMPLE_CLEARTEXT_BLOB: &str = "{\"title\":\"You have a sign \
                                      request\",\"body\":\"example-dapp has sent you a request to \
                                      sign a message\"}";

// This can be any text as echo-server doesn't mutate or read it
const EXAMPLE_ENCRYPTED_BLOB: &str = "encrypted-blob";

#[test]
pub fn check_payload_encrypted() {
    let payload = MessagePayload {
        topic: Some(EXAMPLE_TOPIC.to_string()),
        flags: ENCRYPTED_FLAG,
        blob: EXAMPLE_ENCRYPTED_BLOB.to_string(),
    };

    assert!(payload.is_encrypted())
}

#[test]
pub fn check_payload_not_encrypted() {
    let payload = MessagePayload {
        topic: None,
        flags: 0,
        blob: EXAMPLE_CLEARTEXT_ENCODED_BLOB.to_string(),
    };

    assert_eq!(payload.is_encrypted(), false)
}

#[test]
pub fn parse_blob_from_payload() {
    let payload = MessagePayload {
        topic: None,
        flags: 0,
        blob: EXAMPLE_CLEARTEXT_ENCODED_BLOB.to_string(),
    };

    let blob = DecryptedPayloadBlob::from_base64_encoded(payload.blob)
        .expect("Failed to parse payload's blob");

    assert_eq!(blob, DecryptedPayloadBlob {
        title: EXAMPLE_CLEARTEXT_BLOB_TITLE.to_string(),
        body: EXAMPLE_CLEARTEXT_BLOB_BODY.to_string(),
        image: None,
        url: None
    })
}

#[test]
pub fn parse_encoded_blob() {
    let blob =
        DecryptedPayloadBlob::from_base64_encoded(EXAMPLE_CLEARTEXT_ENCODED_BLOB.to_string())
            .expect("Failed to parse encoded blob");

    assert_eq!(blob, DecryptedPayloadBlob {
        title: EXAMPLE_CLEARTEXT_BLOB_TITLE.to_string(),
        body: EXAMPLE_CLEARTEXT_BLOB_BODY.to_string(),
        image: None,
        url: None
    })
}

#[test]
pub fn parse_blob() {
    let blob = DecryptedPayloadBlob::from_json_string(EXAMPLE_CLEARTEXT_BLOB.to_string())
        .expect("Failed to parse blob");

    assert_eq!(blob, DecryptedPayloadBlob {
        title: EXAMPLE_CLEARTEXT_BLOB_TITLE.to_string(),
        body: EXAMPLE_CLEARTEXT_BLOB_BODY.to_string(),
        image: None,
        url: None
    })
}
