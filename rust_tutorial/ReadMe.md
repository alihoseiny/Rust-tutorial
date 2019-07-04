# Why we need these tests?
Rust is under heavy development and many features are changing. So we need tests those show us the tutorial is correct when a new version is coming.
So we have two kinds of tests for this tutorial.
# Compilation Fail Tests
Those tests are in the **CompileFailTest** directory. Each file contains examples of compiling fail in the tutorial with the same number.

These tests are *document test*s because we can check `compile_faile` only in this kind of test.

For running all the tests, you can simply run `make` command in that directory.

# Syntax and Behaviour Tests
These tests are in the **tests* directory. For running them you can run `cargo test` in the current directory.
One thing you should consider is many of these tests are not **good tests**. Because They are not really *unit tests*. Most of the times we just want to check that this code will compile.

