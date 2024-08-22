#[inline(always)]
pub fn u5_to_u8_naive(x: u8) -> u8 {
    debug_assert!(x < 32);
    let factor = 255.0 / 31.0;
    (x as f32 * factor).round() as u8
}
#[inline(always)]
pub fn u5_to_u8_v2(x: u8) -> u8 {
    debug_assert!(x < 32);
    let factor = 255.0 / 31.0;
    (x as f32 * factor + 0.5) as u8
}
#[inline(always)]
pub fn u5_to_u8_unsafe(x: u8) -> u8 {
    debug_assert!(x < 32);
    let factor = 255.0 / 31.0;
    let f = x as f32 * factor + 0.5;
    unsafe { f.to_int_unchecked() }
}
#[inline(always)]
pub fn u5_to_u8_safer(x: u8) -> u8 {
    debug_assert!(x < 32);
    let factor = 255.0 / 31.0;
    let f = x as f32 * factor + 0.5;
    let g = f.min(255.0);
    unsafe { g.to_int_unchecked() }
}
#[inline(always)]
pub fn u5_to_u8_safer_int(mut x: u8) -> u8 {
    debug_assert!(x < 32);
    x &= 0x1F; // see SAFETY comment
    let factor = 255.0 / 31.0;
    let f = x as f32 * factor + 0.5;
    unsafe {
        // SAFETY: f is in the range [0.5, 255.5], because x is in the range [0, 31]
        f.to_int_unchecked()
    }
}

#[inline(always)]
pub fn u5_to_u8_int(x: u8) -> u8 {
    debug_assert!(x < 32);
    ((x as u16 * 255 + (31 / 2)) / 31) as u8
}

#[inline(always)]
pub fn u5_to_u8_ma(x: u8) -> u8 {
    debug_assert!(x < 32);
    ((x as u16 * 527 + 23) >> 6) as u8
}

#[inline(always)]
pub fn u5_to_u8_lut(x: u8) -> u8 {
    debug_assert!(x < 32);
    const LUT: [u8; 32] = [
        0, 8, 16, 25, 33, 41, 49, 58, 66, 74, 82, 90, 99, 107, 115, 123, 132, 140, 148, 156, 165,
        173, 181, 189, 197, 206, 214, 222, 230, 239, 247, 255,
    ];
    LUT[(x as usize) % LUT.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correctness() {
        for x in 0..=31 {
            let correct = u5_to_u8_naive(x);
            assert_eq!(u5_to_u8_v2(x), correct);
            assert_eq!(u5_to_u8_unsafe(x), correct);
            assert_eq!(u5_to_u8_safer(x), correct);
            assert_eq!(u5_to_u8_safer_int(x), correct);
            assert_eq!(u5_to_u8_int(x), correct);
            assert_eq!(u5_to_u8_ma(x), correct);
            assert_eq!(u5_to_u8_lut(x), correct);
        }
    }
}
