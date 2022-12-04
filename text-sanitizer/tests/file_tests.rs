/*
* @author Bodo (Hugo) Barwich
* @version 2022-11-04
* @package text-sanitizer
* @subpackage tests/file_tests.rs

* This module runs tests on the Text-Sanitizer library
* It processes test files and checks the results against result files
*
*---------------------------------
* Requirements:
* - The Rust crate "text-sanitizer" must be installed
*/



#[cfg(test)]
mod file_tests {
    //use std::process::exit;
    use std::ffi::{OsStr, OsString};
    use std::fs::read;
    use std::io::{Error, ErrorKind};
    use std::path::{Path, PathBuf};

    extern crate text_sanitizer;

    use text_sanitizer::sanitizer;

    // The derive implements <RuntimeOptions> == <RuntimeOptions> comparisons
    #[derive(PartialEq)]
    enum RuntimeOptions {
        Quiet,
        Debug,
    }

    //==============================================================================
    // Auxiliary Functions

    fn find_path_parent(top: &Path, name: &str) -> Option<PathBuf> {
        let mut odir = None;

        let osearch = Some(OsStr::new(name));

        for p in top.ancestors() {
            if odir.is_none() && p.is_dir() && p.file_name() == osearch {
                odir = Some(p);
            }
        }

        if let Some(d) = odir {
            odir = d.parent();
        }

        match odir {
            Some(d) => Some(PathBuf::from(d)),
            None => None,
        }
    }

    fn find_maindir(options: &[RuntimeOptions]) -> Result<PathBuf, Error> {
        let omdpth = match std::env::current_exe() {
            Ok(p) => Some(p),
            Err(_) => {
                eprintln!("Module Path unknown!");
                None
            }
        };
        let omdnm = match &omdpth {
            Some(p) => {
                match p.as_path().file_name() {
                    Some(f) => Some(PathBuf::from(f)),
                    None => {
                        eprintln!("Module Name unknown!");
                        None
                    }
                } //match p.as_path().file_name()
            } //Some(p)
            None => None,
        };

        let owrkdir = match &omdpth {
            Some(pth) => match pth.as_path().parent() {
                Some(prnt) => Some(PathBuf::from(prnt)),
                None => None,
            },
            None => None,
        };

        let mut omndir = match &owrkdir {
            Some(wdir) => Some(PathBuf::from(wdir)),
            None => None,
        };

        match &omndir {
            Some(mdir) => {
                match find_path_parent(mdir.as_path(), "target") {
                    Some(tdir) => omndir = Some(tdir),
                    None => {
                        if let Some(bdir) = find_path_parent(mdir.as_path(), "bin") {
                            omndir = Some(bdir)
                        }
                    }
                } //match get_some_path_parent(&mdir, "target")
            }
            None => {}
        } //if let Some(mdir) = omndir

        if options.contains(&RuntimeOptions::Debug) && !options.contains(&RuntimeOptions::Quiet) {
            println!("md pth : '{:?}'", omdpth);
            println!("md nm : '{:?}'", omdnm);
            println!("mndir: '{:?}'", omndir);
            println!("wrkdir: '{:?}'", owrkdir);
        } //if options.contains(RuntimeOptions::Debug) && ! options.contains(RuntimeOptions::Quiet)

        let mut cnfdir = match &omndir {
            Some(mdir) => PathBuf::from(mdir),
            None => PathBuf::from(""),
        };
        let mut logdir = match &omndir {
            Some(mdir) => PathBuf::from(mdir),
            None => PathBuf::from(""),
        };

        cnfdir.push("config");
        logdir.push("logs");

        if !cnfdir.exists() {
            cnfdir = match &owrkdir {
                Some(wdir) => PathBuf::from(wdir),
                None => PathBuf::from(""),
            };
        }

        if !logdir.exists() {
            logdir = match &owrkdir {
                Some(wdir) => PathBuf::from(wdir),
                None => PathBuf::from(""),
            };
        }

        if options.contains(&RuntimeOptions::Debug) && !options.contains(&RuntimeOptions::Quiet) {
            println!("cnf dir 1: '{}'", cnfdir.to_str().unwrap());
            println!("log dir 1: '{}'", logdir.to_str().unwrap());
        }

        match omndir {
            Some(d) => Ok(d),
            None => Err(Error::new(
                ErrorKind::NotFound,
                "Main Directory: could not extend Directory from Executable Path",
            )),
        } //match omndir
    }

