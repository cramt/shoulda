#[cfg(fmt)]
use ::rustfmt::config::{Config, WriteMode};
#[cfg(fmt)]
use ::rustfmt::format_input;
#[cfg(fmt)]
use ::rustfmt::Input::Text;
#[cfg(fmt)]
use std::collections::VecDeque;
#[cfg(fmt)]
use std::io::BufWriter;

#[cfg(fmt)]
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
#[cfg(not(fmt))]
pub fn format(s: String) -> String {
    s
}
