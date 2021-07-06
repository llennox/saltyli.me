use crate::{
    config::db::Pool,
    constants, 
    models::response::ResponseBody, 
    utils::token_utils,
    models::user::{User},
    };
use actix_service::{Service, Transform};
use actix_web::{
    Error, HttpResponse,
    dev::{ServiceRequest, ServiceResponse},
    http::{Method, HeaderName, HeaderValue},
    web::Data,
};
use futures::{
    future::{ok, Ready},
    Future,
};
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct Authentication;

impl<S, B> Transform<S> for Authentication
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}
pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthenticationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let mut authenticate_pass: bool = false;

        // Bypass some account routes
        let headers = req.headers_mut();
        headers.append(HeaderName::from_static("content-length"),HeaderValue::from_static("true"));
        if Method::OPTIONS == *req.method() {
            authenticate_pass = true;
        } else {
            for ignore_route in constants::IGNORE_ROUTES.iter() {
                if req.path().starts_with(ignore_route) {
                    authenticate_pass = true;
                    break;
                }
            }
            //if req.path().starts_with(constants::ADMIN_ROUTES) {
    //let user = User::find_user_by_id(&user_id.parse::<i32>().unwrap(), &pool.get().unwrap()).unwrap();
    //if user.user_role == String::from("admin") {

            //}
            if !authenticate_pass {
                if let Some(pool) = req.app_data::<Data<Pool>>() {
                    info!("Connecting to database...");
                    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
                        info!("Parsing authorization header...");
                        if let Ok(authen_str) = authen_header.to_str() {
                            if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer") {
                                info!("Parsing token...");
                                let token = authen_str[6..authen_str.len()].trim();
                                if let Ok(token_data) = token_utils::decode_token(token.to_string()) {
                                    info!("Decoding token...");
                                    if let Ok(verified_user_id) = token_utils::verify_token(&token_data, pool) {
                                        if req.path().starts_with(constants::ADMIN_ROUTES) {
                                            let user = User::find_user_by_id(&verified_user_id, &pool.get().unwrap()).unwrap();
                                            if user.user_role == String::from("admin") {
                                                req.headers_mut().insert(HeaderName::from_static(constants::USER_ID), 
                                                    HeaderValue::from_str(&verified_user_id.to_string()).unwrap());
                                                authenticate_pass = true;
                                            } else {
                                                error!("Try changing your user_role to admin");
                                            }
                                        } else {
                                            req.headers_mut().insert(HeaderName::from_static(constants::USER_ID), 
                                                HeaderValue::from_str(&verified_user_id.to_string()).unwrap());
                                            eprintln!("{:?}" ,req.headers());
                                            authenticate_pass = true;
                                        }
                                            
                                    } else {
                                        error!("Invalid token");
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if authenticate_pass {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(ResponseBody::new(
                            constants::MESSAGE_INVALID_TOKEN,
                            constants::EMPTY,
                        ))
                        .into_body(),
                ))
            })
        }
    }
}
