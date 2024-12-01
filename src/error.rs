use thiserror::Error;

#[macro_export]
#[doc(hidden)]
macro_rules! pod5_check_error {
	() => {{
		unsafe {
			let error_code = $crate::pod5_ffi::pod5_get_error_no();
			if error_code != $crate::pod5_ffi::pod5_error_POD5_OK
			{
				let c_str = std::ffi::CStr::from_ptr($crate::pod5_ffi::pod5_get_error_string());
				return Some(Err($crate::error::Error::from_error_code(
					error_code,
					c_str
						.to_str()
						.unwrap_or("Failed to obtain error message")
						.to_string(),
				)));
			}
		}
	}};
}

#[macro_export]
#[doc(hidden)]
macro_rules! pod5_some {
	($result:expr) => {{
		unsafe {
			let error_code = $crate::pod5_ffi::pod5_get_error_no();
			if error_code != $crate::pod5_ffi::pod5_error_POD5_OK
			{
				return None;
			}
		}

		Some($result)
	}};
}

#[macro_export]
#[doc(hidden)]
macro_rules! pod5_ok {
	($result:expr) => {{
		unsafe {
			let error_code = $crate::pod5_ffi::pod5_get_error_no();
			if error_code != $crate::pod5_ffi::pod5_error_POD5_OK
			{
				let c_str = std::ffi::CStr::from_ptr($crate::pod5_ffi::pod5_get_error_string());
				return Err($crate::error::Error::from_error_code(
					error_code,
					c_str
						.to_str()
						.unwrap_or("Failed to obtain error message")
						.to_string(),
				));
			}
		}
		Ok($result)
	}};
	($wrapper:ident, $result:expr) => {{
		unsafe {
			let error_code = $crate::pod5_ffi::pod5_get_error_no();
			if error_code != $crate::pod5_ffi::pod5_error_POD5_OK
			{
				let c_str = std::ffi::CStr::from_ptr($crate::pod5_ffi::pod5_get_error_string());
				return $wrapper(Err($crate::error::Error::from_error_code(
					error_code,
					c_str
						.to_str()
						.unwrap_or("Failed to obtain error message")
						.to_string(),
				)));
			}
		}
		$wrapper(Ok($result))
	}};
}

#[doc(hidden)]
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[doc(hidden)]
#[derive(Error, Debug)]
pub enum Error
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
	#[error("IO Arrow Error")]
	ArrowIOError(#[from] std::io::Error),
	#[error("Compression Arrow Error")]
	ArrowCompressionError(String),
	#[error("String conversion error")]
	StringError(#[from] std::str::Utf8Error),
}

impl Error
{
	pub(crate) fn from_error_code(code: u32, message: String) -> Error
	{
		match code
		{
			1 => Error::MemoryError(message),
			2 => Error::KeyError(message),
			3 => Error::TypeError(message),
			4 => Error::InvalidError(message),
			5 => Error::IOError(message),
			6 => Error::CapacityError(message),
			7 => Error::IndexError(message),
			8 => Error::CancelledError(message),
			9 => Error::UnknownError(message),
			10 => Error::NotImplementedError(message),
			11 => Error::SerialisationError(message),
			12 => Error::StringLengthError(message),
			_ => Error::UnknownError(message),
		}
	}
}
