# RustCalculator
This is a simple calculator programmed with Rust. Before using it, ensure you have already installed `git` and `rustc`.
You can use it by executing the following code:

**Linux/macOS:**
```bash
git clone https://github.com/Haozhe-py/RustCalculator.git
cd ./RustCalculator
rustc ./main.rs
./main
```
If you've already compiled, run `./main` directly.

**Windows:**
*Not supported yet.*

> [!WARNING]
> 1. Please **DON'T** enter any expressions with syntax error, or the program will panic!
> 2. Please **DON'T** enter `=` at the end of the expression!

> [!NOTE]
> The following operators are supported:
> | Symbol | Operation | Symbol | Operation |
> | ------ | --------- | ------ | --------- |
> | `+` | Add | `%` | Mod |
> | `-` | Sub | `^` | Pow |
> | `*` | Mul | `(` `)` | - |
> | `/` | Div | - | - |
> Entering operators that are not supported (e.g. `sqrt()`) will cause program panic!
