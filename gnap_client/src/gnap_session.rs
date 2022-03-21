use model::gnap::GnapOptions;

pub struct GnapSession {
    pub access_token: Option<String>,
    pub tx_contiune: Option<String>,
    pub instance_id: Option<String>,
    pub redirect: Option<String>,
    pub nonce: Option<String>,
    pub options: Option<GnapOptions>,
}

impl GnapSession {
    pub fn new() -> Self {
        Self {
            access_token: None,
            tx_contiune: None,
            instance_id: None,
            redirect: None,
            nonce: None,
            options: None,
        }
    }

    pub fn new_with_options(options: GnapOptions) -> Self {
        Self {
            access_token: None,
            tx_contiune: None,
            instance_id: None,
            redirect: None,
            nonce: None,
            options: Some(options),
        }
    }
}

impl Default for GnapSession {
    fn default() -> Self {
        Self::new()
    }
}