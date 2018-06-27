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
use neon::js::{JsString, Value};
use neon::vm::{Call, FunctionCall, JsResult, This};

trait CheckArgument<'a> {
  fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V>;
}

impl<'a, T: This> CheckArgument<'a> for FunctionCall<'a, T> {
  fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V> {
    self.arguments.require(self.scope, i)?.check::<V>()
  }
}

pub fn sha256(mut call: Call) -> JsResult<JsString> {
  let input = call.check_argument::<JsString>(0)
    ?.value();
  let mut hasher = Sha256::new();
  hasher.input_str(input.as_str());
  let hex = hasher.result_str();
  JsString::new_or_throw(call.scope, hex.as_str())
}

pub fn md5(mut call: Call) -> JsResult<JsString> {
  let input = call.check_argument::<JsString>(0)
    ?.value();
  let mut hasher = Md5::new();
  hasher.input_str(input.as_str());
  let hex = hasher.result_str();
  JsString::new_or_throw(call.scope, hex.as_str())
}

pub fn sha256_asm(mut call: Call) -> JsResult<JsString> {
  let input = call.check_argument::<JsString>(0)
    ?.value();
  let output = digest::digest(&digest::SHA256, input.as_bytes());
  let hex = hex::encode(output.as_ref().to_vec());
  JsString::new_or_throw(call.scope, hex.as_str())
}

register_module!(m, {
  m.export("sha256", sha256)?;
  m.export("md5", md5)?;
  m.export("sha256Asm", sha256_asm)?;
  Ok(())
});
