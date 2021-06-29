fn main(){
    println!(r"cargo:rustc-link-lib=wrap");
    println!(r"cargo:rustc-link-search=wrap/src/c");
}
