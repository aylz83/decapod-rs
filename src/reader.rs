use std::{ffi::CString, ptr};

use std::io::{Read as StdRead, Seek, SeekFrom};

pub use crate::reads::*;
pub use crate::read::*;
pub use crate::readbatch::*;

#[cfg(feature = "polars")]
use polars::prelude::*;

#[cfg(feature = "recursive")]
use ignore::{WalkBuilder, types::TypesBuilder};
use std::path::Path;

/// Contains reader options, passed to Reader.
pub struct ReaderOptions
{
	force_disable_file_mapping: bool,
}

impl ReaderOptions
{
	/// Create a reader options struct to specify when opening pod5 files
	/// The only option is, is to disable file mapping.
	pub fn new(force_disable_file_mapping: bool) -> ReaderOptions
	{
		ReaderOptions {
			force_disable_file_mapping,
		}
	}

	pub(crate) fn to_ffi(&self) -> crate::pod5_ffi::Pod5ReaderOptions_t
	{
		crate::pod5_ffi::Pod5ReaderOptions_t {
			force_disable_file_mapping: self.force_disable_file_mapping as i8,
		}
	}
}

pub(crate) struct InternalReader
{
	pub(crate) inner: *mut crate::pod5_ffi::Pod5FileReader_t,
	pub(crate) has_compression: bool,
}

impl InternalReader
{
	pub(crate) fn count(&self) -> crate::error::Result<usize>
	{
		let mut read_count: usize = 0;
		unsafe {
			crate::pod5_ffi::pod5_get_read_count(self.inner, &mut read_count);
		}

		crate::pod5_ok!(read_count)
	}

	pub(crate) fn read_ids(&self) -> crate::error::Result<Vec<uuid::Uuid>>
	{
		let read_count = self.count()?;
		let mut read_ids = vec![[0; 16]; read_count];
		unsafe {
			crate::pod5_ffi::pod5_get_read_ids(self.inner, read_count, read_ids.as_mut_ptr());
		}

		let read_ids = read_ids
			.iter()
			.map(|&read_id| uuid::Uuid::from_bytes(read_id))
			.collect::<Vec<uuid::Uuid>>();

		crate::pod5_ok!(read_ids)
	}

	fn detect_signal_compression<P: AsRef<Path>>(&mut self, path: P) -> crate::error::Result<()>
	{
		let mut file_data: crate::pod5_ffi::EmbeddedFileData_t = Default::default();

		unsafe {
			crate::pod5_ffi::pod5_get_file_signal_table_location(self.inner, &mut file_data);
		}

		//let c_str = unsafe { CStr::from_ptr(file_data.file_name) };
		//let str_slice = c_str.to_str().expect("Invalid UTF-8 sequence");

		let mut file = std::fs::File::open(path)?;

		file.seek(SeekFrom::Start(file_data.offset as u64))?;

		let mut buffer = vec![0u8; file_data.length];
		file.read_exact(&mut buffer)?;

		let cursor = std::io::Cursor::new(buffer); // Wrap buffer in a Cursor for StreamReader
		let reader = match arrow::ipc::reader::FileReader::try_new(cursor, None)
		{
			Ok(reader) => reader,
			Err(_) =>
			{
				return Err(crate::error::Error::ArrowCompressionError(
					"unable to determine signal compression".to_string(),
				))
			}
		};

		let schema = reader.schema();
		let signal_field = match schema.field_with_name("signal")
		{
			Ok(field) => field,
			Err(_) =>
			{
				return Err(crate::error::Error::ArrowCompressionError(
					"unable to determine signal compression".to_string(),
				))
			}
		};

		match signal_field.data_type()
		{
			arrow::datatypes::DataType::LargeBinary => self.has_compression = true,
			_ => self.has_compression = false,
		};

		Ok(())
	}

	pub(crate) fn get_fetch_path(
		&self,
		fetch: &Option<Vec<uuid::Uuid>>,
		rows: &mut usize,
	) -> Option<Vec<(usize, Vec<u32>)>>
	{
		match fetch
		{
			Some(fetch) =>
			{
				let read_ids: Vec<[u8; 16]> = fetch.iter().map(|uuid| *uuid.as_bytes()).collect();

				let mut batch_count: usize = 0;
				unsafe {
					crate::pod5_ffi::pod5_get_read_batch_count(&mut batch_count, self.inner);
				}

				let mut rows_per_batch: Vec<u32> = vec![0u32; batch_count];
				let mut batch_rows: Vec<u32> = vec![0u32; read_ids.len()];
				let mut count: usize = 0;

				unsafe {
					crate::pod5_ffi::pod5_plan_traversal(
						self.inner,
						read_ids.as_ptr() as *const u8,
						read_ids.len(),
						rows_per_batch.as_mut_ptr(),
						batch_rows.as_mut_ptr(),
						&mut count,
					);
				}

				if count == 0
				{
					return None;
				}

				let mut offset: usize = 0;

				let batch_row_map = rows_per_batch
					.into_iter()
					.enumerate() // Include indices for keys in the result
					.map(|(index, size)| {
						// Capture the rows for this group and update the offset
						let rows = batch_rows[offset..offset + size as usize].to_vec();
						offset += size as usize;
						(index, rows)
					})
					.collect::<Vec<(usize, Vec<u32>)>>();

				*rows = batch_row_map.len();

				Some(batch_row_map)
			}
			None => None,
		}
	}
}

