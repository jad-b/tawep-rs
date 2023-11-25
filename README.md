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

Configurating permissions may be required too:

    sudo sysctl kernel.perf_event_paranoid=-1

### LLVM-MCA

    clang++-15 01_superscalar.C -g -O3 -mavx \
        --std=c++17 -mllvm -x86-asm-syntax=intel -S -o - \
      | llvm-mca-15 -mcpu=btver2 -timeline

If you're seeing:
> fatal error: 'algorithm' file not found

Check the gcc version being selected by `clang -v`
> Selected GCC installation: /usr/bin/../lib/gcc/x86_64-linux-gnu/12

And ensure you've got the matching libstdc++ version to match
([so](https://stackoverflow.com/a/75546125):

    sudo apt-get install libstdc++-12-dev


### C++
#### Building

The included `tawep` script can save some typing:

    ../../tawep build cpp 01_substring_sort.C 01_substring_sort_a.C


#### Profiling
1. Build with profiling enabled:

    ../../tawep build profile 01_substring_sort.C 01_substring_sort_a.C
5. Execute the program:

    CPUPROFILE=prof.data <program>
6. Analyze the data

    pprof -http=":" <program> prof.data

### Rust

### LLVM-MCA

`llvm-mca`-readable assembly can generated using:

    cargo rust -- --emit=asm

Which can be cat'd into `llvm-mca`:

    cat $(ls -t ./target/debug/deps/*.s | head -n1) \
      llvm-mca -mcpu=btver2 -timeline
