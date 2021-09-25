#![allow(unused)]
use std::char;

fn main() {
    let vsparkle_heart = &[240, 159, 119, 150];
    let mut ic: usize = 0;
    let mut sasciiheart = String::with_capacity(vsparkle_heart.len());

    for uc in vsparkle_heart {
        print!(
            "; {} - {}:'{}', {:?}",
            ic,
            uc,
            char::from(*uc),
            (*uc >= 32 as u8 && *uc < 127 as u8) || (*uc == 10 as u8)
        );

        if (*uc >= 32 as u8 && *uc < 127 as u8) || (*uc == 10 as u8) {
            print!(" - ascii");

            //Add the valid ASCII Character
            sasciiheart.push(char::from(*uc));
        } else {
            print!(" - non-ascii");
        }

        ic += 1;
    }

    print!("; chr cnt: '{}'", ic);
    println!("\nascii: '{}'", &sasciiheart);
}
