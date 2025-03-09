use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::{Object, SecurityRequirement, SecurityScheme, SecuritySchemeData};
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use crate::config::Config;

pub struct AdminGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Получаем конфиг из state
        let config = request.guard::<&State<Config>>().await.unwrap();
        // Проверяем наличие заголовка Authorization и сравниваем с токеном из конфига
        match request.headers().get_one("Authorization") {
            Some(token) if token == config.admin_token => Outcome::Success(AdminGuard),
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}


impl<'a> OpenApiFromRequest<'a> for AdminGuard {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        // 1. Описываем схему безопасности
        let security_scheme = SecurityScheme {
            description: Some("Use admin token in `Authorization` header.".to_owned()),
            data: SecuritySchemeData::ApiKey {
                name: "Authorization".to_owned(),
                location: "header".to_owned(),
            },
            extensions: Object::default(),
        };

        // 2. Требование безопасности (SecurityRequirement)
        let mut security_req = SecurityRequirement::new();
        // Ключ должен совпадать с названием схемы:
        security_req.insert("AdminToken".to_string(), Vec::new());

        Ok(RequestHeaderInput::Security(
            "AdminToken".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}
