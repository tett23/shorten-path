use std::{env, path::MAIN_SEPARATOR, path::MAIN_SEPARATOR_STR, path::Path, path::PathBuf};

fn main() {
    let path = env::args().next_back();
    let path = path.expect("msg");

    let home: Option<PathBuf> = env::home_dir();
    let home = home.expect("msg");
    let home = home.to_str().expect("msg").to_owned();

    let rules: Vec<Box<dyn Fn(String) -> String>> = vec![
        Box::new(move |buf| replace_home(buf, &home)),
        Box::new(shorten_dirs),
    ];
    let ret = rules.iter().fold(path, |acc, f| f(acc));

    print!("{}", ret);
}

fn replace_home(buf: String, home: &str) -> String {
    match buf.starts_with(home) {
        true => buf.replace(home, "~"),
        false => buf.clone(),
    }
}

fn shorten_dirs(buf: String) -> String {
    match buf.starts_with("~") {
        true => {
            let path = Path::new(&buf);
            let file_name: &str = path.file_name().expect("msg").to_str().expect("msg");

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
                .join(MAIN_SEPARATOR_STR);

            let tmp: Vec<&str> = vec![parent_elems, file_name];

            tmp.into_iter()
                .filter(|v| !v.is_empty())
                .collect::<Vec<&str>>()
                .join(MAIN_SEPARATOR_STR)
        }
        false => buf,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shorten_dirs_shortens_parent_dirs() {
        assert_eq!(shorten_dirs("~/foo/bar/baz".to_string()), "~/f/b/baz");
    }

    #[test]
    fn shorten_dirs_keeps_direct_child_of_home() {
        assert_eq!(shorten_dirs("~/foo".to_string()), "~/foo");
    }

    #[test]
    fn shorten_dirs_ignores_path_outside_home() {
        assert_eq!(shorten_dirs("/usr/local/bin".to_string()), "/usr/local/bin");
    }

    #[test]
    fn replace_home_replaces_home_prefix() {
        assert_eq!(
            replace_home("/home/alice/foo".to_string(), "/home/alice"),
            "~/foo"
        );
    }

    #[test]
    fn replace_home_replaces_home_itself() {
        assert_eq!(replace_home("/home/alice".to_string(), "/home/alice"), "~");
    }

    #[test]
    fn replace_home_ignores_path_outside_home() {
        assert_eq!(
            replace_home("/usr/local/bin".to_string(), "/home/alice"),
            "/usr/local/bin"
        );
    }

    #[test]
    fn replace_home_matches_by_string_prefix() {
        assert_eq!(
            replace_home("/home/alicefoo/bar".to_string(), "/home/alice"),
            "~foo/bar"
        );
    }
}
