use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;

fn get_title() -> String {
	// Возвращает строку - 'короткий баннер'
	let mut the_title = String::from(env!("CARGO_PKG_NAME"));
	the_title.push_str(" (v");
	the_title.push_str(env!("CARGO_PKG_VERSION"));
	the_title.push_str("), ");
	the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));
	return the_title;
}

fn parse_markdown_file(_filename: &str) {
	// Будет вызываться, когда нам передадут md-файл через CL
	print_short_banner();
	println!("[ INFO ] Starting parser!");

	// Создать переменную path из имени _filename
	let input_filename = Path::new(_filename);

	// Попытаюсь открыть файл
	let file = File::open(&input_filename)
		.expect("[ ERROR ] Failed to open file!");

	let mut _ptag: bool = false;
	let mut _htag: bool = false;

	let mut tokens: Vec<String> = Vec::new();

	let reader = BufReader::new(file);

	for line in reader.lines() {
		let line_contents = line.unwrap();

		let mut first_char: Vec<char> = line_contents
			.chars() // преобразование в последовательность символов
			.take(1) // выхватываем первый элемент
			.collect(); // преобразуем в тип Vec

		let mut output_line = String::new();

		match first_char.pop() {
			Some('#') => {
				if _ptag {
					_ptag = false;
					output_line.push_str("</p\n>");
				}

				if _htag {
					_htag = false;
					output_line.push_str("</h1>\n>");
				}

				_htag = true;
				output_line.push_str("<h1>");
				output_line.push_str(&line_contents[2..]);
			},
			_ => {
				if !_ptag {
					_ptag = true;
					output_line.push_str("<p>");
				}

				output_line.push_str(&line_contents);

				if _htag {
					_htag = false;
					output_line.push_str("</h1>\n");
				}
			}
		};

		if _htag {
			_htag = false;
			output_line.push_str("</h1>\n");
		}

		if _ptag {
			_ptag = false;
			output_line.push_str("</p>\n");
		}

		if output_line != "<p></p>\n" {
			tokens.push(output_line);
		}		
	}

	let mut output_filename = String::from(&_filename[.._filename.len()-3]);
	output_filename.push_str(".html");

	let mut outfile = File::create(output_filename)
		.expect("[ ERROR ] Could not create output file!");

	for line in &tokens {
		outfile.write_all(line.as_bytes())
			.expect("[ ERROR ] Could not write to output file!");
	}

	println!("[ INFO ] Parsing complete!");
}

fn print_short_banner() {
	// Выведет название, версию и описание
	println!("{}", get_title());
}

fn print_long_banner() {
	// Выведет короткий баннер плюс пример
	print_short_banner();
	let mut line = String::from("Written by: ");
	line.push_str(env!("CARGO_PKG_AUTHORS"));
	line.push_str("\nHomepage: ");
	line.push_str(env!("CARGO_PKG_HOMEPAGE"));
	line.push_str("\nUsage: tinymd <somefile>.md");
	println!("{}", line);
}

fn usage() {
	print_long_banner();
}

fn main() {
	// Работа с аргументами CL
	let args: Vec<String> = std::env::args().collect();
	match args.len() {
		2 => parse_markdown_file(&args[1]),
		_ => {
			println!("[ ERROR ] Invalid invocation (you done goofes!)");
			usage();
		}
	}
}