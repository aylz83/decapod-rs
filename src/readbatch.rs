use std::ptr;
use std::ffi::c_void;
use std::any::Any;

use indexmap::IndexMap;

#[cfg(feature = "polars")]
use polars::prelude::*;

pub struct BatchRecord
{
	pub(crate) inner: *mut crate::pod5_ffi::Pod5ReadRecordBatch_t,
	pub(crate) reader: *mut crate::pod5_ffi::Pod5FileReader_t,
	pub(crate) fetch_path: Option<Vec<u32>>,
}

impl BatchRecord
{
	#[cfg(feature = "polars")]
	pub fn to_df(&self, fields: &Option<Vec<&str>>) -> crate::error::Result<DataFrame>
	{
		let which_fields = match fields
		{
			Some(fields) => fields,
			None => &vec![
				"read_id",
				"read_number",
				"start_sample",
				"median_before",
				"channel",
				"well",
				"pore_type",
				"calibration_offset",
				"calibration_scale",
				"end_reason",
				"end_reason_forced",
				"run_info",
				"num_minknow_events",
				"tracked_scaling_scale",
				"tracked_scaling_shift",
				"predicted_scaling_scale",
				"predicted_scaling_shift",
				"num_reads_since_mux_change",
				"time_since_mux_change",
				"signal_row_count",
				"num_samples",
			],
		};

		let mut fields_set: IndexMap<&str, Vec<Box<dyn Any>>> =
			IndexMap::with_capacity(which_fields.len());

		let mut batch_rows: usize = 0;
		let path_to_take = match &self.fetch_path
		{
			Some(path) =>
			{
				batch_rows = path.len();
				path
			}
			None =>
			{
				unsafe {
					crate::pod5_ffi::pod5_get_read_batch_row_count(&mut batch_rows, self.inner);
				}

				&(0..batch_rows as u32).collect::<Vec<u32>>()
			}
		};

		for field in which_fields
		{
			fields_set.insert(*field, Vec::with_capacity(batch_rows) as Vec<Box<dyn Any>>);
		}

		for current_row in path_to_take
		{
			let current_row = *current_row as usize;

			let mut read_ptr: crate::pod5_ffi::ReadBatchRowInfo_t = Default::default();
			let mut table_ver: u16 = 0;
			unsafe {
				crate::pod5_ffi::pod5_get_read_batch_row_info_data(
					self.inner,
					current_row as usize,
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

			let calibration = read_result.calibration();

			for field in which_fields
			{
				match *field
				{
					"read_id" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(read_result.uuid().to_string()) as Box<dyn Any>),
					"signal" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(read_result.signal().unwrap_or(Vec::new())) as Box<dyn Any>),
					"read_number" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(read_result.read_number()) as Box<dyn Any>),
					"start_sample" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(read_result.start_sample()) as Box<dyn Any>),
					"median_before" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(read_result.median_before()) as Box<dyn Any>),
					"channel" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(read_result.channel() as u32) as Box<dyn Any>),
					"well" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(read_result.well() as u32) as Box<dyn Any>),
					"pore_type" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(read_result.pore_type() as i32) as Box<dyn Any>),
					"calibration_offset" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(calibration.offset()) as Box<dyn Any>),
					"calibration_scale" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(calibration.scale()) as Box<dyn Any>),
					"end_reason" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(read_result.end_reason().to_string()) as Box<dyn Any>),
					"end_reason_forced" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(read_result.end_reason_forced() as bool) as Box<dyn Any>),
					"run_info" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(read_result.run_info_num() as i32) as Box<dyn Any>),
					"num_minknow_events" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(read_result.num_minknow_events()) as Box<dyn Any>),
					"tracked_scaling_scale" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(read_result.tracked_scaling_scale()) as Box<dyn Any>),
					"tracked_scaling_shift" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(read_result.tracked_scaling_shift()) as Box<dyn Any>),
					"predicted_scaling_scale" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(read_result.predicted_scaling_scale()) as Box<dyn Any>),
					"predicted_scaling_shift" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(read_result.predicted_scaling_shift()) as Box<dyn Any>),
					"num_reads_since_mux_change" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(read_result.num_reads_since_mux_change()) as Box<dyn Any>),
					"time_since_mux_change" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(read_result.time_since_mux_change()) as Box<dyn Any>),
					"signal_row_count" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(read_result.signal_row_count()) as Box<dyn Any>),
					"num_samples" => fields_set
						.get_mut(field)
						.unwrap()
						.push(Box::new(read_result.num_samples()) as Box<dyn Any>),
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
			else if col_name == "signal"
			{
				/*let values: Vec<Vec<i32>> = data
					.into_iter()
					.map(|v| *v.downcast::<Vec<i32>>().unwrap())
					.collect();
				let slices: Vec<&[i32]> = values.iter().map(|v| v.as_slice()).collect();

				// Create ListChunked directly
				let list_chunked = ListChunked::new("", &slices);
				let list_chunked = ListChunked::from_vec(
					col_name.into(),
					slices, // Convert Vec<Vec<i32>> to ListChunked
				);
				series.push(Series::new(col_name.into(), list_chunked));*/
			}
			else if col_name == "read_number"
			{
				let values: Vec<u32> = data
					.into_iter()
					.map(|v| *v.downcast::<u32>().unwrap())
					.collect();
				series.push(Series::new(col_name.into(), values));
			}
			else if col_name == "start_sample"
			{
				let values: Vec<u64> = data
					.into_iter()
					.map(|v| *v.downcast::<u64>().unwrap())
					.collect();
				series.push(Series::new(col_name.into(), values));
			}
			else if col_name == "median_before"
			{
				let values: Vec<f32> = data
					.into_iter()
					.map(|v| *v.downcast::<f32>().unwrap())
					.collect();
				series.push(Series::new(col_name.into(), values));
			}
			else if col_name == "channel"
			{
				let values: Vec<u32> = data
					.into_iter()
					.map(|v| *v.downcast::<u32>().unwrap())
					.collect();
				series.push(Series::new(col_name.into(), values));
			}
			else if col_name == "well"
			{
				let values: Vec<u32> = data
					.into_iter()
					.map(|v| *v.downcast::<u32>().unwrap())
					.collect();
				series.push(Series::new(col_name.into(), values));
			}
			else if col_name == "pore_type"
			{
				let values: Vec<i32> = data
					.into_iter()
					.map(|v| *v.downcast::<i32>().unwrap())
					.collect();
				series.push(Series::new(col_name.into(), values));
			}
			else if col_name == "calibration_offset"
			{
				let values: Vec<f32> = data
					.into_iter()
					.map(|v| *v.downcast::<f32>().unwrap())
					.collect();
				series.push(Series::new(col_name.into(), values));
			}
			else if col_name == "calibration_scale"
			{
				let values: Vec<f32> = data
					.into_iter()
					.map(|v| *v.downcast::<f32>().unwrap())
					.collect();
				series.push(Series::new(col_name.into(), values));
			}
			else if col_name == "end_reason"
			{
				let values: Vec<String> = data
					.into_iter()
					.map(|v| *v.downcast::<String>().unwrap())
					.collect();
				series.push(Series::new(col_name.into(), values));
			}
			else if col_name == "end_reason_forced"
			{
				let values: Vec<bool> = data
					.into_iter()
					.map(|v| *v.downcast::<bool>().unwrap())
					.collect();
				series.push(Series::new(col_name.into(), values));
			}
			else if col_name == "run_info"
			{
				let values: Vec<i32> = data
					.into_iter()
					.map(|v| *v.downcast::<i32>().unwrap())
					.collect();
				series.push(Series::new(col_name.into(), values));
			}
			else if col_name == "num_minknow_events"
			{
				let values: Vec<u64> = data
					.into_iter()
					.map(|v| *v.downcast::<u64>().unwrap())
					.collect();
				series.push(Series::new(col_name.into(), values));
			}
			else if col_name == "tracked_scaling_scale"
			{
				let values: Vec<f32> = data
					.into_iter()
					.map(|v| *v.downcast::<f32>().unwrap())
					.collect();
				series.push(Series::new(col_name.into(), values));
			}
			else if col_name == "tracked_scaling_shift"
			{
				let values: Vec<f32> = data
					.into_iter()
					.map(|v| *v.downcast::<f32>().unwrap())
					.collect();
				series.push(Series::new(col_name.into(), values));
			}
			else if col_name == "predicted_scaling_scale"
			{
				let values: Vec<f32> = data
					.into_iter()
					.map(|v| *v.downcast::<f32>().unwrap())
					.collect();
				series.push(Series::new(col_name.into(), values));
			}
			else if col_name == "predicted_scaling_shift"
			{
				let values: Vec<f32> = data
					.into_iter()
					.map(|v| *v.downcast::<f32>().unwrap())
					.collect();
				series.push(Series::new(col_name.into(), values));
			}
			else if col_name == "num_reads_since_mux_change"
			{
				let values: Vec<u32> = data
					.into_iter()
					.map(|v| *v.downcast::<u32>().unwrap())
					.collect();
				series.push(Series::new(col_name.into(), values));
			}
			else if col_name == "time_since_mux_change"
			{
				let values: Vec<f32> = data
					.into_iter()
					.map(|v| *v.downcast::<f32>().unwrap())
					.collect();
				series.push(Series::new(col_name.into(), values));
			}
			else if col_name == "signal_row_count"
			{
				let values: Vec<i64> = data
					.into_iter()
					.map(|v| *v.downcast::<i64>().unwrap())
					.collect();
				series.push(Series::new(col_name.into(), values));
			}
			else if col_name == "num_samples"
			{
				let values: Vec<u64> = data
					.into_iter()
					.map(|v| *v.downcast::<u64>().unwrap())
					.collect();
				series.push(Series::new(col_name.into(), values));
			}
		}

		let record_df = DataFrame::new(series);

		let record_df = match record_df
		{
			Ok(df) => df,
			Err(_) => return Err(crate::error::Pod5Error::UnknownError("".to_string())),
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
	pub(crate) reader: std::slice::Iter<'a, crate::reader::InternalReader>,

	pub(crate) current_row: usize,
	pub(crate) inner_reader: Option<&'a crate::reader::InternalReader>,
	pub(crate) fetch: Option<Vec<uuid::Uuid>>,
	pub(crate) fetch_path: Option<Vec<(usize, Vec<u32>)>>,
}

