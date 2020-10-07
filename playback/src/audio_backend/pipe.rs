use super::{Open, Sink};
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::mem;
use std::slice;

pub struct StdoutSink(Box<dyn Write>);

impl Open for StdoutSink {
    fn open(path: Option<String>) -> StdoutSink {
        if let Some(path) = path {
            let file = OpenOptions::new().write(true).open(path).unwrap();
            StdoutSink(Box::new(file))
        } else {
            StdoutSink(Box::new(io::stdout()))
        }
    }
}

impl Sink for StdoutSink {
    fn start(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        let start = std::time::Instant::now();
        let sample_length_us = ((data.len() as f32 * 14.0 / 2.0).round()) as u64;

        let data: &[u8] = unsafe {
            slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * mem::size_of::<i16>(),
            )
        };

        self.0.write_all(data)?;
        self.0.flush()?;

        let elapsed = start.elapsed().as_micros() as u64;

        if elapsed < sample_length_us {
            let wait_us = sample_length_us - elapsed;
            let d_us = std::time::Duration::from_micros(20);
            while (start.elapsed().as_micros() as u64) < wait_us {
                std::thread::sleep(d_us);
            }
        }

        Ok(())
    }
}
