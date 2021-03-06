extern crate md5;
#[macro_use]
extern crate neon;
extern crate hex;
extern crate neon_runtime;
extern crate ring;

use std::ops::Deref;

use md5::digest::generic_array::typenum::U16;
use md5::digest::generic_array::GenericArray;
use md5::{Digest, Md5};
use neon::context::FunctionContext;
use neon::prelude::*;
use neon::types::{JsArray, JsBuffer, JsString, JsValue};
use ring::digest;

pub enum Hasher {
  Md5(Md5),
  Sha2(digest::Context),
}

#[derive(Debug)]
pub enum AbstractDigest {
  Md5(GenericArray<u8, U16>),
  Sha2(digest::Digest),
}

impl Hasher {
  #[inline]
  fn update(&mut self, data: &[u8]) {
    match self {
      Hasher::Md5(ctx) => ctx.input(data),
      Hasher::Sha2(ctx) => ctx.update(data),
    }
  }

  #[inline]
  fn finish(self) -> AbstractDigest {
    match self {
      Hasher::Md5(ctx) => AbstractDigest::Md5(ctx.result()),
      Hasher::Sha2(ctx) => AbstractDigest::Sha2(ctx.finish()),
    }
  }
}

impl AsRef<[u8]> for AbstractDigest {
  #[inline]
  fn as_ref(&self) -> &[u8] {
    match self {
      AbstractDigest::Md5(digest) => digest.as_ref(),
      AbstractDigest::Sha2(digest) => digest.as_ref(),
    }
  }
}

pub fn hash(mut call: FunctionContext) -> JsResult<JsValue> {
  let algorithm = call.argument::<JsString>(0)?.value();
  let algorithm = algorithm.as_str();

  let inputs = call.argument::<JsArray>(1)?;
  let inputs = inputs.to_vec(&mut call)?;
  let mut ctx = if algorithm == "md5" {
    Hasher::Md5(Md5::new())
  } else {
    let algorithm = match algorithm {
      "sha256" => &digest::SHA256,
      "sha1" => &digest::SHA1,
      "sha384" => &digest::SHA384,
      "sha512" => &digest::SHA512,
      _ => return call.throw_type_error(&format!("Unsupported algorithm {}", algorithm)),
    };
    Hasher::Sha2(digest::Context::new(&algorithm))
  };
  for buffer in inputs {
    let input = buffer.downcast::<JsBuffer>().unwrap();
    call.borrow(&input, |data| {
          let d = data.deref();
          ctx.update(d.as_slice());
        });
  }
  let data = ctx.finish();
  let hex = hex::encode(data.as_ref());

  Ok(call.string(hex.as_str()).upcast())
}

register_module!(mut m, {
  m.export_function("hash", hash)?;
  Ok(())
});
