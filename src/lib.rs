use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub fn foo() -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();
    let dummy_json = json::parse(include_str!("../dummy.json")).unwrap();
    let dummy_yaml = yaml_rust::YamlLoader::load_from_str(include_str!("../dummy.yml")).unwrap();
    Ok(JsValue::from_f64((dummy_json.members().as_slice().len() + dummy_yaml.len()) as f64))
}

