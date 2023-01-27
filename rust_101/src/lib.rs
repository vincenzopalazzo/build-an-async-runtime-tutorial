#![allow(dead_code)]
use log::{debug, info};

// Name your user agent after your app?
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// Make an http request to Github API and return the result.
fn ping_github() -> Result<String, reqwest::Error> {
    debug!("Running the https request");
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    let resp = client.get("http://api.github.com/octocat").send()?;
    let body = resp.text()?;
    info!("{}", body);
    Ok(body)
}

#[cfg(test)]
mod tests {
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
    fn safety_test() {
        init();
        let _ = ping_github().unwrap();
    }
}