impl<'a> BatchRecordIter<'a>
{
	fn reset_rows(&mut self)
	{
		if self.rows == self.current_row
		{
			self.current_row = 0;
			self.rows = 0;
		}
	}

	fn next_batch(&mut self) -> Option<crate::error::Result<BatchRecord>>
	{
		self.reset_rows();

		if self.rows == 0
		{
			self.inner_reader = match self.reader.next()
			{
				Some(reader) => Some(reader),
				None => return None,
			};

			unsafe {
				crate::pod5_ffi::pod5_get_read_batch_count(
					&mut self.rows,
					self.inner_reader.unwrap().inner,
				);
			}
		}

		let mut batch_ptr = ptr::null_mut();
		unsafe {
			crate::pod5_ffi::pod5_get_read_batch(
				&mut batch_ptr,
				self.inner_reader.unwrap().inner,
				self.current_row,
			);
		}

		crate::pod5_check_error!();

		let read_result = BatchRecord {
			inner: batch_ptr,
			reader: self.inner_reader.unwrap().inner,
			fetch_path: None,
		};

		self.current_row += 1;

		crate::pod5_ok!(Some, read_result)
	}

	fn next_fetch_batch(&mut self) -> Option<crate::error::Result<BatchRecord>>
	{
		self.reset_rows();

		if self.rows == 0
		{
			self.inner_reader = match self.reader.next()
			{
				Some(reader) => Some(reader),
				None => return None,
			};

			self.fetch_path = self
				.inner_reader
				.as_ref()
				.unwrap()
				.get_fetch_path(&self.fetch, &mut self.rows);
		}

		let Some(fetch_path) = &self.fetch_path
		else
		{
			return None;
		};

		let mut batch_ptr = ptr::null_mut();
		unsafe {
			crate::pod5_ffi::pod5_get_read_batch(
				&mut batch_ptr,
				self.inner_reader.unwrap().inner,
				fetch_path[self.current_row].0,
			);
		}

		crate::pod5_check_error!();

		let read_result = BatchRecord {
			inner: batch_ptr,
			reader: self.inner_reader.unwrap().inner,
			fetch_path: Some(fetch_path[self.current_row].1.clone()),
		};

		self.current_row += 1;

		crate::pod5_ok!(Some, read_result)
	}
}

impl<'a> Iterator for BatchRecordIter<'a>
{
	type Item = crate::error::Result<BatchRecord>;

	fn next(&mut self) -> Option<Self::Item>
	{
		match self.fetch
		{
			Some(_) => self.next_fetch_batch(),
			None => self.next_batch(),
		}
	}
}
