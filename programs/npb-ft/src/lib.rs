use npb_common::dcomplex::*;
use npb_common::print_results::*;
use npb_common::randdp::*;
use npb_common::timers::*;

#[cfg(class = "Z")]
mod params {
    pub const CLASS: char = 'Z';
    pub const NX: i32 = 8;
    pub const NY: i32 = 8;
    pub const NZ: i32 = 8;
    pub const NITER: i32 = 2;
}

#[cfg(class = "S")]
mod params {
    pub const CLASS: char = 'S';
    pub const NX: i32 = 64;
    pub const NY: i32 = 64;
    pub const NZ: i32 = 64;
    pub const NITER: i32 = 6;
}

#[cfg(class = "W")]
mod params {
    pub const CLASS: char = 'W';
    pub const NX: i32 = 128;
    pub const NY: i32 = 128;
    pub const NZ: i32 = 32;
    pub const NITER: i32 = 6;
}

#[cfg(class = "A")]
mod params {
    pub const CLASS: char = 'A';
    pub const NX: i32 = 256;
    pub const NY: i32 = 256;
    pub const NZ: i32 = 128;
    pub const NITER: i32 = 6;
}

#[cfg(class = "B")]
mod params {
    pub const CLASS: char = 'B';
    pub const NX: i32 = 512;
    pub const NY: i32 = 256;
    pub const NZ: i32 = 256;
    pub const NITER: i32 = 20;
}

#[cfg(class = "C")]
mod params {
    pub const CLASS: char = 'C';
    pub const NX: i32 = 512;
    pub const NY: i32 = 512;
    pub const NZ: i32 = 512;
    pub const NITER: i32 = 20;
}

#[cfg(class = "D")]
mod params {
    pub const CLASS: char = 'D';
    pub const NX: i32 = 2048;
    pub const NY: i32 = 1024;
    pub const NZ: i32 = 1024;
    pub const NITER: i32 = 25;
}

#[cfg(class = "E")]
mod params {
    pub const CLASS: char = 'E';
    pub const NX: i32 = 4096;
    pub const NY: i32 = 2048;
    pub const NZ: i32 = 2048;
    pub const NITER: i32 = 25;
}

#[cfg(not(any(
    class = "S",
    class = "W",
    class = "A",
    class = "B",
    class = "C",
    class = "D",
    class = "E",
    class = "Z",
)))]
mod params {
    // Never used
    pub const CLASS: char = 'U';
    pub const NX: i32 = 1;
    pub const NY: i32 = 1;
    pub const NZ: i32 = 1;
    pub const NITER: i32 = 1;
    compile_error!(
        "\n\n\
		Must set a class at compilation time by setting RUSTFLAGS\n\
		class options for FT are: {S, W, A, B, C, D, E}\n\
		For example:\n\
		RUSTFLAGS='--cfg class=\"A\" ' cargo build --release --bin ft\n\n\n\
	"
    );
}

#[cfg(safe = "true")]
pub const UNSAFE: bool = false;
#[cfg(not(safe = "true"))]
pub const UNSAFE: bool = true;

#[cfg(timers = "true")]
pub const TIMERS: bool = true;
#[cfg(not(timers = "true"))]
pub const TIMERS: bool = false;

use params::*;

/*
 * ---------------------------------------------------------------------
 * u0, u1, u2 are the main arrays in the problem.
 * depending on the decomposition, these arrays will have different
 * dimensions. to accomodate all possibilities, we allocate them as
 * one-dimensional arrays and pass them to subroutines for different
 * views
 * - u0 contains the initial (transformed) initial condition
 * - u1 and u2 are working arrays
 * - twiddle contains exponents for the time evolution operator.
 * ---------------------------------------------------------------------
 * large arrays are in common so that they are allocated on the
 * heap rather than the stack. this common block is not
 * referenced directly anywhere else. padding is to avoid accidental
 * cache problems, since all array sizes are powers of two.
 * ---------------------------------------------------------------------
 * we need a bunch of logic to keep track of how
 * arrays are laid out.
 *
 * note: this serial version is the derived from the parallel 0D case
 * of the ft NPB.
 * the computation proceeds logically as
 *
 * set up initial conditions
 * fftx(1)
 * transpose (1->2)
 * ffty(2)
 * transpose (2->3)
 * fftz(3)
 * time evolution
 * fftz(3)
 * transpose (3->2)
 * ffty(2)
 * transpose (2->1)
 * fftx(1)
 * compute residual(1)
 *
 * for the 0D, 1D, 2D strategies, the layouts look like xxx
 *
 *            0D        1D        2D
 * 1:        xyz       xyz       xyz
 * 2:        xyz       xyz       yxz
 * 3:        xyz       zyx       zxy
 * the array dimensions are stored in dims(coord, phase)
 * ---------------------------------------------------------------------
 * if processor array is 1x1 -> 0D grid decomposition
 *
 * cache blocking params. these values are good for most
 * RISC processors.
 * FFT parameters:
 * fftblock controls how many ffts are done at a time.
 * the default is appropriate for most cache-based machines
 * on vector machines, the FFT can be vectorized with vector
 * length equal to the block size, so the block size should
 * be as large as possible. this is the size of the smallest
 * dimension of the problem: 128 for class A, 256 for class B
 * and 512 for class C.
 * ---------------------------------------------------------------------
 */

pub const FFTBLOCK_DEFAULT: i32 = 1;
pub const FFTBLOCKPAD_DEFAULT: i32 = 1;
pub const FFTBLOCK: i32 = FFTBLOCK_DEFAULT;
pub const FFTBLOCKPAD: i32 = FFTBLOCKPAD_DEFAULT;

pub const SEED: f64 = 314159265.0;
pub const A: f64 = 1220703125.0;
pub const PI: f64 = 3.141592653589793238;
pub const ALPHA: f64 = 1.0e-6;
pub const T_TOTAL: usize = 1;
pub const T_SETUP: usize = 2;
pub const T_FFT: usize = 3;
pub const T_EVOLVE: usize = 4;
pub const T_CHECKSUM: usize = 5;
pub const T_FFTX: usize = 6;
pub const T_FFTY: usize = 7;
pub const T_FFTZ: usize = 8;
pub const T_MAX: usize = 8;
pub const DBUG: bool = false;

