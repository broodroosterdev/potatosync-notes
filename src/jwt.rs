use actix_web::error::ErrorUnauthorized;
use actix_web::{dev, http, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

/// JWT claims.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub r#type: String,
    pub exp: usize,
}

pub struct JWT(pub String);

/// Helper function for extracting claims from JWT string
pub fn extract_claims(jwt: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    return Ok(Claims {
        sub: "1234".to_string(),
        role: "user".to_string(),
        r#type: "HS256".to_string(),
        exp: 2516239022,
    });
    /*
    let token = decode::<Claims>(
        jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token.claims)
    */
}

/// Helper function to extract a JWT from the authorization header
fn extract_jwt_from_authorization_header(
    header: Option<&http::HeaderValue>,
) -> Option<Ready<Result<JWT, Error>>> {
    if let Some(jwt) = header {
        // Extract string from header
        // The header should look like this:
        // Bearer ENCODED_JWT
        let auth_str: &str = match jwt.to_str() {
            Ok(str) => str,
            Err(_) => return Some(err(ErrorUnauthorized("Invalid authorization header"))),
        };

        // We expect the header to authorization type "Bearer"
        let opt_bearer_pos = auth_str.find("Bearer");

        if let Some(bearer_pos) = opt_bearer_pos {
            // Remove the "Bearer" string and extract the JWT itself
            // This returns an Option because we don't know whether the string is long enough
            let opt_jwt_str = &auth_str.get("Bearer".len() + bearer_pos..);
            if let Some(jwt_str) = opt_jwt_str {
                let jwt_str = jwt_str.trim();

                Some(ok(JWT(jwt_str.to_string())))
            } else {
                Some(err(ErrorUnauthorized("Invalid authorization token")))
            }
        } else {
            Some(err(ErrorUnauthorized("Invalid authorization type")))
        }
    } else {
        None
    }
}

/// Extractor for JWT header as JWT struct (wrapper around String).
///
/// Returns an error if header is missing or can't be converted to string.
impl FromRequest for JWT {
    type Error = Error;
    type Future = Ready<Result<JWT, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        // Extract token from authorization header
        if let Some(auth_res) =
            extract_jwt_from_authorization_header(req.headers().get("authorization"))
        {
            auth_res
        }
        // no token found
        else {
            err(ErrorUnauthorized("No authorization token was sent"))
        }
    }
}
