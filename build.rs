fn main() {
	println!("cargo:rustc-link-search=native={}/lib", cmake::build("remote").display());
	//println!("cargo:rustc-link-lib=static=remote");
}