use std::convert::Infallible;

use warp::Filter;

type Data = std::sync::Arc<std::sync::Mutex<Vec<String>>>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct NewItem {
    text: String,
}

// I would use #[tokio::main] but the macro kept pissing on itself
fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let db: Data = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

        // GET  /                      -> index.html
        let site = warp::get()
            .and(warp::fs::dir("./client"));

        // POST /api/ {text: string}   -> 201 Created
        let post_item = warp::path!("api")
            .and(warp::post())
            .and(data_as_filter(db.clone()))
            .and(warp::body::json())
            .and_then(add_item_to_data);

        // GET /api/                   -> {value: [data]}
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
    let mut data = rdata.lock().unwrap();

    data.push(item.text);

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
