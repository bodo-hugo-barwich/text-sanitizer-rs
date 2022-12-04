/*
* @author Bodo (Hugo) Barwich
* @version 2022-11-04
* @package text-sanitizer_app
* @subpackage lib.rs

* This module implements the execution section for the Text-Sanitizer application
*
*---------------------------------
* Requirements:
* - The Rust module "app" must be installed
*/

pub mod app;

use app::RunTextSanitizer;

use std::process::exit;

//==============================================================================
// Auxiliary Functions

fn parse_parameters(application: &mut RunTextSanitizer) {
    //-------------------------------------
    //Read the Script Parameters

    let mut sarg;
    let mut iargidx = 0;

    //eprintln!("args: ");

    // Prints each argument on a separate line
    for argument in std::env::args() {
        //eprintln!("[{}] '{}'", iargidx, argument.as_str());

        if argument.starts_with("--") {
            //Parameter with Double Dash
            sarg = argument.split_at(2).1;

            match sarg.to_lowercase().as_str() {
                "import" => application.set_import(true),
                "verbose" => application.set_quiet(false),
                "debug" => {
                    //Reenable Notices
                    application.set_quiet(false);
                    //Enable Debug Output
                    application.set_debug(true);
                }
                "profiling" => application.set_profiling(true),
                _ => {}
            } //match sarg
        } else if argument.starts_with('-') {
            //Parameter with Single Dash
            sarg = argument.split_at(1).1;

            match sarg.to_lowercase().as_str() {
                "i" => application.set_import(true),
                "v" => application.set_quiet(false),
                "d" => {
                    //Reenable Notices
                    application.set_quiet(false);
                    //Enable Debug Output
                    application.set_debug(true);
                }
                "p" => application.set_profiling(true),
                _ => {}
            } //match sarg
        } else if iargidx > 0 {
            //Any parameter of 2 characters
            if argument.len() == 2 {
                application.add_request_language(&argument)
            }
        } //if argument.starts_with("--")

        iargidx += 1;
    } //for argument in std::env::args()

    //eprintln!("args end.");
}

fn run_app() -> i32 {
    //-------------------------------------
    //Create the Application Object

    let mut sanitizer = RunTextSanitizer::new();

    //Suppress Notices by default
    sanitizer.set_quiet(true);

    parse_parameters(&mut sanitizer);

    if sanitizer.is_debug() && !sanitizer.is_quiet() {
        eprintln!("app dmp 1:\n{:?}", sanitizer);
    }

    //------------------------
    //Execute the Application

    let ierr = sanitizer.do_run();

    //------------------------
    //Build the Report

    if !sanitizer.is_quiet() {
        if ierr == 0 {
            eprintln!("Application finished with [{}]", ierr);
        } else {
            eprintln!("Application failed with [{}]", ierr);
        }
    } //if ! sanitizer.is_quiet()

    ierr
}

//==============================================================================
// Executing Section

pub fn main() {
    let ierr = run_app();

    match ierr {
        0 => {}
        _ => {
            exit(ierr);
        }
    }
}
