# Error Handling Challenge

In this project it will be possible for the code to fail due to I/O errors. So
before we get started implementing a database we need to do one more thing that
is crucial to Rust projects: decide on an error handling strategy.

<!-- TODO outline strategies? -->

Rust's error handling is powerful, but involves a lot of boilerplate to use
correctly. For this project the [`failure`] crate will provide the tools to
easily handle errors of all kinds.

[`failure`]: https://docs.rs/failure/0.1.5/failure/

The failure guide describes multiple error handling patterns.

[failure]: https://boats.gitlab.io/failure/
[guidance]: https://boats.gitlab.io/failure/guidance.html

Pick one of those strategies and, in your library, either define your own error
type or import `failure`s `Error`. This is the error type you will use in all of
your `Result`s, converting error types from other crates to your own with the
`?` operator.

After that, define a type alias for `Result` that includes your concrete error
type, so that you don't need to type `Result<T, YourErrorType>` everywhere, but
can simply type `Result<T>`. This is a common Rust pattern.

Finally, import those types into your executable with `use` statements, and
change `main`s function signature to return `Result<()>`. All functions in your
library that may fail will pass these `Results` back down the stack all the way
to `main`, and then to the Rust runtime, which will print an error.

Run `cargo check` to look for compiler errors, then fix them. For now it's
ok to end `main` with `panic!()` to make the project build.

_Set up your error handling strategy before continuing._

As with the previous project, you'll want to create placeholder data structures
and methods so that the tests compile. Now that you have defined an error type
this should be straightforward. Add panics anywhere necessary to get the test
suite to compile (`cargo test --no-run`).


<!--
## Aside: The history of Rust error handling
-->

_Note: Error-handling practices in Rust are still evolving. This course
currently uses the [`failure`] crate to make defining error types easier. While
`failure` has a good design, its use is [arguably not a best practice][nbp]. It
may not continue to be viewed favorably by Rust experts. Future iterations
of the course will likely not use `failure`. In the meantime, it is fine, and
presents an opportunity to learn more of the history and nuance of Rust error
handling._

[nbp]: https://github.com/rust-lang-nursery/rust-cookbook/issues/502#issue-387418261

<!--
Rust error handling has a long and winding history. Expert Rust programmers will
be aware of it, as that history informs and explains modern Rust error handling.

TODO
-->
