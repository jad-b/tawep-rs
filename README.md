The Art of Writing Efficient Programs - In Rust
========================================

Setting Up the Development Environment
----------------------------------------

### C++
#### Building and running

    clang++ -g -O3 -mavx2 -Wall -pedantic 01_substring_sort.C 01_substring_sort_a.C -o example && ./example

### Rust

    cargo test
