use warp::*;

#[tokio::main]
async fn main() {
    let site = warp::path::end()
        .and(warp::get())
        .map(move || warp::reply::html(include_str!("guh.html")));

    /*let api_upload = warp::path!("api" / "v1" / "upload")
        .and(warp::post())
        .and(//i need to have this allow file uploads)
        .and_then(file_upload); */
    

    let route = site;
        //.or(dir);
        //.or(api_upload);

    warp::serve(route)
        .run(([0, 0, 0, 0], 3000))
        .await;
}