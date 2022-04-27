# Rust Bindings to the RDKit

Another POC project to create bindings to the C++ cheminformatics toolkit [RDKit](http://rdkit.org/) for the Rust programming language.

Like with [rdkit_rust](https://github.com/apahl/rdkit_rust) the goal is to bind directly to the C++ library, not via a CFFI layer.  
This project uses the [CXX](https://github.com/dtolnay/cxx) crate to generate the bindings, which is still a very manual effort, but seems to have less limitations than [cpp](https://github.com/mystor/rust-cpp) (so far).

## Disclaimer

More than everything, for me this is an excercise in learning Rust, which means, that these bindings are probably of limited use for others. Until the bindings are in a more mature state, I am also not really looking for contributions, but definitely feel free to re-use the code.

## Status

The bindings in their current state do work on Linux, but not much functionality has been wrapped, yet. Please have a look at the tests for what is available.
In order to compile this project, you need to have a conda installation of the RDKit and define the environment variable `RDKIT_CONDA` with the path to it, e.g. in `~/.profile`:

    export RDKIT_CONDA=/home/pahl/anaconda3/envs/chem

In addition, you will need the Rust toolchain and a C compiler (e.g. gcc).
After downloading this project, the tests can then be executed in the following way:

    $ LD_LIBRARY_PATH=$RDKIT_CONDA/lib cargo test
