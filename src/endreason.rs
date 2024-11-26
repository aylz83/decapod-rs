#[repr(u32)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum EndReason
{
	Unknown = crate::ffi::pod5_end_reason_POD5_END_REASON_UNKNOWN,
	MuxChange = crate::ffi::pod5_end_reason_POD5_END_REASON_MUX_CHANGE,
	UnblockMuxChange = crate::ffi::pod5_end_reason_POD5_END_REASON_UNBLOCK_MUX_CHANGE,
	DataServiceUnblockMuxChange =
		crate::ffi::pod5_end_reason_POD5_END_REASON_DATA_SERVICE_UNBLOCK_MUX_CHANGE,
	SignalPositive = crate::ffi::pod5_end_reason_POD5_END_REASON_SIGNAL_POSITIVE,
	SignalNegative = crate::ffi::pod5_end_reason_POD5_END_REASON_SIGNAL_NEGATIVE,
	APIRequest = crate::ffi::pod5_end_reason_POD5_END_REASON_API_REQUEST,
	DeviceDataError = crate::ffi::pod5_end_reason_POD5_END_REASON_DEVICE_DATA_ERROR,
	AnalysisConfigChange = crate::ffi::pod5_end_reason_POD5_END_REASON_ANALYSIS_CONFIG_CHANGE,
}

impl EndReason
{
	pub fn end_reason_from_code(code: i16) -> EndReason
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
