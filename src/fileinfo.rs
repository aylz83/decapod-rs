pub struct FileInfo
{
	pub(crate) inner: *mut crate::ffi::FileInfo_t,
}

impl FileInfo
{
	pub fn file_identifier(&self) -> [u8; 16]
	{
		unsafe { (*self.inner).file_identifier }
	}

	pub fn major_version(&self) -> u16
	{
		unsafe { (*self.inner).version.major }
	}

	pub fn minor_version(&self) -> u16
	{
		unsafe { (*self.inner).version.major }
	}

	pub fn revision_version(&self) -> u16
	{
		unsafe { (*self.inner).version.major }
	}
}
