# The Rust Programming Language

[Foreword](foreword.html)
[Introduction](ch00-00-introduction.html)

## Getting started

- [Getting Started](ch01-00-getting-started.html)
    - [Installation](ch01-01-installation.html)
    - [Hello, World!](ch01-02-hello-world.html)
    - [Hello, Cargo!](ch01-03-hello-cargo.html)

- [Programming a Guessing Game](ch02-00-guessing-game-tutorial.html)

- [Common Programming Concepts](ch03-00-common-programming-concepts.html)
    - [Variables and Mutability](ch03-01-variables-and-mutability.html)
    - [Data Types](ch03-02-data-types.html)
    - [How Functions Work](ch03-03-how-functions-work.html)
    - [Comments](ch03-04-comments.html)
    - [Control Flow](ch03-05-control-flow.html)

- [Understanding Ownership](ch04-00-understanding-ownership.html)
    - [What is Ownership?](ch04-01-what-is-ownership.html)
    - [References & Borrowing](ch04-02-references-and-borrowing.html)
    - [Slices](ch04-03-slices.html)

- [Using Structs to Structure Related Data](ch05-00-structs.html)
    - [Defining and Instantiating Structs](ch05-01-defining-structs.html)
    - [An Example Program Using Structs](ch05-02-example-structs.html)
    - [Method Syntax](ch05-03-method-syntax.html)

- [Enums and Pattern Matching](ch06-00-enums.html)
    - [Defining an Enum](ch06-01-defining-an-enum.html)
    - [The `match` Control Flow Operator](ch06-02-match.html)
    - [Concise Control Flow with `if let`](ch06-03-if-let.html)

## Basic Rust Literacy

- [Packages, Crates, and Modules](ch07-00-packages-crates-and-modules.html)
    - [Packages and crates for making libraries and executables](ch07-01-packages-and-crates-for-making-libraries-and-executables.html)
    - [Modules and `use` to control scope and privacy](ch07-02-modules-and-use-to-control-scope-and-privacy.html)

- [Common Collections](ch08-00-common-collections.html)
    - [Vectors](ch08-01-vectors.html)
    - [Strings](ch08-02-strings.html)
    - [Hash Maps](ch08-03-hash-maps.html)

- [Error Handling](ch09-00-error-handling.html)
    - [Unrecoverable Errors with `panic!`](ch09-01-unrecoverable-errors-with-panic.html)
    - [Recoverable Errors with `Result`](ch09-02-recoverable-errors-with-result.html)
    - [To `panic!` or Not To `panic!`](ch09-03-to-panic-or-not-to-panic.html)

- [Generic Types, Traits, and Lifetimes](ch10-00-generics.html)
    - [Generic Data Types](ch10-01-syntax.html)
    - [Traits: Defining Shared Behavior](ch10-02-traits.html)
    - [Validating References with Lifetimes](ch10-03-lifetime-syntax.html)

- [Testing](ch11-00-testing.html)
    - [Writing tests](ch11-01-writing-tests.html)
    - [Running tests](ch11-02-running-tests.html)
    - [Test Organization](ch11-03-test-organization.html)

- [An I/O Project: Building a Command Line Program](ch12-00-an-io-project.html)
    - [Accepting Command Line Arguments](ch12-01-accepting-command-line-arguments.html)
    - [Reading a File](ch12-02-reading-a-file.html)
    - [Refactoring to Improve Modularity and Error Handling](ch12-03-improving-error-handling-and-modularity.html)
    - [Developing the Library’s Functionality with Test Driven Development](ch12-04-testing-the-librarys-functionality.html)
    - [Working with Environment Variables](ch12-05-working-with-environment-variables.html)
    - [Writing Error Messages to Standard Error Instead of Standard Output](ch12-06-writing-to-stderr-instead-of-stdout.html)

## Thinking in Rust

