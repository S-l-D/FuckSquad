use serde::de::DeserializeOwned;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

const BASE_URL: &str = "http://172.17.0.1:8080";

async fn get_response(req: &Request) -> Result<String, String> {
    let resp = execute_req(&req).await?;
    let text = match JsFuture::from(match resp.text() {
        Ok(text) => text,
        Err(e) => return Err(format!("Cannot get text from response! {:#?}", e)),
    })
        .await
    {
        Ok(value) => match value.as_string() {
            None => return Err(format!("Value is none!")),
            Some(text) => text,
        },
        Err(e) => return Err(format!("Cannot cast to string! {:#?}", e)),
    };
    Ok(text)
}

async fn execute_req(req: &Request) -> Result<Response, String> {
    let window = match web_sys::window() {
        None => return Err(format!("Cannot get web_sys::window! Window is none!")),
        Some(window) => window,
    };
    let resp_value = match JsFuture::from(window.fetch_with_request(req)).await {
        Ok(resp) => resp,
        Err(e) => return Err(format!("Fetch failed with error: {:#?}", e)),
    };
    let resp: Response = match resp_value.dyn_into() {
        Ok(ok) => ok,
        Err(e) => return Err(format!("Response cast error: {:#?}", e)),
    };
    Ok(resp)
}


pub async fn get<M>(url: &str) -> Result<M, String>
where
    M: DeserializeOwned,
{
    let url = url.to_string();
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let link = format!("{}/{}", BASE_URL, &url);

    let request = match Request::new_with_str_and_init(&link, &opts) {
        Ok(request) => request,
        Err(_) => return Err(format!("Cannot instantiate request!")),
    };

    let text = get_response(&request).await?;

    match serde_json::from_str(text.as_str()) {
        Ok(response) => Ok(response),
        Err(_) => Err(text),
    }
}

pub async fn post<T, M>(url: &str, body: T) -> Result<M, String>
where
    T: serde::Serialize,
    M: DeserializeOwned,
{
    let url = url.to_string();
    let body = match serde_json::to_string(&body) {
        Ok(body) => body,
        Err(e) => return Err(format!("Serialize error! {:#?}", e)),
    };
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&JsValue::from_str(body.as_str()));

    let link = format!("{}/{}", BASE_URL, &url);

    let request = match Request::new_with_str_and_init(&link, &opts) {
        Ok(request) => request,
        Err(_) => return Err(format!("Cannot instantiate request!")),
    };

    match request.headers().set("Content-Type", "application/json") {
        Ok(_) => {}
        Err(e) => return Err(format!("Cannot setup a 'Content-Type' header. {:#?}", e)),
    }

    let text = get_response(&request).await?;

    match serde_json::from_str(text.as_str()) {
        Ok(response) => Ok(response),
        Err(_) => Err(text), // todo try failed deserialize
    }
}

pub async fn put<T, M>(url: &str, body: T) -> Result<M, String>
where
    T: serde::Serialize,
    M: DeserializeOwned,
{
    let url = url.to_string();
    let body = match serde_json::to_string(&body) {
        Ok(body) => body,
        Err(e) => return Err(format!("Serialize error! {:#?}", e)),
    };
    let opts = RequestInit::new();
    opts.set_method("PUT");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&JsValue::from_str(body.as_str()));

    let link = format!("{}/{}", BASE_URL, &url);

    let request = match Request::new_with_str_and_init(&link, &opts) {
        Ok(request) => request,
        Err(_) => return Err("Cannot instantiate request!".to_string()),
    };

    match request.headers().set("Content-Type", "application/json") {
        Ok(_) => {}
        Err(e) => return Err(format!("Cannot setup a 'Content-Type' header. {:#?}", e)),
    }

    let text = get_response(&request).await?;

    match serde_json::from_str(text.as_str()) {
        Ok(response) => Ok(response),
        Err(_) => Err(text), // todo try failed deserialize
    }
}

pub async fn delete<M>(url: &str) -> Result<M, String>
where
    M: DeserializeOwned,
{
    let url = url.to_string();
    let opts = RequestInit::new();
    opts.set_method("DELETE");
    opts.set_mode(RequestMode::Cors);

    let link = format!("{}/{}", BASE_URL, &url);

    let request = match Request::new_with_str_and_init(&link, &opts) {
        Ok(request) => request,
        Err(_) => return Err("Cannot instantiate request!".to_string()),
    };

    let text = get_response(&request).await?;

    match serde_json::from_str(text.as_str()) {
        Ok(response) => Ok(response),
        Err(_) => Err(text), // todo try failed deserialize
    }
}