#! /usr/bin/env rustr

use std::env::consts::{
    ARCH, DLL_EXTENSION, DLL_PREFIX, DLL_SUFFIX, EXE_EXTENSION, EXE_SUFFIX,
    FAMILY, OS,
};

fn main() {
    let mut s = String::new();
    appendarch(&mut s);
    appenddll(&mut s);
    appendexe(&mut s);
    println!("{}", s);
}

fn appendarch(s: &mut String) {
    s.push_str(
        format!("arch:\x1b[1m{},{},{}\x1b[0m ", ARCH, FAMILY, OS).as_str(),
    );
}

fn appenddll(s: &mut String) {
    s.push_str(
        format!(
            "dll:\x1b[1m{},{},{}\x1b[0m ", DLL_EXTENSION, DLL_PREFIX, DLL_SUFFIX,
        ).as_str()
    );
}

fn appendexe(s: &mut String) {
    if (EXE_EXTENSION.len() + EXE_SUFFIX.len()) > 0 {
        s.push_str("exe:");
        s.push_str("\x1b[1m");
        s.push_str([EXE_EXTENSION, EXE_SUFFIX].join(",").as_str());
        s.push_str("\x1b[0m ");
    }
}
