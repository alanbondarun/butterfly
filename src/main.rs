use crate::config::Config;
use crate::statistics::{CategoryStats, ContinuousValueStats};
use std::sync::Arc;

mod config;
mod statistics;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

struct RequestResult {
    status_code: Result<hyper::StatusCode>,
    response_time: f64,
}

struct RequestSummary {
    status_code: CategoryStats<String>,
    response_time: ContinuousValueStats,
}

fn main() -> Result<()> {
    let loaded_config = Config::load("target/debug/config.toml")?;
    let config = Arc::new(loaded_config);

    let mut rt = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .core_threads(config.concurrent_users())
        .max_threads(config.concurrent_users())
        .enable_all()
        .build()?;
    rt.block_on(async {
        let futures = std::iter::repeat(())
            .take(config.total_users())
            .map(|_| {
                let config_copy = config.clone();
                tokio::spawn(
                    async move { make_request(config_copy.api_endpoint()).await },
                )
            })
            .collect::<Vec<tokio::task::JoinHandle<Result<RequestResult>>>>();

        let mut results: Vec<RequestResult> = vec![];
        for future in futures {
            let result = future.await.expect("Error").expect("Error");
            results.push(result);
        }

        let summary = RequestSummary {
            status_code: CategoryStats::new(
                results
                    .iter()
                    .map(|result| match &result.status_code {
                        Ok(code) => format!("{}", code),
                        Err(err) => format!("{}", err),
                    })
                    .collect(),
            ),
            response_time: ContinuousValueStats::new(
                results
                    .iter()
                    .filter(|result| result.status_code.is_ok())
                    .map(|result| result.response_time)
                    .collect(),
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

async fn make_request(endpoint: &str) -> Result<RequestResult> {
    let endpoint: hyper::Uri = endpoint.parse()?;

    let client = hyper::Client::new();

    let now = std::time::SystemTime::now();
    let request_result = client.get(endpoint).await;
    let elapsed_time = now.elapsed()?.as_secs_f64();

    Ok(RequestResult {
        status_code: request_result
            .map(|response| response.status())
            .map_err(|err| err.into()),
        response_time: elapsed_time,
    })
}
