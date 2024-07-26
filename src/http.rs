use anyhow::Result;
use reqwest::{header::HeaderMap, Client, Response};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::Write, path::PathBuf, sync::Arc};

#[derive(Debug)]
pub struct HttpError {
    pub url: String,
    pub error: anyhow::Error,
}

#[derive(Serialize, Deserialize)]
pub struct HttpResponse {
    pub url: String,
    pub method: String,
    pub status: u16,
    pub headers: String,
    pub body: String,
}

impl HttpResponse {
    /// Check if status is within 400-499.
    pub fn is_client_error(&self) -> bool {
        500 > self.status && self.status >= 400
    }
}

pub async fn json_to_file(data_list: Vec<HttpResponse>, file_path: PathBuf) -> Result<()> {
    let json = serde_json::to_string_pretty(&data_list)?;
    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

fn serialize_headers(headers: &HeaderMap) -> String {
    let headers_map: HashMap<String, String> = headers
        .iter()
        .map(|(name, value)| {
            (
                name.to_string(),
                value.to_str().unwrap_or("").to_string(), // Ensure you handle non-UTF8 headers appropriately
            )
        })
        .collect();

    serde_json::to_string(&headers_map).unwrap_or_default()
}

async fn parse_response(response: Result<Response, HttpError>, method: String) -> HttpResponse {
    match response {
        Ok(res) => {
            let headers = serialize_headers(res.headers());
            let data = HttpResponse {
                url: res.url().to_string(),
                method: method,
                status: res.status().into(),
                headers,
                body: {
                    match res.text().await {
                        Ok(b) => b,
                        Err(_) => "".to_string(),
                    }
                },
            };

            return data;
        }
        Err(HttpError { url, error }) => {
            let data = HttpResponse {
                url,
                method,
                status: 400,
                headers: "".to_string(),
                body: format!(
                    "AppScan Error: Failed creating thread to send http request: {}",
                    error
                ),
            };

            return data;
        }
    };
}

pub async fn request(
    urls: Vec<String>,
    client: Arc<Client>,
    methods: Vec<String>,
) -> Vec<HttpResponse> {
    let mut tasks = vec![];
    for url in urls {
        let cloned_methods = methods.clone();
        for method_string in cloned_methods {
            let client = Arc::clone(&client);
            let url_clone = url.clone();
            let method_string_clone = method_string.clone();
            let method_string_clone2 = method_string.clone();
            let task = tokio::spawn(async move {
                // Skill issue
                let url_clone1 = url_clone.clone();
                let url_clone2 = url_clone.clone();

                // Shouldn't panic since the method is checked for validity in main::http()
                let method = reqwest::Method::from_bytes(method_string_clone.as_bytes())
                    .expect("Failed to parse method");
                let response: Result<Response, HttpError> = client
                    .request(method, &url_clone1)
                    .send()
                    .await
                    .map_err(|e| HttpError {
                        url: url_clone2,
                        error: e.into(),
                    });

                return parse_response(response, method_string).await;
            });
            tasks.push((task, url.clone(), method_string_clone2));
        }
    }

    let mut responses: Vec<HttpResponse> = Vec::new();
    for (task, url, method) in tasks {
        match task.await {
            Ok(result) => responses.push(result),
            Err(join_err) => responses.push(HttpResponse {
                url: url,
                method: method,
                status: 400,
                headers: "".to_string(),
                body: format!(
                    "AppScan Error: Failed collecting thread results of http request: {}",
                    join_err
                ),
            }),
        }
    }

    responses
}
