use std::env;

use sendgrid::{Destination, Mail, SGClient};
use tera::{Context, Tera};

use dotenv::dotenv;

use crate::schema::tokens;

lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}

#[table_name = "tokens"]
#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Clone, JsonSchema)]
pub struct VerificationToken {
    pub(crate) account_id: i32,
    pub(crate) token: String,
}

pub(crate) fn create_token_email(name: String, url: String) -> String {
    let mut context = Context::new();
    context.insert("name", &name);
    context.insert("url", &url);
    let rendered = TERA.render("email.html.tera", &context).unwrap();
    return rendered;
}

pub(crate) fn send_email(email: String, address: String) {
    let api_key = env::var("SENDGRID_APIKEY").expect("Could not find SENDGRID APIKEY in .env");
    let sg = SGClient::new(api_key);
    let mail_info = Mail::new()
        .add_to(Destination {
            address: address.as_str(),
            name: address.as_str(),
        })
        .add_from("test@example.com")
        .add_subject("Confirming your new Potatosync account")
        .add_html(email.as_str());
    match sg.send(mail_info) {
        Err(err) => println!("Error: {}", err),
        Ok(body) => println!("Response: {}", body),
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_email_creation() {
        dotenv().ok();
        create_token_email("Test".parse().unwrap(), "http://example.com".parse().unwrap());
    }

    #[test]
    fn check_email_sending() {
        dotenv().ok();
        let email = create_token_email("Test".parse().unwrap(), "http://example.com".parse().unwrap());
        send_email(email, "myth.usa538@gmail.com".parse().unwrap());
    }
}