    fn list_testdata(
        datadir: &Path,
        lstfiles: &mut Vec<PathBuf>,
        options: &[RuntimeOptions],
    ) -> Result<usize, Error> {
        for entry in datadir.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                if options.contains(&RuntimeOptions::Debug)
                    && !options.contains(&RuntimeOptions::Quiet)
                {
                    println!("dta fl: '{:?}'", &entry.path());
                }

                lstfiles.push(entry.path());
            } //if let Ok(entry) = entry
        } //for entry in datadir.read_dir().expect("read_dir call failed")

        Ok(lstfiles.len())
    }

    fn read_file(sourcefile: &Path) -> Result<Vec<u8>, Error> {
        let vcntnt = read(sourcefile)?;

        Ok(vcntnt)
    }

    fn parse_basename(filename: &Path) -> OsString {
        let mut bsnmrs = match filename.file_stem() {
            Some(base) => OsString::from(base),
            None => OsString::from(""),
        };

        if let Some(base) = Path::new(&bsnmrs).file_stem() {
            bsnmrs = OsString::from(base);
        }

        bsnmrs
    }

    fn parse_extension(filename: &Path) -> OsString {
        let extlstrs = match filename.extension() {
            Some(ext) => OsString::from(ext),
            None => OsString::from(""),
        };
        let flnmnoext = filename.with_extension("");
        let mut extrs = match flnmnoext.extension() {
            Some(ext) => OsString::from(ext),
            None => OsString::from(""),
        };

        if !extlstrs.is_empty() {
            extrs.push(".");
            extrs.push(&extlstrs);
        }

        extrs
    }

    fn test_file(datafile: &Path, options: &[RuntimeOptions]) -> Result<(), Error> {
        let vtstdta = read_file(datafile)?;
        let flbsnm = parse_basename(datafile).into_string().unwrap();
        let mut rsflnm = OsString::from(&flbsnm);
        let flext = parse_extension(datafile);

        if options.contains(&RuntimeOptions::Debug) && !options.contains(&RuntimeOptions::Quiet) {
            println!("fl bs nm: '{:?}'\nfl ext: '{:?}'", flbsnm, flext);
        }

        if !flbsnm.ends_with("_result") {
            rsflnm.push("_result");
            rsflnm.push(&flext);

            let resultfile = datafile.with_file_name(&rsflnm);

            if options.contains(&RuntimeOptions::Debug) && !options.contains(&RuntimeOptions::Quiet)
            {
                println!("fl rs nm: '{:?}'", resultfile);
                println!(
                    "fl tst '{}{}' -> fl rs '{:?}'",
                    &flbsnm,
                    String::from(flext.to_str().unwrap()),
                    resultfile.file_name()
                );
            }

            let vrsdta = read_file(&resultfile)?;
            let srsdta = String::from_utf8_lossy(&vrsdta).into_owned();

            let mut vrqlngs: Vec<String> = Vec::new();

            let mut sopt = String::new();

            sopt.push_str(" -d");

            vrqlngs.push(String::from("en"));
            vrqlngs.push(String::from("es"));
            vrqlngs.push(String::from("de"));

            let srsout = sanitizer::sanitize_u8(&vtstdta, &vrqlngs, &sopt);

            if options.contains(&RuntimeOptions::Debug) && !options.contains(&RuntimeOptions::Quiet)
            {
                println!(
                    "fl '{}{}' san dmp:\n'{}'",
                    &flbsnm,
                    String::from(flext.to_str().unwrap()),
                    srsout
                );
            }

            assert_eq!(srsout, srsdta);
        } //if ! flbsnm.ends_with("_result")

        Ok(())
    }

    #[test]
    fn list_files() {
        let mut lstdatafiles: Vec<PathBuf> = Vec::new();

        let maindir = find_maindir(&[RuntimeOptions::Debug]).expect("maindir not found");
        let mut datadir = maindir.clone();

        datadir.push("tests/data");

        println!("mn dir: '{:?}'\ndta dir: '{:?}'", maindir, datadir);

        assert!(list_testdata(&datadir, &mut lstdatafiles, &[RuntimeOptions::Debug]).is_ok());

        println!("lst fls: '{:?}'", lstdatafiles);

        for file in lstdatafiles {
            assert!(test_file(&file, &[RuntimeOptions::Debug]).is_ok());
        }
    }
}
