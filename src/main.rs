#![allow(unused)]



extern crate sanitizer_lib;

use sanitizer_lib::sanitizer;

use std::io::{self, Read};

use std::time::{SystemTime, UNIX_EPOCH};
//use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
//use std::thread::sleep;


//==============================================================================
// Executing Section


#[allow(clippy::cognitive_complexity)]
fn main() {
//let duration_total = SystemTime::now();

//  match duration_total.duration_since(SystemTime::UNIX_EPOCH) {
//    //Ok(n) => eprintln!("0: {:?}", n)
//    , Err(e) => eprintln!("0: SystemTime::now() failed!\nErr: '{:?}'", e)
//  }


  //-------------------------------------
  //Read the Application Parameters

  let mut vrqlngs: Vec<String> = Vec::new();
  let mut sarg;

  let mut sopt = String::new();
  let mut bipt: bool = false;
  let mut bprfg: bool = false;
  let mut bdbg: bool = false;
  let mut bqt: bool = false;


  vrqlngs.push(String::from("en"));

  // Prints each argument on a separate line
  for argument in std::env::args() {
    if argument.starts_with("--") {
      sarg = argument.split_at(2).1;
      sarg.to_lowercase();

      match sarg {
        "input" => bipt = true
        , "profiling" => bprfg = true
        , "quiet" => {
            bqt = true;
            sopt.push_str(" -q");
          }
        , "debug" => {
          bdbg = true;
          sopt.push_str(" -d");
        }
        , _  => {}
      }
    }
    else if argument.starts_with('-') {
      sarg = argument.split_at(1).1;
      sarg.to_lowercase();

      match sarg {
        "i" => bipt = true
        , "p" => bprfg = true
        , "q" | "b" => {
            bqt = true;
            sopt.push_str(" -q");
          }
        , "d" | "v" => {
          bdbg = true;
          sopt.push_str(" -d");
        }
        , _ => {}
      } //match sarg.as_ref()
    } else {
      //Any Parameter
      if argument.len() == 2
        && argument.as_str().to_lowercase() != "en" {
          vrqlngs.push(argument.to_lowercase());
      }
    }  //if argument.starts_with("--")
  } //for argument in std::env::args()


  let mut input = Vec::new();


  if bipt {
    //-------------------------------------
    //Read the Input Data from STDIN

//    let duration_input = SystemTime::now();
    let mut stdin = io::stdin();


//    if bprfg {
//      match duration_input.duration_since(SystemTime::UNIX_EPOCH) {
//        Ok(n) => eprintln!("1: {:?}", n)
//        , Err(e) => eprintln!("1: SystemTime::now() failed!\nErr: '{:?}'", e)
//      }
//    } //if bprfg

    match stdin.read_to_end(&mut input) {
      Ok(_) => {}
      Err(e) => {
        eprintln!("{}", &format!("msg: '{:?}'", e));

        //Clear the invalid Input
        input.clear();
      }
    } //match handle.read_to_string(&mut input)

//    if bprfg {
//      match duration_input.elapsed() {
//        Ok(ti) => eprintln!("Input time: {:?}", ti)
//        , Err(e) => eprintln!("Input time - elapsed() - failed!\nErr: '{:?}'", e)
//      }

//      match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
//        Ok(n) => eprintln!("2: {:?}", n)
//        , Err(e) => eprintln!("2: SystemTime::now() failed!\nErr: '{:?}'", e)
//      }
//    }  //if bprfg
  }  //if bipt


//  if(bdbg && ! bqt) {
//    println!("sleeping 30 secs: go ...");

    // we sleep for 30 seconds
//    sleep(Duration::new(30, 0));

//    println!("sleeping 30 secs: done.");
//  }  //if(bdbg && ! bqt)


  //-------------------------------------
  //Parse the Input Data

//  let duration_parse = SystemTime::now();


//  if bprfg {
//    match duration_parse.duration_since(SystemTime::UNIX_EPOCH) {
//      Ok(tn) => eprintln!("3: {:?}", tn)
//      , Err(e) => eprintln!("3: SystemTime::now() failed!\nErr: '{:?}'", e)
//    }
//  }  //if bprfg

  let srsout = sanitizer::sanitize_u8(&input, &vrqlngs, &sopt);

//  if bprfg {
//    match duration_parse.elapsed() {
//      Ok(tp) => eprintln!("Parse time: {:?}", tp)
//      , Err(e) => eprintln!("Parse time - elapsed() failed!\nErr: '{:?}'", e)
//    }

//    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
//      Ok(tn) => eprintln!("4: {:?}", tn)
//      , Err(e) => eprintln!("4: SystemTime::now() failed!\nErr: '{:?}'", e)
//    }
//  } //if bprfg

  if(bdbg && ! bqt) {
    println!("rs rpt chrs (count : '{}'):\n{:?}", srsout.len(), srsout);
  }

  print!("{}", srsout.as_str());

/*
  match duration_total.elapsed() {
    Ok(tt) => eprintln!("Total time: {:?}", tt)
    , Err(e) => eprintln!("Total time - elapsed() failed!\nErr: '{:?}'", e)
  }
*/

/*
  match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
    Ok(n) => eprintln!("5: {:?}", n)
    , Err(e) => eprintln!("5: SystemTime::now() failed!\nErr: '{:?}'", e)
  }
*/
}
