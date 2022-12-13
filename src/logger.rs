use ansi_term::Color;
use env_logger::{fmt::Formatter, Builder};
use log::{Level, LevelFilter, Record};
use std::io::{self, Write};

fn level_color(level: Level) -> Color {
    match level {
        Level::Error => Color::Red,
        Level::Warn => Color::Yellow,
        Level::Info => Color::Green,
        Level::Debug => Color::Blue,
        Level::Trace => Color::Purple,
    }
}

fn logger_write(buf: &mut Formatter, record: &Record) -> io::Result<()> {
    write!(
        buf,
        "{}",
        level_color(record.level()).paint(format!("[{}]", record.level()))
    )?;
    if let (Some(file), Some(line)) = (record.file(), record.line()) {
        write!(buf, "({file}:{line})")?;
    }
    writeln!(buf, " {} {}", Color::Cyan.paint(":>"), record.args())
}

#[allow(unused)]
pub fn init_default() {
    init(None)
}

#[allow(unused)]
pub fn init_filter(filter: LevelFilter) {
    init(Some(filter))
}

pub fn init(filter: Option<LevelFilter>) {
    Builder::new()
        .format(logger_write)
        .filter(None, filter.unwrap_or(LevelFilter::Info))
        .parse_env("LOG_LEVEL")
        .init();
}
