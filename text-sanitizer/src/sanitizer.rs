#![allow(unused)]
/*
* @author Bodo (Hugo) Barwich
* @version 2022-11-03
* @package text-sanitizer
* @subpackage sanitizer.rs

* This module implements the Text-Sanitizer logic
*
*---------------------------------
* Requirements:
*/

use std::collections::HashMap;
use std::str;

pub fn from_utf8_lossy(mut input: &[u8]) -> String {
    let mut sanitized = String::with_capacity(input.len());

    loop {
        //        println!(
        //            "sequence (cnt: '{}'): '{:x?}' - parsing ...",
        //            input.len(),
        //            &input
        //        );

        match std::str::from_utf8(input) {
            Ok(valid) => {
                unsafe {
                    for c in valid.chars() {
                        if c.is_ascii() {
                            sanitized.push(c);
                        } else {
                            //println!("{}", c.escape_unicode());
                            sanitized.push_str(&format!("{}", c.escape_unicode()));
                        }
                    }
                }
                break;
            }
            Err(error) => {
                //println!("err beyond: '{}'", error.valid_up_to());

                let (valid, after_valid) = input.split_at(error.valid_up_to());

                unsafe {
                    let spec = std::str::from_utf8_unchecked(valid);

                    for c in spec.chars() {
                        if c.is_ascii() {
                            sanitized.push(c);
                        } else {
                            //println!("{}", c.escape_unicode());
                            sanitized.push_str(&format!("{}", c.escape_unicode()));
                        }
                    }
                }

                //println!("{:x?}", &after_valid);

                if let Some(invalid_sequence_length) = error.error_len() {
                    //println!("ivld chrs cnt: '{}'", invalid_sequence_length);

                    for iu in 0..invalid_sequence_length {
                        sanitized.push_str(&format!("(?{:x?})", &after_valid[iu]));
                    }

                    input = &after_valid[invalid_sequence_length..]
                } else {
                    break;
                }
            }
        }
    }

    sanitized
}

pub fn build_unicode(sequence: &[u8], bdebug: bool, bquiet: bool) -> Vec<String> {
    let mut build_result: Vec<String> = Vec::new();
    let mut suni: String;
    let mut icstrt = 0;
    let mut icend = sequence.len();
    let mut ivldps = 0;
    let mut bprsgo: bool = true;

    if (bdebug && !bquiet) {
        println!(
            "; sequence 0 (cnt: '{}', strt: '{}', end: '{}'): '{:x?}' - parsing ...",
            sequence.len(),
            icstrt,
            icend,
            &sequence
        );
    } //if(bdebug && ! bquiet)

    while bprsgo && icstrt < icend {
        if (bdebug && !bquiet) {
            println!(
                "; sequence (cnt: '{}', strt: '{}', end: '{}'): '{:x?}' - parsing ...",
                sequence[icstrt..icend].len(),
                icstrt,
                icend,
                &sequence[icstrt..icend]
            );
        } //if(bdebug && ! bquiet)

        let utf8rs = str::from_utf8(&sequence[icstrt..icend]);

        match utf8rs {
            Ok(s) => {
                bprsgo = false;
                build_result.push(s.to_owned());

                if (bdebug && !bquiet) {
                    println!("utf8 ok: '{}'", s);
                }
            }
            Err(e) => {
                if (bdebug && !bquiet) {
                    println!("utf8 Err: '{:?}'", e);
                }

                ivldps = e.valid_up_to();

                if (bdebug && !bquiet) {
                    println!("vld ps: '{}'", ivldps);
                }

                ivldps += icstrt;

                if (bdebug && !bquiet) {
                    println!("vld idx: '{}'", ivldps);
                }

                if ivldps > icstrt {
                    if (bdebug && !bquiet) {
                        println!("utf8 recovered: '{:x?}'", &sequence[icstrt..ivldps]);
                    }

                    unsafe {
                        build_result.push(
                            std::str::from_utf8_unchecked(&sequence[icstrt..ivldps]).to_owned(),
                        );
                    }

                    icstrt = ivldps;
                } else {
                    ivldps = icstrt;
                } //if ivldps > icstrt

                if let Some(invalid_sequence_length) = e.error_len() {
                    if (bdebug && !bquiet) {
                        println!("ivld chrs cnt: '{}'", invalid_sequence_length);
                    }

                    for iu in (ivldps)..(ivldps + invalid_sequence_length) {
                        if (bdebug && !bquiet) {
                            println!("ivld chr: '{:x?}'", &sequence[iu]);
                        }

                        build_result.push(format!("(?{:x?})", &sequence[iu]));
                    } //for iu in (ivldps)..(ivldps + invalid_sequence_length)

                    icstrt = ivldps + invalid_sequence_length;
                } else {
                    //All Bytes are invalid
                    if (bdebug && !bquiet) {
                        println!("ivld chrs cnt: all");
                    }

                    for iu in (ivldps)..(icend) {
                        if (bdebug && !bquiet) {
                            println!("ivld chr: '{:x?}'", &sequence[iu]);
                        }

                        build_result.push(format!("(?{:x?})", &sequence[iu]));
                    } //for iu in (ivldps)..(ivldps + invalid_sequence_length)

                    bprsgo = false;
                } //if let Some(invalid_sequence_length) = e.error_len()
            }
        }; //match utf8rs
    } //while bprsgo

    build_result
}

