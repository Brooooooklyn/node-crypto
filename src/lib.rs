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

pub struct CryptoHasher {
  ctx: digest::Context,
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
        ctx: digest::Context::new(algorithm)
      })
    }

    method update(mut cx) {
      let value = cx.argument::<JsValue>(0)?.as_value(&mut cx);
      {
        let mut this = cx.this();
        let guard = cx.lock();
        let hasher = this.borrow_mut(&guard);
        let mut ctx = hasher.ctx;
        if value.is_a::<JsBuffer>() {
          let buffer = JsBuffer::from_raw(value.to_raw());
          let binary_data = buffer.borrow(&guard);
          let value = binary_data.deref();
          ctx.update(value.as_slice());
        };
        if value.is_a::<JsString>() {
          let string = JsString::from_raw(value.to_raw());
          let buffer = string.get_unicode_content();
          ctx.update(&buffer);
          mem::forget(buffer);
        }
        if value.is_a::<JsArrayBuffer>() {
          let buffer = JsArrayBuffer::from_raw(value.to_raw());
          let binary_data = buffer.borrow(&guard);
          let value = binary_data.deref();
          ctx.update(value.as_slice());
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
        let mut ctx = hasher.ctx;
        let result = ctx.finish();
        hex::encode(result)
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
