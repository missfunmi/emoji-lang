use clap::Parser;
use std::collections::BTreeMap;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: Option<String>,
    #[arg(long)]
    dry_run: bool,
}

fn main() {
    let args = Args::parse();
    let input_code = fs::read_to_string(&args.input)
        .unwrap_or_else(|_| panic!("❌ Failed to read input file: {}", args.input));
    let output_code = translate_to_emoji_lang(&input_code);
    if args.dry_run {
        println!("{}", output_code);
    } else if let Some(output_file_path) = args.output {
        fs::write(&output_file_path, output_code)
            .unwrap_or_else(|_| panic!("Failed to write to output file: {}", output_file_path));
        println!("✅ Wrote translated emoji code to {}", output_file_path);
    } else {
        eprintln!("❌ Provide an output file path with --output, or use --dry-run");
    }
}

fn translate_to_emoji_lang(input: &str) -> String {
    let mut map = BTreeMap::new();

    // Longer patterns first to avoid premature partial matches
    map.insert("==", "👏👏");
    map.insert("!=", "🙅‍♀️👏");
    map.insert(">=", "📈👏");
    map.insert("<=", "📉👏");
    // map.insert(">=", "🛫");
    // map.insert("<=", "🛬");
    map.insert("+", "🥂");
    map.insert("-", "💔");
    map.insert("*", "✨");
    map.insert("/", "🔪");
    map.insert("%", "⚡️");
    map.insert("=", "👏");
    map.insert(">", "📈");
    map.insert("<", "📉");
    map.insert(",", "🔸");

    // Keywords
    map.insert("function", "🤖");
    map.insert("fn", "🤖");
    map.insert("fun", "🤖");
    map.insert("var", "🪄");
    map.insert("const", "🔒");
    map.insert("if", "🤔");
    map.insert("else", "🤷‍♀️");
    map.insert("true", "👍");
    map.insert("false", "👎");
    map.insert("nil", "🫥");
    map.insert("return", "🔙");
    map.insert("while", "🌀");
    map.insert("for", "⏳");
    map.insert("print", "🖨");
    map.insert("and", "🤝");
    map.insert("or", "🤌");

    // Brackets & delimiters
    map.insert("(", "🫱");
    map.insert(")", "🫲");
    map.insert("{", "🫸");
    map.insert("}", "🫷");
    map.insert(";", "✊");

    // Process replacements in order
    let mut result = input.to_string();
    for (k, v) in &map {
        result = result.replace(k, v);
    }

    // Add an EOF token
    result = result.trim_end().to_string();
    result.push_str("\n🔚\n");
    
    result
}
