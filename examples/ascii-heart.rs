#![allow(unused)]
use std::char;

fn main() {
    let vsparkle_heart = &[240, 159, 151, 119, 150];
    let mut ic: usize = 0;
    let mut sasciiheart = String::with_capacity(vsparkle_heart.len());
	let mut sreport = String::new();


    for uc in vsparkle_heart {
        sreport.push_str(&format!(
            "; {} - {}:'{}', {:?}",
            ic,
            uc,
            char::from(*uc),
            (*uc >= 32 as u8 && *uc < 127 as u8) || (*uc == 10 as u8)
        ));

        if (*uc >= 32 as u8 && *uc < 127 as u8) || (*uc == 10 as u8) {
            sreport.push_str(" - ascii");

            //Add the valid ASCII Character
            sasciiheart.push(char::from(*uc));
        } else {
            sreport.push_str(" - non-ascii");
        }

        ic += 1;
    }


    sreport.push_str(&format!("; chr cnt: '{}'", ic));

	println!("report: '{:?}'", &sreport);

	println!("report: '{}'", &sreport);

      let vsttrpt: Vec<char> = String::from_utf8_lossy(sreport.as_bytes()).to_mut().chars().collect();

      println!("stt rpt chrs (count : '{}'):\n{:?}", vsttrpt.len(), vsttrpt);

      println!("stt chrs ascii:");

      for c in &vsttrpt {
          if ! c.is_ascii() {
              print!("{}|", c.escape_unicode().to_string());
          } else {
              print!("{}|", c);
          }
      } //for c in &vsttrpt

      println!();


    println!("ascii: '{}'", &sasciiheart);

}
