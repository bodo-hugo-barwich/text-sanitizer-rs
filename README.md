# Text-Sanitizer
Rust Library to convert rich UTF-8 Text into plain ASCII Text

This Crate holds a **Library** and an Executable.\
The **Executable** which can be invoked from **command line** taking the requested Data from the STDIN 
and producing the sanitized output to STDOUT

## Features
* Very low Dependencies\
  This leads to:
  * High Compability (compiles even with old _Rust_ Compilers)
  * Very fast Startup Time (Execution Time about **3 - 5 ms**)
* Robust Code (does not use risky `unwrap()` Methods)\
  Developed with the _DevOps_ Mentalitity: "_can fail but will live to tell_"

## Motivation
When dealing with System Outputs they are usually formated in the configured local language that include special characters.\
Those characters are by nature not equivalent to their ASCII representation `'Acción' != 'Accion'` which makes reliable parsing difficult.

## Use Case
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
