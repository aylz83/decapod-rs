#![warn(missing_docs)]

//! Rust bindings for the pod5-file-format (https::/github.com/nanoporetech/pod5-file-format) library.
//!
//! # Example
//!
//! Open a pod5 file and read all ids.
//!
//! ```
//! use decapod::reader::Reader;
//! use uuid::Uuid;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>>
//! {
//!     let reader = Reader::from_path("example.pod5", None)?;
//!
//!     println!("{:?}", &reader.read_ids()?);
//!     Ok(())
//! }
//! ```
//! Multiple pod5 files can also be opened with the `from_vec` function
//!
//! ```
//! use decapod::reader::Reader;
//! use uuid::Uuid;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>>
//! {
//!     let pod5_files = vec!["sample1.pod", "sample2.pod5", "/folder/containing/more/pod5files"];
//!     let reader = Reader::from_vec(pod5_files, None)?;
//!
//!     // read_ids will now contain all read ids from all files opened.
//!     println!("{:?}", &reader.read_ids()?);
//!     Ok(())
//! }
//! ```
//! Open a pod5 file and iterate over all reads.
//!
//! ```
//! use decapod::reader::Reader;
//! use uuid::Uuid;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>>
//! {
//!     let reader = Reader::from_path("example.pod5", None)?;
//!
//!     for read in reader.reads_iter(None)
//!     {
//!         let read = read?;
//!         println!("read_id: {}", read.read_id()?);
//!     }
//!     Ok(())
//! }
//! ```

/// end reason metadata read from pod5 files.
pub mod endreason;
/// pod5 error codes.
pub mod error;
/// Read file info metadata stored within pod5 files.
pub mod fileinfo;
mod pod5_ffi;
mod read;
mod readbatch;
/// Open and read pod5 files.
pub mod reader;
mod reads;
/// obtain additional metadata stored within pod5 files.
pub mod runinfo;
