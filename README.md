# Rust Onboarding

(Not meant for public consumption)

## Goals:

To make neophytes comfortable reading source code
To give a jumping off point for those that want to learn to write Rust

## Non-goals:

To teach you how to write Rust
To discuss the minutiae of the language in depth

## Rust high level:

Systems language, meant to replace C/C++
Adds memory safety and ‘modern’ features
Can be hard to learn because it’s a compiled language with additional compile-time memory constraints
Hard but fair, exploding in popularity / ecosystem growth

## Features:

- Generics
- Traits
- Enums / ADTs
- Structs
- Async  
- Macro system
- Access controls
- Batteries included (std lib, 3rd party)

Non-features:

- Inheritance
- Mutability*
- Overloading
- Dynamic typing*
- Reflection*

Tips:

- If Rust is bad at X, it probably wants you to do X another way
- Favor `cargo check` (partial compilation without codegen or linking)
- Use `cargo fmt` and `cargo clippy` often
- Use `unimplemented!()` and `todo!()` to stub code

Resources:

- https://www.rust-lang.org/learn

Three levels of language ref:
- https://doc.rust-lang.org/book/
- https://doc.rust-lang.org/stable/rust-by-example/
- https://doc.rust-lang.org/reference/

Cargo:
- https://doc.rust-lang.org/cargo/index.html

Install:
- https://www.rust-lang.org/tools/install

Cheat sheets:

Memory:
- https://media.githubusercontent.com/media/usagi/rust-memory-container-cs/master/3840x2160/rust-memory-container-cs-3840x2160-dark-back.png

Comprehensive:
- https://cheats.rs/
