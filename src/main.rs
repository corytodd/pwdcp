extern crate clipboard;
use std::env;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

fn main() {
    let raw_path = env::current_dir().unwrap();
    let os_path = raw_path.into_os_string().into_string().unwrap();
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(os_path.to_owned()).unwrap();
}
