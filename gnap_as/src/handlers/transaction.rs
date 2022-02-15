//! Transaction API Handlers
use crate::grant::request::{process_request, process_continue_request};
use actix_web::{web, HttpResponse};
use dao::service::Service;
use log::{error, trace, debug};
use model::grant::{GrantRequest, ContinuationRequest};

/// HTTP OPTIONS <as>/gnap/tx
pub async fn grant_options(service: web::Data<Service>) -> HttpResponse {
    trace!("grant_options");
    match service.get_grant_options().await {
        Ok(data) => {
            trace!("Retrieved grant options: {:?}", data);
            HttpResponse::Ok().json(data)
        }
        Err(err) => {
            error!("{:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

/// Initiate a grant transaction
/// HTTP POST <as>/gnap/tx
pub async fn grant_request(
    service: web::Data<Service>,
    request: web::Json<GrantRequest>,
) -> HttpResponse {
    // Create a response from the request
    let result = process_request(&service, request.into_inner()).await;
    match result {
        Ok(data) => {
            trace!("processed grant request: {:?}", data);
            HttpResponse::Ok().json(data)
        }
        Err(err) => {
            error!("{:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

/// Continue a grant transaction
/// HTTP POST <as>/gnap/tx/:id
pub async fn continue_request(
    service: web::Data<Service>,
    request: web::Json<ContinuationRequest>,
    tx_id: web::Path<String>
) -> HttpResponse {
    
    let hash = request.into_inner();
    if hash.interact_ref.eq(&tx_id.clone().into_inner()) {
        debug!("Valid hash");
        // This hash should be validate after being computed at the client
    }

    let result = process_continue_request(&service, tx_id.into_inner()).await;
    match result {
        Ok(data) => {
            trace!("processed grant request: {:?}", data);
            HttpResponse::Ok().json(data)
        }
        Err(err) => {
            error!("{:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use model::grant::GrantRequest;
    use serde_json;
    #[test]
    fn happy_test() {
        let re = r#"
        {
            "access_token":
                {
                    "access": ["foo", {"type": "bar", "actions":["read","write"]}],
                    "label": "my_label",
                    "flags": ["bearer", "split"]
                }

        }
        "#;
        let gr: GrantRequest = serde_json::from_str(re).expect("Failed!!");
        println!("GrantRequest: {:?}", &gr);
        assert!(true);
    }
}
