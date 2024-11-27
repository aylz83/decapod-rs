use std::ptr;
use std::ffi::c_void;
use std::any::Any;

use std::collections::HashMap;

#[cfg(feature = "polars")]
use polars::prelude::*;

enum ValueType
{
	String(String),
	U64(u64),
}

pub struct BatchRecord
{
	pub(crate) inner: *mut crate::pod5_ffi::Pod5ReadRecordBatch_t,
	pub(crate) reader: *mut crate::pod5_ffi::Pod5FileReader_t,
}

impl BatchRecord
{
	#[cfg(feature = "polars")]
	pub fn to_df(&self, fields: &Option<Vec<&str>>) -> crate::error::Result<DataFrame>
	{
		use crate::error::Pod5Error;

		let mut batch_rows: usize = 0;
		unsafe {
			crate::pod5_ffi::pod5_get_read_batch_row_count(&mut batch_rows, self.inner);
		}

		let which_fields = match fields
		{
			Some(fields) => fields,
			None => &vec!["read_id"],
		};

		let mut fields_set: HashMap<&str, Vec<Box<dyn Any>>> =
			HashMap::with_capacity(which_fields.len());
		for field in which_fields
		{
			fields_set.insert(*field, Vec::with_capacity(batch_rows) as Vec<Box<dyn Any>>);
		}

		for current_row in 0..batch_rows
		{
			let mut read_ptr: crate::pod5_ffi::ReadBatchRowInfo_t = Default::default();
			let mut table_ver: u16 = 0;
			unsafe {
				crate::pod5_ffi::pod5_get_read_batch_row_info_data(
					self.inner,
					current_row,
					crate::pod5_ffi::READ_BATCH_ROW_INFO_VERSION as u16,
					&mut read_ptr as *mut crate::pod5_ffi::ReadBatchRowInfo_t as *mut c_void,
					&mut table_ver,
				);
			}

			let read_result = crate::read::Read {
				inner: read_ptr,
				table_ver,
				batch_row: current_row,
				reader: self.reader,
				batch_record: self.inner,
				has_compression: true,
			};

			for field in which_fields
			{
				match *field
				{
					"read_id" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(read_result.uuid().to_string()) as Box<dyn Any>),
					"read_number" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(read_result.read_number()) as Box<dyn Any>),
					_ => (),
				};
			}
		}

		let mut series: Vec<Series> = Vec::new();
		for (col_name, data) in fields_set
		{
			if col_name == "read_id"
			{
				let values: Vec<String> = data
					.into_iter()
					.map(|v| *v.downcast::<String>().unwrap())
					.collect();
				series.push(Series::new(col_name.into(), values));
			}
			else if col_name == "read_number"
			{
				let values: Vec<u32> = data
					.into_iter()
					.map(|v| *v.downcast::<u32>().unwrap())
					.collect();
				series.push(Series::new(col_name.into(), values));
			}
		}

		let record_df = DataFrame::new(series);

		let record_df = match record_df
		{
			Ok(df) => df,
			Err(_) => return Err(Pod5Error::UnknownError("".to_string())),
		};

		crate::pod5_ok!(record_df)
	}
}

impl Drop for BatchRecord
{
	fn drop(&mut self)
	{
		unsafe {
			crate::pod5_ffi::pod5_free_read_batch(self.inner);
		}
	}
}

pub struct BatchRecordIter<'a>
{
	pub(crate) rows: usize,
	pub(crate) reader: &'a crate::reader::Reader,

	pub(crate) current_row: usize,
}

impl<'a> Iterator for BatchRecordIter<'a>
{
	type Item = crate::error::Result<BatchRecord>;

	fn next(&mut self) -> Option<Self::Item>
	{
		if self.rows == self.current_row
		{
			self.current_row = 0;
			return None;
		}

		let mut batch_ptr = ptr::null_mut();
		unsafe {
			crate::pod5_ffi::pod5_get_read_batch(
				&mut batch_ptr,
				self.reader.inner,
				self.current_row,
			);
		}

		crate::pod5_check_error!();

		let read_result = BatchRecord {
			inner: batch_ptr,
			reader: self.reader.inner,
		};

		self.current_row += 1;

		crate::pod5_ok!(Some, read_result)
	}
}
