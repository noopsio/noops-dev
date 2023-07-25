use crate::errors::Error::{self, UserNotRegistered};
use crate::github;
use crate::jwt::Jwt;
use crate::repository::user::User;
use crate::repository::{user::UserRepository, Repository};
use dtos::GetJWTDTO;
use jsonwebtoken::{DecodingKey, EncodingKey};
use lazy_static::lazy_static;

const JWT_SECRET: &str = "ieb9upai2pooYoo9guthohchio5xie6Poo1ooThaetubahCheemaixaeZei1rah0";
const JWT_ISSUER: &str = "noops.io";
const JWT_EXPIRATION_DELTA: u64 = 3600; // 1 hour

lazy_static! {
    pub static ref ENCODING_KEY: EncodingKey = EncodingKey::from_secret(JWT_SECRET.as_bytes());
    pub static ref DECODING_KEY: DecodingKey = DecodingKey::from_secret(JWT_SECRET.as_bytes());
}

#[derive(Debug, Clone)]
pub struct AuthService {
    users: UserRepository,
}

impl AuthService {
    pub fn new(users: UserRepository) -> AuthService {
        Self { users }
    }

    pub async fn login(&self, github_access_token: String) -> Result<GetJWTDTO, Error> {
        let gh_user = github::get_user(github_access_token.clone()).await?;
        let result = self.users.read_by_gh_id(gh_user.id)?;

        let user = match result {
            Some(user) => user,
            None => {
                let user = User::new(gh_user.email, gh_user.id, github_access_token);
                self.users.create(&user)?;
                user
            }
        };

        let jwt = Jwt::create_token(
            user.id,
            JWT_ISSUER.to_string(),
            JWT_EXPIRATION_DELTA,
            &ENCODING_KEY,
        )?;
        Ok(GetJWTDTO { jwt })
    }

    pub fn authenticate(&self, jwt: &str) -> Result<User, Error> {
        let (_, claims) = Jwt::decode(jwt, JWT_ISSUER, &DECODING_KEY)?;
        let user = self.users.read(&claims.sub)?;
        if user.is_none() {
            return Err(UserNotRegistered);
        }
        let user = user.unwrap();
        Ok(user)
    }
}
