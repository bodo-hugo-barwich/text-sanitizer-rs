[![Automated Tests](https://github.com/bodo-hugo-barwich/text-sanitizer-rs/actions/workflows/testing.yml/badge.svg)](https://github.com/bodo-hugo-barwich/text-sanitizer-rs/actions/workflows/testing.yml)


# Text-Sanitizer
Rust Application and Library to convert rich UTF-8 Text into plain ASCII Text

This Crate holds a **Library** and an Executable.\
The **Executable** which can be invoked from **command line** taking the requested Data from the STDIN
and producing the sanitized output to STDOUT.\
Precompiled **Binaries** are distributed with the Releases in the Release Section:\
[Download `text-sanitizer` latest](https://github.com/bodo-hugo-barwich/text-sanitizer-rs/releases/tag/lastest)


## Features
* Very low Dependencies\
  This leads to:
  * High Compatibility (compiles even with old _Rust_ Compilers - tested against Versions `[ 1.41, 1.48, 1.60 ]`)
  * Very fast Startup Time (Execution Time less than **< 3 ms** on a 27KB document)
* Robust Code (does not use risky `unwrap()` Methods)\
  Developed with the _DevOps_ Mentalitity: "_can fail but will live to tell_"


## Motivation
When dealing with System Outputs they are usually formated in the configured local language that include special characters.\
Those characters are by nature not equivalent to their ASCII representation `'Acción' != 'Accion'` which makes reliable parsing difficult.\
Those differences are often resolved with unmaintainable `sed` **command chains**

```plain
$ cat -A nut-monitor_status.txt| sed -re 's#\$$##' -e 's#M-bM-\^##g' -e 's#M-CM-\)#e#g' -e 's#TM-\^[T\\]#|-#g' -e 's#TM-\^@#-#g' -e 's#WM-\^O#*#g'
```

which makes it easy to understand why those **handcrafted workarounds** are not desirable.


## Use Cases

### Reconstruction
**Invalid UTF-8 text** cannot be parsed with common command line tools:

```plain
$ cat lanzarote-com_de-ausfluge.html|grep -ioE "<p>[^/]*Sichern Sie sich Ihren Platz[^/]*</p>"|wc -l
0
```
It requires always some kind of **sanitizing**:

```plain
$ cat -A lanzarote-com_de-ausfluge.html|grep -ioE "<p>[^/]*Sichern Sie sich Ihren Platz[^/]*</p>"|wc -l
1
$ cat -A lanzarote-com_de-ausfluge.html|grep -ioE "<p>[^/]*Sichern Sie sich Ihren Platz[^/]*</p>"
<p>Sichern Sie sich Ihren Platz bei einem dieser AusflM-|ge und buchen Sie online mit dem Formular, d.as Sie auf der Informationsseite jedes Ausfluges finden. Sie kM-vnnen Ihre bevorzugten AusflM-|ge auch auf Ihrer persM-vnlichen ReisefM-|hrer-Seite einfM-|gen, um sie stets zur Hand zu haben.</p>
```
but that way it has lost readability. \
So the sanitizing needs to be assisted with a **Reconstruction of the text**. Mostly with `sed` **regular expression workarounds** which often result unmaintainable because of its complex structure

```plain
$ cat -A lanzarote-com_de-ausfluge.html|grep -ioE "<p>[^/]*Sichern Sie sich Ihren Platz[^/]*</p>"|sed -re 's#M\-v#oe#g' -e 's#M\-\|#ue#g'|wc -l
1
$ cat -A lanzarote-com_de-ausfluge.html|grep -ioE "<p>[^/]*Sichern Sie sich Ihren Platz[^/]*</p>"|sed -re 's#M\-v#oe#g' -e 's#M\-\|#ue#g'
<p>Sichern Sie sich Ihren Platz bei einem dieser Ausfluege und buchen Sie online mit dem Formular, d.as Sie auf der Informationsseite jedes Ausfluges finden. Sie koennen Ihre bevorzugten Ausfluege auch auf Ihrer persoenlichen Reisefuehrer-Seite einfuegen, um sie stets zur Hand zu haben.</p>
```
and also are measurable much slower: \
The `cat -A | sed` command chain is **twice as slow as**  the _Text-Sanitizer_

```plain
$ date +"%s.%N" ; cat lanzarote-com_de-ausfluge.html | ../../target/debug/text-sanitizer -i en es de > lanzarote-com_de-ausfluge_result.html ; date +"%s.%N"
1636289191.342594947
1636289191.349053266
$ echo "scale=3; 91.349053266-91.342594947"|bc -l
.006458319
$ date +"%s.%N" ; cat -A lanzarote-com_de-ausfluge.html|sed -re 's#M\-v#oe#g' -e 's#M\-\|#ue#g' > ../lz_out.log ; date +"%s.%N"
1636400374.614412085
1636400374.628543983
$ echo "scale=3; 74.628543983-74.614412085"|bc -l
.014131898
```
In half the time the _Text-Sanitizer_ has already fixed the whole text:

```plain
$ cat lanzarote-com_de-ausfluge_result.html|grep -ioE "<p>[^/]*Sichern Sie sich Ihren Platz[^/]*</p>"
<p>Sichern Sie sich Ihren Platz bei einem dieser Ausfluege und buchen Sie online mit dem Formular, d.as Sie auf der Informationsseite jedes Ausfluges finden. Sie koennen Ihre bevorzugten Ausfluege auch auf Ihrer persoenlichen Reisefuehrer-Seite einfuegen, um sie stets zur Hand zu haben.</p>
```

### Automation
In many Automations the System Output must be parsed and converted to Data Structures to process further.\
Sanitizing the text into ASCII Text helps to create recognizable Data Structures
like in this `Ansible` Task List:

```yaml
- name: Read yum History
  command:
    cmd: "yum --setopt=history_list_view=commands history list *"
    warn: false
  register: yum_history_rs
  changed_when: false
  become: yes
  become_method: su

- name: Sanitize yum History
  command:
    cmd: "roles/yum-history/library/text-sanitizer es -i"
    stdin: "{{ yum_history_rs.stdout }}"
  register: yum_history_rs
  changed_when: false
  delegate_to: localhost
  when: yum_history_rs is defined

- name: Yum History 1
  debug:
    var: yum_history_rs
  when: ansible_verbosity > 1

- name: Check last Full Update
  command:
    cmd: "roles/yum-history/library/check_full-update.pl -i -d"
    stdin: "{{ yum_history_rs.stdout }}"
  register: update_full_rs
  changed_when: false
  delegate_to: localhost
  when: yum_history_rs is defined
```
This task list will produce this output:\
Now the Header Line can much easier transformed,parsed and matched.

```plain
TASK [yum-history : Sanitize yum History] ********************************************************
task path: /path/to/playbook/roles/yum-history/tasks/check_history.yml:52
ok: [host02] => {
    "changed": false,
    "cmd": [
        "roles/yum-history/library/text-sanitizer",
        "es",
        "-i"
    ],
    "delta": "0:00:00.002598",
    "end": "2021-04-29 10:09:57.529444",
    "invocation": {
        "module_args": {
            "_raw_params": "roles/yum-history/library/text-sanitizer es -i",
            "_uses_shell": false,
            "argv": null,
            "chdir": null,
            "creates": null,
            "executable": null,
            "removes": null,
            "stdin": "ID     | Línea de comandos        | Día y hora       | Acción(es)     | Modific\n-------------------------------------------------------------------------------\n     5 |                          | 2021-04-07 14:57 | Install        |    6   \n     4 | -y --best upgrade        | 2020-09-23 06:03 | I, U           |   59 EE\n     3 | -C -y remove firewalld - | 2020-06-11 02:40 | Removed        |   12 EE\n     2 | -C -y remove linux-firmw | 2020-06-11 02:40 | Removed        |    1   \n     1 |                          | 2020-06-11 02:35 | Install        |  441 EE",
            "stdin_add_newline": true,
            "strip_empty_ends": true,
            "warn": true
        }
    },
    "rc": 0,
    "start": "2021-04-29 10:09:57.526846",
    "stderr": "",
    "stderr_lines": [],
    "stdout": "ID     | Linea de comandos        | Dia y hora       | Accion(es)     | Modific\n-------------------------------------------------------------------------------\n     5 |                          | 2021-04-07 14:57 | Install        |    6   \n     4 | -y --best upgrade        | 2020-09-23 06:03 | I, U           |   59 EE\n     3 | -C -y remove firewalld - | 2020-06-11 02:40 | Removed        |   12 EE\n     2 | -C -y remove linux-firmw | 2020-06-11 02:40 | Removed        |    1   \n     1 |                          | 2020-06-11 02:35 | Install        |  441 EE",
    "stdout_lines": [
        "ID     | Linea de comandos        | Dia y hora       | Accion(es)     | Modific",
        "-------------------------------------------------------------------------------",
        "     5 |                          | 2021-04-07 14:57 | Install        |    6   ",
        "     4 | -y --best upgrade        | 2020-09-23 06:03 | I, U           |   59 EE",
        "     3 | -C -y remove firewalld - | 2020-06-11 02:40 | Removed        |   12 EE",
        "     2 | -C -y remove linux-firmw | 2020-06-11 02:40 | Removed        |    1   ",
        "     1 |                          | 2020-06-11 02:35 | Install        |  441 EE"
    ]
}
```

or unmaintainable constructs as seen in the **test file** `nut-monitor_status.txt` \
which would be sanitized with the `cat -A | sed` **command chaining** in that manner:

```plain
$ cat -A nut-monitor_status.txt| sed -re 's#\$$##' -e 's#M-bM-\^##g' -e 's#M-CM-\)#e#g' -e 's#TM-\^[T\\]#|-#g' -e 's#TM-\^@#-#g' -e 's#WM-\^O#*#g'
```
to reproduce the same output as seen in `nut-monitor_status_result.txt`:

```plain
* nut-monitor.service - Network UPS Tools - power device monitor and shutdown controller
    Loaded: loaded (/usr/lib/systemd/system/nut-monitor.service; disabled; vendor preset: disabled)
    Active: active (running) since mie 2019-07-03 20:55:45 WEST; 15h ago
    Process: 23520 ExecStartPre=/usr/bin/systemd-tmpfiles --create /etc/tmpfiles.d/nut-run.conf (code=exited, status=1/FAILURE)
    Main PID: 23523 (upsmon)
   CGroup: /system.slice/nut-monitor.service
           |--23523 /usr/sbin/upsmon -F
           |--23527 /usr/sbin/upsmon -F

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
```
