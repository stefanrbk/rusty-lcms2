#[inline]
pub fn QuickFloor(val: f64) -> i32 {
    union Temp {
        pub val: f64,
        pub halves: [i32; 2],
    }
    const LCMS_DOUBLE2FIXMAGIC: f64 = 68719476736.0 * 1.5;  // 2^36 * 1.5, (52-16=36) uses limited precision to floor

    let temp = Temp { val: val + LCMS_DOUBLE2FIXMAGIC };
    unsafe { 
        if use_big_endian!() {
            temp.halves[0] >> 16
        } else {
            temp.halves[1] >> 16
        }
    }
}

#[inline]
pub fn QuickFloorWord(val: f64) -> u16 {
    QuickFloor(val - 32767.0) as u16 + 32767
}

#[inline]
pub fn QuickSaturateWord(val: f64) -> u16 {
    let val = val + 0.5;
    if val <= 0.0 { return 0; }
    if val >= 65535.0 { return 0xffff; }

    QuickFloorWord(val)
}
