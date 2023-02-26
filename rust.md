# Rust

## Cargo

How does cargo work?<br>
New project binary: `cargo new <project_name> --bin`<br>
Build project: `cargo build`<br>
Run project: `cargo run`<br>
Cargo: `cargo <command>`<br>
Cargo commands: `cargo new <project_name> --bin, cargo build, cargo run, cargo check, cargo update, cargo clean, cargo doc, cargo publish, cargo install, cargo uninstall, cargo search, cargo test, cargo bench`<br>
Cargo features: `cargo build --release, cargo build --target <target>, cargo build --features "<feature>"`<br>
Cargo config: `~/.cargo/config`<br>
Cargo.toml: `[package] name = "<name>" version = "<version>" authors = ["<author>"] [dependencies] <crate_name> = "<version>"`<br>
Cargo.lock: `[[package]] name = "<name>" version = "<version>" dependencies = ["<crate_name> <version>"]`<br>

## Rust basics
Functions: `fn <function_name>(<arg>: <type>) -> <return_type> { <code> }`<br>
Variables: `let <var_name>: <type> = <value>`<br>
Constants: `const <const_name>: <type> = <value>`<br>
Mutable variables: `let mut <var_name>: <type> = <value>`<br>
Shadowing: `let <var_name>: <type> = <value>; let <var_name> = <value>`<br>
Data types: `i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, bool, char`<br>
Tuples: `let <var_name>: (<type>, <type>) = (<value>, <value>)`<br>
Arrays: `let <var_name>: [<type>; <size>] = [<value>; <size>]`<br>
Vectors: `let <var_name>: Vec<<type>> = vec![<value>, <value>]`<br>
Slices: `let <var_name>: &[<type>] = &[<value>, <value>]`<br>
Strings: `let <var_name>: String = String::from("<value>")`<br>
Structs: `struct <struct_name> { <field_name>: <type>, <field_name>: <type> }`<br>
Enums: `enum <enum_name> { <variant_name>, <variant_name> }`<br>
Option: `enum Option<T> { Some(T), None }`<br>
Result: `enum Result<T, E> { Ok(T), Err(E) }`<br>
If: `if <condition> { <code> } else { <code> }`<br>
Loop: `loop { <code> }`<br>
While: `while <condition> { <code> }`<br>
For: `for <var_name> in <iterable> { <code> }`<br>
Match: `match <value> { <pattern> => <code>, <pattern> => <code> }`<br>
Ranges: `let <var_name>: <type> = <start>..<end>`<br>
Functions: `fn <function_name>(<arg>: <type>) -> <return_type> { <code> }`<br>
Methods: `impl <struct_name> { fn <method_name>(&self) -> <return_type> { <code> } }`<br>
Traits: `trait <trait_name> { fn <method_name>(&self) -> <return_type> { <code> } }`<br>
Generics: `fn <function_name><T>(<arg>: T) -> T { <code> }`<br>
Closures: `let <var_name> = |<arg>: <type>| -> <return_type> { <code> };`<br>
Pointers: `let <var_name>: *const <type> = <value>`<br>
References: `let <var_name>: &<type> = &<value>`<br>
Smart pointers: `Box<T>, Rc<T>, RefCell<T>`<br>
Unsafe: `unsafe { <code> }`<br>
Modules: `mod <module_name> { <code> }`<br>
Crates: `extern crate <crate_name>;`<br>
Attributes: `#[<attribute>]`<br>
Comments: `//`<br>



