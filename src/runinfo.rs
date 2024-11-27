use std::ffi::CStr;
use std::fmt;
use std::ptr;

pub struct KeyValueData
{
	inner: crate::pod5_ffi::KeyValueData,
}

impl KeyValueData
{
	pub fn keys(&self) -> &[i8]
	{
		// Create a slice from the pointer and size
		unsafe { std::slice::from_raw_parts(*self.inner.keys, self.inner.size) }
	}

	pub fn values(&self) -> &[i8]
	{
		// Create a slice from the pointer and size
		unsafe { std::slice::from_raw_parts(*self.inner.values, self.inner.size) }
	}
}

pub struct RunInfo
{
	pub(crate) inner: *mut crate::pod5_ffi::RunInfoDictData_t,
}

impl RunInfo
{
	pub fn acquisition_id(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).acquisition_id);
			c_str.to_str()
		}
	}

	pub fn acquisition_start_time_ms(&self) -> i64
	{
		unsafe { (*self.inner).acquisition_start_time_ms }
	}

	pub fn adc_max(&self) -> i16
	{
		unsafe { (*self.inner).adc_max }
	}

	pub fn adc_min(&self) -> i16
	{
		unsafe { (*self.inner).adc_min }
	}

	pub fn context_tags(&self) -> KeyValueData
	{
		KeyValueData {
			inner: unsafe { (*self.inner).context_tags },
		}
	}

	pub fn experiment_name(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).experiment_name);
			c_str.to_str()
		}
	}

	pub fn flow_cell_id(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).flow_cell_id);
			c_str.to_str()
		}
	}

	pub fn flow_cell_product_code(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).flow_cell_product_code);
			c_str.to_str()
		}
	}

	pub fn protocol_name(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).protocol_name);
			c_str.to_str()
		}
	}

	pub fn protocol_run_id(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).protocol_run_id);
			c_str.to_str()
		}
	}

	pub fn protocol_start_time_ms(&self) -> i64
	{
		unsafe { (*self.inner).protocol_start_time_ms }
	}

	pub fn sample_id(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).sample_id);
			c_str.to_str()
		}
	}

	pub fn sample_rate(&self) -> u16
	{
		unsafe { (*self.inner).sample_rate }
	}

	pub fn sequencing_kit(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).sequencing_kit);
			c_str.to_str()
		}
	}

	pub fn sequencer_position(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).sequencer_position);
			c_str.to_str()
		}
	}

	pub fn sequencer_position_type(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).sequencer_position_type);
			c_str.to_str()
		}
	}

	pub fn software(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).software);
			c_str.to_str()
		}
	}

	pub fn system_name(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).system_name);
			c_str.to_str()
		}
	}

	pub fn system_type(&self) -> Result<&str, std::str::Utf8Error>
	{
		unsafe {
			let c_str = CStr::from_ptr((*self.inner).system_type);
			c_str.to_str()
		}
	}

	pub fn tracking_id(&self) -> KeyValueData
	{
		KeyValueData {
			inner: unsafe { (*self.inner).tracking_id },
		}
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
		writeln!(f, "adc_min {}", self.adc_min())?;
		//write!(f, "{}\n", self.context_tags())?;
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
		)
		//write!(f, "{}\n", self.tracking_id())
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

pub struct RunInfoIter<'a>
{
	pub(crate) rows: u16,
	pub(crate) reader: &'a crate::reader::Reader,

	pub(crate) current_row: u16,
}

impl<'a> Iterator for RunInfoIter<'a>
{
	type Item = crate::error::Result<RunInfo>;

	fn next(&mut self) -> Option<Self::Item>
	{
		if self.current_row == self.rows
		{
			self.current_row = 0;
			return None;
		}

		let mut run_info = ptr::null_mut();

		unsafe {
			crate::pod5_ffi::pod5_get_file_run_info(
				self.reader.inner,
				self.current_row,
				&mut run_info,
			);
		}

		crate::pod5_check_error!();
		self.current_row += 1;

		crate::pod5_ok!(Some, RunInfo { inner: run_info })
	}
}
