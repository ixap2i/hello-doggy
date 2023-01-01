mod dog;

use crate::dog::Dog;

use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};
use std::error;
use std::error::Error;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonVal {
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PsqlArr {
    pub datas: Vec<JsonVal>
}

impl fmt::Display for PsqlArr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "json response has error")
    }
}

impl error::Error for PsqlArr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[wasm_bindgen]
pub async fn run() -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = "https://dog.ceo/api/breeds/image/random";

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
