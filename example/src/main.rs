use decapod::reader::Reader;

use serde_json;

use uuid::uuid;

use std::env;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>
{
	let args: Vec<String> = env::args().collect();

	let mut read_ids = Vec::new();
	read_ids.push(uuid!("002fde30-9e23-4125-9eae-d112c18a81a7"));
	read_ids.push(uuid!("006d1319-2877-4b34-85df-34de7250a47b"));

	let read_ids = Some(read_ids);

	let reader = Reader::from_path(args[1].as_str(), None)?;

	println!("{:?}", &reader.read_ids()?);

	let reads = reader.reads_iter(None);

	for read in reads
	{
		let read = read?;
		println!("{:?}", &read.uuid());

		//let serialised = serde_json::to_string(&read).unwrap();
		//println!("{}", serialised);

		println!("pore type = {}", read.pore_type_string()?);
	}

	let runinfo_iter = reader.run_info_iter();

	for runinfo in runinfo_iter
	{
		let runinfo = runinfo?;
		println!("{}", runinfo);
	}

	let batches = reader.batch_records_iter(None);

	let fields = Some(vec!["read_id", "run_info"]);

	for batch in batches
	{
		let batch = batch?;
		println!("{}", batch.to_df(&None)?);
	}

	Ok(())
}
