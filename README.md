[![Documentation on GitHub Pages](https://github.com/marcel-scherzinger/scratch-test-value/actions/workflows/deploy-docs-to-pages.yml/badge.svg)](https://marcel-scherzinger.github.io/scratch-test-value)
[![100% Rust](https://img.shields.io/badge/Rust-100%25-32c955?logo=rust)](https://rust-lang.org)

# About

The block-oriented programming language [Scratch](https://scratch.mit.edu/)
is used by many institutions to teach beginners how programming concepts work.
The `scratch-test` project is planned as a way to unit-test submissions of
learners for defined exercises.

This repository contains types modeling Scratch numbers and values and
mimics how arithmetic, logical, ... operations are working on them
according to the semantics defined by Scratch.

This is needed as Scratch semantics differ in many points from IEEE 754 and
the assumptions programmers would have about values when coming from other
programming languages.

Other crates like the model or interpreter build on top of it.
See those for a more detailed explaination of the project

# Limitations

This project is assumed to be used for **algorithmic exercises**, so the focus is
on control structures, input, output, variables and lists.

- Sounds, movements, colors, etc. are not planned.
- Scratch often tries to do _something_ to avoid exceptions or fatal errors.
  Some expressions don't evaluate to a value programmers would expect based on their
  knowledge from other languages. This project tries to model them but there is still a
  chance of differences in behaviour, especially when it comes to numbers
  or really weird edge cases.
