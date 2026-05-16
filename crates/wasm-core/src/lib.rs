use wasm_bindgen::prelude::*;
use needletail::parse_fastx_reader;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use sha2::{Digest, Sha256};
use std::io::Cursor;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[derive(Serialize, Deserialize)]
pub struct FastaHeader {
    pub uuid: String,
    pub label: String,
    pub sequence_hash: String,
    pub length: u64,
}

#[wasm_bindgen]
pub struct WasmParser {
    pipeline_version: String,
}

#[wasm_bindgen]
impl WasmParser {
    #[wasm_bindgen(constructor)]
    pub fn new(pipeline_version: String) -> Self {
        Self { pipeline_version }
    }

    /// Parse a chunk of bytes and return a list of headers and sequence metadata.
    /// This allows JS to stream data from the File API into WASM.
    pub fn parse_chunk(&self, chunk: &[u8]) -> Result<JsValue, JsValue> {
        let mut results = Vec::new();
        let mut reader = Cursor::new(chunk);
        
        let mut fastx_reader = parse_fastx_reader(&mut reader)
            .map_err(|e| JsValue::from_str(&format!("Failed to create fastx reader: {}", e)))?;

        while let Some(record) = fastx_reader.next() {
            let rec = record.map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;
            
            let mut hasher = Sha256::new();
            hasher.update(rec.seq());
            let hash = hex::encode(hasher.finalize());

            results.push(FastaHeader {
                uuid: Uuid::new_v4().to_string(),
                label: String::from_utf8_lossy(rec.id()).trim().to_string(),
                sequence_hash: hash,
                length: rec.seq().len() as u64,
            });
        }

        Ok(serde_wasm_bindgen::to_value(&results)?)
    }
}
