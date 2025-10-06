# 🦀 Rust Lox Interpreter (Based on *Crafting Interpreters*)

This is a work-in-progress interpreter for the **Lox** language from [*Crafting Interpreters* by Robert Nystrom], written in **Rust**.  
I'm a beginner Rust programmer, so this project was mainly a learning exercise in both **Rust** and **language implementation** — not a perfect reproduction of the book’s code or the full Lox specification.

---

## 📖 About the Project

This interpreter is based on the **tree-walk interpreter** from *Crafting Interpreters*, implemented up to **Chapter 12 (Classes)**.  
It includes the basic foundations of the language but omits later chapters such as classes, inheritance, and the virtual machine.

### ✅ Features Implemented

- [x] **Lexical analysis (Scanner)** — Converts source code into tokens  
- [x] **Parsing (Recursive descent parser)** — Builds an AST from tokens  
- [x] **Expressions** — Arithmetic, comparison, logical, grouping, etc.  
- [x] **Statements** — Print statements, variable declarations, and blocks  
- [x] **Control Flow** — `if`, `else`, `while` and `for` statements  
- [x] **Functions and Closures** — `fun` and `return` statements
- [x] **Variable Resolution (Semantic Analysis)** — reports undeclared, unassigned, unused variables
- [x] **Error handling** — Basic runtime and syntax error reporting  

### 🚧 Not Yet Implemented

- [ ] Classes and inheritance  
- [ ] The bytecode VM (from the second part of the book)  

---

## ⚙️ Building and Running

Make sure you have **Rust** installed.

```bash
# Clone this repository
git clone https://github.com/vRendevski/rlox.git
cd rlox

# Build the project
cargo build

# Run the interpreter
cargo run -- path/to/script.lox
```


## 🧠 Learning Goals

This project was made to:
 - Practice Rust fundamentals (ownership, enums, pattern matching)
 - Understand lexing, parsing, and interpreting
 - Follow along with Crafting Interpreters in a different language
 - I didn’t follow the book’s language specification to the letter —
some syntax and behavior may differ slightly, and certain shortcuts were taken for simplicity.

## 🧩 Example Code

Here’s a simple Lox snippet that works:

```
var x = 3;
while (x > 0) {
  print x;
  x = x - 1;
}

if (x == 0) {
  print "Done!";
}
```

Expected output:

```
3
2
1
Done!
```

## 💬 Notes

This project is not production-ready and likely has bugs,
but it’s been a great way to deepen my understanding of how interpreters work.

If you’re also learning Rust or following Crafting Interpreters,
feel free to explore, fork, or suggest improvements!
