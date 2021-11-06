

#[cfg(test)]
mod file_tests {
	//use std::process::exit;
	use std::path::{Path, PathBuf};
	use std::io::{Error, ErrorKind};
	use std::ffi::OsStr;


  // The derive implements <RuntimeOptions> == <RuntimeOptions> comparisons
  #[derive(PartialEq)]
  enum RuntimeOptions {
     Quiet
    , Debug
  }




	//==============================================================================
	// Auxiliary Functions


	fn find_path_parent(top: &Path, name: &str) -> Option<PathBuf> {
	  let mut odir = None;

	  let osearch = Some(OsStr::new(name));

	  for p in top.ancestors() {
	    if odir.is_none()
	      && p.is_dir()
	      && p.file_name() == osearch {
	      odir = Some(p);
      }
	  }

		if let Some(d) = odir {
		  odir = d.parent();
		}

		match odir {
		  Some(d) => Some(PathBuf::from(d))
		  , None => None
		}
	}


	fn list_testdata(datadir: &Path, _options: &[RuntimeOptions]) -> Result<i32, Error> {
		for entry in datadir.read_dir().expect("read_dir call failed") {
		    if let Ok(entry) = entry {
		        println!("{:?}", entry.path());
		    }
		}


		Ok(4)
	}

	fn find_maindir(options: &[RuntimeOptions]) -> Result<PathBuf, Error> {
    let omdpth = match std::env::current_exe() {
      Ok(p) => Some(p)
      , Err(_) => {
        eprintln!("Module Path unknown!");
        None
      }
    };
    let omdnm = match & omdpth {
      Some(p) => {
        match p.as_path().file_name() {
          Some(f) => Some(PathBuf::from(f))
          , None => {
            eprintln!("Module Name unknown!");
            None
          }
        } //match p.as_path().file_name()
      } //Some(p)
      , None => None
    };

    let owrkdir = match & omdpth {
      Some(pth) => match pth.as_path().parent() {
        Some(prnt) => Some(PathBuf::from(prnt))
        , None => None
      }
      , None => None
    };

    let mut omndir = match & owrkdir {
      Some(wdir) => Some(PathBuf::from(wdir))
      , None => None
    };

    match &omndir {
      Some(mdir) => {
        match find_path_parent(mdir.as_path(), "target") {
          Some(tdir) => omndir = Some(tdir)
          , None => {
            if let Some(bdir) = find_path_parent(mdir.as_path(), "bin") {
              omndir = Some(bdir)
            }
          }
        } //match get_some_path_parent(&mdir, "target")
      }
      , None => {}
    } //if let Some(mdir) = omndir


  if options.contains(&RuntimeOptions::Debug)
    && ! options.contains(&RuntimeOptions::Quiet) {
    println!("md pth : '{:?}'", omdpth);
    println!("md nm : '{:?}'", omdnm);
    println!("mndir: '{:?}'", omndir);
    println!("wrkdir: '{:?}'", owrkdir);
  } //if options.contains(RuntimeOptions::Debug) && ! options.contains(RuntimeOptions::Quiet)


  let mut cnfdir = match &omndir {
    Some(mdir) => PathBuf::from(mdir)
    , None => PathBuf::from("")
  };
  let mut logdir = match & omndir {
    Some(mdir) => PathBuf::from(mdir)
    , None => PathBuf::from("")
  };

  cnfdir.push("config");
  logdir.push("logs");

  if ! cnfdir.exists() {
    cnfdir = match & owrkdir {
      Some(wdir) => PathBuf::from(wdir)
      , None => PathBuf::from("")
    };
  }

  if ! logdir.exists() {
    logdir = match & owrkdir {
      Some(wdir) => PathBuf::from(wdir)
      , None => PathBuf::from("")
    };
  }

  if options.contains(&RuntimeOptions::Debug)
    && ! options.contains(&RuntimeOptions::Quiet) {
    println!("cnf dir 1: '{}'", cnfdir.to_str().unwrap());
    println!("log dir 1: '{}'", logdir.to_str().unwrap());
  }

    match omndir {
      Some(d) => Ok(d)
      , None => {
        Err(Error::new(ErrorKind::NotFound, "Main Directory: could not extend Directory from Executable Path"))
      }
    } //match omndir
	}


  #[test]
  fn it_works() {
    let maindir = find_maindir(&[RuntimeOptions::Debug]).expect("maindir not found");


    assert_eq!(4, list_testdata(&maindir, &[RuntimeOptions::Debug]).unwrap());
  }
}