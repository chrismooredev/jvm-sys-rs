
use std::env;
use std::path::PathBuf;
use jvm_find::JavaHome;

fn main() {
	let out_dir: PathBuf = env::var_os("OUT_DIR").unwrap().into();
	println!("cargo:rerun-if-env-changed=JAVA_HOME");
	let java_home = JavaHome::find_valid_home().unwrap();
	let include_dirs = java_home.include().unwrap().expect("JDK not installed");
	
	let include_root = include_dirs[0].to_str().expect("Java home path contained non-UTF8 characters - this is not supported");

	let mut builder = bindgen::Builder::default();

	#[cfg(feature = "link")]
	let native_path = java_home.native_library().unwrap();

	if cfg!(feature = "link") {
		// add the native library's folder to the native library path
		let mut native_dir = native_path.clone();
		native_dir.pop();
		println!("cargo:rustc-link-search=native={}", native_dir.display());
	
		// so linking on Windows can find jvm.lib and jawt.lib in $JAVA_HOME/lib
		if cfg!(target_os = "windows") {
			let lib_path = java_home.join("lib");
			println!("cargo:rustc-link-search={}", lib_path.display());
		}
	}

	

	if cfg!(feature = "jni") {
		builder = builder.header(include_root.to_owned() + "/jni.h");

		if cfg!(feature = "link") {
			// On MacOS, we need to link to libjli instead of libjvm as a workaround
			// to a Java8 bug. See here for more information:
			// https://bugs.openjdk.java.net/browse/JDK-7131356
			if cfg!(target_os = "macos") {
				println!("cargo:rustc-link-lib=dylib=jli");
			} else {
				println!("cargo:rustc-link-lib=dylib=jvm");
			}
		}
	}

	if cfg!(feature = "jawt") {
		builder = builder.header(include_root.to_owned() + "/jawt.h");

		if cfg!(feature = "link") {
			println!("cargo:rustc-link-lib=dylib=jawt");
		}
	}

	if cfg!(feature = "jvmti") {
		builder = builder.header(include_root.to_owned() + "/jvmti.h");

		// is there an appropriate library to link to?
	}

	let bindings = builder.parse_callbacks(Box::new(bindgen::CargoCallbacks))

		// only include Java stuff - exclude stdlib stuff that would otherwise be included
		.allowlist_function("[jJ].*")
		.allowlist_type("[jJ].*")
		.allowlist_var("[jJ].*")

		.clang_args(include_dirs.iter().map(|pb|
			String::from("-I") + pb.to_str()
				.expect("Java home path contained non-UTF8 characters - this is not supported")
		))
		.generate()
		.expect("Unable to generate bindings");
	
	bindings
		.write_to_file(out_dir.join("bindings.rs"))
		.expect("Couldn't write bindings!");

}
