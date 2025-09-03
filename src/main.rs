use warp::Filter;

#[tokio::main]
async fn main() {
    let site = warp::path::end()
        .and(warp::get())
        .map(move || warp::reply::html(include_str!("guh.html")));

    warp::serve(site)
        .run(([0, 0, 0, 0], 3000))
        .await;
}