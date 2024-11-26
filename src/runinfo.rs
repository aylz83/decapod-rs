use std::ffi::CStr;

pub struct KeyValueData
{
	inner: crate::ffi::KeyValueData,
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
	pub(crate) inner: *mut crate::ffi::RunInfoDictData_t,
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
}

impl Drop for RunInfo
{
	fn drop(&mut self)
	{
		unsafe {
			crate::ffi::pod5_free_run_info(self.inner);
		}
	}
}
