#[macro_use]
extern crate neon;
extern crate neon_runtime;
extern crate hex;
extern crate ring;

mod string;

use std::ops::Deref;

use ring::digest;
use neon::prelude::*;
use neon::types::{JsString};
use neon::context::{FunctionContext};

use string::GetUnicodeContent;

pub fn sha256(mut call: FunctionContext) -> JsResult<JsString> {
  let input = call.argument::<JsString>(0)?;
  let buffer = input.deref().get_unicode_content();
  let output = digest::digest(&digest::SHA256, &buffer);
  let hex = hex::encode(output);
  Ok(JsString::new(&mut call, hex.as_str()))
}

register_module!(mut m, {
  m.export_function("sha256", sha256)?;
  Ok(())
});
