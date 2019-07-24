use std::{env, io};
fn main() {
    let padding = '　';
    let mut inputnumber: usize = 0;
    let mut inputtext: String = String::new();
    let args: Vec<String> = env::args().collect();
    let mut outvec: Vec<Vec<char>> = Vec::new();

    for (index, value) in args.iter().enumerate() {
        if index > 0 {
            match value.trim().parse::<usize>() {
                Ok(number) => inputnumber = number,
                Err(_) => inputtext.push_str(value),
            }
        } else if args.len() == 1 {
            println!("请输入中文文本：");
            match io::stdin().read_line(&mut inputtext) {
                Ok(_) => {
                    inputtext = inputtext.trim().to_string();
                }
                Err(_) => {
                    eprintln!("错误：无效的输入。");
                    std::process::exit(1);
                }
            }
        } else {
            continue;
        }
    }

    let mut text_chars: Vec<char> = if inputtext.is_empty() {
        eprintln!("错误：没有输入中文文本。");
        std::process::exit(2);
    } else {
        inputtext.chars().collect()
    };
    let numberof_lines = if inputnumber == 0 {
        (text_chars.len() as f64).sqrt() as usize
    } else {
        inputnumber
    };

    for _ in 0..numberof_lines {
        outvec.push(Vec::new());
    }

    match text_chars.len() % numberof_lines {
        0 => {}
        remainder => {
            for _ in 0..(numberof_lines - remainder) {
                text_chars.push(padding);
            }
        }
    }

    for _ in 0..(text_chars.len() / numberof_lines) {
        for invec in outvec.iter_mut().rev() {
            invec.push(text_chars.pop().unwrap());
        }
    }

    for invec in &outvec {
        let tmp: String = invec.iter().collect();
        println!("{}", tmp);
    }
}
