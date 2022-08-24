use std::{
    io::{self, BufReader, Read, Seek, SeekFrom, Write},
    path::Path,
};

use crate::{
    data::Data,
    fcs::Fcs,
    header::Header,
    text::{Gate, Metadata, Parameter, Text},
};

pub trait Meta {
    fn metadata(&self) -> Metadata;
}

pub trait Par {
    fn count(&self) -> u64;

    fn parameters(&self) -> Vec<Parameter>;
}

pub trait Gating {
    fn count(&self) -> u64;

    fn gates(&self) -> Vec<Gate>;
}

pub trait ByteRead: Read + Seek {
    /// Read in bytes from start to stop inclusive.
    fn read_bytes(&mut self, offset: u64, start: u64, stop: u64) -> io::Result<Vec<u8>> {
        let mut f = BufReader::new(self);
        let first_byte = offset + start;
        f.seek(SeekFrom::Start(first_byte))?;

        let bytes_to_read = stop - first_byte + 1;
        let mut result = vec![0; bytes_to_read as usize];

        let mut take = f.take(bytes_to_read);
        take.read(&mut result)?;

        Ok(result)
    }
}

pub trait FcsRead: ByteRead + Sized {
    fn read_fcs(&mut self) -> io::Result<Fcs> {
        let header = Header::new(self)?;

        let text = Text::new(&self.read_bytes(0, header.text_start, header.text_end)?)?;

        let data = Data::new(
            &self.read_bytes(0, header.data_start, header.data_end)?,
            text.data_type(),
            text.byteord(),
        )?;

        Ok(Fcs { header, text, data })
    }
}

pub trait FcsWrite: Write {
    fn write_fcs<P>(&mut self, _fcs: &Fcs) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        Ok(())
    }
}
