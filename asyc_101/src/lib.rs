#![allow(dead_code)]
use log::{debug, info};
use surf;

/// Make an http request to Github API and return the result.
async fn ping_github() -> Result<String, surf::Error> {
    debug!("Running the https request");
    let mut res = surf::get("https://api.github.com/octocat").await?;
    let body = res.body_string().await?;
    info!("{}", body);
    Ok(body)
}

#[cfg(test)]
mod tests {
    use custom_rio::runtime::Runtime;
    use custom_rio::CustomRio;
    use std::sync::Once;

    use crate::ping_github;

    static INIT: Once = Once::new();

    fn init() {
        // ignore error
        INIT.call_once(|| {
            env_logger::init();
        });
    }

    #[test]
    fn safety_test_not_running() {
        init();
        let _ = ping_github();
    }

    #[test]
    fn safety_test_async_running() {
        init();
        CustomRio::block_on(async {
            let _ = ping_github().await.unwrap();
        });
    }
}
