use std::io::{self, stdin, stdout, Read, Write};

use clap::{Parser, Subcommand};

/// Simple program to extract the first substring that is delimited
/// by two provided strings.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The string that indicates the start of the substring.
    /// If this string is empty, we start at the beginning of the input.
    start_string: String,
    /// The string that indicates the end of the substring.
    /// If this string is empty, we end at the beginning of the input.
    end_string: String,

    #[command(subcommand)]
    range: RangeType,
}

#[derive(Subcommand)]
enum RangeType {
    /// Returns the substring including both the start and end strings
    InclIncl,
    /// Returns the substring including the start string and excluding the end string
    InclExcl,
    /// Returns the substring excluding the start string and including the end string
    ExclIncl,
    /// Returns the substring between (excluding) the start and end strings
    ExclExcl,
}

const BUF_SIZE: usize = 1024;

fn find_start(
    start: &[u8],
    buf: &mut [u8],
    input: &mut impl Read,
) -> io::Result<Option<(usize, usize)>> {
    if start.is_empty() {
        return Ok(None);
    }

    let mut next_byte_idx = 0;

    loop {
        let count = input.read(buf)?;
        if count == 0 {
            return Ok(None);
        }

        for (i, &b) in buf[..count].iter().enumerate() {
            if b == start[next_byte_idx] {
                next_byte_idx += 1;
                if next_byte_idx == start.len() {
                    return Ok(Some((i + 1, count)));
                }
            } else {
                next_byte_idx = 0;
            }
        }
    }
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let mut stdin = stdin().lock();
    let mut stdout = stdout().lock();

    let start = cli.start_string.as_bytes();
    let end = cli.end_string.as_bytes();

    let mut buf = [0; BUF_SIZE];

    let buf_pos = find_start(start, &mut buf, &mut stdin)?;
    match cli.range {
        RangeType::InclIncl | RangeType::InclExcl => stdout.write_all(start)?,
        RangeType::ExclIncl | RangeType::ExclExcl => {}
    }

    if cli.end_string.is_empty() {
        if let Some((pos, count)) = buf_pos {
            stdout.write_all(&buf[pos..count])?;
        }
        loop {
            let count = stdin.read(&mut buf)?;
            if count == 0 {
                return Ok(());
            }
            stdout.write_all(&buf[..count])?;
        }
    }

    let mut next_byte_idx = 0;

    if let Some((pos, count)) = buf_pos {
        for (i, &b) in buf[pos..count].iter().enumerate() {
            if b == end[next_byte_idx] {
                next_byte_idx += 1;
                if next_byte_idx == end.len() {
                    match cli.range {
                        RangeType::InclIncl | RangeType::ExclIncl => {
                            stdout.write_all(&buf[pos..=pos + i])?;
                        }
                        RangeType::InclExcl | RangeType::ExclExcl => {
                            if pos + i >= end.len() {
                                stdout.write_all(&buf[pos..=pos + i - end.len()])?;
                            }
                        }
                    }
                    return Ok(());
                }
            } else {
                next_byte_idx = 0;
            }
        }
        stdout.write_all(&buf[pos..count])?;
    }

    loop {
        let count = stdin.read(&mut buf)?;
        if count == 0 {
            return Ok(());
        }
        for (i, &b) in buf[..count].iter().enumerate() {
            if b == end[next_byte_idx] {
                next_byte_idx += 1;
                if next_byte_idx == end.len() {
                    match cli.range {
                        RangeType::InclIncl | RangeType::ExclIncl => {
                            stdout.write_all(&buf[..=i])?;
                        }
                        RangeType::InclExcl | RangeType::ExclExcl => {
                            if i >= end.len() {
                                stdout.write_all(&buf[..=i - end.len()])?;
                            }
                        }
                    }
                    return Ok(());
                }
            } else {
                next_byte_idx = 0;
            }
        }
        stdout.write_all(&buf[..count])?;
    }
}