pub fn parse_unicode(sequence: &[u8], bdebug: bool, bquiet: bool) -> Vec<String> {
    let mut parse_result: Vec<String> = Vec::new();

    let vunicode = build_unicode(&sequence, bdebug, bquiet);

    if (bdebug && !bquiet) {
        println!("uni res: '{:?}'", vunicode);
    }

    if !vunicode.is_empty() {
        for sv in vunicode {
            if sv.starts_with("(?") && sv.ends_with(')') {
                parse_result.push(sv[2..(sv.len() - 1)].to_owned());
            } else {
                for c in sv.chars() {
                    let suni = c.escape_unicode().to_string();

                    parse_result.push(suni[3..(suni.len() - 1)].to_owned());
                } // for c in sv.chars()
            } //if sv.starts_with("(?") && sv.ends_with(')')
        } //for sv in vunicode
    } //if ! vunicode.is_empty()

    parse_result
}

pub fn sanitize_u8(text: &[u8], vrqlanguages: &Vec<String>, options: &str) -> String {
    //-------------------------------------
    //Read the Function Options

    let mut sopt;

    let mut bdbg: bool = false;
    let mut bqt: bool = false;

    if (!options.is_empty()) {
        for prm in options.split_whitespace() {
            if prm.starts_with("--") {
                sopt = prm.split_at(2).1;
                sopt.to_lowercase();
            } else if prm.starts_with('-') {
                sopt = prm.split_at(1).1;
                sopt.to_lowercase();

                match sopt {
                    "q" | "b" => bqt = true,
                    "d" | "v" => bdbg = true,
                    _ => {}
                } //match sopt.as_ref()
            }
        } //for mut prm in options.split_whitespace() {
    } //if(!options.is_empty())

    let mut rplmap = HashMap::new();
    let mut olngrplmap = None;

    let mut lngrplmap = HashMap::with_capacity(10);

    lngrplmap.insert("d", "");
    lngrplmap.insert("1b", "");
    lngrplmap.insert("bb", "\"");
    lngrplmap.insert("ab", "\"");
    lngrplmap.insert("25cf", "*");
    lngrplmap.insert("251c", "|-");
    lngrplmap.insert("2514", "|-");
    lngrplmap.insert("2500", "-");
    lngrplmap.insert("2764", "<3");
    lngrplmap.insert("1f496", "<3");

    rplmap.insert("en", lngrplmap);

    let mut lngrplmap = HashMap::with_capacity(5);

    lngrplmap.insert("df", "ss");
    lngrplmap.insert("dc", "Ue");
    lngrplmap.insert("e4", "ae");
    lngrplmap.insert("fc", "ue");
    lngrplmap.insert("f6", "oe");

    rplmap.insert("de", lngrplmap);

    let mut lngrplmap = HashMap::with_capacity(6);

    lngrplmap.insert("d3", "O");
    lngrplmap.insert("e1", "a");
    lngrplmap.insert("e9", "e");
    lngrplmap.insert("ed", "i");
    lngrplmap.insert("f1", "n");
    lngrplmap.insert("f3", "o");

    rplmap.insert("es", lngrplmap);

    let mut srstxt = String::with_capacity(text.len());
    let mut srptchrs = String::new();
    let mut orpl = None;
    let mut ic: usize = 0;
    let mut icstrt: Option<usize> = None;
    let mut icend: Option<usize> = None;

    if (bdbg && !bqt) {
        println!("vtext 0:'{:?}'", text);
    }

    for uc in text {
        if (bdbg && !bqt) {
            srptchrs.push_str(&format!("; {} - {}:'{}'", ic, uc, char::from(*uc)));
        }

        if (*uc >= 32 as u8 && *uc < 127 as u8) || (*uc == 10 as u8) || (*uc == 9 as u8) {
            //------------------------
            //Valid ASCII Character

            //srptchrs.push_str(" - ascii");

            if icstrt.is_some() {
                //------------------------
                //Pending Non ASCII Characters

                icend = Some(ic);

                if (bdbg && !bqt) {
                    println!(
                        "pdg spec chars '{} - {}': '{:?}'",
                        icstrt.unwrap(),
                        icend.unwrap(),
                        &text[icstrt.unwrap()..icend.unwrap()]
                    );
                }

                //Parse the slice of Non ASCII Characters
                let vuni = parse_unicode(&text[icstrt.unwrap()..icend.unwrap()], bdbg, bqt);

                if (bdbg && !bqt) {
                    print!("= {:?}", vuni);
                }

                for suni in vuni {
                    orpl = None;

                    for slng in vrqlanguages {
                        if orpl.is_none() {
                            olngrplmap = rplmap.get(slng.as_str());

                            if let Some(lngmap) = olngrplmap {
                                orpl = lngmap.get(suni.as_str());
                            }
                        } //if orpl.is_none()
                    } //for slng in vrqlanguages

                    match orpl {
                        Some(rpl) => {
                            srstxt.push_str(rpl);

                            if (bdbg && !bqt) {
                                print!(" -> '{}'", rpl);
                            }
                        }
                        None => {
                            srstxt.push_str(&format!("(?{})", &suni));

                            if (bdbg && !bqt) {
                                print!(" -> '(?{})'", &suni);
                            }
                        } //Some(rpl)
                    } //match orpl
                } //for suni in vuni

                if (bdbg && !bqt) {
                    println!("'");
                } //if(bdbg && ! bqt)
                  /**/

                icstrt = None;
            } //if icstrt.is_some()

            //Add the valid ASCII Character
            srstxt.push(char::from(*uc));
        } else {
            //------------------------
            //Non ASCII Character

            srptchrs.push_str(&format!(" - non-ascii '{:?}", icstrt));

            if icstrt.is_none() {
                icstrt = Some(ic);
                //if(bdbg && ! bqt) {
                srptchrs.push_str(&format!(" > {:?} - {:?}'|", icstrt, icend));
                //}
            } else {
                srptchrs.push_str(&format!(" - {:?}'|", icend));
            } //if (uc[0] >= 32 as u8 && uc[0] < 127 as u8)
        } //if (*uc >= 32 as u8 && *uc < 127 as u8) || ( *uc == 10 as u8 )

        ic += 1;
    } //for uc in text

    if icstrt.is_some() {
        icend = Some(ic);

        if (bdbg && !bqt) {
            print!(
                "\nrst spec char '{} - {}': '{:?}",
                icstrt.unwrap(),
                icend.unwrap(),
                &text[icstrt.unwrap()..icend.unwrap()]
            );
        }

        let vuni = parse_unicode(&text[icstrt.unwrap()..icend.unwrap()], bdbg, bqt);

        if (bdbg && !bqt) {
            print!(" | {:?}", vuni);
        }

        for suni in vuni {
            orpl = None;

            for slng in vrqlanguages {
                if orpl.is_none() {
                    olngrplmap = rplmap.get(slng.as_str());

                    if let Some(lngmap) = olngrplmap {
                        orpl = lngmap.get(suni.as_str());
                    }
                } //if orpl.is_none()
            } //for slng in vrqlanguages

            match orpl {
                Some(rpl) => {
                    srstxt.push_str(rpl);

                    if (bdbg && !bqt) {
                        print!(" -> '{}'", rpl);
                    }
                }
                None => {
                    srstxt.push_str(&format!("(?{})", &suni));

                    if (bdbg && !bqt) {
                        print!(" -> '(?{})'", &suni);
                    }
                } //Some(rpl)
            } //match orpl
        } //for suni in vuni

        if (bdbg && !bqt) {
            print!("'");
        } //if(bdbg && ! bqt)

        icstrt = None;
    } //if icstrt.is_some()

    if (bdbg && !bqt) {
        srptchrs.push_str(&format!("; chr cnt '{}'", ic));

        println!("; sanitze done.");
        println!("chrs rpt: '{:?}'", &srptchrs);

        let vsttrpt: Vec<char> = String::from_utf8_lossy(text).to_mut().chars().collect();

        println!("stt rpt chrs (count : '{}'):\n{:?}", vsttrpt.len(), vsttrpt);

        println!("stt chrs ascii:");

        for c in &vsttrpt {
            if !c.is_ascii() {
                print!("{}|", c.escape_unicode().to_string());
            } else {
                print!("{}|", c);
            }
        } //for c in &vsttrpt

        println!();
    } //if(bdbg && ! bqt)

    //Return the sanitized String
    srstxt
}

