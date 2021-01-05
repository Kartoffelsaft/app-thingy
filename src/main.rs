use std::convert::Infallible;
use std::hash::{Hash, Hasher};

use warp::Filter;
use chrono::prelude::*;

type Data = std::sync::Arc<std::sync::Mutex<Vec<Datum>>>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Datum {
    id: String,
    memo: String,
    created_time: DateTime<Utc>,
    event_time: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Hash)]
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
            .and(data_as_filter(db.clone()))
            .and_then(get_data);

        // DELETE
        // /api/:id
        // -> 204 No content | 404 Not found
        let remove_item = warp::path!("api" / String)
            .and(warp::delete())
            .and(data_as_filter(db))
            .and_then(remove_item_from_data);

        let app = site
            .or(post_item)
            .or(get_items)
            .or(remove_item);

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
    let datum = Datum::new(item);
    if let Err(e) = &datum {
        if e.is::<chrono::ParseError>() {
            println!("bad request made: {:?}", e);

            return Ok(warp::http::StatusCode::BAD_REQUEST)
        }
    }

    println!("{:?}", datum);

    let mut data = rdata.lock().unwrap();
    data.push(datum.expect("Unknown error when parsing request"));

    Ok(warp::http::StatusCode::CREATED)
}

async fn remove_item_from_data(
    id: String,
    rdata: Data,
) -> Result<impl warp::Reply, Infallible> {
    let mut data = rdata.lock().unwrap();

    let possible_item_index = data.iter()
        .enumerate()
        .find(|(_, datum)| datum.id == id)
        .map(|(i, _)| i);

    if let Some(i) = possible_item_index {
        data.remove(i);
        Ok(warp::http::StatusCode::NO_CONTENT)
    } else {
        Ok(warp::http::StatusCode::NOT_FOUND)
    }
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
    fn new(nitem: NewItem) -> anyhow::Result<Datum> {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        nitem.hash(&mut hasher);
        Ok(Datum {
            id: format!("{:x}", hasher.finish()),
            memo: nitem.memo,
            created_time: Utc::now(),
            event_time: nitem.event_time.parse()?,
        })
    }
}

