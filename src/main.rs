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

    pub enum State {
        BeginsWithVowel,
        VowelInWord(usize),
        NoVowels,
    }

    pub fn consume_until(
        input: &[u8],
        stop_byte1: u8,
        stop_byte2: u8,
    ) -> Result<(&[u8], State, &[u8], bool), Error> {
        use self::State::*;
        let mut byte2_stopped = false;
        let mut state = NoVowels;
        let (input, remainder) = input.split_at(input
            .iter()
            .enumerate()
            .inspect(|&(idx, b)| {
                if let NoVowels = state {
                    match b {
                        b'a' | b'e' | b'i' | b'o' | b'u' | b'y' if idx == 0 => {
                            state = BeginsWithVowel
                        }
                        b'a' | b'e' | b'i' | b'o' | b'u' | b'y' => state = VowelInWord(idx),
                        _ => {}
                    }
                }
            })
            .map(|(_, b)| b)
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

        Ok((input, state, &remainder[1..], byte2_stopped))
    }

    pub fn word<'a>(input: &'a [u8]) -> Option<(&'a [u8], State, &'a [u8], bool)> {
        consume_until(input, b' ', b'\n').ok()
    }
}

use parse::Error;
use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() -> Result<(), Error> {
    use parse::State::*;
    let buf = {
        let mut b = Vec::with_capacity(1024 * 1024);
        stdin().read_to_end(&mut b)?;
        b
    };
    let mut writer = BufWriter::with_capacity(128 * 1024, stdout());

    let mut cursor = buf.as_slice();
    while let Some((w, wi, ncursor, newline)) = parse::word(cursor) {
        cursor = ncursor;
        match wi {
            BeginsWithVowel => {
                writer.write(w)?;
                writer.write(b"yay")?
            }
            VowelInWord(at) => {
                let before_vowel = &w[..at];
                let remainder = &w[at..];
                writer.write(remainder)?;
                writer.write(before_vowel)?;
                writer.write(b"ay")?
            }
            NoVowels => writer.write(w)?,
        };
        if newline {
            writeln!(writer)?;
        } else {
            write!(writer, " ")?;
        }
    }
    Ok(())
}
