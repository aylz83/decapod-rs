#![allow(dead_code)]

use crate::error::Pod5Error;

pub struct Read
{
	pub(crate) inner: crate::ffi::ReadBatchRowInfo_t,
	pub(crate) table_ver: u16,

	pub(crate) batch_row: usize,
	pub(crate) reader: *mut crate::ffi::Pod5FileReader_t,
	pub(crate) batch_record: *mut crate::ffi::Pod5ReadRecordBatch_t,
}

impl Read
{
	pub fn signal(&self) -> Result<Vec<i16>, Pod5Error>
	{
		let mut signal_count: usize = 0;
		let mut signal: Vec<i16>;
		unsafe {
			crate::ffi::pod5_get_read_complete_sample_count(
				self.reader,
				self.batch_record,
				self.batch_row,
				&mut signal_count,
			);

			signal = vec![0; signal_count];

			crate::ffi::pod5_get_read_complete_signal(
				self.reader,
				self.batch_record,
				self.batch_row,
				signal_count,
				signal.as_mut_ptr(),
			);
		}

		crate::pod5_ok!(signal)
	}

	pub fn read_id(&self) -> [u8; 16]
	{
		self.inner.read_id
	}

	pub fn read_number(&self) -> u32
	{
		self.inner.read_number
	}

	pub fn start_sample(&self) -> u64
	{
		self.inner.start_sample
	}

	pub fn median_before(&self) -> f32
	{
		self.inner.median_before
	}

	pub fn channel(&self) -> u16
	{
		self.inner.channel
	}

	pub fn well(&self) -> u8
	{
		self.inner.well
	}

	pub fn pore_type(&self) -> i16
	{
		self.inner.pore_type
	}

	pub fn calibration_offset(&self) -> f32
	{
		self.inner.calibration_offset
	}

	pub fn calibration_scale(&self) -> f32
	{
		self.inner.calibration_scale
	}

	pub fn end_reason(&self) -> i16
	{
		self.inner.end_reason
	}

	pub fn end_reason_forced(&self) -> u8
	{
		self.inner.end_reason_forced
	}

	pub fn run_info(&self) -> i16
	{
		self.inner.run_info
	}

	pub fn num_minknow_events(&self) -> u64
	{
		self.inner.num_minknow_events
	}

	pub fn tracked_scaling_scale(&self) -> f32
	{
		self.inner.tracked_scaling_scale
	}

	pub fn tracked_scaling_shift(&self) -> f32
	{
		self.inner.tracked_scaling_shift
	}

	pub fn predicted_scaling_scale(&self) -> f32
	{
		self.inner.predicted_scaling_scale
	}

	pub fn predicted_scaling_shift(&self) -> f32
	{
		self.inner.predicted_scaling_shift
	}

	pub fn num_reads_since_mux_change(&self) -> u32
	{
		self.inner.num_reads_since_mux_change
	}

	pub fn time_since_mux_change(&self) -> f32
	{
		self.inner.time_since_mux_change
	}

	pub fn signal_row_count(&self) -> i64
	{
		self.inner.signal_row_count
	}

	pub fn num_samples(&self) -> u64
	{
		self.inner.num_samples
	}
}
