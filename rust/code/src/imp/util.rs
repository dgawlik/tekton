

#[allow(dead_code)]
fn gcd_extended(a: u64, b:u64, x: &mut u64, y: &mut u64) -> u64{
    if a == 0 {
        *x = 0_u64;
        *y = 1_u64;
        return b;
    }
    else {
        let mut x1: u64 = 0_u64;
        let mut y1: u64 = 0_u64;

        let gcd = gcd_extended(b % a, a, &mut x1, &mut y1);

        *x = y1.wrapping_sub((b/a).wrapping_mul(x1));
        *y = x1;

        return gcd;
    }
}

#[allow(dead_code)]
fn mod_inverse(a: u32) -> u32 {
    let m = 4294967296_u64;

    let mut x: u64 = 0;
    let mut y: u64 = 0;

    gcd_extended(a.into(), m, &mut x, &mut y);
    return (((x % m) + m) % m) as u32;
}

#[test]
pub fn calc_inverse(){
    // print!("{}", mod_inverse(112_012_097));
    print!("{}", (1347249345 as u32).wrapping_mul(112_012_097));
}