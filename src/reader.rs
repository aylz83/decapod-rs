use std::{ffi::CString, ptr};

use std::io::{Read as StdRead, Seek, SeekFrom};

pub use crate::reads::*;
pub use crate::read::*;

use crate::error::Pod5Error;

pub struct ReaderOptions
{
	force_disable_file_mapping: i8,
}

impl ReaderOptions
{
	pub fn new(force_disable_file_mapping: i8) -> ReaderOptions
	{
		ReaderOptions {
			force_disable_file_mapping,
		}
	}

	pub(crate) fn to_ffi(&self) -> crate::ffi::Pod5ReaderOptions_t
	{
		crate::ffi::Pod5ReaderOptions_t {
			force_disable_file_mapping: self.force_disable_file_mapping,
		}
	}
}
pub struct Reader
{
	pub(crate) inner: *mut crate::ffi::Pod5FileReader_t,

	pub(crate) has_compression: bool,
}

impl Reader
{
	pub fn from_file_with_options<'a>(
		filename: &'a str,
		options: &'a ReaderOptions,
	) -> crate::error::Result<Reader>
	{
		unsafe {
			crate::ffi::pod5_init();
		}

		let c_string = match CString::new(filename)
		{
			Ok(result) => result,
			Err(_) => return Err(Pod5Error::from_error_code(1, "memory string".to_string())),
		};

		let ptr =
			unsafe { crate::ffi::pod5_open_file_options(c_string.as_ptr(), &options.to_ffi()) };

		let mut reader = Reader {
			inner: ptr,
			has_compression: false,
		};
		reader.detect_signal_compression(filename)?;

		crate::pod5_ok!(reader)
	}

	///
	/// Opens a POD5
	///
	pub fn from_file(filename: &str) -> Result<Reader, crate::error::Pod5Error>
	{
		unsafe {
			crate::ffi::pod5_init();
		}

		let c_string = match CString::new(filename)
		{
			Ok(result) => result,
			Err(_) => return Err(Pod5Error::from_error_code(1, "memory error".to_string())),
		};

		let ptr = unsafe { crate::ffi::pod5_open_file(c_string.as_ptr()) };

		let mut reader = Reader {
			inner: ptr,
			has_compression: false,
		};
		reader.detect_signal_compression(filename)?;

		crate::pod5_ok!(reader)
	}

	pub fn count(&self) -> Result<usize, Pod5Error>
	{
		let mut read_count: usize = 0;
		unsafe {
			crate::ffi::pod5_get_read_count(self.inner, &mut read_count);
		}

		crate::pod5_ok!(read_count)
	}

	pub fn read_ids(&self) -> Result<Vec<uuid::Uuid>, Pod5Error>
	{
		let read_count = self.count()?;
		let mut read_ids = vec![[0; 16]; read_count];
		unsafe {
			crate::ffi::pod5_get_read_ids(self.inner, read_count, read_ids.as_mut_ptr());
		}

		let read_ids = read_ids
			.iter()
			.map(|&read_id| uuid::Uuid::from_bytes(read_id))
			.collect::<Vec<uuid::Uuid>>();

		crate::pod5_ok!(read_ids)
	}

	pub fn info(&self) -> Result<crate::fileinfo::FileInfo, Pod5Error>
	{
		let mut file_ptr: crate::ffi::FileInfo = Default::default();

		unsafe {
			crate::ffi::pod5_get_file_info(self.inner, &mut file_ptr);
		}

		crate::pod5_ok!(crate::fileinfo::FileInfo { inner: file_ptr })
	}

	pub fn run_info_iter(&self) -> crate::error::Result<crate::runinfo::RunInfoIter>
	{
		let mut run_info_count = 0;
		unsafe {
			crate::ffi::pod5_get_file_run_info_count(self.inner, &mut run_info_count);
		}

		crate::pod5_ok!(crate::runinfo::RunInfoIter {
			rows: run_info_count,
			reader: self,
			current_row: 0
		})
	}

	pub fn reads(&self) -> crate::error::Result<Reads>
	{
		let mut batch_count: usize = 0;
		unsafe {
			crate::ffi::pod5_get_read_batch_count(&mut batch_count, self.inner);
		}

		crate::pod5_ok!(Reads {
			reader: self,
			batch_count,
			batch_rows: 0,
			current_batch: 0,
			current_row: 0,
			inner: ptr::null_mut(),
		})
	}

	fn detect_signal_compression(&mut self, path: &str) -> crate::error::Result<()>
	{
		let mut file_data: crate::ffi::EmbeddedFileData_t = Default::default();

		unsafe {
			crate::ffi::pod5_get_file_signal_table_location(self.inner, &mut file_data);
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
				return Err(crate::error::Pod5Error::ArrowCompressionError(
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
				return Err(crate::error::Pod5Error::ArrowCompressionError(
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

	//fn read(&self, read: &mut Read) -> bool
	//{
	//}
}

impl Drop for Reader
{
	fn drop(&mut self)
	{
		unsafe {
			crate::ffi::pod5_close_and_free_reader(self.inner);
			crate::ffi::pod5_terminate();
		}
	}
}
