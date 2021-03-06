use dao::service::Service;
use errors::GnapError;
use log::{error, trace, debug};
use model::tokens::TokenBuilder;
use model::transaction::GnapTransactionState::*;
use model::{grant::*, GnapID};
pub async fn process_request(
    service: &Service,
    request: GrantRequest,
) -> Result<GrantResponse, GnapError> {
    // A valid request?
    if request.client.is_none() {
        // No client identifier
        error!("No client id in grant request");
        return Err(GnapError::BadData);
    }
    // This will fail if either there is no client_id in the request, or the
    // client_id is not a valid uuid.
    trace!("getting id from reqeust...");
    let client_id = request.parse_id()?;
    trace!("parsed id from request: {}", client_id.to_string());
    // This will fail if the client_id provided in the request is not found.
    let _client = service.get_client(&client_id).await?.unwrap();

    // At this point, we have determined that the request contains a valid client_id
    // and the client data was found.  Now we can compare request data against
    // the authorized client.

    // Verify the request data against client config, etc.

    // Start a transaction
    let tx = service.start_transaction(request.clone()).await?;

    let uri = format!("http://localhost:8000/gnap/tx/{}", &tx.tx_id);
    let rc = RequestContinuation::as_uri(&uri);
    let mut interact_response = InteractResponse {
        tx_continue: rc,
        redirect: None,
    };

    // What are the interaction methods?
    for method in request.interact.unwrap().start.iter() {
        match method {
            InteractStartMode::Redirect => {
                trace!("GrantRequest interaction contains Redirect");
                interact_response.redirect = Some(uri.clone());
            }
            InteractStartMode::App => {
                trace!("GrantRequest interaction contains App");
            }
            InteractStartMode::UserCode => {
                trace!("GrantRequest interaction contains UserCode");
            }
        }
    }

    let response = GrantResponse {
        instance_id: tx.tx_id,
        interact: Some(interact_response),
        access_token: None,
    };

    Ok(response)
}

pub async fn process_continue_request(
    service: &Service,
    tx_id: String,
) -> Result<GrantResponse, GnapError> {
    let tx = match service.get_transaction(tx_id.clone()).await {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    match tx.state {
        Authorized => {

            // only create one token for the first access..
            // this should be able to handle multiple token.
            // if there are mutiple access_requests, then there should be generated multiple tokens.
            // and it has to have a unique label for each.
            //let t = Token::create(tx_id.clone());
            

            let mut access_tokens = Vec::<AccessToken>::new();
            let grantrequest = tx.request.clone().unwrap();
            for grant_token in grantrequest.access_token {
                let label = grant_token.label;
                debug!("{:#?}", label);
                let t = TokenBuilder::new(tx_id.clone())
                                            .label(label.clone())
                                            .build();
                let _ = service.store_token(t.clone())
                            .await
                            .expect("Failed to store token");
                let at = AccessToken {
                    label,
                    value: t.access_token.unwrap(),
                    manage: Some(format!(
                        "http://localhost:8000/gnap/token/{}",
                        &t.id.to_owned()
                    )),
                    access: Some(grant_token.access.to_owned()),
                    key: None,
                    expires_in: t.expire,
                    flags: Some(vec![AccessTokenFlag::Bearer]),
                };
               
                access_tokens.push(at)
            }
            //let tokenrequest = grantrequest.access_token.first().unwrap();
            
            let gr = GrantResponse {
                instance_id: tx.tx_id.clone(),
                interact: None,
                access_token: Some(access_tokens), // missing subject
            };
            Ok(gr)
        }
        _ => Err(GnapError::BadData),
    }
}