impl Drop for InternalReader
{
	fn drop(&mut self)
	{
		unsafe {
			crate::pod5_ffi::pod5_close_and_free_reader(self.inner);
			crate::pod5_ffi::pod5_terminate();
		}
	}
}

/// Open pod5 files, directories of pod5 files and iterate over records and reads.
///
/// # Example
///
/// ```
/// use decapod::reader::Reader
/// use uuid::Uuid
/// use std::error::Error
///
/// fn main() -> Result<(), Box<dyn Error>>
/// {
///     let reader = Reader::from_path("example.pod5", None)?;
///
///     println!("{:?}", &reader.read_ids()?);
///     Ok(())
/// }
/// ````

pub struct Reader
{
	pub(crate) inner: Vec<InternalReader>,
}

impl Reader
{
	/// Opens a pod5 file or directory of pod5 files for reading.
	/// # Arguments
	///
	/// * `path` - The path to either a pod5 or a directory containing pod5 files.
	/// * `options` - the [`ReaderOptions`] object. Pass None to use no options (typical).
	pub fn from_path<P: AsRef<Path>>(
		path: P,
		options: Option<ReaderOptions>,
	) -> crate::error::Result<Reader>
	{
		unsafe {
			crate::pod5_ffi::pod5_init();
		}

		let mut readers = Vec::new();
		if path.as_ref().is_file()
		{
			readers.push(Self::_reader_from_file(path, &options)?);
		}
		else
		{
			#[cfg(feature = "recursive")]
			readers.extend(Self::_readers_from_dir(path, &options)?);
		}

		let reader = Reader { inner: readers };

		crate::pod5_ok!(reader)
	}

	/// Opens a combinations of both pod5 file paths and directories containing pod5 files.
	/// # Arguments
	///
	/// * `paths` - The vector of pod5 file and directory paths.
	/// * `options` - the [`ReaderOptions`] object. Pass None to use no options (typical).
	pub fn from_vec<P>(
		paths: Vec<P>,
		options: Option<ReaderOptions>,
	) -> crate::error::Result<Reader>
	where
		P: AsRef<Path>,
	{
		Self::from_iter(paths.iter(), options)
	}

	/// Opens a combinations of both pod5 file paths and directories containing pod5 files.
	/// # Arguments
	///
	/// * `iter` - The iterator object consisting of pod5 file and directory paths.
	/// * `options` - the [`ReaderOptions`] object. Pass None to use no options (typical).
	pub fn from_iter<P, I>(iter: I, options: Option<ReaderOptions>) -> crate::error::Result<Reader>
	where
		I: IntoIterator<Item = P>,
		P: AsRef<Path>,
	{
		unsafe {
			crate::pod5_ffi::pod5_init();
		}

		let mut readers = Vec::new();
		for path in iter
		{
			if path.as_ref().is_file()
			{
				readers.push(Self::_reader_from_file(path, &options)?);
			}
			else
			{
				#[cfg(feature = "recursive")]
				readers.extend(Self::_readers_from_dir(path, &options)?);
			}
		}

		let reader = Reader { inner: readers };

		crate::pod5_ok!(reader)
	}

	fn _reader_from_file<P: AsRef<Path>>(
		path: P,
		options: &Option<ReaderOptions>,
	) -> crate::error::Result<InternalReader>
	{
		let c_string = path
			.as_ref()
			.to_str()
			.ok_or_else(|| crate::error::Error::MemoryError("memory error".to_string()))
			.and_then(|s| {
				CString::new(s)
					.map_err(|_| crate::error::Error::MemoryError("memory error".to_string()))
			});

		let ptr = match options
		{
			Some(options) =>
			unsafe {
				crate::pod5_ffi::pod5_open_file_options(c_string?.as_ptr(), &options.to_ffi())
			},
			None =>
			unsafe { crate::pod5_ffi::pod5_open_file(c_string?.as_ptr()) },
		};

		let mut reader = InternalReader {
			inner: ptr,
			has_compression: false,
		};
		reader.detect_signal_compression(path)?;

		crate::pod5_ok!(reader)
	}

