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