- [Functional Language Features: Iterators and Closures](ch13-00-functional-features.html)
    - [Closures: Anonymous Functions that Can Capture Their Environment](ch13-01-closures.html)
    - [Processing a Series of Items with Iterators](ch13-02-iterators.html)
    - [Improving Our I/O Project](ch13-03-improving-our-io-project.html)
    - [Comparing Performance: Loops vs. Iterators](ch13-04-performance.html)

- [More about Cargo and Crates.io](ch14-00-more-about-cargo.html)
    - [Customizing Builds with Release Profiles](ch14-01-release-profiles.html)
    - [Publishing a Crate to Crates.io](ch14-02-publishing-to-crates-io.html)
    - [Cargo Workspaces](ch14-03-cargo-workspaces.html)
    - [Installing Binaries from Crates.io with `cargo install`](ch14-04-installing-binaries.html)
    - [Extending Cargo with Custom Commands](ch14-05-extending-cargo.html)

- [Smart Pointers](ch15-00-smart-pointers.html)
    - [`Box<T>` Points to Data on the Heap and Has a Known Size](ch15-01-box.html)
    - [The `Deref` Trait Allows Access to the Data Through a Reference](ch15-02-deref.html)
    - [The `Drop` Trait Runs Code on Cleanup](ch15-03-drop.html)
    - [`Rc<T>`, the Reference Counted Smart Pointer](ch15-04-rc.html)
    - [`RefCell<T>` and the Interior Mutability Pattern](ch15-05-interior-mutability.html)
    - [Creating Reference Cycles and Leaking Memory is Safe](ch15-06-reference-cycles.html)

- [Fearless Concurrency](ch16-00-concurrency.html)
    - [Threads](ch16-01-threads.html)
    - [Message Passing](ch16-02-message-passing.html)
    - [Shared State](ch16-03-shared-state.html)
    - [Extensible Concurrency: `Sync` and `Send`](ch16-04-extensible-concurrency-sync-and-send.html)

- [Object Oriented Programming Features of Rust](ch17-00-oop.html)
    - [Characteristics of Object-Oriented Languages](ch17-01-what-is-oo.html)
    - [Using Trait Objects that Allow for Values of Different Types](ch17-02-trait-objects.html)
    - [Implementing an Object-Oriented Design Pattern](ch17-03-oo-design-patterns.html)

## Advanced Topics

- [Patterns Match the Structure of Values](ch18-00-patterns.html)
    - [All the Places Patterns May be Used](ch18-01-all-the-places-for-patterns.html)
    - [Refutability: Whether a Pattern Might Fail to Match](ch18-02-refutability.html)
    - [All the Pattern Syntax](ch18-03-pattern-syntax.html)

- [Advanced Features](ch19-00-advanced-features.html)
    - [Unsafe Rust](ch19-01-unsafe-rust.html)
    - [Advanced Lifetimes](ch19-02-advanced-lifetimes.html)
    - [Advanced Traits](ch19-03-advanced-traits.html)
    - [Advanced Types](ch19-04-advanced-types.html)
    - [Advanced Functions & Closures](ch19-05-advanced-functions-and-closures.html)
    - [Macros](ch19-06-macros.html)

- [Final Project: Building a Multithreaded Web Server](ch20-00-final-project-a-web-server.html)
    - [A Single Threaded Web Server](ch20-01-single-threaded.html)
    - [Turning our Single Threaded Server into a Multithreaded Server](ch20-02-multithreaded.html)
    - [Graceful Shutdown and Cleanup](ch20-03-graceful-shutdown-and-cleanup.html)

- [Appendix](appendix-00.html)
    - [A - Keywords](appendix-01-keywords.html)
    - [B - Operators and Symbols](appendix-02-operators.html)
    - [C - Derivable Traits](appendix-03-derivable-traits.html)
    - [D - Useful Development Tools](appendix-04-useful-development-tools.html)
    - [E - Editions](appendix-05-editions.html)
    - [F - Translations](appendix-06-translation.html)
    - [G - How Rust is Made and “Nightly Rust”](appendix-07-nightly-rust.html)
