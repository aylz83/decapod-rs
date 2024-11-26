# decapod-rs - 
rust bindings for the pod5 library

![decapod-rs logo](assets/decapod_logo.png)

decapod-rs aims to provide rust bindings for the pod5-file-format library in a rust idiomatic way.

It is work in progress, currently only supports reading pod5 files with limited usage, but providing bindings for writing pod5 is planned.

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

# Planned features

- [x] Reading of pod5, including signal
- [ ] Reading additional (metadata) from the pod5 such as run info or end reason
- [x] Serialisation of reads with serde
- [ ] Conversion of reads to something like Polars dataframes
- [ ] Writing pod5 files
- [ ] Support older pod5 specifications other than V3
- [ ] Create documentation
