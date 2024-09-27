mod config;
mod middleware;
mod routes;
mod schema;

use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{
    config::{CookieContentSecurity, PersistentSession, SessionLifecycle},
    storage::CookieSessionStore,
    SessionMiddleware,
};
use actix_web::{cookie, guard, http, web, App, HttpServer};

use ::clap::{Arg, ArgAction, ArgMatches, Command};
use async_graphql::Schema;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

use std::env;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use tracing::{error, info};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::EnvFilter;

use crate::config::ApplicationConfig;
use crate::config::ApplicationData;
use crate::schema::graphql::{MutationRoot, QueryRoot};

fn get_arguments() -> ArgMatches {
    Command::new("backend")
        .arg(
            Arg::new("hostname")
                .long("hostname")
                .value_name("IP ADDRESS")
                .help("Sets the hostname")
                .value_parser(clap::value_parser!(IpAddr))
                .required(false)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .value_name("PORT")
                .help("Sets the port")
                .value_parser(clap::value_parser!(u16).range(3000..))
                .required(false)
                .action(ArgAction::Set),
        )
        .get_matches()
}

fn get_socket_address() -> SocketAddr {
    let arguments = get_arguments();
    let default_ip_address = IpAddr::from([127, 0, 0, 1]);
    let default_port = 8000;

    let ip_address = arguments
        .get_one("hostname")
        .cloned()
        .unwrap_or(default_ip_address);
    let port: u16 = arguments.get_one("port").cloned().unwrap_or(default_port);

    SocketAddr::from((ip_address, port))
}

async fn create_postgres_pool(config: &ApplicationConfig) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.database_max_connections_count)
        .connect(&config.database_url)
        .await
}

fn init_tracing() {
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(false)
        .compact();

    tracing_subscriber::fmt()
        .event_format(format)
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "api=debug,actix_web=debug");

    if let Err(error) = dotenv::dotenv() {
        error!("Failed to load .env file: {}", error);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, error));
    }

    if let Err(error) = dotenv::from_filename(".env_backend") {
        error!("Failed to load .env_backend file: {}", error);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, error));
    }

    init_tracing();

    let secret_key = env::var("NEXTAUTH_SECRET").expect("NEXTAUTH_SECRET not set");

    let socket_addres = get_socket_address();
    let application_config = config::ApplicationConfig::from_env();
    let postgres_pool = match create_postgres_pool(&application_config).await {
        Ok(pool) => pool,
        Err(error) => {
            error!("Failed to create postgres pool: {}", error);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, error));
        }
    };

    // let application_data = Arc::new(web::Data::new(postgres_pool));
    let application_data = web::Data::new(ApplicationData::new(postgres_pool));
    let application_schema =
        Schema::build(QueryRoot, MutationRoot, async_graphql::EmptySubscription)
            .data(application_data.clone())
            .finish();

    info!("Socket address: {:#?}", socket_addres);

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityMiddleware::builder().build())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    cookie::Key::from(secret_key.as_bytes()),
                )
                .cookie_name("session-id".to_string())
                .cookie_secure(false)
                .session_lifecycle(SessionLifecycle::PersistentSession(
                    PersistentSession::default(),
                ))
                .cookie_content_security(CookieContentSecurity::Signed)
                .build(),
            )
            .wrap(Cors::permissive())
            .wrap(TracingLogger::default())
            .service(
                web::resource("/graphql")
                    .app_data(web::Data::new(application_schema.clone()))
                    .guard(guard::Post())
                    .route(web::post().to(routes::graphql_handler)),
            )
            .service(
                web::resource("/graphql/playground")
                    .route(web::get().to(routes::graphql_playground)),
            )
            .service(
                web::scope("/api")
                    .app_data(application_data.clone())
                    .route("/tours", web::get().to(routes::get_tours))
                    .service(
                        web::scope("/auth")
                            .route("/github", web::get().to(routes::github_login))
                            .route("/github/callback", web::get().to(routes::github_callback))
                            .route("/google", web::get().to(routes::google_login))
                            .route("/google/callback", web::get().to(routes::google_callback)),
                    ),
            )
            .route("/", web::get().to(routes::index))
            // .wrap(middleware::AuthMiddleware)
            .route("/protected", web::get().to(routes::protected))
    })
    // .bind_openssl(socket_addres, ssl_builder)?
    .bind(socket_addres)?
    .run()
    .await
}

/*
#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    dotenv::dotenv().ok();
    let github_client_id = env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID not set");
    let github_client_secret =
        env::var("GITHUB_CLIENT_SECRET").expect("GITHUB_CLIENT_SECRET not set");
    // let github_redirect_url = env::var("GITHUB_REDIRECT_URL").expect("GITHUB_REDIRECT_URL not set");

    let client = BasicClient::new(
        ClientId::new(github_client_id),
        Some(ClientSecret::new(github_client_secret)),
        AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).unwrap()),
    );
    // .set_redirect_uri(RedirectUrl::new(github_redirect_url).unwrap());

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::new(client))
            .route("/", web::get().to(index))
            .route("/api/auth/github", web::get().to(github_login))
            .route("/api/auth/github/callback", web::get().to(github_callback));
    };

    Ok(config.into())
}
*/
