/*
Recreating the Test Data:

	$ perl -e 'my @arrchrs = (240, 159, 151, 119, 150, 139); print pack "U*", @arrchrs;' | target/debug/text-sanitizer -i -d

*/



extern crate sanitizer_lib;

use sanitizer_lib::sanitizer;


#[test]
fn sparkle_heart() {
  //-------------------------------------
  //Read the Application Parameters

  let mut vrqlngs: Vec<String> = Vec::new();

  let mut sopt = String::new();
	let vsparkle_heart = vec![240, 159, 146, 150];


	sopt.push_str(" -d");

  vrqlngs.push(String::from("en"));


  let srsout = sanitizer::sanitize_u8(&vsparkle_heart, &vrqlngs, &sopt);


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


  let srsout = sanitizer::sanitize_u8(&vsparkle_heart, &vrqlngs, &sopt);


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


  let srsout = sanitizer::sanitize_u8(&vsparkle_heart, &vrqlngs, &sopt);


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


  let srsout = sanitizer::sanitize_u8(&vsparkle_heart, &vrqlngs, &sopt);


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


  let srsout = sanitizer::sanitize_u8(&vsparkle_heart, &vrqlngs, &sopt);


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


  let srsout = sanitizer::sanitize_u8(&vsparkle_heart, &vrqlngs, &sopt);


    println!("sparkle_heart: '{}'", srsout);

    assert_eq!(srsout, "<3<3w(?fa)(?f8)<3(?f7)(?be)");

}

