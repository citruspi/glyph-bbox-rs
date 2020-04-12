use rust_embed::RustEmbed;
use warp::{http::HeaderValue, reply::Response, Rejection, Reply};

#[derive(RustEmbed)]
#[folder = "assets/web"]
struct Asset;

pub async fn serve_file(path: &str, content_type: &str) -> Result<impl Reply, Rejection> {
    let asset = Asset::get(path).ok_or_else(warp::reject::not_found)?;

    let mut res = Response::new(asset.into());
    res.headers_mut()
        .insert("Content-Type", HeaderValue::from_str(content_type).unwrap());
    Ok(res)
}
