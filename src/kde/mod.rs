mod gaussian;
pub use gaussian::*;

pub fn kde<const D: usize>(
    bandwidth: f32,
    x: &[f32; D],
    data: &[[f32; D]],
    kernel: impl Fn(&[f32; D]) -> f32,
) -> f32 {
    let d = D as i32;
    data.iter()
        .map(|elem| {
            let mut temp = *elem;
            unsafe {
                saxpy(d, -1.0, x, 1, &mut temp, 1);
            }
            kernel(&temp) / bandwidth
        })
        .sum::<f32>()
        / (bandwidth * data.len() as f32)
}
