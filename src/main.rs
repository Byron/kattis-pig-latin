mod parse {
    use std::io;

    #[derive(Debug)]
    pub enum Error {
        Exhausted,
        Io(io::Error),
    }

    impl From<io::Error> for Error {
        fn from(err: io::Error) -> Self {
            Error::Io(err)
        }
    }

    pub fn consume_until(
        input: &[u8],
        stop_byte1: u8,
        stop_byte2: u8,
    ) -> Result<(&[u8], &[u8], bool), Error> {
        let mut byte2_stopped = false;
        let (input, remainder) = input.split_at(input
            .iter()
            .position(|b| {
                if *b == stop_byte1 {
                    true
                } else if *b == stop_byte2 {
                    byte2_stopped = true;
                    true
                } else {
                    false
                }
            })
            .ok_or(Error::Exhausted)?);

        Ok((input, &remainder[1..], byte2_stopped))
    }

    pub fn word<'a>(input: &'a [u8]) -> Option<(&'a [u8], &'a [u8], bool)> {
        consume_until(input, b' ', b'\n').ok()
    }
}

use parse::Error;
use std::io::{stdin, stdout, BufWriter, Read};
use std::collections::HashMap;
use std::str;

fn main() -> Result<(), Error> {
    let buf = {
        let mut b = Vec::with_capacity(1024 * 1024);
        stdin().read_to_end(&mut b)?;
        b
    };
    let mut writer = BufWriter::with_capacity(128 * 1024, stdout());

    let mut cursor = buf.as_slice();
    while let Some((w, ncursor, newline)) = parse::word(cursor) {
        cursor = ncursor;
        eprintln!("{}", str::from_utf8(w).unwrap());
    }
    Ok(())
}
