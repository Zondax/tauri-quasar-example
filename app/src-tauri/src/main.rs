#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

use serde::Serialize;
use std::str;
extern crate peg;
use arithmetic::expression;

peg::parser!( grammar arithmetic() for str {
    pub rule expression() -> i64
        = sum()

    rule sum() -> i64
        = l:product() "+" r:product() { l+r }
        / product()

    rule product() -> i64
        = l:atom() "*" r:atom() { l*r }
        / atom()

    rule atom() -> i64
        = number()
        / "(" v:sum() ")" { v }

    rule number() -> i64
        = n:$(['0'..='9']+) { n.parse().unwrap() }
});

fn main() {
  tauri::AppBuilder::new()
    .setup(|webview, _| {
      let handle = webview.handle();
      tauri::event::listen(String::from("add"), move |msg| {
        #[derive(Serialize)]
        pub struct Reply {
          pub msg: String
        }

        let result = expression(&String::from(msg));
        println!("Sum = {:#?}", result);

        let reply = Reply {
          msg: format!("{:?}", result).to_string(),
        };

        tauri::event::emit(
          &handle,
          String::from("reply"),
          serde_json::to_string(&reply).unwrap(),
        );

      });
    })
    .build()
    .run();
}