pub const EPSILON: f64 = 1.0e-12;
pub const NTOTAL: f64 = NX as f64 * NY as f64 * NZ as f64;
pub const MAXDIM: i32 = {
    let mut x = NX;
    if x < NY {
        x = NY
    }
    if x < NZ {
        x = NZ
    }
    x
};

/* ft */
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub fn main_core() {
    let mut sums: Vec<Dcomplex> = vec![
        Dcomplex {
            real: 0.0,
            imag: 0.0
        };
        NITER as usize + 1
    ];
    let mut twiddle: Vec<[[f64; NX as usize]; NY as usize]> =
        vec![[[0.0; NX as usize]; NY as usize]; NZ as usize];
    let mut u: Vec<Dcomplex> = vec![
        Dcomplex {
            real: 0.0,
            imag: 0.0
        };
        MAXDIM as usize
    ];
    let mut u0: Vec<[[Dcomplex; NX as usize]; NY as usize]> = vec![
        [[Dcomplex {
            real: 0.0,
            imag: 0.0
        }; NX as usize];
            NY as usize];
        NZ as usize
    ];
    let mut u1: Vec<[[Dcomplex; NX as usize]; NY as usize]> = vec![
        [[Dcomplex {
            real: 0.0,
            imag: 0.0
        }; NX as usize];
            NY as usize];
        NZ as usize
    ];
    let mut dims: Vec<i32> = vec![0; 3];

    let (time, mops): (f64, f64);
    let mut verified: i8 = 0;

    /*
     * ---------------------------------------------------------------------
     * run the entire problem once to make sure all data is touched.
     * this reduces variable startup costs, which is important for such a
     * short benchmark. the other NPB 2 implementations are similar.
     * ---------------------------------------------------------------------
     */
    let mut timers = Timer::new();
    for i in 0..T_MAX {
        timers.clear(i);
    }
    setup(&mut dims[..]);
    init_ui(&mut u0[..], &mut u1[..], &mut twiddle[..], dims[0], dims[1]);
    compute_indexmap(&mut twiddle[..], dims[0], dims[1], dims[2]);
    compute_initial_conditions(&mut u1[..], dims[1]);
    fft_init(MAXDIM, &mut u[..]);
    fft(
        1,
        &mut u1[..],
        &mut u0[..],
        dims[0],
        dims[1],
        dims[2],
        &u[..],
        &mut timers,
    );

    /*
     * ---------------------------------------------------------------------
     * start over from the beginning. note that all operations must
     * be timed, in contrast to other benchmarks.
     * ---------------------------------------------------------------------
     */
    for i in 0..T_MAX {
        timers.clear(i);
    }

    timers.start(T_TOTAL);
    if TIMERS {
        timers.start(T_SETUP);
    }

    compute_indexmap(&mut twiddle[..], dims[0], dims[1], dims[2]);

    compute_initial_conditions(&mut u1[..], dims[1]);

    fft_init(MAXDIM, &mut u[..]);

    if TIMERS {
        timers.stop(T_SETUP);
    }
    if TIMERS {
        timers.start(T_FFT);
    }
    fft(
        1,
        &mut u1[..],
        &mut u0[..],
        dims[0],
        dims[1],
        dims[2],
        &mut u[..],
        &mut timers,
    );
    if TIMERS {
        timers.stop(T_FFT);
    }

    for iter in 1..NITER + 1 {
        if TIMERS {
            timers.start(T_EVOLVE);
        }
        evolve(&mut u0[..], &mut u1[..], &mut twiddle[..], dims[0], dims[1]);
        if TIMERS {
            timers.stop(T_EVOLVE);
        }
        if TIMERS {
            timers.start(T_FFT);
        }
        fft(
            -1,
            &mut u1[..],
            &mut u0[..],
            dims[0],
            dims[1],
            dims[2],
            &mut u[..],
            &mut timers,
        );
        if TIMERS {
            timers.stop(T_FFT);
        }
        if TIMERS {
            timers.start(T_CHECKSUM);
        }
        checksum(iter, &mut u1, &mut sums[..]);
        if TIMERS {
            timers.stop(T_CHECKSUM);
        }
    }

    verify(&mut verified, &mut sums[..]);
    timers.stop(T_TOTAL);
    time = timers.read(T_TOTAL).as_secs_f64();

    if time != 0.0 {
        mops = 1.0e-6
            * NTOTAL
            * (14.8157
                + 7.19641 * f64::ln(NTOTAL)
                + (5.23518 + 7.21113 * f64::ln(NTOTAL)) * NITER as f64)
            / time;
    } else {
        mops = 0.0;
    }

    let info = PrintInfo {
        name: String::from("FT"),
        class: CLASS.to_string(),
        size: (NX as usize, NY as usize, NZ as usize),
        num_iter: NITER,
        time,
        mops,
        operation: String::from("Floating point"),
        verified,
        num_threads: 1,
        //uns: UNSAFE
    };
    printer(info);

    /*
     * ---------------------------------------------------------------------
     * more timers
     * ---------------------------------------------------------------------
     */
    if TIMERS {
        let mut tstrings: Vec<String> = vec![String::new(); T_MAX + 1];
        tstrings[1] = ("          total ").to_string();
        tstrings[2] = ("          setup ").to_string();
        tstrings[3] = ("            fft ").to_string();
        tstrings[4] = ("         evolve ").to_string();
        tstrings[5] = ("       checksum ").to_string();
        tstrings[6] = ("           fftx ").to_string();
        tstrings[7] = ("           ffty ").to_string();
        tstrings[8] = ("           fftz ").to_string();

        let mut t_m = timers.read(T_TOTAL).as_secs_f64();
        if t_m <= 0.0 {
            t_m = 1.0;
        }
        for i in 1..T_MAX + 1 {
            let t_i = timers.read(i).as_secs_f64();
            print!(
                " timer {:>2}({:>16}) :{:>9.4} ({:>6.2}%)\n",
                i,
                tstrings[i],
                t_i,
                t_i * 100.0 / t_m
            );
        }
    }
}

