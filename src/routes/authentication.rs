use argon2::{self, Config};
use rand::Rng;
use warp::Filter;


pub async fn register(
    store: crate::store::Store,
    account: crate::types::account::Account
) -> Result<impl warp::Reply, warp::Rejection> {
    let hashed_password = hash_password(account.password.as_bytes());

    let account = crate::types::account::Account {
        id: account.id,
        email: account.email,
        password: hashed_password,
};
  match store.add_account(account).await {
    Ok(_) => {
      Ok(warp::reply::with_status(
        "Account added",
        warp::http::StatusCode::OK
      ))
    },
    Err(e) => Err(warp::reject::custom(e)),
  }
}

pub async fn login(
    store: crate::store::Store,
    login: crate::types::account::Account
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_account(login.email).await {
        Ok(account) => match verify_password(&account.password, login.password.as_bytes()) {
            Ok(verified) => {
                if verified {
                    Ok(warp::reply::json(&issue_token(account.id.expect("id not found"))))
                } else {
                    Err(warp::reject::custom(handle_errors::Error::WrongPassword))
                }
            },
            Err(e) => Err(warp::reject::custom(handle_errors::Error::ArgonLibraryError(e)))
        },
        Err(e) => Err(warp::reject::custom(e))
    }
}

fn hash_password(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

fn verify_password(hash: &str, password: &[u8]) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}

fn issue_token(account_id: crate::types::account::AccountId) -> String {
    let current_date_time = chrono::Utc::now();
    let dt = current_date_time + chrono::Duration::days(1);

    let key = std::env::var("PASETO_KEY").unwrap();

    paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(&Vec::from(key.as_bytes()))
        .set_expiration(&dt)
        .set_not_before(&current_date_time)
        .set_claim("account_id", serde_json::json!(account_id))
        .build()
        .expect("Failed to create token")
}

fn verify_token(token: String) -> Result<crate::types::account::Session, handle_errors::Error> {
    let key = std::env::var("PASETO_KEY").unwrap();
    let token = paseto::tokens::validate_local_token(
        &token,
        None,
        &key.as_bytes(),
        &paseto::tokens::TimeBackend::Chrono
    ).map_err(|_| handle_errors::Error::TokenError)?;

    serde_json::from_value::<crate::types::account::Session>(token)
        .map_err(|_| handle_errors::Error::TokenError)
}

pub fn auth() ->
    impl warp::Filter<Extract = (crate::types::account::Session,), Error = warp::Rejection> + Clone
{
    warp::header::<String>("Authorization").and_then(|token: String| {
        let token = match verify_token(token) {
            Ok(t) => t,
            Err(_) => return std::future::ready(Err(warp::reject::reject())),
        };

        std::future::ready(Ok(token))
    })
}
