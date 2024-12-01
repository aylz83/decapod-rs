use std::ffi::CStr;
use std::fmt;
use std::ptr;
use std::collections::HashMap;

/// Run information metadata from the pod5 such as experiment name or flowcell ID.
/// Obtained either by the [`crate::reader::Reader::run_info_iter`] function for all run info objects,
/// or by calling [`crate::reader::Read::run_info`] directly on a read.
pub struct RunInfo
{
	pub(crate) inner: *mut crate::pod5_ffi::RunInfoDictData_t,
}

impl RunInfo
{
	/// Acquisition id.
	pub fn acquisition_id(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).acquisition_id);
			c_str.to_str()
		}
	}

	/// Acquisition start time in miliseconds.
	pub fn acquisition_start_time_ms(&self) -> i64
	{
		unsafe { (*self.inner).acquisition_start_time_ms }
	}

	/// Max adc.
	pub fn adc_max(&self) -> i16
	{
		unsafe { (*self.inner).adc_max }
	}

	/// Min adc.
	pub fn adc_min(&self) -> i16
	{
		unsafe { (*self.inner).adc_min }
	}

	/// Context data.
	pub fn context_tags(&self) -> HashMap<String, String>
	{
		let keys = unsafe {
			std::slice::from_raw_parts(
				(*self.inner).context_tags.keys,
				(*self.inner).context_tags.size,
			)
		};

		let values = unsafe {
			std::slice::from_raw_parts(
				(*self.inner).context_tags.values,
				(*self.inner).context_tags.size,
			)
		};

		keys.iter()
			.map(|value| unsafe { CStr::from_ptr(*value).to_string_lossy().into() })
			.zip(
				values
					.iter()
					.map(|value| unsafe { CStr::from_ptr(*value).to_string_lossy().into() }),
			)
			.collect()
	}

	/// Get the experiment name if set.
	pub fn experiment_name(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).experiment_name);
			c_str.to_str()
		}
	}

	/// Get the flowcell ID.
	pub fn flow_cell_id(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).flow_cell_id);
			c_str.to_str()
		}
	}

	/// Flowcell product code.
	pub fn flow_cell_product_code(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).flow_cell_product_code);
			c_str.to_str()
		}
	}

	/// Protocol name if set.
	pub fn protocol_name(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).protocol_name);
			c_str.to_str()
		}
	}

	/// Protocol run id if set.
	pub fn protocol_run_id(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).protocol_run_id);
			c_str.to_str()
		}
	}

	/// Start time.
	pub fn protocol_start_time_ms(&self) -> i64
	{
		unsafe { (*self.inner).protocol_start_time_ms }
	}

	/// Sample id if set.
	pub fn sample_id(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).sample_id);
			c_str.to_str()
		}
	}

	/// The sample rate of the flowcell.
	pub fn sample_rate(&self) -> u16
	{
		unsafe { (*self.inner).sample_rate }
	}

	/// The sequencing kit used.
	pub fn sequencing_kit(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).sequencing_kit);
			c_str.to_str()
		}
	}

	/// Sequencer position.
	pub fn sequencer_position(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).sequencer_position);
			c_str.to_str()
		}
	}

	/// Sequencer position type.
	pub fn sequencer_position_type(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).sequencer_position_type);
			c_str.to_str()
		}
	}

	/// Software used for sequencing.
	pub fn software(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).software);
			c_str.to_str()
		}
	}

	/// System name from the seequencing software if set.
	pub fn system_name(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).system_name);
			c_str.to_str()
		}
	}

	/// System type.
	pub fn system_type(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).system_type);
			c_str.to_str()
		}
	}

	/// Tracking id data.
	pub fn tracking_id(&self) -> HashMap<String, String>
	{
		let keys = unsafe {
			std::slice::from_raw_parts(
				(*self.inner).tracking_id.keys,
				(*self.inner).tracking_id.size,
			)
		};

		let values = unsafe {
			std::slice::from_raw_parts(
				(*self.inner).tracking_id.values,
				(*self.inner).tracking_id.size,
			)
		};

		keys.iter()
			.map(|value| unsafe { CStr::from_ptr(*value).to_string_lossy().into() })
			.zip(
				values
					.iter()
					.map(|value| unsafe { CStr::from_ptr(*value).to_string_lossy().into() }),
			)
			.collect()
	}

	fn handle_result<T: fmt::Display, E: fmt::Debug>(result: Result<T, E>) -> String
	{
		result
			.map(|v| v.to_string())
			.unwrap_or_else(|_| "<invalid UTF-8>".to_string())
	}
}

