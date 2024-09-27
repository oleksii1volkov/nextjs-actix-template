use actix_utils::future::{ok, ready, Ready};
use actix_web::{
    body::MessageBody,
    cookie::Cookie,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
};
use std::task::{Context, Poll};
use std::{future::Future, pin::Pin};

use hkdf::{Hkdf, InvalidLength};
use sha2::Sha256;

use josekit::jwe::{self, alg::direct::DirectJweAlgorithm, JweHeader};
use josekit::jwk;
use josekit::jwt::{self, JwtPayload};
use josekit::JoseError;

use std::env;
use tracing::{error, info};

fn hkdf_sha256(
    key_material: &[u8],
    salt: &[u8],
    info: &str,
    length: usize,
) -> Result<Vec<u8>, InvalidLength> {
    let hk = Hkdf::<Sha256>::new(Some(salt), key_material);
    let info = info.as_bytes();
    let mut okm = vec![0u8; length];
    hk.expand(info, &mut okm)?;

    Ok(okm)
}

fn decrypt_next_auth_token(
    encrypted_token: &str,
    secret_key: &[u8],
) -> Result<(JwtPayload, JweHeader), JoseError> {
    let mut jwk = jwk::Jwk::new("oct");
    jwk.set_key_value(secret_key);
    let decrypter = DirectJweAlgorithm::Dir.decrypter_from_jwk(&jwk)?;

    josekit::jwt::decode_with_decrypter(encrypted_token.as_bytes(), &decrypter)
}

fn extract_next_auth_token(request: &Vec<Cookie>) -> Option<String> {
    for cookie in request {
        if cookie.name() == "next-auth.session-token" {
            return Some(cookie.value().to_owned());
        }
    }

    None
}

pub struct NextAuthMiddleware;

/*
impl<S, B, Store> Transform<S, ServiceRequest> for SessionMiddleware<Store>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
    Store: SessionStore + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = InnerSessionMiddleware<S, Store>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
*/

impl<S, B> Transform<S, ServiceRequest> for NextAuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = NextAuthServiceMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(NextAuthServiceMiddleware { service })
    }
}

pub struct NextAuthServiceMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for NextAuthServiceMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let cookies = match request.cookies() {
            Ok(cookies) => cookies.clone(),
            Err(error) => {
                error!("Failed to get cookies: {}", error);
                return Box::pin(async move {
                    Err(actix_web::error::ErrorInternalServerError(
                        "Failed to get cookies",
                    ))
                });
            }
        };

        let future = self.service.call(request);

        Box::pin(async move {
            let session_token = match extract_next_auth_token(&cookies) {
                Some(token) => token,
                None => {
                    return Err(actix_web::error::ErrorInternalServerError(
                        "'next-auth.session-token' not found",
                    ));
                }
            };

            let secret_key = match env::var("NEXTAUTH_SECRET") {
                Ok(variable) => variable,
                Err(_) => {
                    error!("'NEXTAUTH_SECRET' not set");
                    return Err(actix_web::error::ErrorInternalServerError(
                        "'NEXTAUTH_SECRET' not found",
                    ));
                }
            };

            let salt = "";
            let info = "NextAuth.js Generated Encryption Key";
            let secret_key = match hkdf_sha256(secret_key.as_bytes(), salt.as_bytes(), info, 32) {
                Ok(key) => key,
                Err(error) => {
                    error!("Failed to create secret key: {}", error);
                    return Err(actix_web::error::ErrorInternalServerError(
                        "Failed to create secret key",
                    ));
                }
            };

            match decrypt_next_auth_token(&session_token, &secret_key) {
                Ok((decrypted_payload, _decrypted_header)) => {
                    let decrypted_token = decrypted_payload.to_string();
                    info!("Decrypted token: {}", decrypted_token);

                    match decrypted_payload.claim("name") {
                        Some(name) => {
                            info!("Name: {}", name);

                            if name == "admin" {
                                future.await
                            } else {
                                Err(actix_web::error::ErrorUnauthorized(
                                    "Invalid or missing token",
                                ))
                            }
                        }
                        None => {
                            error!("'name' claim not found in token");
                            Err(actix_web::error::ErrorInternalServerError(
                                "'name' claim not found in token",
                            ))
                        }
                    }
                }
                Err(error) => {
                    error!("Failed to decrypt token: {}", error);

                    Err(actix_web::error::ErrorUnauthorized(
                        "Invalid or missing token",
                    ))
                }
            }
        })
    }
}