pub fn sanitize_string(text: String, vrqlanguages: &Vec<String>, options: &str) -> String {
    sanitize_u8(text.as_bytes(), vrqlanguages, options)
}

//==============================================================================
// Unit Tests

/*
Recreating the Test Data:

    $ perl -e 'my @arrchrs = (240, 159, 151, 119, 150, 139); print pack "U*", @arrchrs;' | target/debug/text-sanitizer -i -d

*/

#[test]
fn sparkle_heart() {
    //-------------------------------------
    //Read the Application Parameters

    let mut vrqlngs: Vec<String> = Vec::new();

    let mut sopt = String::new();
    let vsparkle_heart = vec![240, 159, 146, 150];

    sopt.push_str(" -d");

    vrqlngs.push(String::from("en"));

    let srsout = sanitize_u8(&vsparkle_heart, &vrqlngs, &sopt);

    println!("sparkle_heart: '{}'", srsout);

    assert_eq!(srsout, "<3");
}

#[test]
fn sparkle_heart_broken() {
    //-------------------------------------
    //Read the Application Parameters

    let mut vrqlngs: Vec<String> = Vec::new();

    let mut sopt = String::new();
    let vsparkle_heart = vec![240, 159, 119, 150];

    sopt.push_str(" -d");

    vrqlngs.push(String::from("en"));

    let srsout = sanitize_u8(&vsparkle_heart, &vrqlngs, &sopt);

    println!("sparkle_heart: '{}'", srsout);

    assert_eq!(srsout, "(?f0)(?9f)w(?96)");
}

