use anyhow::bail;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use rocket::{request, Request};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeviceInfos {
    pub width: u32,
    pub height: u32,
    pub fw_version: String,
    pub id: String,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for DeviceInfos {
    type Error = anyhow::Error;

    async fn from_request(request: &'a Request<'_>) -> request::Outcome<Self, Self::Error> {
        let id = request.headers().get_one("ID");
        let width = request.headers().get_one("Width");
        let height = request.headers().get_one("Height");
        let fwversion = request.headers().get_one("FW-Version");

        parse_headers(id, width, height, fwversion).map_or_else(
            |e| Outcome::Error((Status::Unauthorized, e)),
            Outcome::Success,
        )
    }
}

fn parse_headers(
    id: Option<&str>,
    width: Option<&str>,
    height: Option<&str>,
    fwversion: Option<&str>,
) -> anyhow::Result<DeviceInfos> {
    match (id, width, height, fwversion) {
        (Some(id), Some(width), Some(height), Some(fwversion)) => Ok(DeviceInfos {
            width: width.to_string().parse()?,
            height: height.to_string().parse()?,
            fw_version: fwversion.to_string(),
            id: id.to_string(),
        }),
        _ => bail!("Missing headers"),
    }
}
