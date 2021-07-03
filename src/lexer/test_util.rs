use std::fs;
use std::io::Read;

pub(super) fn read_md_file(name: &str) -> String {
    let mut cwd = std::env::current_dir().unwrap();

    cwd.push("src/lexer");
    cwd.push(name);

    let file_path = cwd.into_os_string().into_string().unwrap();
    let mut file = fs::File::open(file_path).expect("Unable to open file");
    let mut md_str = String::new();
    let _ = file.read_to_string(&mut md_str);

    md_str
}
