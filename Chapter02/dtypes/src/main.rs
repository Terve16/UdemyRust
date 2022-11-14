fn main() {
    let _x1: i8 = 0;
    let _x2: i16 = 0;
    let _x3: i32 = 0;
    let _x4: i64 = 0;
    let _x5: i128 = i128::MIN;

    let _x6: u8 = u8::MAX;
    let _x7: u16 = u16::MAX;
    let _x8: u32 = u32::MAX;
    let _x9: u64 = u64::MAX;
    let _x10: u128 = u128::MAX;

    println!("{}, {}, {}", _x6, _x7, _x8);

    let _dec = 255;
    println!("dec: {}", _dec);
    let _hex = 0xff;
    println!("hex: {}", _hex);
    let _bin = 0b11111111;
    println!("bin: {}", _bin);

    let _f1 = 2.0;
    let _f2: f64 = 0.0;
    let _f3: f32 = 0.0;

    let _f4 = 2.0_f32;
    let _f5 = 2.0_f64;

    let _flag1: bool = true;
    let _flag2 = false;
}
