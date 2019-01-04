use std::slice;

use neon::handle::Managed;
use neon::types::JsString;
use neon_runtime::raw::Local;

#[repr(C)]
pub struct StringStruct {
  content: *mut u8,
  length: i32,
}

extern "C" {
  #[link_name = "Node_Crypto_Get_V8_String_Unicode_Content"]
  #[inline]
  pub fn unicode_content(str: Local) -> StringStruct;
}

pub trait GetUnicodeContent {
  fn get_unicode_content(&self) -> &'static [u8];
}

impl GetUnicodeContent for JsString {
  #[inline]
  fn get_unicode_content(&self) -> &'static [u8] {
    let local = self.to_raw();
    unsafe {
      let string_struct = unicode_content(local);
      let buffer = string_struct.content;
      let buffer_len = string_struct.length as usize;
      slice::from_raw_parts(buffer, buffer_len)
    }
  }
}
