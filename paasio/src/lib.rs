use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    data: R,
    bytes_through: usize,
    reads: usize
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: R) -> ReadStats<R> {
        ReadStats {
            data: _wrapped,
            bytes_through: 0,
            reads: 0
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let result = self.data.read(buf);
        match result {
            Ok(d) => {
                self.reads += 1;
                self.bytes_through += d;
            },
            _ => ()
        }
        result
    }
}

pub struct WriteStats<W> {
        data: W,
        bytes_through: usize,
        writes: usize
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: W) -> WriteStats<W> {
        WriteStats {
            data: _wrapped,
            bytes_through: 0,
            writes: 0
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let result = self.data.write(buf);
        match result {
            Ok(d) => {
                self.writes += 1;
                self.bytes_through += d;
            },
            _ => ()
        }
        result
    }

    fn flush(&mut self) -> Result<()> {
        self.data.flush()
    }
}
