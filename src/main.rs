mod wad;

fn main() {
    let _wad = wad::Wad::from_file("examples/freedoom1.wad").unwrap();
}
