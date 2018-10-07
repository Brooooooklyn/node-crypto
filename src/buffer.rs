use neon_runtime::raw::Local;
use neon::types::{JsBuffer};
use neon::handle::Managed;

extern "C" {
  #[link_name = "Node_Crypto_New_Buffer_From_Contents"]
  pub fn new_from_content(out: &mut Local, data: *const u8, size: u32) -> bool;
}

pub trait NewFromContent {
  fn new_from_content(data: &[u8]) -> JsBuffer;
}

impl NewFromContent for JsBuffer {
  fn new_from_content(data: &[u8]) -> JsBuffer {
    let len = data.len();
    unsafe {
      let mut local: Local = std::mem::zeroed();
      new_from_content(&mut local, data.as_ptr(), len as u32);
      JsBuffer::from_raw(local)
    }
  }
}
