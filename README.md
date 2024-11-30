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

# Example usage

```{rust}
use decapod::reader::Reader;
use uuid::Uuid;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>
{
    let reader = Reader::from_path("example.pod5", None)?;
    println!("{:?}", &reader.read_ids()?);

    Ok(())
}
````

Also, see the [example](example/src/main.rs) application.

# Features

- Iterate over all reads.
- Iterate over only specific reads to save time.
- Extract run info.
- Extract calibration info.
- Serialise reads with serde (enabled with the 'serde' feature).
- Convert batch records of reads to Polars dataframes (enabled with the 'polars' feature).
- Supports reading of multiple pod5 files.
- Reading directory of pod5 files (enabled with the 'recursive' feature).

# Planned features

- [x] Reading of pod5, including signal.
- [x] Reading additional (metadata) from the pod5.
- [x] Serialisation of reads with serde.
- [x] Conversion of reads to Polars dataframes.
- [ ] Writing pod5 files.
- [ ] Remove bindgen warnings.
- [x] Support older pod5 specifications other than V3.

# TODO

- Documentation
