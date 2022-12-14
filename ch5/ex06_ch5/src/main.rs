const BIAS: i32 = 127;
const RADIX: f32 = 2.0;


fn main() {
    let n: f32 = 42.42;

    let (sign, exp, frac) = to_parts(n);
    let (sign_, exp_, mant) = decode(sign, exp, frac);
    let n_ = from_parts(sign_, exp_, mant);

    println!("{} -> {}", n, n_);
    println!("field      |  as bits   |  as real number");
    println!("sign       |  {:01b}    |  {}", sign, sign_);
    println!("exponent   |  {:08b}    |  {}", exp, exp_);
    println!("mantissa   |  {:023b}   |  {}", frac, mant);

}

fn to_parts(n: f32) -> (u32, u32, u32) {
    let n_bits: u32 = n.to_bits();
    let sign = (n_bits>>31) & 1;
    let exponent = (n_bits>>23) & 0xff;
    let fraction = (n_bits) & 0x7fffff;

    (sign, exponent, fraction)
}

fn decode(sign: u32, exp: u32, frac: u32) -> (f32, f32, f32) {
    let sign_decode = (-1.0_f32).powf(sign as f32);

    let exponent = (exp as i32)-BIAS;
    let exponent = RADIX.powf(exponent as f32);

    let mut mantissa: f32 = 1.0;

    for i in 0..23{
        let mask = 1 << i;
        let one_at_bit_i = frac & mask;
        
        if one_at_bit_i !=0 {
            let i_ = i as f32;
            let weight = 2_f32.powf( i_ - 23.0);
            mantissa += weight;
        }
    }

    (sign_decode, exponent, mantissa)
}

fn from_parts(sign_: f32, exp: f32, mant: f32) -> f32 {
    sign_ * exp * mant
}