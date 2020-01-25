fn main() -> Result<(), Box<dyn std::error::Error>> {
    let total_users = 10000;
    let concurrent_users = 10;

    let mut rt = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .core_threads(concurrent_users)
        .build()?;
    rt.block_on(async {
        let futures = std::iter::repeat(())
            .take(total_users)
            .enumerate()
            .map(|(idx, _)| {
                tokio::spawn(async move {
                    println!("task #{} done.", idx);
                })
            })
            .collect::<Vec<tokio::task::JoinHandle<()>>>();

        for future in futures {
            future.await.expect("Error");
        }
    });

    Ok(())
}
