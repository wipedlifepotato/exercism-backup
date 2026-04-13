use std::io::{Read, Result, Write};
use std::marker::PhantomData;

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them
// https://doc.rust-lang.org/nomicon/phantom-data.html 
#[derive(Debug)]
pub struct ReadStats<R> {
    bytes: Vec::<u8>,
    _marker: PhantomData<R>,
    _wrapped: R, 
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: R) -> ReadStats<R> {
        ReadStats { bytes: Vec::<u8>::new(), _marker: PhantomData, _wrapped, reads: 0 }
    }

    pub fn get_ref(&self) -> &R {
        &self._wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes.len()
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads+=1;
        let n = self._wrapped.read(buf)?;
        self.bytes.extend_from_slice(&buf[..n]);
        Ok(n)
    }
}

pub struct WriteStats<W> { //(::std::marker::PhantomData<W>) {
    bytes: Vec::<u8>,
    _marker: PhantomData<W>,
    _wrapped: W, 
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: W) -> WriteStats<W> {
        WriteStats { bytes: Vec::<u8>::new(), _marker: PhantomData, _wrapped, writes:0 }
    }

    pub fn get_ref(&self) -> &W {
        &self._wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes.len()
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes+=1;
        let n = self._wrapped.write(buf)?;
        self.bytes.extend_from_slice(&buf[..n]);
        Ok(n)      
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
/*
    const CHUNK_SIZE: usize = 2;
    static INPUT: &[u8] = b"Twas brillig, and the slithy toves/Did gyre and gimble in the wabe:/All mimsy were the borogoves,/And the mome raths outgrabe.";
    fn read_passthrough() {
        let data = INPUT;
        dbg!(data);
        let size = data.len();
        dbg!(size);
        let mut reader = ReadStats::new(data);
        dbg!(&reader);
        let mut buffer = Vec::with_capacity(size);
        dbg!(&buffer);
        let qty_read = reader.read_to_end(&mut buffer);
        dbg!(&qty_read);
        assert!(qty_read.is_ok());
        assert_eq!(size, qty_read.unwrap());
        assert_eq!(size, buffer.len());
        // 2: first to read all the data, second to check that
        // there wasn't any more pending data which simply didn't
        // fit into the existing buffer
        assert_eq!(2, reader.reads());
        assert_eq!(size, reader.bytes_through());
}
*/
/*
    fn write_passthrough() {

        

          

        let data = INPUT;

        

          

        let size = data.len();

        

          

        let mut writer = WriteStats::new(Vec::with_capacity(size));

        

          

        let written = writer.write(data);

        

          

        assert!(written.is_ok());

        

          

        assert_eq!(size, written.unwrap());

        

          

        assert_eq!(size, writer.bytes_through());

        

          

        assert_eq!(1, writer.writes());

        

          

        assert_eq!(data, writer.get_ref().as_slice());

        

          

    }
fn main() {
    let mut data: Vec<u8> = Vec::new();
    let _ = ReadStats::new(data.as_slice());
    let _ = WriteStats::new(data.as_mut_slice());
    read_passthrough();
    write_passthrough();
}
*/
