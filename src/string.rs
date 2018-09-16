use neon_runtime::raw::Local;
use neon_runtime::string;
use neon::types::JsString;
use neon::handle::Managed;

extern "C" {
  #[link_name = "Node_Crypto_Get_V8_String_Unicode_Content"]
  pub fn unicode_content(str: Local) -> *mut u8;
}

pub trait GetUnicodeContent {
  fn get_unicode_content(&self) -> Vec<u8>;
}

impl GetUnicodeContent for JsString {
  fn get_unicode_content(&self) -> Vec<u8> {
    let local = self.to_raw();
    unsafe {
      let buffer_len = string::utf8_len(local) as usize;
      let buffer = unicode_content(local);
      Vec::from_raw_parts(buffer, buffer_len, buffer_len)
    }
  }
}
