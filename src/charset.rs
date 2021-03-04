use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};
use rocket::http::Header;

pub(crate) struct Charset {}

impl Fairing for Charset {
    fn info(&self) -> Info {
        Info{
            name: "Charset",
            kind: Kind::Response
        }
    }

    fn on_response(&self, _request: &Request, response: &mut Response){
        let content_header = response.headers().get_one("Content-Type");
        if content_header.is_some() && content_header.unwrap().eq("application/json") {
            response.set_header(Header::new("Content-Type", "application/json; charset=utf-8"));
        }
    }
}