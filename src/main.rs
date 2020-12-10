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

        println!("{}", serde_json::to_string(&NewItem {text: String::from("test")}).unwrap());
        
        let get = warp::get()
            .and(warp::fs::dir("./client"));
        let post = warp::post()
            .and(data_as_filter(db))
            .and(warp::body::json())
            .and_then(add_item_to_data);

        warp::serve(post.or(get)).run(([127, 0, 0, 1], 3030)).await;
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
    println!("new item: {:#?}", item);

    let mut data = rdata.lock().unwrap();

    data.push(item.text);

    let val = &*data;
    Ok(warp::reply::json(&serde_json::json!({
        "value": val
    })))
}
