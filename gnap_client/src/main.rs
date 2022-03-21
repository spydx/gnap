use dotenv::dotenv;
use gnap_client::make_request;
use log::trace;
use model::gnap::GnapOptions;
use model::grant::*;
use std::error::Error as StdError;
use gnap_client::gnap_session::GnapSession;
use std::io;
use model::instances::{InstanceRequest, InstanceResponse};

const GNAP_AS_HOST: &str = "http://localhost:8000";

fn as_path(part: &str) -> String {
    format!("{}/{}", GNAP_AS_HOST, part)
}


async fn get_config() -> Result<GnapOptions, Box<dyn StdError>> {
    let path = as_path(".well-known/gnap-as-rs");
    println!("Using path: {}", &path);
    let response: GnapOptions = reqwest::Client::new()
        .get(&path)
        .send()
        .await?
        .json()
        .await?;
    println!("Options: {:#?}", &response);
    Ok(response)
}

/// Using the tokio runtime via actix_web.
/// When we extend the client with a user agent (browser), it will be an easy
/// extension.
#[actix_web::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    dotenv().ok();
    pretty_env_logger::init();
    
    // Get the GNAP well knowns from the server
    let options = get_config().await?;

    let mut gnap_session = GnapSession::new_with_options(options.clone());

    // 2.
    let request = make_request();
    let session = request.interact.clone().unwrap().finish.unwrap();
    gnap_session.redirect = Some(session.uri.clone());
    gnap_session.nonce = Some(session.nonce.clone());

    println!("Request: {:#?}", &request);
    trace!(
        "Using {}",
        &options.service_endpoints.grant_request_endpoint
    );
    // 3.
    let step3: GrantResponse = reqwest::Client::new()
        .post(options.service_endpoints.grant_request_endpoint)
        .json(&request)
        .send()
        .await?
        .json()
        .await?;

    // server http response
    println!("\nResponse: {:#?}", step3);

    gnap_session.instance_id = Some(step3.instance_id);
    gnap_session.tx_contiune = Some(step3.interact.unwrap().tx_continue.uri);
    
    //let (username, password ) = get_user_input().expect("Failed to get user input");

    let (username, password) = ("kenneth", "password");

    let secret = base64::encode(format!("{}:{}", username, password));
 
    let instance = InstanceRequest::create(gnap_session.instance_id.clone().unwrap());
    
    let step4: InstanceResponse = reqwest::Client::new()
        .post(format!("http://{}",&gnap_session.redirect.unwrap()))
        .header("Content-type", "application/x-www-form-urlencoded")
        .header("Authorization", "Basic ".to_owned() + &secret)
        .json(&instance)
        .send()
        .await?
        .json()
        .await?;

    println!("Response: {:#?}", step4);
    
    let continue_request = ContinuationRequest::create_with_ref(gnap_session.instance_id.clone().unwrap());
    let target = gnap_session.tx_contiune.unwrap().to_string();
    println!("{}", target);
    let step8: GrantResponse = reqwest::Client::new()
        .post(target)
        .json(&continue_request)
        .send()
        .await?
        .json()
        .await?;

    println!("Response: {:#?}", step8);

    Ok(())
}


#[allow(dead_code)]
fn get_user_input() -> Result<(String, String), Box<dyn StdError>> {
    let mut username = String::new();
    let mut password = String::new();

    println!("Username: ");
    io::stdin().read_line(&mut username)?;
    println!("Password: ");
    io::stdin().read_line(&mut password)?;

    Ok((username.trim_end().to_string(), password.trim_end().to_string()))
}
/*

1.  The client instance establishes a verifiable session to the user, in the role of the end-user.

2.  The client instance requests access to the resource (Section 2).
    The client instance indicates that it can redirect to an arbitrary URL (Section 2.5.1.1)
    and receive a redirect from the browser (Section 2.5.2.1).
    The client instance stores verification information for its redirect in the session created in (1).

3.  The AS determines that interaction is needed and responds (Section 3)
    with a URL to send the user to (Section 3.3.1) and information needed to verify the redirect
    (Section 3.3.4) in (7).
    The AS also includes information the client instance will need to continue
    the request (Section 3.1) in (8).
    The AS associates this continuation information with
    an ongoing request that will be referenced in (4), (6), and (8).

4.  The client instance stores the verification and continuation information from (3)
    in the session from (1). The client instance then redirects the user to the URL (Section 4.1.1)
    given by the AS in (3).
    The user's browser loads the interaction redirect URL.
    The AS loads the pending request based on the incoming URL generated in (3).

5.  The user authenticates at the AS, taking on the role of the RO.

6.  As the RO, the user authorizes the pending request from the client instance.

7.  When the AS is done interacting with the user,
    the AS redirects the user back (Section 4.2.1) to the client instance
    using the redirect URL provided in (2).
    The redirect URL is augmented with an interaction reference that
    the AS associates with the ongoing request created in (2) and referenced in (4).
    The redirect URL is also augmented with a hash of the security information
    provided in (2) and (3).
    The client instance loads the verification information from (2) and (3)
    from the session created in (1).
    The client instance calculates a hash (Section 4.2.3)
    based on this information and continues only if the hash validates.
    Note that the client instance needs to ensure that the parameters for the incoming request
    match those that it is expecting from the session created in (1).
    The client instance also needs to be prepared for the end-user never being returned
    to the client instance and handle timeouts appropriately.

8.  The client instance loads the continuation information from (3)
    and sends the interaction reference from (7) in a request to continue the request (Section 5.1).
    The AS validates the interaction reference ensuring that
    the reference is associated with the request being continued.

9.  If the request has been authorized, the AS grants access to the information in
    the form of access tokens (Section 3.2) and direct subject information (Section 3.4)
    to the client instance.

10. The client instance uses the access token (Section 7.2) to call the RS.
11. The RS validates the access token and returns an appropriate response for the API.
*/
