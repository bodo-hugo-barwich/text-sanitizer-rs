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

    assert_eq!(srsout, "💖");

}
