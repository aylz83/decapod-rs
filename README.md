# decapod-rs 
Rust bindings for the pod5 library

![decapod-rs logo](assets/decapod_logo.png)

decapod-rs aims to provide Rust bindings for the ![pod5-file-format](https://github.com/nanoporetech/pod5-file-format) library in a Rust idiomatic way.

> [!NOTE]
> It is work in progress and currently only supports reading pod5 files, but providing bindings for writing pod5 files is planned.

# Building

```
git clone https://github.com/aylz83/decapod-rs.git
cd decapod-rs
git submodule update --init --recursive # clones https://github.com/nanoporetech/pod5-file-format
cargo build # builds the crate and pod5-file-format library
cd example
cargo build # build the example
./target/debug/decapod_example ../third_party/pod5-file-format/test_data/multi_fast5_zip_v3.pod5 
```

- Serialisation can be enabled with the 'serde' feature.
- Polars DF generation can be enabled with the 'polars' feature

# Features

- Iterate over all reads.
- Extract run info.
- Serialise reads with serde.

# Planned features

- [x] Reading of pod5, including signal.
- [x] Reading additional (metadata) from the pod5.
- [x] Serialisation of reads with serde.
- [x] Conversion of reads to Polars dataframes.
- [ ] Writing pod5 files.
- [ ] Support older pod5 specifications other than V3.
- [ ] Create documentation.
