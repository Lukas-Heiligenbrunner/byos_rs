use anyhow::bail;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use rocket::{request, Request};

#[derive(Debug, Clone)]
pub struct DeviceInfos {
    pub width: u32,
    pub height: u32,
    pub fw_version: String,
    pub id: String,
    pub batt_voltage: f32,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for DeviceInfos {
    type Error = anyhow::Error;

    async fn from_request(request: &'a Request<'_>) -> request::Outcome<Self, Self::Error> {
        let id = request.headers().get_one("ID");
        let width = request.headers().get_one("Width");
        let height = request.headers().get_one("Height");
        let fwversion = request.headers().get_one("FW-Version");
        let batt_voltage = request.headers().get_one("Battery-Voltage");

        parse_headers(id, width, height, fwversion, batt_voltage).map_or_else(
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
    batt_voltage: Option<&str>,
) -> anyhow::Result<DeviceInfos> {
    match (id, width, height, fwversion, batt_voltage) {
        (Some(id), Some(width), Some(height), Some(fwversion), Some(batt_voltage)) => {
            Ok(DeviceInfos {
                width: width.to_string().parse()?,
                height: height.to_string().parse()?,
                fw_version: fwversion.to_string(),
                id: id.to_string(),
                batt_voltage: batt_voltage.to_string().parse()?,
            })
        }
        _ => bail!("Missing headers"),
    }
}
