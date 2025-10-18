use poem::{http::StatusCode, Endpoint, EndpointExt, Error};
use poem_openapi::{ApiExtractor, ExtractParamOptions};

use crate::api::Token;

pub fn guest_forbidden(ep: impl Endpoint) -> impl Endpoint {
    ep.before(|req| async move {
        let token = Token::from_request(
            &req,
            &mut Default::default(),
            ExtractParamOptions {
                name: "",
                default_value: None,
                explode: false,
            },
        )
        .await?;
        if token.is_guest {
            return Err(Error::from_status(StatusCode::FORBIDDEN));
        }
        Ok(req)
    })
}

pub fn guest_allowed(ep: impl Endpoint) -> impl Endpoint {
    ep.before(|mut req| async move {
        // Check if X-API-Key header exists
        if req.headers().get("X-API-Key").is_none() {
            // For unauthenticated requests, create a guest token
            let guest_token = "guest_token_for_public_access"; // This is a placeholder
            req.headers_mut().insert("X-API-Key", guest_token.parse().unwrap());
        }
        Ok(req)
    })
}
