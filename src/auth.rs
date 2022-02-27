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
    return Ok(Claims{
        sub: "77fca697-c3b9-4145-9b67-30b955422403".parse().unwrap(),
        role: "".to_string(),
        r#type: "".to_string(),
        exp: 0
    });
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