#[test]
fn two_hearts_center() {
    //-------------------------------------
    //Read the Application Parameters

    let mut vrqlngs: Vec<String> = Vec::new();

    let mut sopt = String::new();
    let vsparkle_heart = vec![240, 159, 146, 150, 119, 250, 240, 159, 146, 150];

    sopt.push_str(" -d");

    vrqlngs.push(String::from("en"));

    let srsout = sanitize_u8(&vsparkle_heart, &vrqlngs, &sopt);

    println!("sparkle_heart: '{}'", srsout);

    assert_eq!(srsout, "<3w(?fa)<3");
}

#[test]
fn two_hearts_start() {
    //-------------------------------------
    //Read the Application Parameters

    let mut vrqlngs: Vec<String> = Vec::new();

    let mut sopt = String::new();
    let vsparkle_heart = vec![250, 240, 159, 146, 150, 119, 240, 159, 146, 150];

    sopt.push_str(" -d");

    vrqlngs.push(String::from("en"));

    let srsout = sanitize_u8(&vsparkle_heart, &vrqlngs, &sopt);

    println!("sparkle_heart: '{}'", srsout);

    assert_eq!(srsout, "(?fa)<3w<3");
}

#[test]
fn two_hearts_end() {
    //-------------------------------------
    //Read the Application Parameters

    let mut vrqlngs: Vec<String> = Vec::new();

    let mut sopt = String::new();
    let vsparkle_heart = vec![240, 159, 146, 150, 119, 240, 159, 146, 150, 250];

    sopt.push_str(" -d");

    vrqlngs.push(String::from("en"));

    let srsout = sanitize_u8(&vsparkle_heart, &vrqlngs, &sopt);

    println!("sparkle_heart: '{}'", srsout);

    assert_eq!(srsout, "<3w<3(?fa)");
}

#[test]
fn hearts_game() {
    //-------------------------------------
    //Read the Application Parameters

    let mut vrqlngs: Vec<String> = Vec::new();

    let mut sopt = String::new();
    let vsparkle_heart = vec![
        226, 157, 164, 240, 159, 146, 150, 119, 250, 248, 240, 159, 146, 150, 247, 190,
    ];

    sopt.push_str(" -d");

    vrqlngs.push(String::from("en"));

    let srsout = sanitize_u8(&vsparkle_heart, &vrqlngs, &sopt);

    println!("sparkle_heart: '{}'", srsout);

    assert_eq!(srsout, "<3<3w(?fa)(?f8)<3(?f7)(?be)");
}
