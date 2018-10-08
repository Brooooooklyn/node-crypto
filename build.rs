extern crate neon_build;

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
  neon_build::setup();

  let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
  let ext = match target_os.as_str() {
    "macos" => "dylib",
    "windows" => "dll",
    "linux" => "so",
    _ => panic!("Unsupported OS"),
  };
  let prefix = match target_os.as_str() {
    "macos" => "lib",
    "windows" => "",
    "linux" => "lib",
    _ => panic!("Unsupported OS"),
  };
  let file_name = format!("{}crypto.{}", prefix, ext);
  let mut dist = PathBuf::from(env::current_dir().unwrap());
  dist.push("copy-binary.js");
  let code = format!(
                     "
const fs = require('fs')
const path = require('path')

fs.copyFileSync(path.join('target', '{}' ,'{}'), path.join(process.cwd(), 'native.node'))
",
                     env::var("PROFILE").unwrap(),
                     file_name
  );
  fs::write(dist, code).unwrap();
}
