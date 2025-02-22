use crate::config::types::Config;
use anyhow::anyhow;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use rocket::{request, Request};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Token(pub String);

#[rocket::async_trait]
impl<'a> FromRequest<'a> for Token {
    type Error = anyhow::Error;

    async fn from_request(request: &'a Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Access-Token");
        match token {
            Some(token) => {
                let conf = request.rocket().state::<Config>().unwrap();

                match conf.devices.iter().find(|d| d.token == token) {
                    None => Outcome::Error((Status::Unauthorized, anyhow!("Invalid token header"))),
                    Some(t) => Outcome::Success(Token(t.token.to_string())),
                }
            }
            None => Outcome::Error((Status::Unauthorized, anyhow!("Missing token header"))),
        }
    }
}