fn cffts1(
    is: i32,
    d1: i32,
    d2: i32,
    d3: i32,
    xout: &mut [[[Dcomplex; NX as usize]; NY as usize]],
    y1: &mut [[Dcomplex; FFTBLOCKPAD as usize]],
    y2: &mut [[Dcomplex; FFTBLOCKPAD as usize]],
    u: &[Dcomplex],
    timers: &mut Timer,
) {
    let logd1: i32 = ilog2(d1);

    if TIMERS {
        timers.start(T_FFTX);
    }
    for k in 0..d3 as usize {
        for jj in (0..(d2 - FFTBLOCK) as usize + 1).step_by(FFTBLOCK as usize) {
            for j in 0..FFTBLOCK as usize {
                for i in 0..d1 as usize {
                    if UNSAFE {
                        unsafe {
                            *y1.get_unchecked_mut(i).get_unchecked_mut(j) =
                                *xout.get_unchecked(k).get_unchecked(j + jj).get_unchecked(i);
                        }
                    } else {
                        y1[i][j] = xout[k][j + jj][i];
                    }
                }
            }
            cfftz(is, logd1, d1, y1, y2, u);
            for j in 0..FFTBLOCK as usize {
                for i in 0..d1 as usize {
                    if UNSAFE {
                        unsafe {
                            *xout
                                .get_unchecked_mut(k)
                                .get_unchecked_mut(j + jj)
                                .get_unchecked_mut(i) = *y1.get_unchecked(i).get_unchecked(j);
                        }
                    } else {
                        xout[k][j + jj][i] = y1[i][j];
                    }
                }
            }
        }
    }
    if TIMERS {
        timers.stop(T_FFTX);
    }
}

fn cffts2(
    is: i32,
    d1: i32,
    d2: i32,
    d3: i32,
    xout: &mut [[[Dcomplex; NX as usize]; NY as usize]],
    y1: &mut [[Dcomplex; FFTBLOCKPAD as usize]],
    y2: &mut [[Dcomplex; FFTBLOCKPAD as usize]],
    u: &[Dcomplex],
    timers: &mut Timer,
) {
    let logd2: i32 = ilog2(d2);

    if TIMERS {
        timers.start(T_FFTY);
    }
    for k in 0..d3 as usize {
        for ii in (0..(d1 - FFTBLOCK) as usize + 1).step_by(FFTBLOCK as usize) {
            for j in 0..d2 as usize {
                for i in 0..FFTBLOCK as usize {
                    if UNSAFE {
                        unsafe {
                            *y1.get_unchecked_mut(j).get_unchecked_mut(i) =
                                *xout.get_unchecked(k).get_unchecked(j).get_unchecked(i + ii);
                        }
                    } else {
                        y1[j][i] = xout[k][j][i + ii];
                    }
                }
            }
            cfftz(is, logd2, d2, y1, y2, u);
            for j in 0..d2 as usize {
                for i in 0..FFTBLOCK as usize {
                    if UNSAFE {
                        unsafe {
                            *xout
                                .get_unchecked_mut(k)
                                .get_unchecked_mut(j)
                                .get_unchecked_mut(i + ii) = *y1.get_unchecked(j).get_unchecked(i);
                        }
                    } else {
                        xout[k][j][i + ii] = y1[j][i];
                    }
                }
            }
        }
    }
    if TIMERS {
        timers.stop(T_FFTY);
    }
}

fn cffts3(
    is: i32,
    d1: i32,
    d2: i32,
    d3: i32,
    xout: &mut [[[Dcomplex; NX as usize]; NY as usize]],
    y1: &mut [[Dcomplex; FFTBLOCKPAD as usize]],
    y2: &mut [[Dcomplex; FFTBLOCKPAD as usize]],
    u: &[Dcomplex],
    timers: &mut Timer,
) {
    let logd3 = ilog2(d3);

    if TIMERS {
        timers.start(T_FFTZ);
    }
    for j in 0..d2 as usize {
        for ii in (0..(d1 - FFTBLOCK) as usize + 1).step_by(FFTBLOCK as usize) {
            for k in 0..d3 as usize {
                for i in 0..FFTBLOCK as usize {
                    if UNSAFE {
                        unsafe {
                            *y1.get_unchecked_mut(k).get_unchecked_mut(i) =
                                *xout.get_unchecked(k).get_unchecked(j).get_unchecked(i + ii);
                        }
                    } else {
                        y1[k][i] = xout[k][j][i + ii];
                    }
                }
            }
            cfftz(is, logd3, d3, y1, y2, u);
            for k in 0..d3 as usize {
                for i in 0..FFTBLOCK as usize {
                    if UNSAFE {
                        unsafe {
                            *xout
                                .get_unchecked_mut(k)
                                .get_unchecked_mut(j)
                                .get_unchecked_mut(i + ii) = *y1.get_unchecked(k).get_unchecked(i);
                        }
                    } else {
                        xout[k][j][i + ii] = y1[k][i];
                    }
                }
            }
        }
    }
    if TIMERS {
        timers.stop(T_FFTZ);
    }
}

fn cffts3_2(
    is: i32,
    d1: i32,
    d2: i32,
    d3: i32,
    x: &[[[Dcomplex; NX as usize]; NY as usize]],
    xout: &mut [[[Dcomplex; NX as usize]; NY as usize]],
    y1: &mut [[Dcomplex; FFTBLOCKPAD as usize]],
    y2: &mut [[Dcomplex; FFTBLOCKPAD as usize]],
    u: &[Dcomplex],
    timers: &mut Timer,
) {
    let logd3 = ilog2(d3);

    if TIMERS {
        timers.start(T_FFTZ);
    }
    for j in 0..d2 as usize {
        for ii in (0..(d1 - FFTBLOCK) as usize + 1).step_by(FFTBLOCK as usize) {
            for k in 0..d3 as usize {
                for i in 0..FFTBLOCK as usize {
                    if UNSAFE {
                        unsafe {
                            *y1.get_unchecked_mut(k).get_unchecked_mut(i) =
                                *x.get_unchecked(k).get_unchecked(j).get_unchecked(i + ii);
                        }
                    } else {
                        y1[k][i] = x[k][j][i + ii];
                    }
                }
            }
            cfftz(is, logd3, d3, y1, y2, u);
            for k in 0..d3 as usize {
                for i in 0..FFTBLOCK as usize {
                    if UNSAFE {
                        unsafe {
                            *xout
                                .get_unchecked_mut(k)
                                .get_unchecked_mut(j)
                                .get_unchecked_mut(i + ii) = *y1.get_unchecked(k).get_unchecked(i);
                        }
                    } else {
                        xout[k][j][i + ii] = y1[k][i];
                    }
                }
            }
        }
    }
    if TIMERS {
        timers.stop(T_FFTZ);
    }
}

