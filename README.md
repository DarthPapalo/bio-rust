# bio-rust
Bioinformatics library for Rust with a focus on balancing ergonomics and strongly typed systems.

## Features
- [x] **Sequence type** with methods for common operations over biological sequences.
- [x] **Alphabet type** with methods for sequence validation.
- [x] **Sequence views** that provide concrete methods only available for specific sequence types (e.g. Nucleotide, Dna, Rna).
- [x] **IO module** with FASTA and FASTQ readers.
- [x] **Quality view** with methods to work over quality strings such as the ones found in FASTQ files.

## Usage with cargo
Add the following line in your `Cargo.toml` under the `[dependencies]` section:
```toml
bio = { git = "https://github.com/DarthPapalo/bio-rust", version = "0.1.0" }
```

Or through the CLI: `cargo add --git https://github.com/DarthPapalo/bio-rust`

> [!NOTE]
> This crate is not uploaded to crates.io as it is a personal project. You should probably use the *de-facto* crate [rust-bio](https://crates.io/crates/bio)
