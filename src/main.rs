//#![feature(bench_black_box)]

#[link(name = "bad")]
extern "C" {
    pub fn c_read_value() -> u16;
}

fn main() {
    let value = unsafe { c_read_value() };
    dbg!(value); // ok
    dbg!(value as usize); // nok
    dbg!(usize::from(value)); // nok
    dbg!((value as usize) & 0xFFFF); // nok

    //dbg!(std::hint::black_box(value) as usize); // ok
}
