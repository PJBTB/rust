use test_float_parse::validate;

fn main() {
    for e in 300..310 {
        for i in 0..100000 {
            validate(&format!("{}e{}", i, e));
        }
    }
}
