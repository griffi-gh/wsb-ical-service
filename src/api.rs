use std::sync::Arc;
use icalendar::{Calendar, Component, Event, EventLike};
use tokio::sync::Mutex;
use rusqlite::Connection;
use serde::Deserialize;
use warp::{Filter, filters::BoxedFilter, reply::Reply};
use chrono_tz as Tz;
use scraper::{Selector, Html};
use crate::{auth, ServiceConfig};

mod schedule;

#[derive(Deserialize)]
struct IcalQuery {
  identity: String,
}

async fn handle_ical_request(db: Arc<Mutex<Connection>>, service: ServiceConfig, query: IcalQuery) -> Result<String, warp::Rejection> {
  let access_token = auth::handle_auth(db, &service, &query.identity).await;

  let client = reqwest::Client::new();
  let schedule: Vec<schedule::Event> = client.post(format!("{}/mobile/grafik/schedule", service.api_endpoint))
    .json(&serde_json::json!({
      "access_token": access_token
    }))
    .send().await.unwrap()
    .json().await.unwrap();

  let mut schedule_ical = Calendar::new();
  for event in schedule {
    let start_time = event.begin_date.and_local_timezone(Tz::Europe::Warsaw).unwrap().to_utc();
    let end_time = event.end_date.and_local_timezone(Tz::Europe::Warsaw).unwrap().to_utc();

    let mut cal_event = Event::new();

    cal_event
      .summary(&event.subject.name)
      .description(&format!(
        "Lecturers: {}\nLocation: {}\n{}",
        event.lecturers.iter().map(|l| l.name.clone()).collect::<Vec<String>>().join(", "),
        event.locations.iter().map(|l| l.classroom.clone()).collect::<Vec<String>>().join(", "),
        event.comments.clone().unwrap_or_default()
      ))
      .timestamp(start_time)
      .starts(start_time)
      .ends(end_time);

    //TODO: add location

    if let Some(url) = event.comments_meeting {
      //try to get the url from the comments_meeting field
      //usually it's an html link, but it might be a raw URL
      //for now, only handle the case where it's a single `a` tag
      let fragment = Html::parse_fragment(&url);
      let selector = Selector::parse("a").unwrap();
      let mut selection = fragment.select(&selector);
      if let Some(element) = selection.next() {
        if let Some(url) = element.value().attr("href") {
          cal_event.url(url);
        }
      }
    }

    schedule_ical.push(cal_event.done());
  }
  let schedule_ical = schedule_ical.done();
  Ok(schedule_ical.to_string())
}

fn apiv1_route(db: Arc<Mutex<Connection>>, service: ServiceConfig) -> BoxedFilter<(impl Reply,)> {
  warp::path!("ical")
    .and(warp::any().map(move || Arc::clone(&db)))
    .and(warp::any().map(move || service.clone()))
    .and(warp::query::<IcalQuery>())
    .and_then(handle_ical_request)
    .boxed()
}

pub fn api_route(db: Arc<Mutex<Connection>>, service: ServiceConfig) -> BoxedFilter<(impl Reply,)> {
  warp::path!("api" / ..)
    .and(warp::path!("v1" / ..).and(apiv1_route(db, service)))
    .boxed()
}
