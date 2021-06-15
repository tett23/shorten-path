#![feature(box_syntax)]

extern crate dirs;

use std::{env, path::Path, path::PathBuf, path::MAIN_SEPARATOR};

fn main() {
    let arg = env::args();
    let path = arg.last();
    let path = path.expect("msg");

    let rules: Vec<Box<dyn Fn(String) -> String>> = vec![box replace_home, box shorten_dirs];
    let ret = rules.iter().fold(path, |acc, f| f(acc));

    print!("{}", &ret);

    ()
}

fn replace_home(buf: String) -> String {
    let home: Option<PathBuf> = dirs::home_dir();
    let home = home.expect("msg");
    let home = home.to_str().expect("msg").to_owned();

    match buf.starts_with(&home) {
        true => buf.replace(&home, "~"),
        false => buf.clone(),
    }
}

fn shorten_dirs(buf: String) -> String {
    match buf.starts_with("~") {
        true => {
            let path = Path::new(&buf);
            let file_name: &str = &path
                .file_name()
                .expect("msg")
                .to_str()
                .expect("msg")
                .to_string();

            let parent_elems: &str = &path
                .parent()
                .expect("msg")
                .to_str()
                .expect("msg")
                .split(MAIN_SEPARATOR)
                .filter(|v| !v.is_empty())
                .map(|v| v.chars().next().expect("msg"))
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(&MAIN_SEPARATOR.to_string());

            let tmp: Vec<&str> = vec![parent_elems, file_name];

            tmp.into_iter()
                .filter(|v| !v.is_empty())
                .collect::<Vec<&str>>()
                .join(&MAIN_SEPARATOR.to_string())
        }
        false => buf,
    }
}
