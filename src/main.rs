use std::sync::Arc;
use rusqlite::Connection;
use tokio::sync::Mutex;

pub(crate) mod auth;
pub(crate) mod api;

#[derive(Clone)]
pub(crate) struct ServiceConfig {
  pub api_endpoint: String,
  pub oauth_endpoint: String,
  pub oauth_client_id: String,
  pub oauth_client_secret: String,
}

//TODO historical data

#[tokio::main]
async fn main() {
  env_logger::init();

  let db = Arc::new(Mutex::new(Connection::open("./.db.sqlite").unwrap()));
  db.lock().await.execute(r#"
    CREATE TABLE IF NOT EXISTS tokens (
      access_token TEXT,
      refresh_token TEXT,
      identity TEXT
    )"#, ()
  ).unwrap();

  //TODO: do not hardcode this
  let service = ServiceConfig {
    api_endpoint: "https://app.wsb.poznan.pl".to_string(),
    oauth_endpoint: "https://oauth.wsb.poznan.pl".to_string(),
    oauth_client_id: "OhK2xohyuphi5aephoo3uquichooxuu0mohbaixuNgieD8yeiziequai4iqu4thesh3oongeinae1osu".to_string(),
    oauth_client_secret: "ohKungiifiepeejivoazoothoo4quieB5aen8chiesiPee1voZ9uTahs9heLah5Ai3Shurohsh6ceeSh".to_string(),
  };

  warp::serve(api::api_route(db, service))
    .run(([127, 0, 0, 1], 3030))
    .await;
}
