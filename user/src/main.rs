use wrap;

fn main() {
    unsafe{
        let result = wrap::mytan(1.0);
        println!("tan(1.0) = {}", result);
    }
}
