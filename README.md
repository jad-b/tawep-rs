The Art of Writing Efficient Programs - In Rust
========================================

Setting Up the Development Environment
----------------------------------------
### Tooling
#### [gperftools](https://github.com/gperftools/gperftools/releases)

    ./configure
    make
    make check
    make install


If `$LD_LIBRARY_PATH` is unset:

    export LD_LIBRARY_PATH=/usr/local/lib

#### pprof

    go install github.com/google/pprof@latest

#### perf
You may need a newer version of `perf`. Building from source gave me 6.17, while my box came with 4.18.

The battle of getting all the correct packages installed and configuration options passed is a lonesome road we all must walk alone,
but here are some of the resources and commands I ended up using.

* [Perf wiki](https://perf.wiki.kernel.org/index.php/Perf_tools_support_for_Intel%C2%AE_Processor_Trace#Downloading_and_building_the_latest_perf_tools)
* [How to change the install destination](https://stackoverflow.com/a/72922164)

### C++
#### Building

    clang++ -g -O3 -mavx2 -Wall -pedantic 01_substring_sort.C 01_substring_sort_a.C -o example


#### Profiling
4. Add `-lprofiler` to the compiler arguments
5. Execute the program:

    CPUPROFILE=prof.data <program>
6. Analyze the data

    pprof -http=":" <program> prof.data

### Rust

    cargo test
