use wasm_bindgen::prelude::*;
use mavlink::{read_versioned_msg, Message, MavlinkVersion};

use serde::{Deserialize, Serialize};



#[wasm_bindgen]
pub struct ParserEmitter {
    data: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MAVLinkMessage<T> {
    pub header: mavlink::MavHeader,
    pub message: T,
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
        self.data.extend_from_slice(input);
    }

    #[wasm_bindgen]
    pub fn emit(&mut self, callback: &js_sys::Function) -> Result<(), JsValue> {
        let mut cursor = std::io::Cursor::new(&self.data);
        match read_versioned_msg::<mavlink::ardupilotmega::MavMessage, _>(&mut cursor, MavlinkVersion::V2) {
            Ok((header, msg)) => {
                // Get the size of the successfully parsed message
                let bytes_read = cursor.position() as usize;

                // Remove the parsed bytes from the buffer
                self.data.drain(0..bytes_read);

                // Call the callback with the number of bytes read
                let this = JsValue::NULL;

                let value = serde_json::to_string(&MAVLinkMessage { header, message: msg }).unwrap();
                callback.call1(&this, &JsValue::from(value))?;
            }
            Err(_) => {
                // If we can't parse a message and the buffer is too large, clear it
                if self.data.len() > 1024 {
                    self.data.clear();
                }
            }
        }
        Ok(())
    }
}