/*
 * ---------------------------------------------------------------------
 * computes NY N-point complex-to-complex FFTs of X using an algorithm due
 * to swarztrauber. X is both the input and the output array, while Y is a
 * scratch array. it is assumed that N = 2^M. before calling CFFTZ to
 * perform FFTs, the array U must be initialized by calling CFFTZ with is
 * set to 0 and M set to MX, where MX is the maximum value of M for any
 * subsequent call.
 * ---------------------------------------------------------------------
 */
fn cfftz(
    is: i32,
    m: i32,
    n: i32,
    x: &mut [[Dcomplex; FFTBLOCKPAD as usize]],
    y: &mut [[Dcomplex; FFTBLOCKPAD as usize]],
    u: &[Dcomplex],
) {
    /*
     * ---------------------------------------------------------------------
     * check if input parameters are invalid.
     * ---------------------------------------------------------------------
     */
    let mx: i32 = u[0].real as i32;
    if (is != 1 && is != -1) || m < 1 || m > mx {
        println!("CFFTZ: Either U has not been initialized, or else one of the input parameters is invalid {} {} {}", is, m, mx);
        std::process::exit(0);
    }

    /*
     * ---------------------------------------------------------------------
     * perform one variant of the Stockham FFT.
     * ---------------------------------------------------------------------
     */
    for l in (1..m + 1).step_by(2) {
        fftz2(is, l, m, n, FFTBLOCK, u, x, y);
        if l == m {
            /*
             * ---------------------------------------------------------------------
             * copy Y to X.
             * ---------------------------------------------------------------------
             */
            x.copy_from_slice(y);
            break;
        }
        fftz2(is, l + 1, m, n, FFTBLOCK, u, y, x);
    }
}

fn checksum(i: i32, u1: &mut [[[Dcomplex; NX as usize]; NY as usize]], sums: &mut [Dcomplex]) {
    let (mut q, mut r, mut s): (i32, i32, i32);
    let mut chk = Dcomplex::dcomplex_create(&0.0, &0.0);
    for j in 1..1025 {
        q = j % NX;
        r = (3 * j) % NY;
        s = (5 * j) % NZ;
        chk = Dcomplex::dcomplex_add(&chk, &u1[s as usize][r as usize][q as usize]);
    }
    chk = Dcomplex::dcomplex_div2(&chk, &NTOTAL);
    println!(
        " T ={:>5}     Checksum = {:>22.12e} {:>22.12e}",
        i, chk.real, chk.imag
    );
    sums[i as usize] = chk;
}

/*
 * compute function from local (i,j,k) to ibar^2+jbar^2+kbar^2
 * for time evolution exponent.
 */
fn compute_indexmap(twiddle: &mut [[[f64; NX as usize]; NY as usize]], d1: i32, d2: i32, d3: i32) {
    /*
     * ---------------------------------------------------------------------
     * basically we want to convert the fortran indices
     * 1 2 3 4 5 6 7 8
     * to
     * 0 1 2 3 -4 -3 -2 -1
     * the following magic formula does the trick:
     * mod(i-1+n/2, n) - n/2
     * ---------------------------------------------------------------------
     */
    let ap: f64 = -4.0 * ALPHA * PI * PI;
    (0..d3)
        .into_iter()
        .zip(&mut twiddle[..])
        .for_each(|(k, twiddle)| {
            let kk = ((k + (NZ >> 1)) % NZ) - (NZ >> 1);
            let kk2 = kk * kk;
            for j in 0..d2 {
                let jj = ((j + (NY >> 1)) % NY) - (NY >> 1);
                let kj2 = jj * jj + kk2;
                for i in 0..d1 {
                    let ii = ((i + (NX >> 1)) % NX) - (NX >> 1);
                    twiddle[j as usize][i as usize] = (ap * (ii * ii + kj2) as f64).exp();
                }
            }
        });
}

/*
 * ---------------------------------------------------------------------
 * fill in array u0 with initial conditions from
 * random number generator
 * ---------------------------------------------------------------------
 */
fn compute_initial_conditions(u0: &mut [[[Dcomplex; NX as usize]; NY as usize]], d2: i32) {
    let (mut start, mut an): (f64, f64);
    let mut starts: [f64; NZ as usize] = [0.0; NZ as usize];
    start = SEED;
    an = 0.0;

    /*
     * ---------------------------------------------------------------------
     * jump to the starting element for our first plane.
     * ---------------------------------------------------------------------
     */
    ipow46(A, 0, &mut an);
    randlc(&mut start, an);
    ipow46(A, (NX << 1) * NY, &mut an);

    starts[0] = start;
    (starts[1..]).iter_mut().for_each(|starts| {
        randlc(&mut start, an);
        *starts = start;
    });

    /*
     * ---------------------------------------------------------------------
     * go through by z planes filling in one square at a time.
     * ---------------------------------------------------------------------
     */
    (&mut u0[..])
        .into_iter()
        .zip(&starts[..])
        .for_each(|(u0, starts)| {
            let mut x0 = *starts;
            for j in 0..d2 as usize {
                vranlc_dcomplex(NX, &mut x0, A, &mut u0[j]);
            }
        });
}

/*
 * ---------------------------------------------------------------------
 * evolve u0 -> u1 (t time steps) in fourier space
 * ---------------------------------------------------------------------
 */
