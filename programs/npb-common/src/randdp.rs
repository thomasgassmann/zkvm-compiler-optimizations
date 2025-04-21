use crate::dcomplex;

pub const R23: f64 = 0.5
    * 0.5
    * 0.5
    * 0.5
    * 0.5
    * 0.5
    * 0.5
    * 0.5
    * 0.5
    * 0.5
    * 0.5
    * 0.5
    * 0.5
    * 0.5
    * 0.5
    * 0.5
    * 0.5
    * 0.5
    * 0.5
    * 0.5
    * 0.5
    * 0.5
    * 0.5;
pub const T23: f64 = (2 << 22) as f64;
pub const R46: f64 = R23 * R23;
pub const T46: f64 = T23 * T23;

///---------------------------------------------------------------------
///
///this routine returns a uniform pseudorandom double precision number in the
///range (0, 1) by using the linear congruential generator
///
///x_{k+1} = a x_k  (mod 2^46)
///
///where 0 < x_k < 2^46 and 0 < a < 2^46. this scheme generates 2^44 numbers
///before repeating. the argument A is the same as 'a' in the above formula,
///and X is the same as x_0.  A and X must be odd double precision integers
///in the range (1, 2^46). the returned value RANDLC is normalized to be
///between 0 and 1, i.e. RANDLC = 2^(-46) * x_1.  X is updated to contain
///the new seed x_1, so that subsequent calls to RANDLC using the same
///arguments will generate a continuous sequence.
///
///this routine should produce the same results on any computer with at least
///48 mantissa bits in double precision floating point data.  On 64 bit
///systems, double precision should be disabled.
///
///David H. Bailey, October 26, 1990
///
///---------------------------------------------------------------------
#[allow(dead_code)]
pub fn randlc(x: &mut f64, a: f64) -> f64 {
    /*
     * ---------------------------------------------------------------------
     * break A into two parts such that A = 2^23 * A1 + A2.
     * ---------------------------------------------------------------------
     */
    let t1 = R23 * a;
    let a1 = t1 as i64;
    let a2 = a - T23 * a1 as f64;

    /*
     * ---------------------------------------------------------------------
     * break X into two parts such that X = 2^23 * X1 + X2, compute
     * Z = A1 * X2 + A2 * X1  (mod 2^23), and then
     * X = 2^23 * Z + A2 * X2  (mod 2^46).
     * ---------------------------------------------------------------------
     */
    let t1 = R23 * *x;
    let x1 = t1 as i64;
    let x2 = *x - T23 * x1 as f64;
    let t1 = a1 as f64 * x2 + a2 * x1 as f64;
    let t2 = (R23 * t1) as i64;
    let z = t1 - T23 * t2 as f64;
    let t3 = T23 * z + a2 * x2;
    let t4 = (R46 * t3) as i64;
    *x = t3 - T46 * t4 as f64;

    R46 * *x
}

///---------------------------------------------------------------------
///
///this routine generates N uniform pseudorandom double precision numbers in
///the range (0, 1) by using the linear congruential generator
///
///x_{k+1} = a x_k  (mod 2^46)
///
///where 0 < x_k < 2^46 and 0 < a < 2^46. This scheme generates 2^44 numbers
///before repeating. The argument a is the same as 'a' in the above formula,
///and x is the same as x_0. A and X must be odd double precision integers
///in the range (1, 2^46). the N results are placed in Y and are normalized
///to be between 0 and 1. X is updated to contain the new seed, so that
///subsequent calls to VRANLC using the same arguments will generate a
///continuous sequence.  if N is zero, only initialization is performed, and
///the variables X, A and Y are ignored.
///
///this routine is the standard version designed for scalar or RISC systems.
///however, it should produce the same results on any single processor
///computer with at least 48 mantissa bits in double precision floating point
///data. on 64 bit systems, double precision should be disabled.
///
///---------------------------------------------------------------------
#[allow(dead_code)]
pub fn vranlc(n: i32, x: &mut f64, a: f64, y: &mut [f64]) {
    let (mut t1, mut x2, mut z, mut t3): (f64, f64, f64, f64);
    let (mut x1, mut t2, mut t4): (i64, i64, i64);
    /*
     * ---------------------------------------------------------------------
     * break A into two parts such that A = 2^23 * A1 + A2.
     * ---------------------------------------------------------------------
     */
    t1 = R23 * a;
    let a1 = t1 as i64;
    let a2 = a - T23 * a1 as f64;

    /*
     * ---------------------------------------------------------------------
     * generate N results. this loop is not vectorizable.
     * ---------------------------------------------------------------------
     */
    for i in 0..n {
        /*
         * ---------------------------------------------------------------------
         * break X into two parts such that X = 2^23 * X1 + X2, compute
         * Z = A1 * X2 + A2 * X1  (mod 2^23), and then
         * X = 2^23 * Z + A2 * X2  (mod 2^46).
         * ---------------------------------------------------------------------
         */
        t1 = R23 * *x;
        x1 = t1 as i64;
        x2 = *x - T23 * x1 as f64;
        t1 = a1 as f64 * x2 + a2 * x1 as f64;
        t2 = (R23 * t1) as i64;
        z = t1 - T23 * t2 as f64;
        t3 = T23 * z + a2 * x2;
        t4 = (R46 * t3) as i64;
        *x = t3 - T46 * t4 as f64;
        y[i as usize] = R46 * *x;
    }
}

#[allow(dead_code)]
pub fn vranlc_dcomplex(n: i32, x: &mut f64, a: f64, y: &mut [dcomplex::Dcomplex]) {
    let (mut t1, mut x2, mut z, mut t3, mut xi): (f64, f64, f64, f64, f64);
    let (mut x1, mut t2, mut t4): (i64, i64, i64);
    /*
     * ---------------------------------------------------------------------
     * break A into two parts such that A = 2^23 * A1 + A2.
     * ---------------------------------------------------------------------
     */
    t1 = R23 * a;
    let a1 = t1 as i64;
    let a2 = a - T23 * a1 as f64;

    /*
     * ---------------------------------------------------------------------
     * generate N results. this loop is not vectorizable.
     * ---------------------------------------------------------------------
     */
    for i in 0..n {
        /*
         * ---------------------------------------------------------------------
         * break X into two parts such that X = 2^23 * X1 + X2, compute
         * Z = A1 * X2 + A2 * X1  (mod 2^23), and then
         * X = 2^23 * Z + A2 * X2  (mod 2^46).
         * ---------------------------------------------------------------------
         */
        t1 = R23 * *x;
        x1 = t1 as i64;
        x2 = *x - T23 * x1 as f64;
        t1 = a1 as f64 * x2 + a2 * x1 as f64;
        t2 = (R23 * t1) as i64;
        z = t1 - T23 * t2 as f64;
        t3 = T23 * z + a2 * x2;
        t4 = (R46 * t3) as i64;
        *x = t3 - T46 * t4 as f64;

        xi = *x;
        t1 = R23 * *x;
        x1 = t1 as i64;
        x2 = *x - T23 * x1 as f64;
        t1 = a1 as f64 * x2 + a2 * x1 as f64;
        t2 = (R23 * t1) as i64;
        z = t1 - T23 * t2 as f64;
        t3 = T23 * z + a2 * x2;
        t4 = (R46 * t3) as i64;
        *x = t3 - T46 * t4 as f64;
        y[i as usize] = dcomplex::Dcomplex {
            real: R46 * xi,
            imag: R46 * *x,
        };
    }
}