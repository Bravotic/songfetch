// Songfetch - Created by Collin McKinley [https://nootnoot.co]
// Backend for @songsterr_search_bot on telegram (Created by @dirtydan63) 

extern crate reqwest;
extern crate regex;
use std::env;

extern crate serde_json;

use serde_json::{Value, Error};

fn main() {
  let args: Vec<String> = env::args().collect();
  let pattern = args[1].replace(" ", "%20");
  let body = reqwest::get(format!("https://www.songsterr.com/api/songs?pattern={}&size=200", pattern).as_str()).unwrap()
    .text().unwrap();
  let v: Value = serde_json::from_str(body.as_str()).unwrap();
  let mut arraynum = 0;
  let mut done = false;
  while !done { 

    let jsontitle = &v[arraynum]["title"];
    let jsonartist = &v[arraynum]["artist"];
    let jsonid = &v[arraynum]["songId"];



    if jsontitle != &v[arraynum]["artist"] { 

    let url = format!("https://www.songsterr.com/a/wsa/{}-tab-s{}", jsontitle.as_str().unwrap().to_string().replace(" ", "-"), jsonid);
    println!("=-= [{} - {}] =-=", jsonartist.as_str().unwrap().to_string(), jsontitle.as_str().unwrap().to_string());
    println!("{} \n", url);
    arraynum = arraynum + 1;
    }
    else {
      done=true;      
    }
  }
}
