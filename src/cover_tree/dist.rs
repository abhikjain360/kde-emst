use cblas::{saxpy, snrm2};

/// Calculates the Euclidean distance between 2 points, given in input as `&[f32; D]`.
pub fn distance<const D: usize>(a: &[f32; D], b: &[f32; D]) -> f32 {
    let d = D as i32;
    let mut temp = *a;
    unsafe {
        saxpy(d, -1.0, b, 1, &mut temp, 1);
        snrm2(d, &temp, 1)
    }
}

/// Gives the covering distance at the given `level`
#[inline]
pub fn covdist(level: i32) -> f32 {
    2f32.powi(level)
}

/// Gives the seperation distance at the given `level`
#[inline]
pub fn sepdist(level: i32) -> f32 {
    2f32.powi(level - 1)
}
