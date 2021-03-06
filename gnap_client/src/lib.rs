use model::grant::*;
use uuid::Uuid;
pub mod gnap_session;

fn generate_nonce() -> String {
    Uuid::new_v4().to_simple().to_string()
}

pub fn make_request() -> GrantRequest {
    let client_id = "7e057b0c-17e8-4ab4-9260-2b33f32b2cce".to_owned();
    //let ac_foo = AccessRequest::Reference("foo".to_owned());
    let ac_ref = AccessRequest::Value {
        resource_type: "waterbowl-access".to_owned(),
        actions: Some(vec!["read".to_owned(), "create".to_owned()]),
        locations: Some(vec!["https://localhost:8080/bowls/".to_owned()]),
        data_types: None,
    };

    let at = AccessTokenRequest {
        label: Some("bowls".to_owned()),
        access: vec![ac_ref],
        flags: Some(vec![AccessTokenFlag::Bearer]),
    };

    let access_tokens = vec![at];

    let interact = InteractRequest {
        start: vec![InteractStartMode::Redirect],
        finish: Some(InteractFinishRequest {
            method: InteractFinishMethodType::Redirect,
            uri: "localhost:8000/gnap/auth".to_owned(),
            nonce: generate_nonce(),
        }),
    };

    GrantRequest {
        access_token: access_tokens,
        subject: None,
        client: Some(GnapClientInstance::Ref(client_id)),
        user: None,
        interact: Some(interact),
    }
}
