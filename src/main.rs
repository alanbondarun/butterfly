use crate::statistics::{CategoryStats, ContinuousValueStats};

mod statistics;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

struct RequestResult {
    status_code: hyper::StatusCode,
    response_time: f64,
}

struct RequestSummary {
    status_code: CategoryStats<hyper::StatusCode>,
    response_time: ContinuousValueStats,
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

        let mut results: Vec<RequestResult> = vec![];
        for future in futures {
            let result = future.await.expect("Error").expect("Error");
            results.push(result);
        }

        let summary = RequestSummary {
            status_code: CategoryStats::new(
                results.iter().map(|result| result.status_code).collect(),
            ),
            response_time: ContinuousValueStats::new(
                results.iter().map(|result| result.response_time).collect(),
            ),
        };

        println!("status_code: {}", summary.status_code.histogram_as_str());
        println!("response_time_mean: {}", summary.response_time.mean());
        println!("response_time_median: {}", summary.response_time.median());
        println!(
            "response_time_90th_percentile: {}",
            summary.response_time.percentile_90th()
        );
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
        status_code: response.status(),
        response_time: elapsed_time,
    })
}
