#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(clippy::upper_case_acronyms)]

use std::ptr;

include!("bindings.rs");

impl Default for ReadBatchRowInfo_t
{
	fn default() -> Self
	{
		ReadBatchRowInfo_t {
			read_id: [0; 16],
			read_number: 0,
			start_sample: 0,
			calibration_offset: 0.0,
			calibration_scale: 0.0,
			channel: 0,
			median_before: 0.0,
			well: 0,
			pore_type: 0,
			end_reason: 0,
			end_reason_forced: 0,
			run_info: 0,
			signal_row_count: 0,
			num_reads_since_mux_change: 0,
			num_minknow_events: 0,
			num_samples: 0,
			tracked_scaling_scale: 0.0,
			tracked_scaling_shift: 0.0,
			predicted_scaling_scale: 0.0,
			predicted_scaling_shift: 0.0,
			time_since_mux_change: 0.0,
		}
	}
}

impl Default for FileInfo_t
{
	fn default() -> Self
	{
		FileInfo_t {
			file_identifier: [0; 16],
			version: FileInfo_Version {
				major: 0,
				minor: 0,
				revision: 0,
			},
		}
	}
}

impl Default for EmbeddedFileData_t
{
	fn default() -> Self
	{
		EmbeddedFileData_t {
			file_name: ptr::null(),
			offset: 0,
			length: 0,
		}
	}
}
