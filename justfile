# Run the emoji-lang interpreter.
# Example command:
#   `just run test/test.emoji`
run file_path:
    cargo run --bin emoji-lang -- -f {{file_path}}

# Translate from text to emoji-lang syntax.
# Runs in --dry-run mode by default and prints the translated text to the terminal,
# unless an additional flag `-o {{output_file}}` is passed.
# Example commands:
#    `just translate test/test.txt`
#    `just translate test/test.txt -o test/test.emoji`
translate input *FLAGS='--dry-run':
    cargo run --bin emoji-translator -- {{FLAGS}} -i {{input}}
