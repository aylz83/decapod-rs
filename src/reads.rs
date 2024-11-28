use std::{ffi::c_void, ptr};

pub struct Reads<'a>
{
	pub(crate) reader: std::slice::Iter<'a, crate::reader::InternalReader>,
	pub(crate) batch_count: usize,
	pub(crate) batch_rows: usize,

	pub(crate) current_batch: usize,
	pub(crate) current_row: usize,

	pub(crate) inner_reader: Option<&'a crate::reader::InternalReader>,
	pub(crate) inner: *mut crate::pod5_ffi::Pod5ReadRecordBatch_t,
}

impl<'a> Iterator for Reads<'a>
{
	type Item = crate::error::Result<crate::read::Read>;

	fn next(&mut self) -> Option<Self::Item>
	{
		// Clean up the previous batch if we've finished processing all rows
		if self.current_row == self.batch_rows && !self.inner.is_null()
		{
			unsafe {
				crate::pod5_ffi::pod5_free_read_batch(self.inner);
				self.inner = ptr::null_mut();
			}

			self.batch_count = 0;
		}

		// Process the next batch if we're on the first row of a batch
		if self.batch_count == 0
		{
			// Try to fetch a new "reader" from the iterator
			self.inner_reader = match self.reader.next()
			{
				Some(reader) => Some(reader),
				None => return None, // No more data available
			};

			// Interact with the FFI to set the batch count
			unsafe {
				crate::pod5_ffi::pod5_get_read_batch_count(
					&mut self.batch_count,
					self.inner_reader.as_ref().unwrap().inner,
				);
			}
		}

		// Start a new batch if we don't have an active one
		if self.inner.is_null()
		{
			unsafe {
				crate::pod5_ffi::pod5_get_read_batch(
					&mut self.inner,
					self.inner_reader.as_ref().unwrap().inner,
					self.current_batch,
				);
			}

			crate::pod5_check_error!();

			self.current_batch += 1;

			// Get the number of rows in the current batch
			let mut batch_rows: usize = 0;
			unsafe {
				crate::pod5_ffi::pod5_get_read_batch_row_count(&mut batch_rows, self.inner);
			}

			crate::pod5_check_error!();
			self.batch_rows = batch_rows;
		}

		// Fetch the next row of data from the current batch
		let mut read_ptr: crate::pod5_ffi::ReadBatchRowInfo_t = Default::default();
		let mut table_ver: u16 = 0;
		unsafe {
			crate::pod5_ffi::pod5_get_read_batch_row_info_data(
				self.inner,
				self.current_row,
				crate::pod5_ffi::READ_BATCH_ROW_INFO_VERSION as u16,
				&mut read_ptr as *mut crate::pod5_ffi::ReadBatchRowInfo_t as *mut c_void,
				&mut table_ver,
			);
		}

		// Prepare the resulting `Read` struct to be returned
		let read_result = crate::read::Read {
			inner: read_ptr,
			table_ver,
			batch_row: self.current_row,
			reader: self.inner_reader.as_ref().unwrap().inner,
			batch_record: self.inner,
			has_compression: self.inner_reader.as_ref().unwrap().has_compression,
		};

		// Move to the next row in the batch
		self.current_row += 1;

		// Return the processed `Read` object wrapped in a `Result::Ok`
		crate::pod5_ok!(Some, read_result)
	}
}