fn evolve(
    u0: &mut [[[Dcomplex; NX as usize]; NY as usize]],
    u1: &mut [[[Dcomplex; NX as usize]; NY as usize]],
    twiddle: &mut [[[f64; NX as usize]; NY as usize]],
    d1: i32,
    d2: i32,
) {
    (&mut u0[..])
        .into_iter()
        .zip(&mut u1[..])
        .zip(&twiddle[..])
        .for_each(|((u0, u1), twiddle)| {
            for j in 0..d2 as usize {
                for i in 0..d1 as usize {
                    u0[j][i] = Dcomplex::dcomplex_mul2(&u0[j][i], &twiddle[j][i]);
                    u1[j][i] = u0[j][i];
                }
            }
        });
}

fn fft(
    dir: i32,
    x1: &mut [[[Dcomplex; NX as usize]; NY as usize]],
    x2: &mut [[[Dcomplex; NX as usize]; NY as usize]],
    d1: i32,
    d2: i32,
    d3: i32,
    u: &[Dcomplex],
    timers: &mut Timer,
) {
    let mut y1: [[Dcomplex; FFTBLOCKPAD as usize]; MAXDIM as usize] = [[Dcomplex {
        real: 0.0,
        imag: 0.0,
    }; FFTBLOCKPAD as usize];
        MAXDIM as usize];
    let mut y2: [[Dcomplex; FFTBLOCKPAD as usize]; MAXDIM as usize] = [[Dcomplex {
        real: 0.0,
        imag: 0.0,
    }; FFTBLOCKPAD as usize];
        MAXDIM as usize];

    /*
     * ---------------------------------------------------------------------
     * note: args x1, x2 must be different arrays
     * note: args for cfftsx are (direction, layout, xin, xout, scratch)
     * xin/xout may be the same and it can be somewhat faster
     * if they are
     * ---------------------------------------------------------------------
     */

    if dir == 1 {
        cffts1(1, d1, d2, d3, x1, &mut y1[..], &mut y2[..], u, timers);
        cffts2(1, d1, d2, d3, x1, &mut y1[..], &mut y2[..], u, timers);
        cffts3_2(1, d1, d2, d3, x1, x2, &mut y1[..], &mut y2[..], u, timers);
    } else {
        cffts3(-1, d1, d2, d3, x1, &mut y1[..], &mut y2[..], u, timers);
        cffts2(-1, d1, d2, d3, x1, &mut y1[..], &mut y2[..], u, timers);
        cffts1(-1, d1, d2, d3, x1, &mut y1[..], &mut y2[..], u, timers);
    }
}

fn fft_init(n: i32, u: &mut [Dcomplex]) {
    let mut t: f64;
    /*
     * ---------------------------------------------------------------------
     * initialize the U array with sines and cosines in a manner that permits
     * stride one access at each FFT iteration.
     * ---------------------------------------------------------------------
     */
    let m = ilog2(n);
    u[0] = Dcomplex::dcomplex_create(&(m as f64), &0.0);
    let mut ku = 2;
    let mut ln = 1;

    for _ in 0..m {
        t = PI / ln as f64;

        (0..ln)
            .into_iter()
            .zip(&mut u[ku as usize - 1..(ln + ku - 1) as usize])
            .for_each(|(i, u)| {
                let ti = i as f64 * t;
                *u = Dcomplex::dcomplex_create(&(ti.cos()), &(ti.sin()));
            });

        ku = ku + ln;
        ln = ln << 1;
    }
}

/*
 * ---------------------------------------------------------------------
 * performs the l-th iteration of the second variant of the stockham FFT
 * ---------------------------------------------------------------------
 */
fn fftz2(
    is: i32,
    l: i32,
    m: i32,
    n: i32,
    ny: i32,
    u: &[Dcomplex],
    x: &mut [[Dcomplex; FFTBLOCKPAD as usize]],
    y: &mut [[Dcomplex; FFTBLOCKPAD as usize]],
) {
    /*
     * ---------------------------------------------------------------------
     * set initial parameters.
     * ---------------------------------------------------------------------
     */
    let n1 = (n >> 1) as usize;
    let lk = (1 << (l - 1)) as usize;
    let li = (1 << (m - l)) as usize;
    let lj = lk << 1;
    let ku = li;

    (0..li).into_iter().zip(&u[ku..ku + li]).for_each(|(i, u)| {
        let i11 = i * lk;
        let i12 = i11 + n1;
        let i21 = i * lj;
        let i22 = i21 + lk;
        let u1: Dcomplex = if is >= 1 { *u } else { Dcomplex::dconjg(&u) };

        /*
         * ---------------------------------------------------------------------
         * this loop is vectorizable.
         * ---------------------------------------------------------------------
         */
        for k in 0..lk {
            for j in 0..ny as usize {
                if UNSAFE {
                    unsafe {
                        let x11 = *x.get_unchecked(i11 + k).get_unchecked(j);
                        let x21 = *x.get_unchecked(i12 + k).get_unchecked(j);
                        *y.get_unchecked_mut(i21 + k).get_unchecked_mut(j) =
                            Dcomplex::dcomplex_add(&x11, &x21);
                        *y.get_unchecked_mut(i22 + k).get_unchecked_mut(j) =
                            Dcomplex::dcomplex_mul(&u1, &Dcomplex::dcomplex_sub(&x11, &x21));
                    }
                } else {
                    let x11 = x[i11 + k][j];
                    let x21 = x[i12 + k][j];
                    y[i21 + k][j] = Dcomplex::dcomplex_add(&x11, &x21);
                    y[i22 + k][j] =
                        Dcomplex::dcomplex_mul(&u1, &Dcomplex::dcomplex_sub(&x11, &x21));
                }
            }
        }
    });
}

fn ilog2(n: i32) -> i32 {
    let (mut nn, mut lg): (i32, i32);
    if n == 1 {
        return 0;
    }
    lg = 1;
    nn = 2;
    while nn < n {
        nn = nn << 1;
        lg += 1;
    }
    return lg;
}

/*
 * ---------------------------------------------------------------------
 * touch all the big data
 * ---------------------------------------------------------------------
 */
