use anyhow::anyhow;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use rocket::{request, Request};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeviceId(pub String);

#[rocket::async_trait]
impl<'a> FromRequest<'a> for DeviceId {
    type Error = anyhow::Error;

    async fn from_request(request: &'a Request<'_>) -> request::Outcome<Self, Self::Error> {
        let id = request.headers().get_one("ID");
        match id {
            Some(id) => {
                // check validity
                Outcome::Success(DeviceId(id.to_string()))
            }
            None => Outcome::Error((Status::Unauthorized, anyhow!("Missing ID header"))),
        }
    }
}
