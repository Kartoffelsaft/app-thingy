use std::convert::Infallible;

use warp::Filter;
use chrono::prelude::*;

type Data = std::sync::Arc<std::sync::Mutex<Vec<Datum>>>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Datum {
    memo: String,
    created_time: DateTime<Utc>,
    event_time: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct NewItem {
    memo: String,
    event_time: String,
}

// I would use #[tokio::main] but the macro kept pissing on itself
fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let db: Data = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

        // GET  
        // /
        // -> index.html
        let site = warp::get()
            .and(warp::fs::dir("./client"));

        // POST 
        // /api/ 
        // {memo: string, event_time: string}   
        // -> 201 Created
        let post_item = warp::path!("api")
            .and(warp::post())
            .and(data_as_filter(db.clone()))
            .and(warp::body::json())
            .and_then(add_item_to_data);

        // GET 
        // /api/
        // -> {value: [Data]}
        let get_items = warp::path!("api")
            .and(warp::get())
            .and(data_as_filter(db))
            .and_then(get_data);

        let app = site
            .or(post_item)
            .or(get_items);

        warp::serve(app).run(([127, 0, 0, 1], 3030)).await;
    });
}

fn data_as_filter(
    rdata: Data,
) -> impl Filter<Extract = (Data,), Error = Infallible> + Clone {
    warp::any().map(move || rdata.clone())
}

async fn add_item_to_data(
    rdata: Data,
    item: NewItem,
) -> Result<impl warp::Reply, Infallible> {
    let datum = Datum::new(item.memo, &item.event_time);
    if let Err(e) = &datum {
        if e.is::<chrono::ParseError>() {
            println!("bad request made: {:?}", e);

            return Ok(warp::http::StatusCode::BAD_REQUEST)
        }
    }

    let mut data = rdata.lock().unwrap();
    data.push(datum.expect("Unknown error when parsing request"));

    Ok(warp::http::StatusCode::CREATED)
}

async fn get_data(
    rdata: Data,
) -> Result<impl warp::Reply, Infallible> {
    let data = &*rdata.lock().unwrap();

    Ok(warp::reply::json(&serde_json::json!({
        "value": data
    })))
}

impl Datum {
    fn new(nmemo: String, nevent_time: &str) -> anyhow::Result<Datum> {
        Ok(Datum {
            memo: nmemo,
            created_time: Utc::now(),
            event_time: nevent_time.parse()?,
        })
    }
}
