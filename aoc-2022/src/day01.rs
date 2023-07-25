use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn sum_elf(buf: &mut String, reader: &mut BufReader<File>) -> Result<Option<usize>, Error> {
    let mut sum = 0usize;

    loop {
        buf.clear();
        let n = reader.read_line(buf)?;
        if n == 0 {
            return Ok(None);
        }

        let line = buf.trim_end_matches('\n');
        if line.is_empty() {
            break;
        }

        let num = line.parse::<usize>();
        match num {
            Ok(n) => sum += n,
            Err(e) => println!("{e}: {buf}"),
        }
    }

    Ok(Some(sum))
}

fn prob01(buf: &mut String, reader: &mut BufReader<File>) -> usize {
    let mut max = 0usize;

    loop {
        let next = sum_elf(buf, reader).unwrap();
        match next {
            Some(n) => max = cmp::max(max, n),
            None => break,
        }
    }

    max
}

fn prob02(buf: &mut String, reader: &mut BufReader<File>) -> usize {
    let mut max = [0usize; 3];

    loop {
        let next = sum_elf(buf, reader).unwrap();
        match next {
            Some(sum) => {
                for i in 0..max.len() {
                    if max[i] >= sum {
                        continue;
                    }

                    let range = i..max.len() - 1;
                    max.copy_within(range, i + 1);
                    max[i] = sum;
                    break;
                }
            }
            None => break,
        }
    }

    max.iter().sum()
}

pub fn solution(args: &[String], reader: &mut BufReader<File>) {
    let mut buf = String::new();

    let result: usize;
    if args[0] == "1" {
        result = prob01(&mut buf, reader);
    } else {
        result = prob02(&mut buf, reader);
    }
    println!("{result}");
}
