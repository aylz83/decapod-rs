use std::fmt;

#[doc(hidden)]
#[repr(u32)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub enum EndReason
{
	Unknown = crate::pod5_ffi::pod5_end_reason_POD5_END_REASON_UNKNOWN,
	MuxChange = crate::pod5_ffi::pod5_end_reason_POD5_END_REASON_MUX_CHANGE,
	UnblockMuxChange = crate::pod5_ffi::pod5_end_reason_POD5_END_REASON_UNBLOCK_MUX_CHANGE,
	DataServiceUnblockMuxChange =
		crate::pod5_ffi::pod5_end_reason_POD5_END_REASON_DATA_SERVICE_UNBLOCK_MUX_CHANGE,
	SignalPositive = crate::pod5_ffi::pod5_end_reason_POD5_END_REASON_SIGNAL_POSITIVE,
	SignalNegative = crate::pod5_ffi::pod5_end_reason_POD5_END_REASON_SIGNAL_NEGATIVE,
	APIRequest = crate::pod5_ffi::pod5_end_reason_POD5_END_REASON_API_REQUEST,
	DeviceDataError = crate::pod5_ffi::pod5_end_reason_POD5_END_REASON_DEVICE_DATA_ERROR,
	AnalysisConfigChange = crate::pod5_ffi::pod5_end_reason_POD5_END_REASON_ANALYSIS_CONFIG_CHANGE,
}

impl EndReason
{
	pub(crate) fn end_reason_from_code(code: i16) -> EndReason
	{
		match code
		{
			1 => EndReason::MuxChange,
			2 => EndReason::UnblockMuxChange,
			3 => EndReason::DataServiceUnblockMuxChange,
			4 => EndReason::SignalPositive,
			5 => EndReason::SignalNegative,
			6 => EndReason::APIRequest,
			7 => EndReason::DeviceDataError,
			8 => EndReason::AnalysisConfigChange,
			_ => EndReason::Unknown,
		}
	}
}

impl fmt::Display for EndReason
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{:?}", self)
	}
}
