use std::sync::{Arc, Mutex};
use std::io::Write;
use tracing_subscriber::fmt::MakeWriter;

#[derive(Clone)]
pub struct MemoryBuffer {
    buf: Arc<Mutex<Vec<u8>>>,
}

impl MemoryBuffer {
    pub fn new() -> Self {
        Self { buf: Arc::new(Mutex::new(Vec::new())) }
    }

    pub fn dump_to_file(&self, path: &str) -> std::io::Result<()> {
        let data = self.buf.lock().unwrap();
        std::fs::write(path, &*data)
    }

    pub fn clear(&self) {
        self.buf.lock().unwrap().clear();
    }
}

pub struct BufWriter(Arc<Mutex<Vec<u8>>>);

impl Write for BufWriter {
 fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for MemoryBuffer {
    type Writer = BufWriter;
    fn make_writer(&'a self) -> Self::Writer {
        BufWriter(self.buf.clone())
    }
}