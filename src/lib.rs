extern crate md5;
#[macro_use]
extern crate neon;
extern crate hex;
extern crate neon_runtime;
extern crate ring;

mod string;

use std::mem;
use std::ops::Deref;

use md5::{Digest, Md5};
use neon::context::FunctionContext;
use neon::prelude::*;
use neon::types::JsString;
use ring::digest;

use string::GetUnicodeContent;

pub fn hash(mut call: FunctionContext) -> JsResult<JsValue> {
  let algorithm = call.argument::<JsString>(0)?.value();
  let algorithm = algorithm.as_str();

  let input = call.argument::<JsString>(1)?;
  let input = input.deref();
  let buffer = input.get_unicode_content();

  let hex = if algorithm == "md5" {
    let mut hasher = Md5::new();
    hasher.input(&buffer);
    hex::encode(hasher.result())
  } else {
    let algorithm = match algorithm {
      "sha256" => &digest::SHA256,
      "sha1" => &digest::SHA1,
      "sha384" => &digest::SHA384,
      "sha512" => &digest::SHA512,
      _ => return call.throw_type_error(&format!("Unsupported algorithm {}", algorithm)),
    };
    hex::encode(digest::digest(algorithm, &buffer))
  };

  mem::forget(buffer);
  Ok(JsString::new(&mut call, hex.as_str()).as_value(&mut call))
}

register_module!(mut m, {
  m.export_function("hash", hash)?;
  Ok(())
});
