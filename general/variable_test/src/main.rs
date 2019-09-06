fn main() {
    println!("variable test(for debugger)");

    let var_int8: i8    = -8;
    let var_uint8: u8   =  8;
    let var_int16: i16  = -16;
    let var_int32: i32  = -32;
    let var_int64: i64  = -64;
    let var_fp32: f32 = 0.32;
    let var_fp64: f64 = 0.64;

    println!("int8: {}, uint8: {}", var_int8, var_uint8);
    println!("int16: {}, uint32: {}, int64: {}", var_int16, var_int32, var_int64);
    println!("float32: {}, float64{}", var_fp32, var_fp64);

    println!("end(place for breakpoint)");
}
