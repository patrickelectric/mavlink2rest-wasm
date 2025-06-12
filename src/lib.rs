use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub struct ParserEmitter {
    value: String,
}

#[wasm_bindgen]
impl ParserEmitter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ParserEmitter {
        ParserEmitter {
            value: String::new(),
        }
    }

    #[wasm_bindgen]
    pub fn parser(&mut self, input: &str) {
        self.value = input.to_string();
    }

    #[wasm_bindgen]
    pub fn emit(&self, callback: &js_sys::Function) -> Result<(), JsValue> {
        let output = format!("{}!", self.value);
        let this = JsValue::NULL;
        callback.call1(&this, &JsValue::from_str(&output))?;
        Ok(())
    }
}