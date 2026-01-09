# emoji-lang

<a href='https://www.recurse.com/scout/click?t=c7bc9ba4cb3e6725e05e413f16f8c5a3' title='Made with love at the Recurse Center'><img src='https://cloud.githubusercontent.com/assets/2883345/11325206/336ea5f4-9150-11e5-9e90-d86ad31993d8.png' height='20px'/></a>

Emojis are criminally underrated. This programming language attempts to restore their rightful place in the universe.

VERY MUCH a work in progress.

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
