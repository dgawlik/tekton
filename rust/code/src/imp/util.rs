

#[allow(dead_code)]
fn gcd_extended(a: u16, b:u16, x: &mut u16, y: &mut u16) -> u16{
    if a == 0 {
        *x = 0_u16;
        *y = 1_u16;
        return b;
    }
    else {
        let mut x1: u16 = 0_u16;
        let mut y1: u16 = 0_u16;

        let gcd = gcd_extended(b % a, a, &mut x1, &mut y1);

        *x = y1.wrapping_sub((b/a).wrapping_mul(x1));
        *y = x1;

        return gcd;
    }
}

#[allow(dead_code)]
fn mod_inverse(a: u8) -> u8 {
    let m = 256_u16;

    let mut x: u16 = 0;
    let mut y: u16 = 0;

    gcd_extended(a.into(), m, &mut x, &mut y);
    return (((x % m) + m) % m).to_be_bytes()[1];
}