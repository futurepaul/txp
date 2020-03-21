use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct ParsedTx {
  pieces: Vec<TxPiece>,
}

#[derive(Serialize)]
pub struct TxPiece {
  kind: String,
  value: String,
}

impl ParsedTx {
  fn demo(tx: &str) -> Self {
    Self {
      pieces: vec![
        TxPiece {
          kind: "kind one".to_string(),
          value: format!("sample value: {}", tx),
        },
        TxPiece {
          kind: "kind two".to_string(),
          value: format!("another sample value: {}", tx),
        },
      ],
    }
  }
}

#[wasm_bindgen]
pub fn hello_txp(tx: &str) -> JsValue {
  let demo = ParsedTx::demo(tx);

  JsValue::from_serde(&demo).unwrap()
}
