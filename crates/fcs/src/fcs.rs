use crate::{data::Data, header::Header, text::Text};

#[derive(Debug)]
pub struct Fcs {
    pub header: Header,
    pub data: Data,
    pub text: Text,
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io, path::PathBuf};

    use crate::traits::FcsRead;

    #[test]
    fn it_opens_a_file() -> io::Result<()> {
        dbg!(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/100715.fcs"));
        File::open(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/100715.fcs"))?;

        Ok(())
    }

    #[test]
    fn it_returns_data() -> io::Result<()> {
        dbg!(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/100715.fcs"));
        let mut file =
            File::open(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/100715.fcs"))?;

        let _fcs = file.read_fcs()?;
        dbg!(&_fcs.text.pairs);
        dbg!(_fcs.text.total_events());
        assert_eq!(true, false);

        Ok(())
    }
}
