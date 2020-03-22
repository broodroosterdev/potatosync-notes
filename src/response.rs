#[derive(Serialize)]
pub struct StatusResponse {
    pub(crate) message: String,
    pub(crate) status: bool,
}

impl StatusResponse {
    pub fn new(message: String, status: bool) -> StatusResponse {
        StatusResponse {
            message,
            status,
        }
    }
}

impl ToString for StatusResponse {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

