pub struct FileInfo
{
	pub(crate) inner: crate::pod5_ffi::FileInfo_t,
}

impl FileInfo
{
	pub fn file_identifier(&self) -> [u8; 16]
	{
		self.inner.file_identifier
	}

	pub fn major_version(&self) -> u16
	{
		self.inner.version.major
	}

	pub fn minor_version(&self) -> u16
	{
		self.inner.version.major
	}

	pub fn revision_version(&self) -> u16
	{
		self.inner.version.major
	}
}
