use decapod::reader;

use serde_json;
use anyhow;

use std::env;

fn main() -> anyhow::Result<()>
{
	let args: Vec<String> = env::args().collect();

	let reader = reader::Reader::from_file(args[1].as_str())?;

	println!("{:?}", &reader.read_ids()?);

	let reads = reader.reads_iter()?;

	for read in reads
	{
		let read = read?;
		println!("{:?}", &read.uuid());

		//let serialised = serde_json::to_string(&read).unwrap();
		//println!("{}", serialised);

		println!("pore type = {}", read.pore_type_string()?);
	}

	let runinfo_iter = reader.run_info_iter()?;

	for runinfo in runinfo_iter
	{
		let runinfo = runinfo?;
		println!("{}", runinfo);
	}

	let batches = reader.batch_records_iter()?;

	let fields = Some(vec!["read_id", "run_info"]);

	for batch in batches
	{
		let batch = batch?;
		println!("{}", batch.to_df(&None)?);
	}

	Ok(())
}
