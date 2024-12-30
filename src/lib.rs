use num::complex::Complex;
use num::Float;

pub trait ConstLogcosh<T> {
    const LN_2: T;
    const SMALL: T; // Chosen such that error in x**2 / 2 is small enough
    const LARGE: T; // Chosen such that error in |x|-ln(2) is small enough
    const TWO: T;
    const ZERO: T;
    const ONEPLX: Complex<T>;
    const LN_2PLX: Complex<T>;
    const TWOPLX: Complex<T>;
}

impl<T> ConstLogcosh<f32> for T {
    const LN_2: f32 = std::f32::consts::LN_2;
    const SMALL: f32 = 2e-2;
    const LARGE: f32 = 4.0;
    const TWO: f32 = 2.0;
    const ZERO: f32 = 0.0;
    const ONEPLX: Complex<f32> = Complex::new(1.0, 0.0);
    const LN_2PLX: Complex<f32> = Complex::new(std::f32::consts::LN_2, 0.0);
    const TWOPLX: Complex<f32> = Complex::new(2.0, 0.0);
}

impl<T> ConstLogcosh<f64> for T {
    const LN_2: f64 = std::f64::consts::LN_2;
    const SMALL: f64 = 2e-2;
    const LARGE: f64 = 4.0;
    const TWO: f64 = 2.0;
    const ZERO: f64 = 0.0;
    const ONEPLX: Complex<f64> = Complex::new(1.0, 0.0);
    const LN_2PLX: Complex<f64> = Complex::new(std::f64::consts::LN_2, 0.0);
    const TWOPLX: Complex<f64> = Complex::new(2.0, 0.0);
}

pub fn approx_logcosh<T>(x: T) -> T
where
    T: ConstLogcosh<T> + Float,
{
    if x.abs() > T::LARGE {
        return x.abs() - T::LN_2;
    } else if x.abs() < T::SMALL {
        return x * x / T::TWO;
    } else {
        return logcosh(x);
    }
}

pub fn approx_clogcosh<T>(x: Complex<T>) -> Complex<T>
where
    T: ConstLogcosh<T> + Float,
{
    if x.re.abs() > T::LARGE {
        let s: Complex<T> = {
            match x.re == T::ZERO {
                true => x,
                false => x * x.re.signum(),
            }
        };
        return s - T::LN_2PLX;
    } else if x.norm() < T::SMALL {
        return (x * x) / T::TWO;
    } else {
        return clogcosh(x);
    }
}

pub fn logcosh<T>(x: T) -> T
where
    T: ConstLogcosh<T> + Float,
{
    let xabs = x.abs();
    let p = T::exp(-T::TWO * xabs);
    xabs + T::ln_1p(p) - T::LN_2
}

pub fn clogcosh<T>(x: Complex<T>) -> Complex<T>
where
    T: ConstLogcosh<T> + Float,
{
    let s: Complex<T> = {
        match x.re == T::ZERO {
            true => x,
            false => x * x.re.signum(),
        }
    };
    let p: Complex<T> = Complex::exp(-T::TWOPLX * s);
    s + Complex::ln(T::ONEPLX + p) - T::LN_2PLX
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trivial_same_as_mine() {
        let input: Vec<f32> = vec![0., 1., 13., 1e-40, -1e-40, -1., -13.];
        let triv: Vec<f32> = input.clone().into_iter().map(logcosh).collect();
        let mine: Vec<f32> = input.into_iter().map(approx_logcosh).collect();
        assert_eq!(triv, mine)
    }

    #[test]
    fn ctrivial_same_as_mine() {
        let mut input: Vec<Complex<f32>> = Vec::new();
        input.push(Complex::new(1., 1.));
        input.push(Complex::new(10., 1.));
        input.push(Complex::new(1., 10.));
        input.push(Complex::new(2e-4, 6e-5));
        let triv: Vec<Complex<f32>> = input.clone().into_iter().map(clogcosh).collect();
        let mine: Vec<Complex<f32>> = input.into_iter().map(approx_clogcosh).collect();
        for (a, b) in triv.iter().zip(mine) {
            assert!((a - b).norm() < 1e-7)
        }
    }
}
