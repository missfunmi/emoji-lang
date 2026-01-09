# emoji-lang

Emojis are criminally underrated. This programming language attempts to restore their rightful place in

# Run the interpreter

To run all `.emoji` files in the `/test` directory:

```bash
for test_file in test/*.emoji; do 
  echo "testing ${test_file}"; 
  just run ${test_file}; 
done
```

To run a single file:

```bash
just run test/test.emoji  
```

# Run the translator CLI — does not actually run the emoji-lang code

To convert code in a text file to `.emoji` format and output the result to the terminal:

```bash
just translate test/test.txt
```

To convert code and output to an `.emoji` file:

```bash
just translate test/test.txt -o test/test.emoji
```
