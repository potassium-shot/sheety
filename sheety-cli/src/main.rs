extern crate sheety;

static VERSION_MSG: &str = include_str!("version.txt");
static USAGE_MSG: &str = include_str!("usage.txt");
static VER_USAGE_MSG: &str = concat!(include_str!("version.txt"), "\n", include_str!("usage.txt"));

fn main() {
    println!("{}", VER_USAGE_MSG);
}
