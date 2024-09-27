use actix_session::Session;
use actix_web::{http, web, HttpRequest, HttpResponse, Responder};

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl,
    TokenResponse, TokenUrl,
};

use std::env;
use tracing::{error, info};

use crate::config::ApplicationData;
use crate::schema::graphql::{ApplicationSchema, Tour};

pub async fn graphql_handler(
    application_schema: web::Data<ApplicationSchema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    application_schema
        .execute(request.into_inner())
        .await
        .into()
}

pub async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

pub async fn index(session: Session) -> impl Responder {
    info!("Index route");

    HttpResponse::Ok().body("Hello World!")
}

pub async fn get_tours(application_data: web::Data<ApplicationData>) -> impl Responder {
    match sqlx::query_as::<_, Tour>("SELECT id, title FROM public.\"Tours\"")
        .fetch_all(&application_data.database_pool)
        .await
    {
        Ok(tours) => HttpResponse::Ok().json(tours),
        Err(error) => {
            error!("Failed to get tours: {}", error);
            HttpResponse::InternalServerError().body(format!("Failed to get tours: {}", error))
        }
    }
}

pub async fn protected() -> impl Responder {
    info!("Protected route");

    HttpResponse::Ok().body("Hello Protected World!")
}

#[derive(Debug, serde::Deserialize)]
pub struct OAuth2Callback {
    code: String,
    state: String,
}

pub async fn github_login() -> impl Responder {
    let oauth_client = get_github_client();
    let (mut authorize_url, _csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(oauth2::Scope::new("user:email".to_string()))
        .add_scope(oauth2::Scope::new("read:user".to_string()))
        .url();

    authorize_url
        .query_pairs_mut()
        .append_pair("prompt", "consent");

    info!("Redirecting to {}", authorize_url);

    HttpResponse::Found()
        .append_header((http::header::LOCATION, authorize_url.to_string()))
        .finish()
}

pub async fn github_callback(
    session: Session,
    callback_data: web::Query<OAuth2Callback>,
) -> impl Responder {
    info!("GitHub Callback Data: {:#?}", callback_data);

    let oauth_client = get_github_client();
    let token_result = oauth_client
        .exchange_code(AuthorizationCode::new(callback_data.code.clone()))
        .request_async(oauth2::reqwest::async_http_client)
        .await;

    match token_result {
        Ok(token) => {
            let request_client = reqwest::Client::new();
            let user_info = request_client
                .get("https://api.github.com/user")
                .header(
                    "Authorization",
                    format!("token {}", token.access_token().secret()),
                )
                .header("User-Agent", "actix-web-app")
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap();

            info!("Token: {:#?}", token);
            info!("User info: {:#?}", user_info);

            session
                .insert(
                    "github_user",
                    user_info["login"].as_str().unwrap().to_string(),
                )
                .unwrap();

            HttpResponse::Found()
                .append_header((http::header::LOCATION, "/"))
                .finish()
        }

        Err(error) => {
            error!("Error: {:#?}", error);
            HttpResponse::Unauthorized().finish()
        }
    }
}

pub async fn google_login() -> impl Responder {
    let oauth_client = get_google_client();
    let (mut authorize_url, _csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(oauth2::Scope::new("openid".to_string()))
        .add_scope(oauth2::Scope::new("email".to_string()))
        .add_scope(oauth2::Scope::new("profile".to_string()))
        .url();

    authorize_url
        .query_pairs_mut()
        .append_pair("prompt", "consent");

    info!("Redirecting to {}", authorize_url);

    HttpResponse::Found()
        .append_header((http::header::LOCATION, authorize_url.to_string()))
        .finish()
}

pub async fn google_callback(
    session: Session,
    callback_data: web::Query<OAuth2Callback>,
) -> impl Responder {
    info!("Google Callback Data: {:#?}", callback_data);

    let oauth_client = get_google_client();
    let token_result = oauth_client
        .exchange_code(AuthorizationCode::new(callback_data.code.clone()))
        .request_async(oauth2::reqwest::async_http_client)
        .await;

    match token_result {
        Ok(token) => {
            let request_client = reqwest::Client::new();
            let user_info = request_client
                .get("https://www.googleapis.com/oauth2/v2/userinfo")
                .header(
                    "Authorization",
                    format!("Bearer {}", token.access_token().secret()),
                )
                .header("User-Agent", "actix-web-app")
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap();

            info!("Token: {:#?}", token);
            info!("User info: {:#?}", user_info);

            session
                .insert(
                    "google_user",
                    user_info["email"].as_str().unwrap().to_string(),
                )
                .unwrap();

            HttpResponse::Found()
                .append_header((http::header::LOCATION, "/"))
                .finish()
        }

        Err(error) => {
            error!("Error: {:#?}", error);
            HttpResponse::Unauthorized().finish()
        }
    }
}

pub fn get_github_client() -> BasicClient {
    let github_client_id = env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID not set");
    let github_client_secret =
        env::var("GITHUB_CLIENT_SECRET").expect("GITHUB_CLIENT_SECRET not set");
    // let github_redirect_url = env::var("GITHUB_REDIRECT_URL").expect("GITHUB_REDIRECT_URL not set");

    BasicClient::new(
        ClientId::new(github_client_id),
        Some(ClientSecret::new(github_client_secret)),
        AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).unwrap()),
    )
    // .set_redirect_uri(RedirectUrl::new(github_redirect_url).unwrap());
}

pub fn get_google_client() -> BasicClient {
    let google_client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set");
    let google_client_secret =
        env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET not set");
    let google_redirect_url = env::var("GOOGLE_REDIRECT_URL").expect("GOOGLE_REDIRECT_URL not set");

    BasicClient::new(
        ClientId::new(google_client_id),
        Some(ClientSecret::new(google_client_secret)),
        AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string()).unwrap(),
        Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(google_redirect_url).unwrap())
}
