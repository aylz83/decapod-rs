#![allow(dead_code)]

use std::ffi::CStr;
use std::ptr;

pub use uuid;

#[cfg(feature = "serde")]
use serde::ser::{Serialize, SerializeStruct, Serializer};

/// Calibration and calibration extra data from the associated read.
pub struct Calibration<'a>
{
	pub(crate) inner: &'a Read,

	pub(crate) digitisation: Option<u16>,
	pub(crate) range: Option<f32>,
}

impl<'a> Calibration<'a>
{
	/// Calibration offset
	pub fn offset(&self) -> f32
	{
		self.inner.inner.calibration_offset
	}

	/// Calibration scale
	pub fn scale(&self) -> f32
	{
		self.inner.inner.calibration_scale
	}

	/// Obtain the digitisation field from the extra calibration data within the pod5.
	pub fn digitisation(&mut self) -> u16
	{
		match self.digitisation
		{
			Some(value) => value,
			None =>
			{
				let mut calibration_data = crate::pod5_ffi::CalibrationExtraData_t {
					digitisation: 0,
					range: 0.0,
				};

				unsafe {
					crate::pod5_ffi::pod5_get_calibration_extra_info(
						self.inner.batch_record,
						self.inner.batch_row,
						&mut calibration_data,
					);
				}

				self.digitisation = Some(calibration_data.digitisation);
				self.range = Some(calibration_data.range);

				calibration_data.digitisation
			}
		}
	}

	/// obtain the range field from the extra calibration data within the pod5.
	pub fn range(&mut self) -> f32
	{
		match self.range
		{
			Some(value) => value,
			None =>
			{
				let mut calibration_data = crate::pod5_ffi::CalibrationExtraData_t {
					digitisation: 0,
					range: 0.0,
				};

				unsafe {
					crate::pod5_ffi::pod5_get_calibration_extra_info(
						self.inner.batch_record,
						self.inner.batch_row,
						&mut calibration_data,
					);
				}

				self.digitisation = Some(calibration_data.digitisation);
				self.range = Some(calibration_data.range);

				calibration_data.range
			}
		}
	}
}

/// pod5 read information.
pub struct Read
{
	pub(crate) inner: crate::pod5_ffi::ReadBatchRowInfo_t,
	pub(crate) table_ver: u16,

	pub(crate) batch_row: usize,
	pub(crate) reader: *mut crate::pod5_ffi::Pod5FileReader_t,
	pub(crate) batch_record: *mut crate::pod5_ffi::Pod5ReadRecordBatch_t,

	pub(crate) has_compression: bool,
}

impl Read
{
	/// The uncompressed signal for the associated read.
	pub fn signal(&self) -> crate::error::Result<Vec<i16>>
	{
		let mut signal_count: usize = 0;
		let mut signal: Vec<i16>;
		unsafe {
			crate::pod5_ffi::pod5_get_read_complete_sample_count(
				self.reader,
				self.batch_record,
				self.batch_row,
				&mut signal_count,
			);

			signal = vec![0; signal_count];

			crate::pod5_ffi::pod5_get_read_complete_signal(
				self.reader,
				self.batch_record,
				self.batch_row,
				signal_count,
				signal.as_mut_ptr(),
			);
		}

		crate::pod5_ok!(signal)
	}

	/// The read id as a uuid.
	pub fn uuid(&self) -> uuid::Uuid
	{
		uuid::Uuid::from_bytes(self.inner.read_id)
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

	/// Attempts to get the pore type as a String.
	pub fn pore_type_string(&self) -> crate::error::Result<String>
	{
		let mut c_string = vec![0i8; 10];
		let mut str_length: usize = 10;
		unsafe {
			crate::pod5_ffi::pod5_get_pore_type(
				self.batch_record,
				self.pore_type(),
				c_string.as_mut_ptr(),
				&mut str_length,
			);
		}

		crate::pod5_ok!(unsafe { CStr::from_ptr(c_string.as_ptr()) }
			.to_str()
			.map(|s| s.to_string())?)
	}

	pub fn calibration(&self) -> Calibration
	{
		Calibration {
			inner: self,
			digitisation: None,
			range: None,
		}
	}

	pub fn end_reason(&self) -> crate::endreason::EndReason
	{
		crate::endreason::EndReason::end_reason_from_code(self.inner.end_reason)
	}

	pub fn end_reason_forced(&self) -> bool
	{
		self.inner.end_reason_forced == 1
	}

	pub fn run_info_num(&self) -> i16
	{
		self.inner.run_info
	}

	/// The runinfo struct from the meta data for the associated read.
	pub fn run_info(&self) -> crate::error::Result<crate::runinfo::RunInfo>
	{
		let mut run_info = ptr::null_mut();

		unsafe {
			crate::pod5_ffi::pod5_get_file_run_info(
				self.reader,
				self.run_info_num() as u16,
				&mut run_info,
			);
		}

		crate::pod5_ok!(crate::runinfo::RunInfo { inner: run_info })
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

#[cfg(feature = "serde")]
impl Serialize for Read
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		// Start serializing the struct with the specified number of fields
		let mut state = serializer.serialize_struct("Read", 21)?;

		let calibration = self.calibration();

		// Serialize each field with its name
		state.serialize_field("uuid", &self.uuid())?;
		state.serialize_field("signal", &self.signal().unwrap_or(vec![0i16; 0]))?;
		state.serialize_field("read_number", &self.read_number())?;
		state.serialize_field("start_sample", &self.start_sample())?;
		state.serialize_field("median_before", &self.median_before())?;
		state.serialize_field("channel", &self.channel())?;
		state.serialize_field("well", &self.well())?;
		state.serialize_field("pore_type", &self.pore_type())?;
		state.serialize_field("calibration_offset", &calibration.offset())?;
		state.serialize_field("calibration_scale", &calibration.scale())?;
		state.serialize_field("end_reason", &self.end_reason())?;
		state.serialize_field("end_reason_forced", &self.end_reason_forced())?;
		state.serialize_field("run_info", &self.run_info_num())?;
		state.serialize_field("num_minknow_events", &self.num_minknow_events())?;
		state.serialize_field("tracked_scaling_scale", &self.tracked_scaling_scale())?;
		state.serialize_field("tracked_scaling_shift", &self.tracked_scaling_shift())?;
		state.serialize_field("predicted_scaling_scale", &self.predicted_scaling_scale())?;
		state.serialize_field("predicted_scaling_shift", &self.predicted_scaling_shift())?;
		state.serialize_field(
			"num_reads_since_mux_change",
			&self.num_reads_since_mux_change(),
		)?;
		state.serialize_field("time_since_mux_change", &self.time_since_mux_change())?;
		state.serialize_field("signal_row_count", &self.signal_row_count())?;
		state.serialize_field("num_samples", &self.num_samples())?;

		// End serialization
		state.end()
	}
}
