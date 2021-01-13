fn get_title() -> String {
	// Возвращает строку - 'короткий баннер'
	let mut the_title = String::from(env!("CARGO_PKG_NAME"));
	the_title.push_str(" (v");
	the_title.push_str(env!("CARGO_PKG_VERSION"));
	the_title.push_str("), ");
	the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));
	return the_title;
}

fn parse_markdown_file() {
	// Будет вызываться, когда нам передадут md-файл через cl
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
	usage();
}