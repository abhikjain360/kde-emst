use cblas::{saxpy, sdot, sgemv, Transpose};
use lapacke::{spotrf, spotri};

pub fn gaussian_kernel<const D: usize>(
    // moving stuff instead of using references as we return a closure which captures these
    // variables
    mean: [f32; D],
    cov: [[f32; D]; D],
) -> impl Fn(&[f32; D]) -> f32 {
    let d = D as i32;

    let mut lower = cov;

    // cholensky decomposition for determinant
    let return_val = unsafe { spotrf(lapacke::Layout::RowMajor, b'l', d, &mut lower[0], d) };
    if return_val != 0 {
        panic!("not a positive-semidefinite matrix!");
    }

    let mut determinant = 1f32;
    #[allow(clippy::clippy::needless_range_loop)]
    for i in 0..D {
        determinant *= lower[i][i];
    }
    determinant *= 2.0;

    // finding the inverse
    let mut cov_inv = cov;
    let return_val = unsafe { spotri(lapacke::Layout::RowMajor, b'l', d, &mut cov_inv[0], d) };
    if return_val != 0 {
        panic!("not a positive-semidefinite matrix!");
    }

    move |x| {
        let mut y = [0f32; D];
        let mut diff = mean;
        unsafe {
            saxpy(d, -1.0, x, 1, &mut diff, 1);
            sgemv(
                cblas::Layout::RowMajor,
                Transpose::None,
                d,
                d,
                1.0,
                &cov_inv[0],
                d,
                &diff,
                1,
                0.0,
                &mut y,
                1,
            );
        }

        let numerator = (-0.5 * unsafe { sdot(d, &diff, 1, &y, 1) }).exp();
        let denominator = ((2.0 * std::f32::consts::PI).powi(d) * determinant).sqrt();

        numerator / denominator
    }
}
