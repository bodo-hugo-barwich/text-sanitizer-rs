#![allow(unused)]
/*
* @author Bodo (Hugo) Barwich
* @version 2022-11-04
* @package text-sanitizer_app
* @subpackage examples/nut-monitor.rs

* This module shows the usage on a real-world use case
* It parses the output of a system command
*
*---------------------------------
* Requirements:
* - The Rust crate "text-sanitizer" must be installed
*/


extern crate text_sanitizer;

use text_sanitizer::sanitizer;

use std::process::Command;

fn main() {
    let mut sttcmd = Command::new("systemctl");
    //let mut sttcmd = Command::new("ipsc");
    let mut sttrpt: Vec<u8> = Vec::new();
    let mut stterr: Vec<u8> = Vec::new();
    let mut sttstt = 0;

    sttcmd.args(&["status", "nut-monitor", "-l"]);
    //sttcmd.args(&["-s", "-t"]);

    let sttrs = sttcmd.output();

    match sttrs {
        Ok(mut rs) => {
            sttrpt.append(&mut rs.stdout);
            stterr.append(&mut rs.stderr);

            if let Some(stt) = rs.status.code() {
                sttstt = stt;
            }
        }
        Err(e) => {
            stterr.append(&mut Vec::from(
                "cmd 'systemctl status nut-driver -l': execution failed!",
            ));
            stterr.append(&mut Vec::from(format!("msg: '{:?}'", e)));

            sttstt = 1;
        }
    } //match sttrs

    /*
        let sttrpt = Vec::from("❤\n!
    ● nut-monitor.service - Network UPS Tools - power device monitor and shutdown controller
        Loaded: loaded (/usr/lib/systemd/system/nut-monitor.service; disabled; vendor preset: disabled)
        Active: active (running) since mié 2019-07-03 20:55:45 WEST; 15h ago
        Process: 23520 ExecStartPre=/usr/bin/systemd-tmpfiles --create /etc/tmpfiles.d/nut-run.conf (code=exited, status=1/FAILURE)
        Main PID: 23523 (upsmon)
       CGroup: /system.slice/nut-monitor.service
               ├─23523 /usr/sbin/upsmon -F
               └─23527 /usr/sbin/upsmon -F

    jul 03 20:55:45 <host_name> upsmon[23523]: UPS: salicru@localhost (master) (power value 1)
    jul 03 20:55:45 <host_name> upsmon[23523]: Using power down flag file /etc/ups/killpower
    jul 03 20:55:45 <host_name> upsmon[23523]: UPS [salicru@localhost]: connect failed: Connection failure: Connection refused
    jul 03 20:55:45 <host_name> upsmon[23523]: Communications with UPS salicru@localhost lost
    jul 03 20:55:46 <host_name> upsmon[23523]: Network UPS Tools upsmon 2.7.2
    jul 03 20:55:50 <host_name> upsmon[23523]: UPS [salicru@localhost]: connect failed: Connection failure: Connection refused
    jul 03 20:55:50 <host_name> upsmon[23523]: UPS salicru@localhost is unavailable
    jul 03 20:55:50 <host_name> upsmon[23523]: Network UPS Tools upsmon 2.7.2
    jul 03 20:55:55 <host_name> upsmon[23523]: Communications with UPS salicru@localhost established
    jul 03 20:55:55 <host_name> upsmon[23523]: Network UPS Tools upsmon 2.7.2



    <pre><font color=\"#55FF55\"><b>●</b></font> nut-monitor.service - Network UPS Tools - power device monitor and shutdown controller
       Loaded: loaded (/usr/lib/systemd/system/nut-monitor.service; disabled; vendor preset: disabled)
       Active: <font color=\"#55FF55\"><b>active (running)</b></font> since mié 2019-07-03 20:55:45 WEST; 15h ago
      Process: 23520 ExecStartPre=/usr/bin/systemd-tmpfiles --create /etc/tmpfiles.d/nut-run.conf <font color=\"#FF5555\"><b>(code=exited, status=1/FAILURE)</b></font>
     Main PID: 23523 (upsmon)
       CGroup: /system.slice/nut-monitor.service
               ├─23523 /usr/sbin/upsmon -F
               └─23527 /usr/sbin/upsmon -F

    jul 03 20:55:45 <host_name> upsmon[23523]: UPS: salicru@localhost (master) (power value 1)
    jul 03 20:55:45 <host_name> upsmon[23523]: Using power down flag file /etc/ups/killpower
    jul 03 20:55:45 <host_name> upsmon[23523]: UPS [salicru@localhost]: connect failed: Connection failure: Connection refused
    jul 03 20:55:45 <host_name> upsmon[23523]: Communications with UPS salicru@localhost lost
    jul 03 20:55:46 <host_name> upsmon[23523]: Network UPS Tools upsmon 2.7.2
    jul 03 20:55:50 <host_name> upsmon[23523]: UPS [salicru@localhost]: connect failed: Connection failure: Connection refused
    jul 03 20:55:50 <host_name> upsmon[23523]: UPS salicru@localhost is unavailable
    jul 03 20:55:50 <host_name> upsmon[23523]: Network UPS Tools upsmon 2.7.2
    jul 03 20:55:55 <host_name> upsmon[23523]: Communications with UPS salicru@localhost established
    jul 03 20:55:55 <host_name> upsmon[23523]: Network UPS Tools upsmon 2.7.2
    </pre>

    ● nut-monitor.service - Network UPS Tools - power device monitor and shutdown controller
       Loaded: loaded (/usr/lib/systemd/system/nut-monitor.service; enabled; vendor preset: disabled)
       Active: active (running) since jue 2019-07-04 09:42:37 WEST; 47min ago
      Process: 20169 ExecStartPre=/usr/bin/systemd-tmpfiles --create /etc/tmpfiles.d/nut-run.conf (code=exited, status=1/FAILURE)
     Main PID: 20172 (upsmon)
       CGroup: /system.slice/nut-monitor.service
               ├─20172 /usr/sbin/upsmon -F
               └─20176 /usr/sbin/upsmon -F
    Eine große Überschrift für eine Logmeldung.
    Alguna anotación s\\n sentido {, pero ejecutable}.
    Ejecución fallida con [3]!
    Y alguna |tabla| => mostrando algo

    Conexión activada con éxito (D-Bus active path: /org/freedesktop/NetworkManager/ActiveConnection/3)
    ");
    */

    println!("cmd stt: '{}'", sttstt);
    println!("cmd rpt vec u8 (count: '{}'):\n{:?}", sttrpt.len(), sttrpt);
    println!("cmd err vec u8 (count: '{}'):\n{:?}", stterr.len(), stterr);

    let mut vrqlngs: Vec<String> = Vec::new();

    vrqlngs.push(String::from("en"));

    let srsrpt = sanitizer::sanitize_u8(&sttrpt, &vrqlngs, "-d");
    let srserr = sanitizer::sanitize_u8(&stterr, &vrqlngs, "-d");

    println!("rs rpt chrs (count : '{}'):\n{:?}", srsrpt.len(), srsrpt);
    println!("rs err chrs (count : '{}'):\n{:?}", srserr.len(), srserr);
}
