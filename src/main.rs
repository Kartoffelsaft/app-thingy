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

        let site = warp::get()
            .and(warp::fs::dir("./client"));
        let post_item = warp::path!("api")
            .and(warp::post())
            .and(data_as_filter(db))
            .and(warp::body::json())
            .and_then(add_item_to_data);

        let app = site
            .or(post_item);

        warp::serve(app).run(([127, 0, 0, 1], 3030)).await;
    });
}

fn data_as_filter(
    rdata: Data
) -> impl Filter<Extract = (Data,), Error = Infallible> + Clone {
    warp::any().map(move || rdata.clone())
}

async fn add_item_to_data(
    rdata: Data,
    item: NewItem
) -> Result<impl warp::Reply, Infallible> {
    let mut data = rdata.lock().unwrap();

    data.push(item.text);

    let val = &*data;
    Ok(warp::reply::json(&serde_json::json!({
        "value": val
    })))
}
