use crate::errors::Error::{self, UserNotRegistered};
use crate::github::GithubClient;
use crate::jwt::Jwt;
use crate::repository::user::User;
use crate::repository::{user::UserRepository, Repository};
use common::dtos::GetJWTDTO;
use jsonwebtoken::{DecodingKey, EncodingKey};
use lazy_static::lazy_static;

const JWT_SECRET: &str = "ieb9upai2pooYoo9guthohchio5xie6Poo1ooThaetubahCheemaixaeZei1rah0";
const JWT_ISSUER: &str = "noops.io";
const JWT_EXPIRATION_DELTA: u64 = 86400; // 24 hours

lazy_static! {
    pub static ref ENCODING_KEY: EncodingKey = EncodingKey::from_secret(JWT_SECRET.as_bytes());
    pub static ref DECODING_KEY: DecodingKey = DecodingKey::from_secret(JWT_SECRET.as_bytes());
}

#[derive(Debug, Clone)]
pub struct AuthService {
    github_client: GithubClient,
    users: UserRepository,
}

impl AuthService {
    pub fn new(github_client: GithubClient, users: UserRepository) -> AuthService {
        Self {
            github_client,
            users,
        }
    }

    pub async fn login(&self, github_access_token: String) -> Result<GetJWTDTO, Error> {
        let gh_user = self
            .github_client
            .get_user(github_access_token.clone())
            .await?;
        let result = self.users.read_by_gh_id(gh_user.id)?;

        let user = match result {
            Some(user) => user,
            None => {
                let user = User::new(
                    gh_user.email,
                    gh_user.name,
                    gh_user.location,
                    gh_user.company,
                    gh_user.id,
                    gh_user.login,
                    github_access_token,
                );
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

#[cfg(test)]
mod tests {
    use faux::when;

    use super::*;
    use crate::{github::GithubUser, repository::user::UserRepository};

    const USER_EMAIL: &str = "test@example.com";
    const USER_NAME: &str = "user_name";
    const USER_LOCATION: &str = "Hamburg";
    const USER_COMPANY: &str = "Noops.io";
    const USER_GH_ACCESS_TOKEN: &str = "Yiu0Hae4ietheereij4OhneuNe6tae0e";
    const USER_GH_LOGIN: &str = "login_name";
    const USER_GH_ID: i32 = 42;

    lazy_static! {
        static ref USER: User = User::new(
            USER_EMAIL.to_string(),
            Some(USER_NAME.to_string()),
            Some(USER_LOCATION.to_string()),
            Some(USER_COMPANY.to_string()),
            USER_GH_ID,
            USER_GH_LOGIN.to_string(),
            USER_GH_ACCESS_TOKEN.to_string()
        );
        static ref JWT: String = Jwt::create_token(
            USER.id.clone(),
            JWT_ISSUER.to_string(),
            JWT_EXPIRATION_DELTA,
            &ENCODING_KEY,
        )
        .unwrap();
        static ref GITHUB_USER: GithubUser = GithubUser {
            id: USER_GH_ID,
            email: USER_EMAIL.to_string(),
            name: Some(USER_NAME.to_string()),
            location: Some(USER_LOCATION.to_string()),
            company: Some(USER_COMPANY.to_string()),
            login: USER_GH_LOGIN.to_string(),
            access_token: USER_GH_ACCESS_TOKEN.to_string(),
        };
    }

    #[tokio::test]
    async fn login_ok() -> anyhow::Result<()> {
        let mut users_mock = UserRepository::faux();
        when!(users_mock.read_by_gh_id(USER_GH_ID))
            .once()
            .then_return(Ok(Some(USER.clone())));

        let mut github_client_mock = GithubClient::faux();
        when!(github_client_mock.get_user(USER_GH_ACCESS_TOKEN.to_string()))
            .once()
            .then_return(Ok(GITHUB_USER.clone()));

        // -------------------------------------------------------------------------------------

        let auth_service = AuthService::new(github_client_mock, users_mock);
        let _ = auth_service
            .login(USER_GH_ACCESS_TOKEN.to_string())
            .await?
            .jwt;

        Ok(())
    }

    #[test]
    fn authenticate_ok() -> anyhow::Result<()> {
        let mut users_mock = UserRepository::faux();
        when!(users_mock.read(USER.id.as_ref()))
            .once()
            .then_return(Ok(Some(USER.clone())));

        let github_client_mock = GithubClient::faux();

        // -------------------------------------------------------------------------------------

        let auth_service = AuthService::new(github_client_mock, users_mock);
        let user = auth_service.authenticate(&JWT)?;
        assert_eq!(*USER, user);

        Ok(())
    }

    #[test]
    fn authenticate_user_not_registered() -> anyhow::Result<()> {
        let mut users_mock = UserRepository::faux();
        when!(users_mock.read(USER.id.as_ref()))
            .once()
            .then_return(Ok(None));

        let github_client_mock = GithubClient::faux();

        // -------------------------------------------------------------------------------------

        let auth_service = AuthService::new(github_client_mock, users_mock);
        let result = auth_service.authenticate(&JWT);
        assert!(result.is_err());

        Ok(())
    }
}
