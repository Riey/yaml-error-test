use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::*;
use yaml_rust::YamlLoader;

#[wasm_bindgen]
pub async fn foo() -> Result<JsValue, JsValue> {
    let win = window().expect("window");
    let mut opts = RequestInit::new();
    opts.method("GET").mode(RequestMode::Cors);
    let req = Request::new_with_str_and_init("dummy.yml", &opts)?;
    req.headers().set("Accept", "text/x-yaml")?;

    let res = JsFuture::from(win.fetch_with_request(&req)).await?;
    let res: Response = res.dyn_into()?;

    let res = if res.status() != 200 {
        return Err(JsValue::UNDEFINED);
    } else {
        let res = JsFuture::from(res.text()?).await?;
        res.as_string().expect("response")
    };

    let msg = JsValue::from_str(&format!("yaml digest: {:X}", md5::compute(&res)));
    console::log_1(&msg);

    let ret = YamlLoader::load_from_str(&res).unwrap();
    Ok(JsValue::from_f64(ret.len() as f64))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
