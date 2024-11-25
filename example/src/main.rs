use decapod::reader;

use serde_json;
use anyhow;

fn main() -> anyhow::Result<()>
{
	let reader = reader::Reader::from_file("test.pod5")?;

	println!("{:?}", &reader.read_ids()?);

	let reads = reader.reads()?;

	for read in reads
	{
		let read = read?;
		println!("{:?}", &read.uuid());

		let serialised = serde_json::to_string(&read).unwrap();
		println!("{}", serialised);

		//println!("{:?}", read.signal());
	}

	Ok(())
}
