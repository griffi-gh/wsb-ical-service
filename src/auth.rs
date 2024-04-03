use rusqlite::Connection;
use serde::Deserialize;

use crate::ServiceConfig;

/// A pair of OAuth tokens
#[derive(Clone, Debug, Deserialize)]
pub struct OAuthTokenPair {
  pub access_token: String,
  pub refresh_token: String,
}

//TODO proper error handling
//XXX: ensure only one handle_auth task can run at a time?

/// Either returns the same token pair, or a new one if the old one is expired
async fn renew_internal(service: &ServiceConfig, token: &OAuthTokenPair) -> Option<OAuthTokenPair> {
  log::info!("checking token");

  let client = reqwest::Client::new();

  // first, check if the access token is still valid
  let resource = client.get(&format!("{}/resource", service.oauth_endpoint))
    .query(&[("access_token", &token.access_token)])
    .send().await.unwrap();

  // check the rquest status and parse the JSON response
  let is_ok = resource.status().is_success();
  let json: serde_json::Value = resource.json().await.unwrap();

  // if the access token is still valid, just return the token as-is
  if is_ok {
    log::info!("token is valid, moving on");
    return None;
  }

  log::warn!("token is invalid, needs a refresh?");

  //assume an error happened, and check the root cause of the error
  //(scary!)
  let error = json.get("error").unwrap().as_str().unwrap();

  //we only ever expect the "expired_token" error, cannot handle anything else
  assert_eq!(error, "expired_token", "unexpected error: {}", error);

  //refresh the token
  let refresh = client
    .post(&format!("{}/token", service.oauth_endpoint))
    .json(&[
      ("grant_type", "refresh_token"),
      ("refresh_token", &token.refresh_token),
      ("client_id", &service.oauth_client_id),
      ("client_secret", &service.oauth_client_secret),
    ])
    .send().await.unwrap();

  //parse the JSON response, and return the new token pair
  //we assume everything went well, probably not the best idea :3
  let token_pair: OAuthTokenPair = refresh.json().await.unwrap();

  log::info!("yep, got a new token pair");

  Some(token_pair)
}

pub async fn handle_auth(db: std::sync::Arc<tokio::sync::Mutex<Connection>>, service: &ServiceConfig, identity: &str) -> String {
  //TODO handle missing row:

  let token = db.lock().await
    .query_row_and_then(
      r#"
        SELECT access_token, refresh_token
        FROM tokens
        WHERE identity = ?
      "#,
      (identity,),
      |row| {
        Ok::<_, rusqlite::Error>(OAuthTokenPair {
          access_token: row.get(0)?,
          refresh_token: row.get(1)?,
        })
      }
    ).unwrap();

  if let Some(new_token) = renew_internal(service, &token).await {
    db.lock().await.execute(
      r#"
        UPDATE tokens
        SET access_token = ?, refresh_token = ?
        WHERE identity = ?
      "#,
      (&new_token.access_token, &new_token.refresh_token, identity)
    ).unwrap();

    return new_token.access_token;
  }

  token.access_token
}