	#[cfg(feature = "recursive")]
	fn _readers_from_dir<P: AsRef<Path>>(
		path: P,
		options: &Option<ReaderOptions>,
	) -> crate::error::Result<Vec<InternalReader>>
	{
		let mut types_builder = TypesBuilder::new();
		types_builder.add("pod5", "*.pod5").expect("REASON");
		types_builder.select("pod5");
		let matcher = types_builder.build().unwrap();

		let walker = WalkBuilder::new(path).types(matcher).build();

		let mut results = Vec::new();

		for result in walker
		{
			match result
			{
				Ok(entry) =>
				{
					let path = entry.path();
					if path.is_file()
					{
						let reader = Self::_reader_from_file(path, &options)?;
						results.push(reader);
					}
				}
				Err(_) =>
				{}
			}
		}

		Ok(results)
	}

	/// Returns the total number of read ids from all open pod5 files.
	pub fn count(&self) -> crate::error::Result<usize>
	{
		self.inner.iter().try_fold(0usize, |acc, item| {
			// `try_fold` will propagate errors in the `Result` from `item.count()`
			item.count().map(|count| acc + count)
		})
	}

	/// Returns the uuids for every read id from all open pod5 files.
	pub fn read_ids(&self) -> crate::error::Result<Vec<uuid::Uuid>>
	{
		self.inner.iter().try_fold(Vec::new(), |mut acc, item| {
			item.read_ids().map(|ids| {
				acc.extend(ids); // Push the count (String) to the accumulator
				acc
			})
		})
	}

	/// Returns a vector of all the file info structs found within the open pod5 files.
	pub fn info(&self) -> Vec<crate::error::Result<crate::fileinfo::FileInfo>>
	{
		self.inner
			.iter()
			.map(|reader| {
				let mut file_ptr: crate::pod5_ffi::FileInfo = Default::default();

				unsafe {
					crate::pod5_ffi::pod5_get_file_info(reader.inner, &mut file_ptr);
				}

				crate::pod5_ok!(crate::fileinfo::FileInfo { inner: file_ptr })
			})
			.collect()
	}

	/// Obtain the runinfo iterator.
	pub fn run_info_iter(&self) -> crate::runinfo::RunInfoIter
	{
		crate::runinfo::RunInfoIter {
			rows: 0,
			reader: self.inner.iter(),
			current_row: 0,
			current_reader: None,
		}
	}

	/// Create a Reads iterator for reads found within the open pod5 files.
	///
	/// # Arguments
	///
	/// * `fetch` - Specify None to obtain all reads, or a vector of uuids for specific reads of interest
	///
	/// # Example
	///
	/// ```
	/// let reader = Reader::from_path("sample.pod5", None)?;
	/// let read_ids = vec![uuid!("002fde30-9e23-4125-9eae-d112c18a81a7")];
	/// for read in reader.reads_iter(Some(read_ids))
	/// {
	///     let read = read?;
	///     println!("{}", read.read_id()?);
	/// }
	/// ```
	///
	/// # Returns
	///
	/// A reads iterator.
	pub fn reads_iter(&self, fetch: Option<Vec<uuid::Uuid>>) -> Reads
	{
		Reads {
			reader: self.inner.iter(),
			batch_count: 0,
			batch_rows: 0,
			current_batch: 0,
			current_row: 0,
			inner: ptr::null_mut(),
			inner_reader: None,
			fetch,
			fetch_path: None,
		}
	}

	//#[cfg(feature = "polars")]
	//pub fn to_df(&self, fields: &Option<Vec<&str>>) -> crate::error::Result<DataFrame>
	//{
	//	Ok(self
	//		.batch_records_iter()?
	//		.map(|result| result.expect("REASON").to_df(&fields).expect("REASON"))
	//		.reduce(|acc_df, next_df| {
	//			acc_df
	//				.clone()
	//				.vstack_mut(&next_df)
	//				.map(|_| acc_df)
	//				.expect("REASON") // Combine vertically
	//		})
	//		.unwrap())
	//}

	/// Obtain the batch records iterator.
	/// Currently only useful for converting a record of reads to a Polars DataFrame.
	/// # Arguments
	///
	/// * `fetch` - Specific read ids can be requested in the same way as [`Reader::reads_iter`].
	pub fn batch_records_iter(&self, fetch: Option<Vec<uuid::Uuid>>) -> BatchRecordIter
	{
		BatchRecordIter {
			reader: self.inner.iter(),
			rows: 0,
			current_row: 0,
			inner_reader: None,
			fetch,
			fetch_path: None,
		}
	}

	//fn read(&self, read: &mut Read) -> bool
	//{
	//}
}