fn init_ui(
    u0: &mut [[[Dcomplex; NX as usize]; NY as usize]],
    u1: &mut [[[Dcomplex; NX as usize]; NY as usize]],
    twiddle: &mut [[[f64; NX as usize]; NY as usize]],
    d1: i32,
    d2: i32,
) {
    (&mut u0[..])
        .into_iter()
        .zip(&mut u1[..])
        .zip(&mut twiddle[..])
        .for_each(|((u0, u1), twiddle)| {
            for j in 0..d2 as usize {
                for i in 0..d1 as usize {
                    u0[j][i] = Dcomplex::dcomplex_create(&0.0, &0.0);
                    u1[j][i] = Dcomplex::dcomplex_create(&0.0, &0.0);
                    twiddle[j][i] = 0.0;
                }
            }
        });
}

fn setup(dims: &mut [i32]) {
    println!("\n\n NAS Parallel Benchmarks 4.1 Serial Rust version - FT Benchmark\n");
    println!(" Size                : {NX:>4}  x{NY:>4}  x{NZ:>4}");
    println!(" Iterations          :{:>7}", NITER);
    println!("");

    dims[0] = NX;
    dims[1] = NY;
    dims[2] = NZ;

    /*
     * ---------------------------------------------------------------------
     * set up info for blocking of ffts and transposes. this improves
     * performance on cache-based systems. blocking involves
     * working on a chunk of the problem at a time, taking chunks
     * along the first, second, or third dimension.
     *
     * - in cffts1 blocking is on 2nd dimension (with fft on 1st dim)
     * - in cffts2/3 blocking is on 1st dimension (with fft on 2nd and 3rd dims)
     *
     * since 1st dim is always in processor, we'll assume it's long enough
     * (default blocking factor is 16 so min size for 1st dim is 16)
     * the only case we have to worry about is cffts1 in a 2d decomposition.
     * so the blocking factor should not be larger than the 2nd dimension.
     * ---------------------------------------------------------------------
     */
    /* block values were already set */
    /* fftblock = FFTBLOCK_DEFAULT; */
    /* fftblockpad = FFTBLOCKPAD_DEFAULT; */
    /* if(fftblock!=FFTBLOCK_DEFAULT){fftblockpad=fftblock+3;} */
}

fn ipow46(a: f64, exponent: i32, result: &mut f64) {
    let (mut q, mut r): (f64, f64);
    let (mut n, mut n2): (i32, i32);

    /*
     * --------------------------------------------------------------------
     * use
     * a^n = a^(n/2)*a^(n/2) if n even else
     * a^n = a*a^(n-1)       if n odd
     * -------------------------------------------------------------------
     */
    *result = 1.0;
    if exponent == 0 {
        return;
    }
    q = a;
    r = 1.0;
    n = exponent;

    while n > 1 {
        n2 = n >> 1;
        if n2 * 2 == n {
            let q_aux: f64 = q;
            randlc(&mut q, q_aux);
            n = n2;
        } else {
            randlc(&mut r, q);
            n = n - 1;
        }
    }
    randlc(&mut r, q);
    *result = r;
}

