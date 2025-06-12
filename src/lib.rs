use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub struct ParserEmitter {
    data: Vec<u8>,
}

#[wasm_bindgen]
impl ParserEmitter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ParserEmitter {
        ParserEmitter {
            data: Vec::new(),
        }
    }

    #[wasm_bindgen]
    pub fn parser(&mut self, input: &[u8]) {
        self.data = input.to_vec();
    }

    #[wasm_bindgen]
    pub fn emit(&self, callback: &js_sys::Function) -> Result<(), JsValue> {
        let size = self.data.len() as u32;
        let this = JsValue::NULL;
        callback.call1(&this, &JsValue::from_f64(size as f64))?;
        Ok(())
    }
}