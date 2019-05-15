extern crate md5;
#[macro_use]
extern crate neon;
extern crate hex;
extern crate neon_runtime;
extern crate ring;

use std::ops::Deref;
use std::ptr;
use std::slice;

use md5::digest::generic_array::typenum::U16;
use md5::digest::generic_array::GenericArray;
use md5::{Digest, Md5};
use neon::context::FunctionContext;
use neon::handle::Managed;
use neon::prelude::*;
use neon::types::{JsBuffer, JsString, JsValue};
use neon_runtime::buffer;
use ring::digest;

#[derive(Clone)]
pub enum Hasher {
  Md5(Md5),
  Sha2(digest::Context),
}

unsafe impl Send for Hasher {}
unsafe impl Sync for Hasher {}

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

pub struct RustHasher {
  ctx: Hasher,
}

declare_types! {
  pub class JsHasher for RustHasher {
    init(mut call) {
      let algorithm = call.argument::<JsString>(0)?.value();
      let algorithm = algorithm.as_str();
      let ctx = if algorithm == "md5" {
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
      Ok(RustHasher { ctx })
    }

    method update(mut call) {
      let mut this = call.this();
      let input = call.argument::<JsValue>(0)?;
      call.borrow_mut(&mut this, move |mut hasher| {
        let mut data_ptr = ptr::null_mut();
        let mut len = 0;
        unsafe {
          buffer::data(&mut data_ptr, &mut len, input.to_raw());
          let data = slice::from_raw_parts(data_ptr as *mut u8, len);
          hasher.ctx.update(data);
        };
      });
      Ok(call.undefined().upcast())
    }

    method digest(mut call) {
      let this = call.this();
      let ctx = call.borrow(&this, |hasher| {
        hasher.ctx.clone()
      });
      let data = ctx.finish();
      let hex = hex::encode(data.as_ref());

      Ok(call.string(hex.as_str()).upcast())
    }
  }
}

pub fn hash(mut call: FunctionContext) -> JsResult<JsValue> {
  let algorithm = call.argument::<JsString>(0)?.value();
  let algorithm = algorithm.as_str();

  let inputs = call.argument::<JsBuffer>(1)?;
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
  let buf = inputs.downcast::<JsBuffer>().unwrap();
  call.borrow(&buf, |data| {
        let d = data.deref();
        ctx.update(d.as_slice());
      });
  let data = ctx.finish();
  let hex = hex::encode(data.as_ref());

  Ok(call.string(hex.as_str()).upcast())
}

register_module!(mut m, {
  m.export_function("hash", hash)?;
  m.export_class::<JsHasher>("JsHasher")?;
  Ok(())
});
