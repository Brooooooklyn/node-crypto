#[macro_use]
extern crate neon;
extern crate neon_runtime;
extern crate hex;
extern crate ring;

mod buffer;
mod string;

use std::mem;
use std::ops::Deref;

use ring::digest;
use neon::prelude::*;
use neon::handle::Managed;
use neon::borrow::Borrow;
use neon::types::{JsBuffer, JsString, JsValue, JsUndefined};

use buffer::NewFromContent;
use string::GetUnicodeContent;

#[derive(Clone, Copy)]
pub struct CryptoHasher {
  pending: [u8; digest::MAX_BLOCK_LEN],
  algorithm: &'static digest::Algorithm,
  num_pending: usize,
}

declare_types! {
  pub class Hasher for CryptoHasher {
    init(mut cx) {
      let algorithm = cx.argument::<JsString>(0)?.value();
      let algorithm = algorithm.as_str();
      let algorithm = match algorithm {
        "sha256" => &digest::SHA256,
        _ => &digest::SHA1,
      };
      Ok(CryptoHasher {
        pending: [0u8; digest::MAX_BLOCK_LEN],
        num_pending: 0,
        algorithm,
      })
    }

    method update(mut cx) {
      let value = cx.argument::<JsValue>(0)?.as_value(&mut cx);
      {
        let mut this = cx.this();
        let guard = cx.lock();
        let mut hasher = this.borrow_mut(&guard);
        if value.is_a::<JsBuffer>() {
          let buffer = JsBuffer::from_raw(value.to_raw());
          let binary_data = buffer.borrow(&guard);
          let value = binary_data.deref();
          let data = value.as_slice();
          let num_pending = hasher.num_pending;
          hasher.num_pending += data.len();
          hasher.pending[num_pending..(num_pending + data.len())]
            .copy_from_slice(data);
        };
        if value.is_a::<JsString>() {
          let string = JsString::from_raw(value.to_raw());
          let buffer = string.get_unicode_content();
          let num_pending = hasher.num_pending;
          hasher.num_pending += buffer.len();
          hasher.pending[num_pending..(num_pending + buffer.len())]
            .copy_from_slice(&buffer);
          mem::forget(buffer);
        }
        if value.is_a::<JsArrayBuffer>() {
          let buffer = JsArrayBuffer::from_raw(value.to_raw());
          let binary_data = buffer.borrow(&guard);
          let value = binary_data.deref();
          let data = value.as_slice();
          let num_pending = hasher.num_pending;
          hasher.num_pending += data.len();
          hasher.pending[num_pending..(num_pending + data.len())]
            .copy_from_slice(data);
        }
      };
      Ok(cx.undefined().as_value(&mut cx))
    }

    method digest(mut cx) {
      let value = cx.argument::<JsValue>(0)?.as_value(&mut cx);
      let hex = {
        let mut this = cx.this();
        let guard = cx.lock();
        let hasher = this.borrow(&guard);
        let algorithm = hasher.algorithm;
        let data = hasher.pending;
        let hex = digest::digest(algorithm, &data[0..hasher.num_pending]);
        hex::encode(hex)
      };
      if value.is_a::<JsUndefined>() {
        Ok(JsBuffer::new_from_content(hex.as_bytes()).as_value(&mut cx))
      } else if value.is_a::<JsString>() {
        let value = JsString::from_raw(value.to_raw()).value();
        let value = value.as_str();
        match value {
          "hex" => {
            Ok(cx.string(hex.as_str()).as_value(&mut cx))
          },
          _ => {
            cx.type_error("Unsupported digest type").map(|v| v.as_value(&mut cx))
          }
        }
      } else {
        cx.type_error("Unsupported digest type").map(|v| v.as_value(&mut cx))
      }
    }
  }
}

register_module!(mut m, {
  m.export_class::<Hasher>("Hasher")?;
  Ok(())
});
