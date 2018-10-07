use std::ops::Deref;

use neon_runtime::raw::Local;
use neon_runtime::string;
use neon::types::JsString;
use neon::handle::Managed;

extern "C" {
  #[link_name = "Node_Crypto_Get_V8_String_Unicode_Content"]
  pub fn unicode_content(str: Local) -> *mut u8;
}

pub trait GetUnicodeContent {
  fn get_unicode_content(&self) -> JsChars;
}

impl GetUnicodeContent for JsString {
  fn get_unicode_content(&self) -> JsChars {
    let local = self.to_raw();
    let chars = unsafe {
      let buffer_len = string::utf8_len(local) as usize;
      let buffer = unicode_content(local);
      Vec::from_raw_parts(buffer, buffer_len, buffer_len)
    };
    JsChars(chars)
  }
}

#[derive(Debug, Clone)]
pub struct JsChars(Vec<u8>);

impl Drop for JsChars {
  fn drop(&mut self) {}
}

impl Deref for JsChars {
  type Target = [u8];

  fn deref(&self) -> &[u8] {
    self.0.as_slice()
  }
}
