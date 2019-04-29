use std::sync::Arc;

use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use rocket::State;

use crate::config::Config;

pub struct AuthToken(String);

impl<'a, 'r> FromRequest<'a, 'r> for AuthToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthToken, ()> {
        let config = request
            .guard::<State<Arc<Config>>>()
            .expect("can't retrieve the config state in auth handler");

        let auth_tokens: Vec<_> = request.headers().get("X-Auth-Token").collect();

        if auth_tokens.len() != 1 {
            return Outcome::Failure((Status::Forbidden, ()));
        }

        let token = auth_tokens[0]; // OK

        // That's not exactly high level cryptography, but honnestly, who cares about security anyway ¯\_(ツ)_/¯.
        if config.auth_token() == token {
            Outcome::Success(AuthToken(token.to_string()))
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}
