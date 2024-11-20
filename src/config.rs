use std::env::var;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_max_age: i64,
    pub port: u16,
}

impl Config {
    pub fn init() -> Self {
        dotenvy::dotenv().expect("Error loading .env file");
        let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_max_age = var("JWT_MAX_AGE").expect("JWT_MAX_AGE must be set");

        Self { 
            database_url,
            jwt_secret,
            jwt_max_age: jwt_max_age.parse::<i64>().unwrap(),
            port: 8080,
        }
    }
}