//! HTTP client wrapper with configurable options

use reqwest::{Client, ClientBuilder, Method, Response, Proxy};
use std::time::Duration;
use std::collections::HashMap;
use crate::error::Result;

/// HTTP client configuration
#[derive(Clone, Debug)]
pub struct HttpConfig {
    pub user_agent: String,
    pub timeout: Duration,
    pub insecure: bool,
    pub follow_redirect: bool,
    pub proxy: Option<String>,
    pub headers: HashMap<String, String>,
    pub cookies: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            user_agent: "rbuster/1.0".to_string(),
            timeout: Duration::from_secs(10),
            insecure: false,
            follow_redirect: false,
            proxy: None,
            headers: HashMap::new(),
            cookies: None,
            username: None,
            password: None,
        }
    }
}

/// HTTP client with connection pooling and configurable options
pub struct HttpClient {
    client: Client,
    config: HttpConfig,
}

impl HttpClient {
    pub fn new(config: HttpConfig) -> Result<Self> {
        let mut builder = ClientBuilder::new()
            .user_agent(&config.user_agent)
            .timeout(config.timeout)
            .danger_accept_invalid_certs(config.insecure)
            .pool_max_idle_per_host(100)
            .pool_idle_timeout(Duration::from_secs(90))
            .tcp_nodelay(true)
            .tcp_keepalive(Duration::from_secs(60));

        // Configure redirect policy
        if !config.follow_redirect {
            builder = builder.redirect(reqwest::redirect::Policy::none());
        }

        // Configure proxy
        if let Some(ref proxy_url) = config.proxy {
            let proxy = Proxy::all(proxy_url)?;
            builder = builder.proxy(proxy);
        }

        let client = builder.build()?;

        Ok(Self { client, config })
    }

    /// Make a GET request
    pub async fn get(&self, url: &str) -> Result<Response> {
        self.request(Method::GET, url, None).await
    }

    /// Make a request with specified method
    pub async fn request(&self, method: Method, url: &str, body: Option<&str>) -> Result<Response> {
        let mut request = self.client.request(method, url);

        // Add custom headers
        for (key, value) in &self.config.headers {
            request = request.header(key.as_str(), value.as_str());
        }

        // Add cookies
        if let Some(ref cookies) = self.config.cookies {
            request = request.header("Cookie", cookies.as_str());
        }

        // Add basic auth
        if let (Some(ref user), Some(ref pass)) = (&self.config.username, &self.config.password) {
            request = request.basic_auth(user, Some(pass));
        }

        // Add body if present
        if let Some(data) = body {
            request = request.body(data.to_string());
        }

        Ok(request.send().await?)
    }

    /// Check if URL returns a valid response
    pub async fn check_url(&self, url: &str, method: &str) -> Result<(u16, usize, Option<String>)> {
        let method = Method::from_bytes(method.as_bytes()).unwrap_or(Method::GET);
        let response = self.request(method, url, None).await?;
        
        let status = response.status().as_u16();
        let redirect = response
            .headers()
            .get("location")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        
        let body = response.bytes().await?;
        let size = body.len();
        
        Ok((status, size, redirect))
    }
}

/// Parse headers from command line format "Key: Value"
pub fn parse_headers(headers: &[String]) -> HashMap<String, String> {
    headers
        .iter()
        .filter_map(|h| {
            let parts: Vec<&str> = h.splitn(2, ':').collect();
            if parts.len() == 2 {
                Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
            } else {
                None
            }
        })
        .collect()
}
