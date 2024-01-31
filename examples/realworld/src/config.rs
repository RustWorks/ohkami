use ohkami::utils::{Deserialize, Serialize, unix_timestamp};
use uuid::Uuid;
use crate::errors::RealWorldError;


#[allow(non_snake_case)]
pub fn JWT_SECRET_KEY() -> Result<String, RealWorldError> {
    std::env::var("JWT_SECRET_KEY")
        .map_err(|e| RealWorldError::Config(e.to_string()))
}

#[allow(non_snake_case)]
pub fn PEPPER() -> Result<String, RealWorldError> {
    std::env::var("PEPPER")
        .map_err(|e| RealWorldError::Config(e.to_string()))
}

#[derive(Serialize, Deserialize)]
pub struct JWTPayload {
    pub iat:     u64,
    pub user_id: Uuid,
}

pub fn issue_jwt_for_user_of_id(user_id: Uuid) -> Result<String, RealWorldError> {
    let secret = JWT_SECRET_KEY()?;
    Ok(ohkami::utils::JWT(secret).clone().issue(JWTPayload {
        user_id,
        iat: unix_timestamp(),
    }))
}
