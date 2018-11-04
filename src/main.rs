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

    pub fn consume_until(input: &[u8], stop_byte: u8) -> Result<(&[u8], &[u8]), Error> {
        let (input, remainder) = input.split_at(input
            .iter()
            .position(|b| *b == stop_byte)
            .ok_or(Error::Exhausted)?);

        Ok((input, &remainder[1..]))
    }

    pub fn names<'a>(input: &'a [u8], mut cb: impl FnMut(&'a [u8])) -> Result<(), Error> {
        let mut cursor = input;
        loop {
            let (name, ncursor) = consume_until(cursor, b'\n')?;
            match name.get(0) {
                Some(c) if *c == b'*' => return Ok(()),
                None => return Err(Error::Exhausted),
                _ => cb(name),
            }
            cursor = ncursor;
        }
    }
}

use parse::Error;
use std::io::{stdin, Read};
use std::collections::HashMap;
use std::str;

fn main() -> Result<(), Error> {
    let mut buf = Vec::with_capacity(1024 * 1024);
    stdin().read_to_end(&mut buf)?;
    Ok(())
}
