use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn other_val(ch: &str) -> i32 {
    ch.chars().nth(0).unwrap() as i32 - 'A' as i32
}

fn my_val(ch: &str) -> i32 {
    ch.chars().nth(0).unwrap() as i32 - 'X' as i32
}

fn win_score(ov: i32, mv: i32) -> usize {
    let diff = mv - ov;
    if diff.abs() <= 1 {
        return (diff * 3 + 3) as usize;
    }

    if diff == -2 {
        return 6;
    }

    return 0;
}

fn sum_score<F>(buf: &mut String, reader: &mut BufReader<File>, scorer: &F) -> usize
where
    F: Fn(&mut usize, i32, i32),
{
    let mut sum = 0usize;

    loop {
        buf.clear();
        let n = reader.read_line(buf).unwrap();
        if n == 0 {
            break;
        }

        let line = buf.trim_end_matches('\n');
        if line.is_empty() {
            break;
        }

        let args: Vec<&str> = line.split_ascii_whitespace().collect();
        let ov = other_val(args[0]);
        let mv = my_val(args[1]);
        scorer(&mut sum, ov, mv);
    }

    sum
}

fn prob01(buf: &mut String, reader: &mut BufReader<File>) -> usize {
    fn scorer(sum: &mut usize, ov: i32, mv: i32) {
        *sum += win_score(ov, mv) + mv as usize + 1;
    }

    sum_score(buf, reader, &scorer)
}

fn win_score_my(mv: i32) -> i32 {
    (mv - 1) * 3 + 3
}

fn find_my(mut ov: i32, mv: i32) -> i32 {
    if mv == 1 {
        return ov;
    }

    ov += 3;
    if mv == 0 {
        return (ov - 1) % 3;
    }

    (ov + 1) % 3
}

fn prob02(buf: &mut String, reader: &mut BufReader<File>) -> usize {
    fn scorer(sum: &mut usize, ov: i32, mv: i32) {
        *sum += (win_score_my(mv) + find_my(ov, mv) + 1) as usize;
    }

    sum_score(buf, reader, &scorer)
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
