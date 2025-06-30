# Structopt Challenge

Welcome to the **Structopt** challenge, inspired by the [PingCap Talent Plan](https://github.com/pingcap/talent-plan). This is a bite-sized Rust project that introduces you to implementing a simple in-memory key-value store.

## 📚 Overview

In this project we used clap to parse command line arguments. It's typical to represent a program's parsed command line arguments as a struct, perhaps named Config or Options. Doing so requires calling the appropriate methods on clap's ArgMatches type. Both steps, for larger programs, require a lot of boilerplate code. The structopt crate greatly reduces boilerplate by allowing you to define a Config struct, annotated to automatically produce a clap command line parser that produces that struct. Some find this approach nicer than writing the clap code explicitly.

Modify your program to use structopt for parsing command line arguments instead of using clap directly.

Happy coding! 🚀
