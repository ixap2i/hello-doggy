//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::{*, console_log};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};
use std::result::Result::Ok;
use std::{error, result, assert, print, println, assert_ne, debug_assert_ne};
use std::error::Error;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_futures::future_to_promise;
use web_sys::{Request, RequestInit, RequestMode, Response};
use js_sys::Promise;
use futures::executor::block_on;

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Dog {
    pub message: String,
    pub status: String,
}

impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "json response has error")
    }
}

impl error::Error for Dog {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}
#[cfg(test)]
pub async fn get_a_dog_data() -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = "http://localhost:3000/posts";

    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    let dog_image: Dog = serde_wasm_bindgen::from_value(json)?;
    Ok(serde_wasm_bindgen::to_value(&dog_image)?)
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen_test(async)]
async fn test() {
    let f_to_p = future_to_promise(get_a_dog_data());
    let p_exec = js_sys::Promise::resolve(&f_to_p);

    let f_to_p2 = future_to_promise(get_a_dog_data());

    // 同じ
    console_log!("{:?}", &p_exec);
    console_log!("{:?}", &f_to_p2);
}
