use proc_macro::TokenStream;

const DAY_MAX: usize = 3;

#[proc_macro]
pub fn generate_aoc_main(_: TokenStream) -> TokenStream {
    let mut code = String::new();
    let range = 1..DAY_MAX + 1;

    for i in range.clone() {
        code += &format!("mod day{i:02};\n");
    }

    code += "fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day: i32 = args[1].parse().unwrap();
    let mut reader = std::io::BufReader::new(std::fs::File::open(&args[2]).unwrap());
    let other_args = &args[3..];

    match day {";

    for i in range {
        code += &format!(
            "
        {i} => day{i:02}::solution(other_args, &mut reader),"
        );
    }

    code += "
        _ => {
            eprintln!(\"Error: day {} not yet implemented\", day);
            std::process::exit(1);
        }
    }
}
";

    code.parse().unwrap()
}
