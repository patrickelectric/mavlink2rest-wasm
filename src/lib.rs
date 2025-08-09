use wasm_bindgen::prelude::*;
use mavlink::{read_versioned_msg, write_versioned_msg, MavHeader, MavlinkVersion};
use serde::{Deserialize, Serialize};
use js_sys::Uint8Array;

#[wasm_bindgen]
pub struct ParserEmitter {
    data: Vec<u8>,
    sequence: u8,
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
            sequence: 0,
        }
    }

    #[wasm_bindgen]
    pub fn parser(&mut self, input: &[u8]) {
        self.data.extend_from_slice(input);
    }

    #[wasm_bindgen]
    pub fn emit(&mut self, callback: &js_sys::Function) -> Result<(), JsValue> {
        while !self.data.is_empty() {
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
                    break; // Exit loop on parse error
                }
            }
        }
        Ok(())
    }

    #[wasm_bindgen]
    pub fn rest2mavlink(&mut self, input: js_sys::JsString) -> Result<JsValue, JsValue> {
        let Some(json_string) = input.as_string() else {
            return Ok(JsValue::from_str("[]"));
        };

        if let Ok(content) =
            json5::from_str::<MAVLinkMessage<mavlink::ardupilotmega::MavMessage>>(&json_string)
        {
            let mut data = Vec::new();
            let header = MavHeader {
                sequence: self.sequence,
                system_id: content.header.system_id,
                component_id: content.header.component_id,
            };
            let _ = write_versioned_msg(&mut data, MavlinkVersion::V2, header, &content.message);
            self.sequence += 1;
            let array = Uint8Array::new_with_length(data.len() as u32);
            array.copy_from(&data);
            return Ok(array.into());
        }

        if let Ok(content) =
            json5::from_str::<MAVLinkMessage<mavlink::common::MavMessage>>(&json_string)
        {
            let mut data = Vec::new();
            let header = MavHeader {
                sequence: self.sequence,
                system_id: content.header.system_id,
                component_id: content.header.component_id,
            };
            let message = content.message;
            let _ = write_versioned_msg(&mut data, MavlinkVersion::V2, header, &message);
            self.sequence += 1;
            let array = Uint8Array::new_with_length(data.len() as u32);
            array.copy_from(&data);
            return Ok(array.into());
        }
        Ok(JsValue::from_str("[]"))
    }
}