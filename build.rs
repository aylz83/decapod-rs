use std::process::Command;

fn main()
{
	let _ = Command::new("python")
		.args([
			"-m",
			"build",
			"--sdist",
			"--no-isolation",
			"third_party/pod5-file-format",
		])
		.status()
		.expect("Failed to generate _version.py for pod5-file-format");

	let _ = Command::new("python")
		.args(["pod5_make_version.py"])
		.current_dir("third_party/pod5-file-format")
		.status()
		.expect("Failed to copy version information for pod5-file-format");

	let _ = Command::new("cmake")
		.args([
			"-S",
			"third_party/pod5-file-format",
			"-B",
			"third_party/pod5-file-format/build",
			"-DCMAKE_BUILD_TYPE=Release",
		])
		.status()
		.expect("Failed to run CMake configuration");

	let _ = Command::new("cmake")
		.args([
			"--build",
			"third_party/pod5-file-format/build",
			"--config",
			"Release",
		])
		.status()
		.expect("Failed to build with CMake");

	pkg_config::Config::new()
		.atleast_version("1.0.0")
		.probe("libzstd")
		.unwrap();
	pkg_config::Config::new()
		.atleast_version("9.0.0")
		.probe("arrow")
		.unwrap();

	// Specify the static library
	println!("cargo:rustc-link-search=native=third_party/pod5-file-format/build/Release/lib");
	println!("cargo:rustc-link-lib=static=pod5_format");
	//println!("cargo:rustc-link-lib=arrow");
	//println!("cargo:rustc-link-lib=zstd");
	println!("cargo:rustc-link-lib=dylib=stdc++");
	println!(
		"cargo:rerun-if-changed=third_party/pod5-file-format/build/Release/lib/ibpod5_format.a"
	);

	let bindings = bindgen::Builder::default()
		.header("third_party/pod5-file-format/c++/pod5_format/c_api.h")
		.formatter(bindgen::Formatter::Prettyplease)
		//.rustfmt_configuration_file(Some(PathBuf::from_str(".rustfmt.toml").expect("")))
		.generate()
		.expect("Unable to generate bindings");

	// Write the bindings to a file
	bindings
		.write_to_file("src/bindings.rs")
		.expect("Couldn't write bindings!");
}
