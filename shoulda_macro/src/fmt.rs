use ::rustfmt::config::{Config, WriteMode};
use ::rustfmt::format_input;
use ::rustfmt::Input::Text;
use std::collections::VecDeque;
use std::io::BufWriter;

pub fn format(s: String) -> String {
    let mut out = BufWriter::new(Vec::new());
    let mut config = Config::default();
    config.set().write_mode(WriteMode::Display);
    let _ = format_input(Text(format!("fn a(){{ {} }}", s)), &config, Some(&mut out));
    let out = String::from_utf8(out.into_inner().unwrap()).unwrap();
    let mut out = out.lines().collect::<VecDeque<&str>>();
    out.pop_front();
    out.pop_back();
    let out: String = out.into_iter().collect::<Vec<&str>>().join("\r\n");
    out.trim().to_string()
}