fn verify(verified: &mut i8, sums: &mut [Dcomplex]) {
    let mut err: f64;

    /*
     * ---------------------------------------------------------------------
     * reference checksums
     * ---------------------------------------------------------------------
     */
    let mut csum_ref: [Dcomplex; 25 + 1] = [Dcomplex {
        real: 0.0,
        imag: 0.0,
    }; 25 + 1];

    if CLASS == 'S' {
        /*
         * ---------------------------------------------------------------------
         * sample size reference checksums
         * ---------------------------------------------------------------------
         */
        csum_ref[1] = Dcomplex::dcomplex_create(&5.546087004964E+02, &4.845363331978E+02);
        csum_ref[2] = Dcomplex::dcomplex_create(&5.546385409189E+02, &4.865304269511E+02);
        csum_ref[3] = Dcomplex::dcomplex_create(&5.546148406171E+02, &4.883910722336E+02);
        csum_ref[4] = Dcomplex::dcomplex_create(&5.545423607415E+02, &4.901273169046E+02);
        csum_ref[5] = Dcomplex::dcomplex_create(&5.544255039624E+02, &4.917475857993E+02);
        csum_ref[6] = Dcomplex::dcomplex_create(&5.542683411902E+02, &4.932597244941E+02);
    } else if CLASS == 'Z' {
        /*
         * ---------------------------------------------------------------------
         * sample size reference checksums
         * ---------------------------------------------------------------------
         */
        csum_ref[1] = Dcomplex::dcomplex_create(&6.196115311106e2, &4.518706877572e2);
        csum_ref[2] = Dcomplex::dcomplex_create(&6.195552278027e2, &4.518814052791e2);
    } else if CLASS == 'W' {
        /*
         * ---------------------------------------------------------------------
         * class_npb W size reference checksums
         * ---------------------------------------------------------------------
         */
        csum_ref[1] = Dcomplex::dcomplex_create(&5.673612178944E+02, &5.293246849175E+02);
        csum_ref[2] = Dcomplex::dcomplex_create(&5.631436885271E+02, &5.282149986629E+02);
        csum_ref[3] = Dcomplex::dcomplex_create(&5.594024089970E+02, &5.270996558037E+02);
        csum_ref[4] = Dcomplex::dcomplex_create(&5.560698047020E+02, &5.260027904925E+02);
        csum_ref[5] = Dcomplex::dcomplex_create(&5.530898991250E+02, &5.249400845633E+02);
        csum_ref[6] = Dcomplex::dcomplex_create(&5.504159734538E+02, &5.239212247086E+02);
    } else if CLASS == 'A' {
        /*
         * ---------------------------------------------------------------------
         * class_npb A size reference checksums
         * ---------------------------------------------------------------------
         */
        csum_ref[1] = Dcomplex::dcomplex_create(&5.046735008193E+02, &5.114047905510E+02);
        csum_ref[2] = Dcomplex::dcomplex_create(&5.059412319734E+02, &5.098809666433E+02);
        csum_ref[3] = Dcomplex::dcomplex_create(&5.069376896287E+02, &5.098144042213E+02);
        csum_ref[4] = Dcomplex::dcomplex_create(&5.077892868474E+02, &5.101336130759E+02);
        csum_ref[5] = Dcomplex::dcomplex_create(&5.085233095391E+02, &5.104914655194E+02);
        csum_ref[6] = Dcomplex::dcomplex_create(&5.091487099959E+02, &5.107917842803E+02);
    } else if CLASS == 'B' {
        /*
         * --------------------------------------------------------------------
         * class_npb B size reference checksums
         * ---------------------------------------------------------------------
         */
        csum_ref[1] = Dcomplex::dcomplex_create(&5.177643571579E+02, &5.077803458597E+02);
        csum_ref[2] = Dcomplex::dcomplex_create(&5.154521291263E+02, &5.088249431599E+02);
        csum_ref[3] = Dcomplex::dcomplex_create(&5.146409228649E+02, &5.096208912659E+02);
        csum_ref[4] = Dcomplex::dcomplex_create(&5.142378756213E+02, &5.101023387619E+02);
        csum_ref[5] = Dcomplex::dcomplex_create(&5.139626667737E+02, &5.103976610617E+02);
        csum_ref[6] = Dcomplex::dcomplex_create(&5.137423460082E+02, &5.105948019802E+02);
        csum_ref[7] = Dcomplex::dcomplex_create(&5.135547056878E+02, &5.107404165783E+02);
        csum_ref[8] = Dcomplex::dcomplex_create(&5.133910925466E+02, &5.108576573661E+02);
        csum_ref[9] = Dcomplex::dcomplex_create(&5.132470705390E+02, &5.109577278523E+02);
        csum_ref[10] = Dcomplex::dcomplex_create(&5.131197729984E+02, &5.110460304483E+02);
        csum_ref[11] = Dcomplex::dcomplex_create(&5.130070319283E+02, &5.111252433800E+02);
        csum_ref[12] = Dcomplex::dcomplex_create(&5.129070537032E+02, &5.111968077718E+02);
        csum_ref[13] = Dcomplex::dcomplex_create(&5.128182883502E+02, &5.112616233064E+02);
        csum_ref[14] = Dcomplex::dcomplex_create(&5.127393733383E+02, &5.113203605551E+02);
        csum_ref[15] = Dcomplex::dcomplex_create(&5.126691062020E+02, &5.113735928093E+02);
        csum_ref[16] = Dcomplex::dcomplex_create(&5.126064276004E+02, &5.114218460548E+02);
        csum_ref[17] = Dcomplex::dcomplex_create(&5.125504076570E+02, &5.114656139760E+02);
        csum_ref[18] = Dcomplex::dcomplex_create(&5.125002331720E+02, &5.115053595966E+02);
        csum_ref[19] = Dcomplex::dcomplex_create(&5.124551951846E+02, &5.115415130407E+02);
        csum_ref[20] = Dcomplex::dcomplex_create(&5.124146770029E+02, &5.115744692211E+02);
    } else if CLASS == 'C' {
        /*
         * ---------------------------------------------------------------------
         * class_npb C size reference checksums
         * ---------------------------------------------------------------------
         */
        csum_ref[1] = Dcomplex::dcomplex_create(&5.195078707457E+02, &5.149019699238E+02);
        csum_ref[2] = Dcomplex::dcomplex_create(&5.155422171134E+02, &5.127578201997E+02);
        csum_ref[3] = Dcomplex::dcomplex_create(&5.144678022222E+02, &5.122251847514E+02);
        csum_ref[4] = Dcomplex::dcomplex_create(&5.140150594328E+02, &5.121090289018E+02);
        csum_ref[5] = Dcomplex::dcomplex_create(&5.137550426810E+02, &5.121143685824E+02);
        csum_ref[6] = Dcomplex::dcomplex_create(&5.135811056728E+02, &5.121496764568E+02);
        csum_ref[7] = Dcomplex::dcomplex_create(&5.134569343165E+02, &5.121870921893E+02);
        csum_ref[8] = Dcomplex::dcomplex_create(&5.133651975661E+02, &5.122193250322E+02);
        csum_ref[9] = Dcomplex::dcomplex_create(&5.132955192805E+02, &5.122454735794E+02);
        csum_ref[10] = Dcomplex::dcomplex_create(&5.132410471738E+02, &5.122663649603E+02);
        csum_ref[11] = Dcomplex::dcomplex_create(&5.131971141679E+02, &5.122830879827E+02);
        csum_ref[12] = Dcomplex::dcomplex_create(&5.131605205716E+02, &5.122965869718E+02);
        csum_ref[13] = Dcomplex::dcomplex_create(&5.131290734194E+02, &5.123075927445E+02);
        csum_ref[14] = Dcomplex::dcomplex_create(&5.131012720314E+02, &5.123166486553E+02);
        csum_ref[15] = Dcomplex::dcomplex_create(&5.130760908195E+02, &5.123241541685E+02);
        csum_ref[16] = Dcomplex::dcomplex_create(&5.130528295923E+02, &5.123304037599E+02);
        csum_ref[17] = Dcomplex::dcomplex_create(&5.130310107773E+02, &5.123356167976E+02);
        csum_ref[18] = Dcomplex::dcomplex_create(&5.130103090133E+02, &5.123399592211E+02);
        csum_ref[19] = Dcomplex::dcomplex_create(&5.129905029333E+02, &5.123435588985E+02);
        csum_ref[20] = Dcomplex::dcomplex_create(&5.129714421109E+02, &5.123465164008E+02);
    } else if CLASS == 'D' {
        /*
         * ---------------------------------------------------------------------
         * class_npb D size reference checksums
         * ---------------------------------------------------------------------
         */
        csum_ref[1] = Dcomplex::dcomplex_create(&5.122230065252E+02, &5.118534037109E+02);
        csum_ref[2] = Dcomplex::dcomplex_create(&5.120463975765E+02, &5.117061181082E+02);
        csum_ref[3] = Dcomplex::dcomplex_create(&5.119865766760E+02, &5.117096364601E+02);
        csum_ref[4] = Dcomplex::dcomplex_create(&5.119518799488E+02, &5.117373863950E+02);
        csum_ref[5] = Dcomplex::dcomplex_create(&5.119269088223E+02, &5.117680347632E+02);
        csum_ref[6] = Dcomplex::dcomplex_create(&5.119082416858E+02, &5.117967875532E+02);
        csum_ref[7] = Dcomplex::dcomplex_create(&5.118943814638E+02, &5.118225281841E+02);
        csum_ref[8] = Dcomplex::dcomplex_create(&5.118842385057E+02, &5.118451629348E+02);
        csum_ref[9] = Dcomplex::dcomplex_create(&5.118769435632E+02, &5.118649119387E+02);
        csum_ref[10] = Dcomplex::dcomplex_create(&5.118718203448E+02, &5.118820803844E+02);
        csum_ref[11] = Dcomplex::dcomplex_create(&5.118683569061E+02, &5.118969781011E+02);
        csum_ref[12] = Dcomplex::dcomplex_create(&5.118661708593E+02, &5.119098918835E+02);
        csum_ref[13] = Dcomplex::dcomplex_create(&5.118649768950E+02, &5.119210777066E+02);
        csum_ref[14] = Dcomplex::dcomplex_create(&5.118645605626E+02, &5.119307604484E+02);
        csum_ref[15] = Dcomplex::dcomplex_create(&5.118647586618E+02, &5.119391362671E+02);
        csum_ref[16] = Dcomplex::dcomplex_create(&5.118654451572E+02, &5.119463757241E+02);
        csum_ref[17] = Dcomplex::dcomplex_create(&5.118665212451E+02, &5.119526269238E+02);
        csum_ref[18] = Dcomplex::dcomplex_create(&5.118679083821E+02, &5.119580184108E+02);
        csum_ref[19] = Dcomplex::dcomplex_create(&5.118695433664E+02, &5.119626617538E+02);
        csum_ref[20] = Dcomplex::dcomplex_create(&5.118713748264E+02, &5.119666538138E+02);
        csum_ref[21] = Dcomplex::dcomplex_create(&5.118733606701E+02, &5.119700787219E+02);
        csum_ref[22] = Dcomplex::dcomplex_create(&5.118754661974E+02, &5.119730095953E+02);
        csum_ref[23] = Dcomplex::dcomplex_create(&5.118776626738E+02, &5.119755100241E+02);
        csum_ref[24] = Dcomplex::dcomplex_create(&5.118799262314E+02, &5.119776353561E+02);
        csum_ref[25] = Dcomplex::dcomplex_create(&5.118822370068E+02, &5.119794338060E+02);
    } else if CLASS == 'E' {
        /*
         * ---------------------------------------------------------------------
         * class_npb E size reference checksums
         * ---------------------------------------------------------------------
         */
        csum_ref[1] = Dcomplex::dcomplex_create(&5.121601045346E+02, &5.117395998266E+02);
        csum_ref[2] = Dcomplex::dcomplex_create(&5.120905403678E+02, &5.118614716182E+02);
        csum_ref[3] = Dcomplex::dcomplex_create(&5.120623229306E+02, &5.119074203747E+02);
        csum_ref[4] = Dcomplex::dcomplex_create(&5.120438418997E+02, &5.119345900733E+02);
        csum_ref[5] = Dcomplex::dcomplex_create(&5.120311521872E+02, &5.119551325550E+02);
        csum_ref[6] = Dcomplex::dcomplex_create(&5.120226088809E+02, &5.119720179919E+02);
        csum_ref[7] = Dcomplex::dcomplex_create(&5.120169296534E+02, &5.119861371665E+02);
        csum_ref[8] = Dcomplex::dcomplex_create(&5.120131225172E+02, &5.119979364402E+02);
        csum_ref[9] = Dcomplex::dcomplex_create(&5.120104767108E+02, &5.120077674092E+02);
        csum_ref[10] = Dcomplex::dcomplex_create(&5.120085127969E+02, &5.120159443121E+02);
        csum_ref[11] = Dcomplex::dcomplex_create(&5.120069224127E+02, &5.120227453670E+02);
        csum_ref[12] = Dcomplex::dcomplex_create(&5.120055158164E+02, &5.120284096041E+02);
        csum_ref[13] = Dcomplex::dcomplex_create(&5.120041820159E+02, &5.120331373793E+02);
        csum_ref[14] = Dcomplex::dcomplex_create(&5.120028605402E+02, &5.120370938679E+02);
        csum_ref[15] = Dcomplex::dcomplex_create(&5.120015223011E+02, &5.120404138831E+02);
        csum_ref[16] = Dcomplex::dcomplex_create(&5.120001570022E+02, &5.120432068837E+02);
        csum_ref[17] = Dcomplex::dcomplex_create(&5.119987650555E+02, &5.120455615860E+02);
        csum_ref[18] = Dcomplex::dcomplex_create(&5.119973525091E+02, &5.120475499442E+02);
        csum_ref[19] = Dcomplex::dcomplex_create(&5.119959279472E+02, &5.120492304629E+02);
        csum_ref[20] = Dcomplex::dcomplex_create(&5.119945006558E+02, &5.120506508902E+02);
        csum_ref[21] = Dcomplex::dcomplex_create(&5.119930795911E+02, &5.120518503782E+02);
        csum_ref[22] = Dcomplex::dcomplex_create(&5.119916728462E+02, &5.120528612016E+02);
        csum_ref[23] = Dcomplex::dcomplex_create(&5.119902874185E+02, &5.120537101195E+02);
        csum_ref[24] = Dcomplex::dcomplex_create(&5.119889291565E+02, &5.120544194514E+02);
        csum_ref[25] = Dcomplex::dcomplex_create(&5.119876028049E+02, &5.120550079284E+02);
    }
    if CLASS != 'U' {
        *verified = 1;
        for i in 1..NITER + 1 {
            err = Dcomplex::dcomplex_abs(&Dcomplex::dcomplex_div(
                &Dcomplex::dcomplex_sub(&sums[i as usize], &csum_ref[i as usize]),
                &csum_ref[i as usize],
            ));
            if !(err <= EPSILON) {
                *verified = 0;
                break;
            }
        }
        if *verified == 1 {
            println!(" Result verification successful");
        } else {
            println!(" Result verification failed");
        }
    }
    println!(" class_npb = {}", CLASS);
}
