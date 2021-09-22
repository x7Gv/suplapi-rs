pub use failure::Error;
pub use async_trait::async_trait;

#[async_trait]
pub trait HttpClient {
    fn user_agent(&mut self, user_agent: String);
    async fn get<'a, I>(&self, url: &str, args: I) -> Result<String, Error>
        where I: Iterator<Item=(&'a str, &'a str)> + Send;
}

#[cfg(feature="http-client")]
pub mod default {
    use reqwest;
    use failure::{ensure, err_msg};

    use super::{Error, HttpClient, async_trait};

    #[derive(Debug, Clone)]
    pub struct Client {
        user_agent: String,
    }

    impl Default for Client {
        fn default() -> Self {
            Client { user_agent: "".to_owned() }
        }
    }

    #[async_trait]
    impl HttpClient for Client {
        fn user_agent(&mut self, user_agent: String) {
            self.user_agent = user_agent;
        }

        async fn get<'a, I>(&self, url: &str, args: I) -> Result<String, Error>
        where I: Iterator<Item=(&'a str, &'a str)> + Send {
            let url = reqwest::Url::parse_with_params(url, args)?;
            let client = reqwest::Client::new();
            let response = client.get(url)
                .header(reqwest::header::USER_AGENT, self.user_agent.clone())
                .send().await?;

            ensure!(response.status().is_success(), err_msg("Bad status"));

            Ok(response.text().await?)
        }
    }
}
