
extern crate thiserror;
extern crate time;
extern crate fileaccess;

use std::process::exit;
use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::process::Command;
use ini::Ini;
use thiserror::Error;
//use std::error;

use fileaccess::FileDriver;





#[derive(Debug, Error)]
enum Error {
    #[error("Module Path unknown")]
    ExePath(#[from] io::Error),

    #[error("Directory parsing error")]
    ParseDirectory,

    #[error("Target directory not found")]
    NoTarget,
}

//==============================================================================
// Auxiliary Functions


fn get_path(top: &Path, name: &str) -> Result<PathBuf, Error> {
    let target = Some(OsStr::new("target"));
    let dir = top
        .ancestors()
        .find(|p| p.is_dir() && p.file_name() == target)
        .ok_or(Error::NoTarget)?;

    let mut dir = dir.parent().ok_or(Error::ParseDirectory)?.to_path_buf();
    dir.push(name);

    Ok(dir)
}

fn get_some_path_parent(top: &Path, name: &str) -> Option<PathBuf> {
  let mut odir = None;

  let otarget = Some(OsStr::new("target"));

  for p in top.ancestors() {
    if odir.is_none()
      && p.is_dir()
      && p.file_name() == otarget {
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




//==============================================================================
// Executing Section


#[allow(clippy::cognitive_complexity)]
//#[allow(unused_assignments)]
fn main() -> Result<(), Error> {
  //-------------------------------------
  //Check the Server Environment

  let sserver = match Command::new("hostname").output() {
    Ok(rs) => rs.stdout
    , Err(_) => Vec::from("default")
  };
  let mut sserver = String::from_utf8_lossy(&sserver).into_owned();
  let mut ssvrprfx = String::new();


  match sserver.find('.') {
    Some(ps) =>  {
      ssvrprfx.push_str(sserver.split_at(ps).0);
      }
    , None => { //Server Name does not have a dot
      sserver = sserver.trim().to_string();
      ssvrprfx.push_str(&sserver);
      }
  } //match sserver.find(".")


  //-------------------------------------
  //Read the Script Parameters

  let mut mailmsg = String::new();


  mailmsg.push_str(&format!("server: {}", &sserver));


  let mut srqsvcsttmsg = String::new();

  let mut sscrprms = String::new();
  let mut sarg;

  let mut bdbg: bool = false;
  let mut bqt: bool = false;


  sscrprms.push_str(&"args:");
  mailmsg.push_str(&"args:");

  // Prints each argument on a separate line
  for argument in std::env::args() {
    sscrprms.push_str(&format!("'{}'", argument));

    mailmsg.push_str(&argument);

    if argument.starts_with("--") {
      sarg = argument.split_at(2).1;
      sarg.to_lowercase();
    }
    else if argument.starts_with('-') {
      sarg = argument.split_at(1).1;
      sarg.to_lowercase();

      match sarg {
        "q" | "b" => bqt = true
        , "d" | "v" => bdbg = true
        , _ => {}
      } //match sarg.as_ref()
    }
    else {
      srqsvcsttmsg = argument;
    }
  } //for argument in std::env::args()

  sscrprms.push_str(&"args end.");
  mailmsg.push_str(&"args:");

  if bdbg
    && ! bqt {
    println!("{}", sscrprms);
  }


  let mut srqsvcdvnm = String::new();
  let mut srqsvcstt = String::new();
  let irqsvcstttm = time::strftime("%s", &time::now()).unwrap();


  mailmsg.push_str(&"vars:");

  sscrprms = String::new();

  if bdbg
    && ! bqt {
    sscrprms.push_str(&"vars:");
  }

  // Prints each variable and its value on a separate line
  for (key, value) in std::env::vars() {
    if bdbg
      && ! bqt {
      sscrprms.push_str(&format!("{}: {}", key, value));
    }
    mailmsg.push_str(&format!("{}: '{}'", key, value));

    match key.as_ref() {
      "UPSNAME" => srqsvcdvnm = value
      , "NOTIFYTYPE" => srqsvcstt = value
      , _ => {}
    }
  } //for (key, value) in std::env::vars()

  sscrprms.push_str(&"vars end.");
  mailmsg.push_str(&"vars end.");

  if bdbg
    && !bqt {
    println!("{}", sscrprms);
    println!("device: '{}'", srqsvcdvnm);
    println!("time: '{:?}'", irqsvcstttm);
    println!("event: '{}'", srqsvcstt);
    println!("msg: '{}'", srqsvcsttmsg);
  } //if bdbg && !bqt

  mailmsg.push_str(&format!("device: {}", srqsvcdvnm));
  mailmsg.push_str(&format!("time: {:?}", irqsvcstttm));
  mailmsg.push_str(&format!("event: {}", srqsvcstt));
  mailmsg.push_str(&format!("msg: {}", srqsvcsttmsg));


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
      }
    }
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

  if let Some(mdir) = &omndir {
    match get_some_path_parent(mdir, "target") {
      Some(tdir) => omndir = Some(tdir)
      , None => {
        if let Some(bdir) = get_some_path_parent(mdir, "bin") {
          omndir = Some(bdir)
        }
      }
    } //match get_some_path_parent(&mdir, "target")
  } //if let Some(mdir) = omndir


  if bdbg
    && ! bqt {
    println!("md pth : '{:?}'", omdpth);
    println!("md nm : '{:?}'", omdnm);
    println!("mndir: '{:?}'", omndir);
    println!("wrkdir: '{:?}'", owrkdir);
  } //if bdbg && ! bqt


  let mut cnfdir = match &omndir {
    Some(mdir) => PathBuf::from(mdir)
    , None => PathBuf::from("")
  };
  let mut logdir = match & omndir {
    Some(mdir) => PathBuf::from_path(mdir)
    , None => PathBuf::from("")
  };

  cnfdir.push("config");
  logdir.push("logs");

  if ! cnfdir.exists() {
    cnfdir = match owrkdir {
      Some(wdir) => PathBuf::from(wdir)
      , None => PathBuf::from("")
    };
  }

  if ! logdir.exists() {
    logdir = match owrkdir {
      Some(wdir) => PathBuf::from(wdir)
      , None => PathBuf::from("")
    };
  }

  if bdbg
    && ! bqt {
    println!("cnf dir 1: '{}'", cnfdir.to_str().unwrap());
    println!("log dir 1: '{}'", logdir.to_str().unwrap());
  }


  //---------------------------------
  //Load the Configuration from the File

  let mut svcscnfflnm = PathBuf::from(cnfdir);


  svcscnfflnm.push(format!("services_{}", ssvrprfx.as_str()));
  svcscnfflnm.set_extension("conf");

  if bdbg
    && ! bqt {
    println!("svc cnf pth: '{}'", svcscnfflnm.to_string_lossy());
  }


  let mut cnffl = FileDriver::from_directory_name(&cnfdir.as_path().to_string_lossy().into_owned());
  let mut cnfcntnt = None;
  let mut lstcnf = Ini::new();

  if let Some(f) = svcscnfflnm.file_name() {
    cnffl.set_file_name(&f.to_string_lossy().into_owned());
  }

  if ! cnffl.exists() {
    cnffl.set_file_name("services.conf");
  }

  if cnffl.read() {
    cnfcntnt = Some(cnffl.take_content());
  }
  else {
    eprintln!("FALLO: El Fichero de Configuracion '{}' no podia ser leido!", cnffl.get_path_name());
    eprintln!("Informe (Codigo: '{}'): '{}'", cnffl.get_error_code(), cnffl.get_error_string());

    exit(2);
  } //if cnffl.read()

  println!("Fichero de Configuracion '{}':", cnffl.get_path_name());
  println!("Informe (Codigo: '{}'): '{}'", cnffl.get_error_code(), cnffl.get_error_string());


  if bdbg
    && ! bqt {
    println!("Fichero de Configuracion '{}':", cnffl.get_path_name());
    println!("Informe (Codigo: '{}'): '{}'", cnffl.get_error_code(), cnffl.get_error_string());

    println!("cnf cntnt:\n{:?}", cnfcntnt);
  } //if bdbg && ! bqt


  if let Some(scntnt) = cnfcntnt {
    lstcnf = Ini::load_from_str(scntnt.as_str()).unwrap();
  } //if let Some(scntnt) = cnfcntnt

  //println!("lst cnf dmp:\n{:?}", lstcnf);

  if let Some(_opts) = lstcnf.section(Some(ssvrprfx.as_str())) {

  }
  else {
    eprintln!("FALLO: Servidor '{}' no esta configurado!", ssvrprfx.as_str());

    exit(2);
  }




  //---------------------------------
  //Check an Existing Notification

  let sspldir = "/var/spool/services/messages/";
  let mut ssplflpth = String::from(sspldir);


  ssplflpth.push_str("message_ups");
  ssplflpth.push_str(".lock");

  println!("spl fl pth:\n{:?}", ssplflpth);

  let splflnm = Path::new(&ssplflpth);

  println!("spl fl:\n{:?}", ssplflpth);

  let mut splfl = FileDriver::from_path_name(splflnm.to_str().unwrap());
  let mut upsevent = HashMap::new();

  let splcntnt = if splfl.exists() {
    if splfl.read() {
      splfl.get_content()
    }
    else {
      println!("File '{}': Open Read failed!\nError (Code: '{}'):\n{}"
        , splfl.get_path_name(), splfl.get_error_code(), splfl.get_error_string());
      &""
    } //if splfl.read()
  }
  else {  //The File does not exist
    &""
  };

  println!("spl cntnt:\n{:?}", splcntnt);

  if ! splcntnt.is_empty() {
    for line in splcntnt.lines() {
      if let Some(_ps) = line.find('=') {
        let set: Vec<&str> = line.split_terminator('=').collect();
        upsevent.insert(set[0].trim(), set[1].trim());
      }
    }
  } //if ! splcntnt.is_empty()

  println!("upsevent dmp:\n{:?}", upsevent);


  if ! upsevent.is_empty() {

  } //if ! upsevent.is_empty()

    Ok(())


}
