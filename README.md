# Rust Start

Repository for learning [Rust](https://www.rust-lang.org/).

## Description

### Introduction

_Rust is a multi-paradigm, general-purpose programming language. Rust emphasizes performance, type safety, and concurrency. Rust enforces memory safety—that is, that all references point to valid memory—without requiring the use of a garbage collector or reference counting present in other memory-safe languages. To simultaneously enforce memory safety and prevent concurrent data races, Rust's "borrow checker" tracks the object lifetime of all references in a program during compilation. Rust is popular for systems programming but also offers high-level features including some functional programming constructs._

_Software developer Graydon Hoare created Rust as a personal project while working at Mozilla Research in 2006. Mozilla officially sponsored the project in 2009. Since the first stable release in May 2015, Rust has been adopted by companies including Amazon, Discord, Dropbox, Facebook (Meta), Google (Alphabet), and Microsoft._

_Rust has been noted for its growth as a newer language and has been the subject of academic programming languages research._

 \- ___from [wikipedia](https://en.wikipedia.org/wiki/Rust_(programming_language))___

### Details

|Property|Value|
|---|---|
|Paradigms|Multi-paradigm: `concurrent`, `functional`, `generic`, `imperative`, `structured`|
|Designed by|Graydon Hoare|
|First Release|2010-07-07|
|Typing Discipline|`affine`, `inferred`, `nominal`, `static`, `strong`|
|Implemented in|Rust|
|Platform|Cross-Platform|
|Operating System|Cross-System|
|License|`Apache` and `MIT`|
|File extension(s)|`.rs`, `.rlib`|
|Website|[www.rust-lang.org](https://www.rust-lang.org)|

### Hello World

```rs
fn main() {
    println!("Hello, World!");
}
```

## Rust Documentation

1. [Official Documentation](https://www.rust-lang.org/learn)
2. [Playground](https://play.rust-lang.org/)
3. [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

## Development Environment Setup

__Note:__ You will require a personal access token to clone the repository.

1. `chmod +x scripts/*` to give all global scripts permission to execute.
2. `sh scripts/build.sh <release | debug | both>` to build the executable file.
3. `sh scripts/run.sh <release | debug>`
