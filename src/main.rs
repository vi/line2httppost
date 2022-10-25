use http::{Request, header::CONTENT_TYPE};
use hyper::{Body, Client};
use hyper_tls::HttpsConnector;
use tokio::io::AsyncBufReadExt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let uri = std::env::args()
        .into_iter()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Specify URI"))?;
    let uri: http::Uri = uri.parse()?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let si = tokio::io::stdin();
    let si = tokio::io::BufReader::with_capacity(2048, si);
    let mut si = si.lines();

    while let Ok(Some(line)) = si.next_line().await {
        let rq = Request::builder()
            .method("POST")
            .header(CONTENT_TYPE, "text/plain")
            .uri(&uri)
            .body(Body::from(line))?;
        match client.request(rq).await {
            Ok(rs) => {
                if rs.status().is_success() {
                    // No action
                } else {
                    eprintln!("Status is {}", rs.status());
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    Ok(())
}
