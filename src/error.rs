use thiserror::Error;

#[macro_export]
macro_rules! pod5_check_error {
	() => {{
		unsafe {
			let error_code = $crate::ffi::pod5_get_error_no();
			if error_code != $crate::ffi::pod5_error_POD5_OK
			{
				let c_str = std::ffi::CStr::from_ptr($crate::ffi::pod5_get_error_string());
				return Some(Err($crate::error::Pod5Error::from_error_code(
					error_code,
					c_str.to_str().expect("error").to_string(),
				)));
			}
		}
	}};
}

#[macro_export]
macro_rules! pod5_some {
	($result:expr) => {{
		unsafe {
			let error_code = $crate::ffi::pod5_get_error_no();
			if error_code != $crate::ffi::pod5_error_POD5_OK
			{
				return None;
			}
		}

		Some($result)
	}};
}

#[macro_export]
macro_rules! pod5_ok {
	($result:expr) => {{
		unsafe {
			let error_code = $crate::ffi::pod5_get_error_no();
			if error_code != $crate::ffi::pod5_error_POD5_OK
			{
				let c_str = std::ffi::CStr::from_ptr($crate::ffi::pod5_get_error_string());
				return Err($crate::error::Pod5Error::from_error_code(
					error_code,
					c_str.to_str().expect("error").to_string(),
				));
			}
		}
		Ok($result)
	}};
	($wrapper:ident, $result:expr) => {{
		unsafe {
			let error_code = $crate::ffi::pod5_get_error_no();
			if error_code != $crate::ffi::pod5_error_POD5_OK
			{
				let c_str = std::ffi::CStr::from_ptr($crate::ffi::pod5_get_error_string());
				return $wrapper(Err($crate::error::Pod5Error::from_error_code(
					error_code,
					c_str
						.to_str()
						.expect("Failed to convert error string")
						.to_string(),
				)));
			}
		}
		$wrapper(Ok($result))
	}};
}

pub type Result<T, E = Pod5Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Pod5Error
{
	#[error("Out of memory error: {0}")]
	MemoryError(String),
	#[error("Key error: {0}")]
	KeyError(String),
	#[error("Type error: {0}")]
	TypeError(String),
	#[error("Invalid: {0}")]
	InvalidError(String),
	#[error("unable to read file: {0}")]
	IOError(String),
	#[error("Capacity error: {0}")]
	CapacityError(String),
	#[error("Index error: {0}")]
	IndexError(String),
	#[error("Operation cancelled: {0}")]
	CancelledError(String),
	#[error("An unknown error has occured: {0}")]
	UnknownError(String),
	#[error("Not implemented: {0}")]
	NotImplementedError(String),
	#[error("Serialisation error: {0}")]
	SerialisationError(String),
	#[error("String not long enough: {0}")]
	StringLengthError(String),
}

impl Pod5Error
{
	pub fn from_error_code(code: u32, message: String) -> Pod5Error
	{
		match code
		{
			1 => Pod5Error::MemoryError(message),
			2 => Pod5Error::KeyError(message),
			3 => Pod5Error::TypeError(message),
			4 => Pod5Error::InvalidError(message),
			5 => Pod5Error::IOError(message),
			6 => Pod5Error::CapacityError(message),
			7 => Pod5Error::IndexError(message),
			8 => Pod5Error::CancelledError(message),
			9 => Pod5Error::UnknownError(message),
			10 => Pod5Error::NotImplementedError(message),
			11 => Pod5Error::SerialisationError(message),
			12 => Pod5Error::StringLengthError(message),
			_ => Pod5Error::UnknownError(message),
		}
	}
}
