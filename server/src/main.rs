extern crate project1;
extern crate tokio;

use tokio::prelude::*;

struct Struct;
struct YetAnother;

type Res = Vec<u8>;

fn another() -> Res {
     Vec::new()
}

fn fetch_struct() -> Struct {
     Struct
}

/// Some documentation.
fn some_function() -> u32 {
     42
}

fn main() {
     let x = fetch_struct();
     let a = 3;
     let b = a + 2;

     let one = future::lazy(|| Ok(1));
     let add = project1::addone(one);
     let done = add.and_then(|v| {
          println!("v={}", v);
          Ok(())
     });

     tokio::run(done);
}
