use decapod::reader;

use anyhow;

fn main() -> anyhow::Result<()>
{
	let reader = reader::Reader::from_file("test.pod5")?;

	println!("{:?}", &reader.read_ids()?);

	let reads = reader.reads()?;

	for read in reads
	{
		println!("{:?}", read?.read_id());
		//println!("{:?}", read.signal());
	}

	Ok(())
}
