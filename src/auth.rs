use tonic::metadata::MetadataMap;
use jsonwebtoken::{decode, DecodingKey, Algorithm, Validation};

/// JWT claims.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub r#type: String,
    pub exp: usize,
}

pub fn extract_claims(metadata: &MetadataMap) -> Result<Claims, String> {
    let secret = std::env::var("JWT_SECRET").expect("Cant find JWT_SECRET variable");

    if let Some(jwt) = metadata.get("authorization") {
        return match decode::<Claims>(
            jwt.to_str().unwrap(),
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(tokendata) => Ok(tokendata.claims),
            Err(error) => {
                Err(error.to_string())
            }
        }
    } else {
        Err(String::from("No valid auth token"))
    }
}
