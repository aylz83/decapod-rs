use std::{ffi::c_void, ptr};

pub struct Reads<'a>
{
	pub(crate) reader: &'a crate::reader::Reader,
	pub(crate) batch_count: usize,
	pub(crate) batch_rows: usize,

	pub(crate) current_batch: usize,
	pub(crate) current_row: usize,

	pub(crate) inner: *mut crate::ffi::Pod5ReadRecordBatch_t,
}

impl<'a> Iterator for Reads<'a>
{
	type Item = crate::error::Result<crate::read::Read>;

	fn next(&mut self) -> Option<Self::Item>
	{
		if self.current_row == self.batch_rows
		{
			unsafe {
				crate::ffi::pod5_free_read_batch(self.inner);
				self.inner = ptr::null_mut();
			}
		}

		if self.batch_count == self.current_batch && self.current_row == self.batch_rows
		{
			self.current_batch = 0;
			self.current_row = 0;
			return None;
		}

		if self.inner.is_null()
		{
			unsafe {
				crate::ffi::pod5_get_read_batch(
					&mut self.inner,
					self.reader.ptr,
					self.current_batch,
				);
			}

			crate::pod5_check_error!();

			self.current_batch += 1;

			let mut batch_rows: usize = 0;
			unsafe {
				crate::ffi::pod5_get_read_batch_row_count(&mut batch_rows, self.inner);
			}

			crate::pod5_check_error!();
			self.batch_rows = batch_rows;
		}

		let mut read_ptr: crate::ffi::ReadBatchRowInfo_t = Default::default();
		let mut table_ver: u16 = 0;
		unsafe {
			crate::ffi::pod5_get_read_batch_row_info_data(
				self.inner,
				self.current_row,
				crate::ffi::READ_BATCH_ROW_INFO_VERSION as u16,
				&mut read_ptr as *mut crate::ffi::ReadBatchRowInfo_t as *mut c_void,
				&mut table_ver,
			);
		}

		let read_result = crate::read::Read {
			inner: read_ptr,
			table_ver,
			batch_row: self.current_row,
			reader: self.reader.ptr,
			batch_record: self.inner,
		};

		self.current_row += 2;

		crate::pod5_ok!(Some, read_result)
	}
}