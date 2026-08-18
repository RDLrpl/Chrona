use std::{panic};

use chrono::Local;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::logs::{log_format::ChronaFormatter, memory::MemoryBuffer};

pub struct LoggerEngine {
    pub memory_buffer: MemoryBuffer
}

pub struct LoggerConfig {
    pub console: bool,
    pub console_color: bool,
    pub env_filter: String,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            console: cfg!(debug_assertions),
            console_color: cfg!(debug_assertions),
            env_filter: if cfg!(debug_assertions) { "debug".into() } else { "info".into() },
        }
    }
}

impl LoggerEngine {
    pub fn init(config: LoggerConfig) -> Self {
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new(config.env_filter.clone()));

        let memory_buffer = MemoryBuffer::new();

        let memory_layer = fmt::layer()
            .with_ansi(false)
            .event_format(ChronaFormatter { color: false })
            .with_writer(memory_buffer.clone());

        let console_layer = if config.console {
            Some(
                fmt::layer()
                    .event_format(ChronaFormatter { color: config.console_color })
                    .with_writer(std::io::stdout),
            )
        } else {
            None
        };
        
        tracing_subscriber::registry()
            .with(filter)
            .with(memory_layer)
            .with(console_layer)
            .init();

        let engine = Self { memory_buffer };

        engine.install_panic_hook();
    
        engine
    }

    pub fn droplog(&self) {
        let _ = std::fs::create_dir_all("./logs");

        let path = format!(
            "./logs/ChronaEngine_{}.log",
            Local::now().format("%Y-%m-%d_%H-%M-%S")
        );

        if let Err(e) = self.memory_buffer.dump_to_file(&path) {
            eprintln!("ERROR+: FAILED TO DUMP LOG| {path}: {e}");
        }

        self.memory_buffer.clear();        
    }

    fn install_panic_hook(&self) {
        let mb = self.memory_buffer.clone();

        panic::set_hook(Box::new(move |info| {
            let msg = if let Some(s) = info.payload().downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = info.payload().downcast_ref::<String>() {
                s.clone()
            } else {
                "unknown panic payload".to_string()
            };

            let location = info
                .location()
                .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
                .unwrap_or_else(|| "unknown location".to_string());

            tracing::error!("CRASHED: {location}| {msg}");

            let _ = std::fs::create_dir_all("./logs");

            let path = format!(
                "./logs/CRASH_{}.log",
                Local::now().format("%Y-%m-%d_%H-%M-%S")
            );

            if let Err(e) = mb.dump_to_file(&path) {
                eprintln!("ERROR+: FAILED TO DUMP CRASH LOG | {path}: {e}");
            }
        }));
    }

} 