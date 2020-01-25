type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

struct RequestResult {
    _status_code: hyper::StatusCode,
    _response_time: f64,
}

fn main() -> Result<()> {
    let total_users = 100;
    let concurrent_users = 10;

    let mut rt = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .core_threads(concurrent_users)
        .enable_all()
        .build()?;
    rt.block_on(async {
        let futures = std::iter::repeat(())
            .take(total_users)
            .map(|_| tokio::spawn(async move { make_request().await }))
            .collect::<Vec<tokio::task::JoinHandle<Result<RequestResult>>>>();

        for future in futures {
            future.await.expect("Error").expect("Error");
        }
    });

    Ok(())
}

async fn make_request() -> Result<RequestResult> {
    let endpoint: hyper::Uri = "http://127.0.0.1:8080/api/task".parse()?;

    let client = hyper::Client::new();

    let now = std::time::SystemTime::now();
    let response = client.get(endpoint).await?;
    let elapsed_time = now.elapsed()?.as_secs_f64();

    Ok(RequestResult {
        _status_code: response.status(),
        _response_time: elapsed_time,
    })
}
