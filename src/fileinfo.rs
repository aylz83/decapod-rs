/// file identifier metadata within the pod5 file being read.
/// # Example
/// ````
/// let reader = Reader::from_path("sample.pod", None);
/// for fileinfo in reader.info().iter()
/// {
///     let fileinfo = fileinfo?;
///     println!("{}", fileinfo.file_identifier());
/// }
/// ````
pub struct FileInfo
{
	pub(crate) inner: crate::pod5_ffi::FileInfo_t,
}

impl FileInfo
{
	/// Obtain the uuid of the file identifier of the pod5.
	pub fn file_identifier(&self) -> uuid::Uuid
	{
		uuid::Uuid::from_bytes(self.inner.file_identifier)
	}

	/// The major version of the pod5 file.
	pub fn major_version(&self) -> u16
	{
		self.inner.version.major
	}

	/// The minor version of the pod5 file.
	pub fn minor_version(&self) -> u16
	{
		self.inner.version.major
	}

	/// The revision version of the pod5 file.
	pub fn revision_version(&self) -> u16
	{
		self.inner.version.major
	}
}
