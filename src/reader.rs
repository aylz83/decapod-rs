use std::{ffi::CString, ptr};

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
	pub(crate) ptr: *mut crate::ffi::Pod5FileReader_t,
}

impl Reader
{
	pub fn from_file_with_options<'a, 'b>(
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

		crate::pod5_ok!(Reader { ptr })
	}

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

		crate::pod5_ok!(Reader { ptr })
	}

	pub fn count(&self) -> Result<usize, Pod5Error>
	{
		let mut read_count: usize = 0;
		unsafe {
			crate::ffi::pod5_get_read_count(self.ptr, &mut read_count);
		}

		crate::pod5_ok!(read_count)
	}

	pub fn read_ids(&self) -> Result<Vec<uuid::Uuid>, Pod5Error>
	{
		let read_count = self.count()?;
		let mut read_ids = vec![[0; 16]; read_count];
		unsafe {
			crate::ffi::pod5_get_read_ids(self.ptr, read_count, read_ids.as_mut_ptr());
		}

		let read_ids = read_ids
			.iter()
			.map(|&read_id| uuid::Uuid::from_bytes(read_id))
			.collect::<Vec<uuid::Uuid>>();

		crate::pod5_ok!(read_ids)
	}

	pub fn info(&self) -> Result<crate::fileinfo::FileInfo, Pod5Error>
	{
		let file_ptr = ptr::null_mut();
		unsafe {
			crate::ffi::pod5_get_file_info(self.ptr, file_ptr);
		}

		crate::pod5_ok!(crate::fileinfo::FileInfo { inner: file_ptr })
	}

	pub fn reads(&self) -> crate::error::Result<Reads>
	{
		let mut batch_count: usize = 0;
		unsafe {
			crate::ffi::pod5_get_read_batch_count(&mut batch_count, self.ptr);
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

	//fn read(&self, read: &mut Read) -> bool
	//{
	//}
}

impl Drop for Reader
{
	fn drop(&mut self)
	{
		unsafe {
			crate::ffi::pod5_close_and_free_reader(self.ptr);
			crate::ffi::pod5_terminate();
		}
	}
}
