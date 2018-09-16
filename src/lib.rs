extern crate crypto;
#[macro_use]
extern crate neon;
extern crate neon_runtime;
extern crate hex;
extern crate ring;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::md5::Md5;
use ring::digest;
use neon::prelude::*;
use neon::types::{JsString};
use neon::context::{FunctionContext};

pub fn sha256(mut call: FunctionContext) -> JsResult<JsString> {
  let input = call.argument::<JsString>(0)?;
  let mut hasher = Sha256::new();
  hasher.input_str(input.value().as_str());
  let hex = hasher.result_str();
  Ok(JsString::new(&mut call, hex.as_str()))
}

pub fn md5(mut call: FunctionContext) -> JsResult<JsString> {
  let input = call.argument::<JsString>(0)?;
  let mut hasher = Md5::new();
  hasher.input_str(input.value().as_str());
  let hex = hasher.result_str();
  Ok(JsString::new(&mut call, hex.as_str()))
}

pub fn sha256_asm(mut call: FunctionContext) -> JsResult<JsString> {
  let input = call.argument::<JsString>(0)?;
  let output = digest::digest(&digest::SHA256, input.value().as_bytes());
  let hex = hex::encode(output.as_ref().to_vec());
  Ok(JsString::new(&mut call, hex.as_str()))
}

register_module!(mut m, {
  m.export_function("sha256", sha256)?;
  m.export_function("md5", md5)?;
  m.export_function("sha256Asm", sha256_asm)?;
  Ok(())
});
