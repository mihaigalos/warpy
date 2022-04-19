use argon2::{self, Config};
use rand::Rng;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::reply::Reply;
use warp::{filters::BoxedFilter, http::StatusCode, Filter};

#[derive(Debug, Deserialize)]
pub struct User {
    user: String,
    pass: String,
}

pub struct AuthN {
    pub db: Arc<Mutex<HashMap<String, User>>>,
    pub can_register: bool,
}

impl AuthN {
    pub async fn new(can_register: bool) -> AuthN {
        AuthN {
            can_register: can_register,
            db: Arc::new(Mutex::new(HashMap::<String, User>::new())),
        }
    }
    pub async fn run(&'static self, can_register: bool) -> BoxedFilter<(impl Reply + 'static)> {
        let db = warp::any().map(move || Arc::clone(&self.db));

        let login = warp::post()
            .and(warp::path("login"))
            .and(warp::body::json())
            .and(db.clone())
            .and_then(AuthN::login);

        if can_register {
            let register = warp::post()
                .and(warp::path("register"))
                .and(warp::body::json())
                .and(db.clone())
                .and_then(AuthN::register);
            let routes = register.or(login);
            return routes.boxed();
        } else {
            let register = warp::post()
                .and(warp::path(""))
                .and(warp::body::json())
                .and(db.clone())
                .and_then(AuthN::register);
            let routes = register.or(login);
            return routes.boxed();
        }
    }

    async fn register(
        new_user: User,
        db: Arc<Mutex<HashMap<String, User>>>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let mut users = db.lock().await;
        if users.contains_key(&new_user.user) {
            return Ok(StatusCode::BAD_REQUEST);
        }
        let hashed_user = User {
            user: new_user.user,
            pass: AuthN::hash(new_user.pass.as_bytes()),
        };
        users.insert(hashed_user.user.clone(), hashed_user);
        Ok(StatusCode::CREATED)
    }

    async fn login(
        credentials: User,
        db: Arc<Mutex<HashMap<String, User>>>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let users = db.lock().await;
        match users.get(&credentials.user) {
            None => Ok(StatusCode::BAD_REQUEST),
            Some(user) => {
                if AuthN::verify(&user.pass, credentials.pass.as_bytes()) {
                    Ok(StatusCode::OK)
                } else {
                    Ok(StatusCode::UNAUTHORIZED)
                }
            }
        }
    }

    fn hash(pass: &[u8]) -> String {
        let salt = rand::thread_rng().gen::<[u8; 32]>();
        let config = Config::default();
        argon2::hash_encoded(pass, &salt, &config).unwrap()
    }

    fn verify(hash: &str, pass: &[u8]) -> bool {
        argon2::verify_encoded(hash, pass).unwrap_or(false)
    }
}