impl fmt::Display for RunInfo
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		writeln!(
			f,
			"acquisition_id = {}",
			RunInfo::handle_result(self.acquisition_id())
		)?;
		writeln!(
			f,
			"acquisition_start_time_ms = {}",
			self.acquisition_start_time_ms()
		)?;
		writeln!(f, "adc_max = {}", self.adc_max())?;
		writeln!(f, "adc_min = {}", self.adc_min())?;
		writeln!(f, "context_tags = {:?}", self.context_tags())?;
		writeln!(
			f,
			"experiment_name = {}",
			RunInfo::handle_result(self.experiment_name())
		)?;
		writeln!(
			f,
			"flow_cell_id = {}",
			RunInfo::handle_result(self.flow_cell_id())
		)?;
		writeln!(
			f,
			"flow_cell_product_code = {}",
			RunInfo::handle_result(self.flow_cell_product_code())
		)?;
		writeln!(
			f,
			"product_name = {}",
			RunInfo::handle_result(self.protocol_name())
		)?;
		writeln!(
			f,
			"protocol_run_id = {}",
			RunInfo::handle_result(self.protocol_run_id())
		)?;
		writeln!(
			f,
			"protocol_start_time_ms = {}",
			self.protocol_start_time_ms()
		)?;
		writeln!(
			f,
			"sample_id = {}",
			RunInfo::handle_result(self.sample_id())
		)?;
		writeln!(f, "sample_rate = {}", self.sample_rate())?;
		writeln!(
			f,
			"sequencing_kit = {}",
			RunInfo::handle_result(self.sequencing_kit())
		)?;
		writeln!(
			f,
			"sequencer_position = {}",
			RunInfo::handle_result(self.sequencer_position())
		)?;
		writeln!(
			f,
			"sequencer_position_type = {}",
			RunInfo::handle_result(self.sequencer_position_type())
		)?;
		writeln!(f, "software = {}", RunInfo::handle_result(self.software()))?;
		writeln!(
			f,
			"system_name = {}",
			RunInfo::handle_result(self.system_name())
		)?;
		writeln!(
			f,
			"system_type = {}",
			RunInfo::handle_result(self.system_type())
		)?;
		writeln!(f, "tracking_id = {:?}", self.tracking_id())
	}
}

impl Drop for RunInfo
{
	fn drop(&mut self)
	{
		unsafe {
			crate::pod5_ffi::pod5_free_run_info(self.inner);
		}
	}
}

/// Iterator for run info metadata.
/// See [`crate::reader::Reader::run_info_iter`] for full usage.
pub struct RunInfoIter<'a>
{
	pub(crate) rows: u16,
	pub(crate) reader: std::slice::Iter<'a, crate::reader::InternalReader>,

	pub(crate) current_row: u16,
	pub(crate) current_reader: Option<&'a crate::reader::InternalReader>,
}

impl<'a> Iterator for RunInfoIter<'a>
{
	type Item = crate::error::Result<RunInfo>;

	fn next(&mut self) -> Option<Self::Item>
	{
		if self.current_row == self.rows
		{
			self.current_row = 0;
		}

		if self.current_row == 0
		{
			self.current_reader = match self.reader.next()
			{
				Some(reader) => Some(reader),
				None => return None,
			};

			unsafe {
				crate::pod5_ffi::pod5_get_file_run_info_count(
					self.current_reader.unwrap().inner,
					&mut self.rows,
				);
			}
		}

		let mut run_info = ptr::null_mut();

		unsafe {
			crate::pod5_ffi::pod5_get_file_run_info(
				self.current_reader.unwrap().inner,
				self.current_row,
				&mut run_info,
			);
		}

		self.current_row += 1;

		crate::pod5_ok!(Some, RunInfo { inner: run_info })
	}
}
