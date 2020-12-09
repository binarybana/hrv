use warp::http::StatusCode;
use warp::Filter;

use serde::{Deserialize, Serialize};

use std::sync::{Arc, Mutex};

#[cfg(target_arch = "aarch64")]
mod hw;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub fan1_speed: f64,
    pub fan2_speed: f64,
}

pub type Db = Arc<Mutex<Config>>;

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (Config,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn create_api(
    config: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let update_config = warp::path!("api")
        .and(warp::post())
        .and(with_db(config.clone()))
        .and(json_body())
        .map(|db: Db, new_config: Config| {
            let mut db = db.lock().expect("Couldnt unlock db");
            (*db).fan1_speed = new_config.fan1_speed;
            (*db).fan2_speed = new_config.fan2_speed;
            Ok(StatusCode::CREATED)
        });

    let read_config = warp::path!("api")
        .and(warp::get())
        .and(with_db(config.clone()))
        .map(|db: Db| {
            let config = db.lock().expect("Couldn't unlock db");
            Ok(warp::reply::json(&*config))
        });

    update_config.or(read_config)
}

#[tokio::main]
async fn main() {
    let config = Arc::new(Mutex::new(Config {
        fan1_speed: 0.0,
        fan2_speed: 0.0,
    }));
    let api = create_api(config.clone());
    let ui = warp::filters::any::any().and(warp::filters::fs::dir("static"));
    let root = api.or(ui);

    #[cfg(target_arch = "aarch64")]
    hw::setup_hardware(config.clone()).expect("Can't setup PWM");

    warp::serve(root).run(([0, 0, 0, 0], 8000)).await;
}

#[cfg(test)]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;

    use super::*;

    #[tokio::test]
    async fn test_post() {
        let config = Arc::new(Mutex::new(Config {
            fan1_speed: 0.0,
            fan2_speed: 0.0,
        }));
        let api = create_api(config);

        let resp = request().method("GET").path("/api").reply(&api).await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            serde_json::from_slice::<Config>(resp.body()).expect("Convert to json"),
            Config {
                fan1_speed: 0.0,
                fan2_speed: 0.0
            }
        );

        let resp = request()
            .method("POST")
            .path("/api")
            .json(&Config {
                fan1_speed: 10.0,
                fan2_speed: 1.0,
            })
            .reply(&api)
            .await;
        assert_eq!(resp.status(), StatusCode::CREATED);

        let resp = request().method("GET").path("/api").reply(&api).await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            serde_json::from_slice::<Config>(resp.body()).expect("Convert to json"),
            Config {
                fan1_speed: 10.0,
                fan2_speed: 1.0
            }
        );
    }
}
