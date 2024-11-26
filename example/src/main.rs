use decapod::reader;

use serde_json;
use anyhow;

use std::env;

fn main() -> anyhow::Result<()>
{
	let args: Vec<String> = env::args().collect();

	let reader = reader::Reader::from_file(args[1].as_str())?;

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
