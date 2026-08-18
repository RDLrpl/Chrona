use std::fmt;
use chrono::Local;
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::registry::LookupSpan;

pub struct ChronaFormatter {
    pub color: bool,
}

impl<S, N> FormatEvent<S, N> for ChronaFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let time = Local::now().format("%Y-%m-%d %H:%M:%S");
        let level = *event.metadata().level();

        let level_str = match level {
            Level::ERROR => "ERROR",
            Level::WARN  => "WARN ",
            Level::INFO  => "INFO ",
            Level::DEBUG => "DEBUG",
            Level::TRACE => "TRACE",
        };

        if self.color {
            let color_code = match level {
                Level::ERROR => "\x1b[31m",
                Level::WARN  => "\x1b[33m",
                Level::INFO  => "\x1b[32m",
                Level::DEBUG => "\x1b[36m",
                Level::TRACE => "\x1b[35m",
            };
            write!(writer, "{time} | {color_code}[{level_str}]\x1b[0m CHRONA: ")?;
        } else {
            write!(writer, "{time} | [{level_str}] CHRONA: ")?;
        }

        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}