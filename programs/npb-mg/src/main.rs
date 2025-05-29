#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

use npb_common::print_results::*;
use npb_common::randdp::*;
use npb_common::timers::*;

#[cfg(class = "Z")]
mod params {
    pub const CLASS: char = 'Z';
    pub const VERIFY_VALUE: f64 = 1.0425035497746e-1;
    pub const NX_DEFAULT: usize = 8;
    pub const NY_DEFAULT: usize = 8;
    pub const NZ_DEFAULT: usize = 8;
    pub const NIT_DEFAULT: usize = 1;
    pub const LM: usize = usize::ilog2(NX_DEFAULT) as usize;
    pub const LT_DEFAULT: usize = LM;
    pub const NDIM1: usize = LM;
    pub const NDIM2: usize = LM;
    pub const NDIM3: usize = LM;
}

#[cfg(class = "S")]
mod params {
    pub const CLASS: char = 'S';
    pub const VERIFY_VALUE: f64 = 0.5307707005734e-04;
    pub const NX_DEFAULT: usize = 32;
    pub const NY_DEFAULT: usize = 32;
    pub const NZ_DEFAULT: usize = 32;
    pub const NIT_DEFAULT: usize = 4;
    pub const LM: usize = usize::ilog2(NX_DEFAULT) as usize;
    pub const LT_DEFAULT: usize = LM;
    pub const NDIM1: usize = LM;
    pub const NDIM2: usize = LM;
    pub const NDIM3: usize = LM;
}

#[cfg(class = "W")]
mod params {
    pub const CLASS: char = 'W';
    pub const VERIFY_VALUE: f64 = 0.6467329375339e-05;
    pub const NX_DEFAULT: usize = 128;
    pub const NY_DEFAULT: usize = 128;
    pub const NZ_DEFAULT: usize = 128;
    pub const NIT_DEFAULT: usize = 4;
    pub const LM: usize = usize::ilog2(NX_DEFAULT) as usize;
    pub const LT_DEFAULT: usize = LM;
    pub const NDIM1: usize = LM;
    pub const NDIM2: usize = LM;
    pub const NDIM3: usize = LM;
}

#[cfg(class = "A")]
mod params {
    pub const CLASS: char = 'A';
    pub const VERIFY_VALUE: f64 = 0.2433365309069e-05;
    pub const NX_DEFAULT: usize = 256;
    pub const NY_DEFAULT: usize = 256;
    pub const NZ_DEFAULT: usize = 256;
    pub const NIT_DEFAULT: usize = 4;
    pub const LM: usize = usize::ilog2(NX_DEFAULT) as usize;
    pub const LT_DEFAULT: usize = LM;
    pub const NDIM1: usize = LM;
    pub const NDIM2: usize = LM;
    pub const NDIM3: usize = LM;
}

#[cfg(class = "B")]
mod params {
    pub const CLASS: char = 'B';
    pub const VERIFY_VALUE: f64 = 0.1800564401355e-05;
    pub const NX_DEFAULT: usize = 256;
    pub const NY_DEFAULT: usize = 256;
    pub const NZ_DEFAULT: usize = 256;
    pub const NIT_DEFAULT: usize = 20;
    pub const LM: usize = usize::ilog2(NX_DEFAULT) as usize;
    pub const LT_DEFAULT: usize = LM;
    pub const NDIM1: usize = LM;
    pub const NDIM2: usize = LM;
    pub const NDIM3: usize = LM;
}

#[cfg(class = "C")]
mod params {
    pub const CLASS: char = 'C';
    pub const VERIFY_VALUE: f64 = 0.5706732285740e-06;
    pub const NX_DEFAULT: usize = 512;
    pub const NY_DEFAULT: usize = 512;
    pub const NZ_DEFAULT: usize = 512;
    pub const NIT_DEFAULT: usize = 20;
    pub const LM: usize = usize::ilog2(NX_DEFAULT) as usize;
    pub const LT_DEFAULT: usize = LM;
    pub const NDIM1: usize = LM;
    pub const NDIM2: usize = LM;
    pub const NDIM3: usize = LM;
}

#[cfg(class = "D")]
mod params {
    pub const CLASS: char = 'D';
    pub const VERIFY_VALUE: f64 = 0.1583275060440e-09;
    pub const NX_DEFAULT: usize = 1024;
    pub const NY_DEFAULT: usize = 1024;
    pub const NZ_DEFAULT: usize = 1024;
    pub const NIT_DEFAULT: usize = 50;
    pub const LM: usize = usize::ilog2(NX_DEFAULT) as usize;
    pub const LT_DEFAULT: usize = LM;
    pub const NDIM1: usize = LM;
    pub const NDIM2: usize = LM;
    pub const NDIM3: usize = LM;
}

#[cfg(class = "E")]
mod params {
    pub const CLASS: char = 'E';
    pub const VERIFY_VALUE: f64 = 0.8157592357404e-10;
    pub const NX_DEFAULT: usize = 2048;
    pub const NY_DEFAULT: usize = 2048;
    pub const NZ_DEFAULT: usize = 2048;
    pub const NIT_DEFAULT: usize = 50;
    pub const LM: usize = usize::ilog2(NX_DEFAULT) as usize;
    pub const LT_DEFAULT: usize = LM;
    pub const NDIM1: usize = LM;
    pub const NDIM2: usize = LM;
    pub const NDIM3: usize = LM;
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
    pub const VERIFY_VALUE: f64 = 1.0;
    pub const NX_DEFAULT: usize = 1;
    pub const NY_DEFAULT: usize = 1;
    pub const NZ_DEFAULT: usize = 1;
    pub const NIT_DEFAULT: usize = 1;
    pub const LM: usize = 1;
    pub const LT_DEFAULT: usize = 1;
    pub const NDIM1: usize = 1;
    pub const NDIM2: usize = 1;
    pub const NDIM3: usize = 1;
    compile_error!(
        "\n\n\
		Must set a class at compilation time by setting RUSTFLAGS\n\
		class options for MG are: {S, W, A, B, C, D, E}\n\
		For example:\n\
		RUSTFLAGS='--cfg class=\"A\" ' cargo build --release --bin mg\n\n\n\
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

pub const NM: usize = 2 + (1 << LM); /* actual dimension including ghost cells for communications */
pub const NV: usize = (2 + (1 << NDIM1)) * (2 + (1 << NDIM2)) * (2 + (1 << NDIM3)); /* size of rhs array */
pub const NR: usize = ((NV + NM * NM + 5 * NM + 7 * LM + 6) / 7) * 8; /* size of residual array */
pub const MAXLEVEL: usize = LT_DEFAULT + 1; /* maximum number of levels */
pub const M: usize = NM + 1; /* set at m=1024, can handle cases up to 1024^3 case */
pub const MM: usize = 10;
pub const A: f64 = 1220703125.0; //f64::powf(5.0, 13.0);
pub const X: f64 = 314159265.0;
pub const T_INIT: usize = 0;
pub const T_BENCH: usize = 1;
pub const T_MG3P: usize = 2;
pub const T_PSINV: usize = 3;
pub const T_RESID: usize = 4;
pub const T_RESID2: usize = 5;
pub const T_RPRJ3: usize = 6;
pub const T_INTERP: usize = 7;
pub const T_NORM2: usize = 8;
pub const T_COMM3: usize = 9;
pub const T_LAST: usize = 10;
pub const DEBUG_DEFAULT: usize = 0;
const DEBUG_VEC: [usize; 8] = [DEBUG_DEFAULT; 8];

pub const EPSILON: f64 = 1.0e-8;

/* mg */
fn main() {
    let mut ir: Vec<usize> = vec![0; MAXLEVEL + 1];
    let mut u: Vec<f64> = vec![0.0; NR];
    let mut v: Vec<f64> = vec![0.0; NV];
    let mut r: Vec<f64> = vec![0.0; NR];

    let mut _lb: usize;
    let lt: usize;

    /*
     * -------------------------------------------------------------------------
     * k is the current level. it is passed down through subroutine args
     * and is not global. it is the current iteration
     * -------------------------------------------------------------------------
     */
    let (mut t, tinit, mops): (f64, f64, f64);

    let mut a: [f64; 4] = [0.0; 4];
    let mut c: [f64; 4] = [0.0; 4];

    let (mut rnm2, mut rnmu): (f64, f64) = (0.0, 0.0);
    let nit: usize;
    let (nn, err): (f64, f64);
    let verified: i8;

    let mut tmax: f64;

    let mut timers = Timer::new();
    for i in T_INIT..T_LAST {
        timers.clear(i);
    }
    timers.start(T_INIT);

    print!(" Using compiled defaults\n");
    lt = LT_DEFAULT;
    nit = NIT_DEFAULT;

    /*
     * ---------------------------------------------------------------------
     * use these for debug info:
     * ---------------------------------------------------------------------
     * DEBUG_VEC(0) = 1 !=> report all norms
     * DEBUG_VEC(1) = 1 !=> some setup information
     * DEBUG_VEC(1) = 2 !=> more setup information
     * DEBUG_VEC(2) = k => at level k or below, show result of resid
     * DEBUG_VEC(3) = k => at level k or below, show result of psinv
     * DEBUG_VEC(4) = k => at level k or below, show result of rprj
     * DEBUG_VEC(5) = k => at level k or below, show result of interp
     * DEBUG_VEC(6) = 1 => (unused)
     * DEBUG_VEC(7) = 1 => (unused)
     * ---------------------------------------------------------------------
     */
    a[0] = -8.0 / 3.0;
    a[1] = 0.0;
    a[2] = 1.0 / 6.0;
    a[3] = 1.0 / 12.0;

    if CLASS == 'A' || CLASS == 'S' || CLASS == 'W' {
        /* coefficients for the s(a) smoother */
        c[0] = -3.0 / 8.0;
        c[1] = 1.0 / 32.0;
        c[2] = -1.0 / 64.0;
        c[3] = 0.0;
    } else {
        /* coefficients for the s(b) smoother */
        c[0] = -3.0 / 17.0;
        c[1] = 1.0 / 33.0;
        c[2] = -1.0 / 61.0;
        c[3] = 0.0;
    }

    _lb = 1;

    let (n1, n2, n3, is1, ie1, is2, ie2, is3, ie3, nx, ny, nz, m1, m2, m3) = setup();
    ir[LT_DEFAULT] = 0;
    for j in (1..LT_DEFAULT).rev() {
        ir[j] = ir[j + 1] + m1[j + 1] * m2[j + 1] * m3[j + 1];
    }
    if DEBUG_VEC[1] >= 1 {
        print!(" in setup, \n");
        print!("   k  lt  nx  ny  nz  n1  n2  n3 is1 is2 is3 ie1 ie2 ie3\n");
        print!(
            "{:4}{:4}{:4}{:4}{:4}{:4}{:4}{:4}{:4}{:4}{:4}{:4}{:4}{:4}\n",
            LT_DEFAULT,
            LT_DEFAULT,
            nx[LT_DEFAULT],
            ny[LT_DEFAULT],
            nz[LT_DEFAULT],
            n1,
            n2,
            n3,
            is1,
            is2,
            is3,
            ie1,
            ie2,
            ie3
        );
    }

    zero3(&mut u[..], n1, n2, n3);
    zran3(
        &mut v[..],
        n1,
        n2,
        n3,
        nx[lt],
        ny[lt],
        is1,
        is2,
        is3,
        ie1,
        ie2,
        ie3,
        &mut timers,
    );
    norm2u3(
        &mut v[..],
        n1,
        n2,
        n3,
        &mut rnm2,
        &mut rnmu,
        nx[lt],
        ny[lt],
        nz[lt],
        &mut timers,
    );

    println!("\n\n NAS Parallel Benchmarks 4.1 Serial Rust version - MG Benchmark");
    println!(
        "\n Size: {}  x  {}  x  {} (class_npb {})",
        nx[lt], ny[lt], nz[lt], CLASS
    );
    println!(" Iterations:    {:>3}\n", nit);

    resid(
        &mut u[..],
        &v[..],
        &mut r[..],
        n1,
        n2,
        n3,
        &nx[..],
        &ny[..],
        &nz[..],
        &a[..],
        LT_DEFAULT,
        &mut timers,
    );

    norm2u3(
        &mut r[..],
        n1,
        n2,
        n3,
        &mut rnm2,
        &mut rnmu,
        nx[lt],
        ny[lt],
        nz[lt],
        &mut timers,
    );

    /*
     * ---------------------------------------------------------------------
     * one iteration for startup
     * ---------------------------------------------------------------------
     */
    mg3_p(
        &mut u[..],
        &mut v[..],
        &mut r[..],
        &a[..],
        &c[..],
        n1,
        n2,
        n3,
        _lb,
        &m1[..],
        &m2[..],
        &m3[..],
        &nx[..],
        &ny[..],
        &nz[..],
        &mut timers,
        &mut ir,
    );

    resid(
        &mut u[..],
        &v[..],
        &mut r[..],
        n1,
        n2,
        n3,
        &nx[..],
        &ny[..],
        &nz[..],
        &a[..],
        LT_DEFAULT,
        &mut timers,
    );

    let (n1, n2, n3, is1, ie1, is2, ie2, is3, ie3, nx, ny, nz, m1, m2, m3) = setup();
    ir[LT_DEFAULT] = 0;
    for j in (1..LT_DEFAULT).rev() {
        ir[j] = ir[j + 1] + m1[j + 1] * m2[j + 1] * m3[j + 1];
    }
    if DEBUG_VEC[1] >= 1 {
        print!(" in setup, \n");
        print!("   k  lt  nx  ny  nz  n1  n2  n3 is1 is2 is3 ie1 ie2 ie3\n");
        print!(
            "{:4}{:4}{:4}{:4}{:4}{:4}{:4}{:4}{:4}{:4}{:4}{:4}{:4}{:4}\n",
            LT_DEFAULT,
            LT_DEFAULT,
            nx[LT_DEFAULT],
            ny[LT_DEFAULT],
            nz[LT_DEFAULT],
            n1,
            n2,
            n3,
            is1,
            is2,
            is3,
            ie1,
            ie2,
            ie3
        );
    }

    zero3(&mut u[..], n1, n2, n3);
    zran3(
        &mut v[..],
        n1,
        n2,
        n3,
        nx[lt],
        ny[lt],
        is1,
        is2,
        is3,
        ie1,
        ie2,
        ie3,
        &mut timers,
    );

    timers.stop(T_INIT);
    tinit = timers.read(T_INIT).as_secs_f64();
    print!(" Initialization time: {:>15.3} seconds\n", tinit);

    for i in T_BENCH..T_LAST {
        timers.clear(i);
    }
    timers.start(T_BENCH);

    if TIMERS {
        timers.start(T_RESID2);
    }
    resid(
        &mut u[..],
        &v[..],
        &mut r[..],
        n1,
        n2,
        n3,
        &nx[..],
        &ny[..],
        &nz[..],
        &a[..],
        LT_DEFAULT,
        &mut timers,
    );
    if TIMERS {
        timers.stop(T_RESID2);
    }

    norm2u3(
        &mut r[..],
        n1,
        n2,
        n3,
        &mut rnm2,
        &mut rnmu,
        nx[lt],
        ny[lt],
        nz[lt],
        &mut timers,
    );

    for it in 1..nit + 1 {
        if it == 1 || it == nit || it % 5 == 0 {
            print!("  iter {:>3}\n", it);
        }
        if TIMERS {
            timers.start(T_MG3P);
        }
        mg3_p(
            &mut u[..],
            &mut v[..],
            &mut r[..],
            &a[..],
            &c[..],
            n1,
            n2,
            n3,
            _lb,
            &m1[..],
            &m2[..],
            &m3[..],
            &nx[..],
            &ny[..],
            &nz[..],
            &mut timers,
            &mut ir,
        );
        if TIMERS {
            timers.stop(T_MG3P);
        }
        if TIMERS {
            timers.start(T_RESID2);
        }
        resid(
            &mut u[..],
            &v[..],
            &mut r[..],
            n1,
            n2,
            n3,
            &nx[..],
            &ny[..],
            &nz[..],
            &a[..],
            LT_DEFAULT,
            &mut timers,
        );
        if TIMERS {
            timers.stop(T_RESID2);
        }
    }
    norm2u3(
        &mut r[..],
        n1,
        n2,
        n3,
        &mut rnm2,
        &mut rnmu,
        nx[lt],
        ny[lt],
        nz[lt],
        &mut timers,
    );

    timers.stop(T_BENCH);
    t = timers.read(T_BENCH).as_secs_f64();

    print!(" Benchmark completed\n");

    if CLASS != 'U' {
        err = (rnm2 - VERIFY_VALUE).abs() / VERIFY_VALUE;
        if err <= EPSILON {
            verified = 1;
            print!(" VERIFICATION SUCCESSFUL\n");
            print!(" L2 Norm is {:>20.13e}\n", rnm2);
            print!(" Error is   {:>20.13e}\n", err);
        } else {
            verified = 0;
            print!(" VERIFICATION FAILED\n");
            print!(" L2 Norm is             {:>20.13e}\n", rnm2);
            print!(" The correct L2 Norm is {:>20.13e}\n", VERIFY_VALUE);
        }
    } else {
        verified = 0;
        print!(" Problem size unknown\n");
        print!(" NO VERIFICATION PERFORMED\n");
    }

    nn = (nx[lt] * ny[lt] * nz[lt]) as f64;

    if t != 0.0 {
        mops = 58.0 * nit as f64 * nn * 1.0e-6 / t;
    } else {
        mops = 0.0;
    }

    let info = PrintInfo {
        name: String::from("MG"),
        class: CLASS.to_string(),
        size: (nx[LT_DEFAULT], ny[LT_DEFAULT], nz[LT_DEFAULT]),
        num_iter: nit as i32,
        time: t,
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
        let mut t_names: Vec<String> = vec![String::new(); T_LAST];
        t_names[T_INIT] = "init".to_string();
        t_names[T_BENCH] = "benchmk".to_string();
        t_names[T_MG3P] = "mg3P".to_string();
        t_names[T_PSINV] = "psinv".to_string();
        t_names[T_RESID] = "resid".to_string();
        t_names[T_RPRJ3] = "rprj3".to_string();
        t_names[T_INTERP] = "interp".to_string();
        t_names[T_NORM2] = "norm2".to_string();
        t_names[T_COMM3] = "comm3".to_string();

        tmax = timers.read(T_BENCH).as_secs_f64();
        if tmax == 0.0 {
            tmax = 1.0;
        }
        print!("  SECTION   Time (secs)\n");
        for i in T_BENCH..T_LAST {
            t = timers.read(i).as_secs_f64();
            if i == T_RESID2 {
                t = timers.read(T_RESID).as_secs_f64() - t;
                print!(
                    "    --> mg-resid:{:>9.3}  ({:>6.2}%)\n",
                    t,
                    t * 100.0 / tmax
                );
            } else {
                print!(
                    "  {:<8}:{:>9.3}  ({:>6.2}%)\n",
                    t_names[i],
                    t,
                    t * 100.0 / tmax
                );
            }
        }
    }
}

/*
 * ---------------------------------------------------------------------
 * bubble does a bubble sort in direction dir
 * ---------------------------------------------------------------------
 */
fn bubble(
    ten: &mut [[f64; MM]; 2],
    j1: &mut [[usize; MM]; 2],
    j2: &mut [[usize; MM]; 2],
    j3: &mut [[usize; MM]; 2],
    m: usize,
    ind: usize,
) {
    let mut temp: f64;
    let mut j_temp: usize;

    if ind == 1 {
        for i in 0..m - 1 {
            if ten[ind][i] > ten[ind][i + 1] {
                temp = ten[ind][i + 1];
                ten[ind][i + 1] = ten[ind][i];
                ten[ind][i] = temp;

                j_temp = j1[ind][i + 1];
                j1[ind][i + 1] = j1[ind][i];
                j1[ind][i] = j_temp;

                j_temp = j2[ind][i + 1];
                j2[ind][i + 1] = j2[ind][i];
                j2[ind][i] = j_temp;

                j_temp = j3[ind][i + 1];
                j3[ind][i + 1] = j3[ind][i];
                j3[ind][i] = j_temp;
            } else {
                return;
            }
        }
    } else {
        for i in 0..m - 1 {
            if ten[ind][i] < ten[ind][i + 1] {
                temp = ten[ind][i + 1];
                ten[ind][i + 1] = ten[ind][i];
                ten[ind][i] = temp;

                j_temp = j1[ind][i + 1];
                j1[ind][i + 1] = j1[ind][i];
                j1[ind][i] = j_temp;

                j_temp = j2[ind][i + 1];
                j2[ind][i + 1] = j2[ind][i];
                j2[ind][i] = j_temp;

                j_temp = j3[ind][i + 1];
                j3[ind][i + 1] = j3[ind][i];
                j3[ind][i] = j_temp;
            } else {
                return;
            }
        }
    }
}

/*
 * ---------------------------------------------------------------------
 * comm3 organizes the communication on all borders
 * ---------------------------------------------------------------------
 */
fn comm3(u: &mut [f64], n1: usize, n2: usize, n3: usize, timers: &mut Timer) {
    if TIMERS {
        timers.start(T_COMM3);
    }
    /* axis = 1 */
    for i3 in 1..n3 - 1 {
        for i2 in 1..n2 - 1 {
            u[(i3 * n2 + i2) * n1] = u[(i3 * n2 + i2) * n1 + n1 - 2];
            u[(i3 * n2 + i2) * n1 + n1 - 1] = u[(i3 * n2 + i2) * n1 + 1];
        }
    }
    /* axis = 2 */
    for i3 in 1..n3 - 1 {
        for i1 in 0..n1 {
            u[i3 * n2 * n1 + i1] = u[(i3 * n2 + (n2 - 2)) * n1 + i1];
            u[(i3 * n2 + (n2 - 1)) * n1 + i1] = u[i3 * n2 * n1 + n1 + i1];
        }
    }
    /* axis = 3 */
    for i2 in 0..n2 {
        for i1 in 0..n1 {
            u[i2 * n1 + i1] = u[((n3 - 2) * n2 + i2) * n1 + i1];
            u[((n3 - 1) * n2 + i2) * n1 + i1] = u[(n2 + i2) * n1 + i1];
        }
    }
    if TIMERS {
        timers.stop(T_COMM3);
    }
}

/*
 * --------------------------------------------------------------------
 * interp adds the trilinear interpolation of the correction
 * from the coarser grid to the current approximation: u = u + Qu'
 *
 * observe that this  implementation costs  16A + 4M, where
 * A and M denote the costs of addition and multiplication.
 * note that this vectorizes, and is also fine for cache
 * based machines. vector machines may get slightly better
 * performance however, with 8 separate "do i1" loops, rather than 4.
 * --------------------------------------------------------------------
 */
fn interp(
    irj: usize,
    irk: usize,
    mm1: usize,
    mm2: usize,
    mm3: usize,
    u: &mut [f64],
    n1: usize,
    n2: usize,
    n3: usize,
    nx: &[usize],
    ny: &[usize],
    nz: &[usize],
    k: usize,
    timers: &mut Timer,
) {
    let (d1, d2, d3, t1, t2, t3): (usize, usize, usize, usize, usize, usize);

    /*
     * --------------------------------------------------------------------
     * note that m = 1037 in globals.h but for this only need to be
     * 535 to handle up to 1024^3
     * integer m
     * parameter( m=535 )
     * --------------------------------------------------------------------
     */
    let mut z1: [f64; M] = [0.0; M];
    let mut z2: [f64; M] = [0.0; M];
    let mut z3: [f64; M] = [0.0; M];

    if TIMERS {
        timers.start(T_INTERP);
    }
    if n1 != 3 && n2 != 3 && n3 != 3 {
        for i3 in 0..mm3 - 1 {
            for i2 in 0..mm2 - 1 {
                if UNSAFE {
                    unsafe {
                        for i1 in 0..mm1 {
                            *z1.get_unchecked_mut(i1) = *u
                                .get_unchecked((i3 * mm2 + (i2 + 1)) * mm1 + i1 + irj)
                                + *u.get_unchecked((i3 * mm2 + i2) * mm1 + i1 + irj);
                            *z2.get_unchecked_mut(i1) = *u
                                .get_unchecked(((i3 + 1) * mm2 + i2) * mm1 + i1 + irj)
                                + *u.get_unchecked((i3 * mm2 + i2) * mm1 + i1 + irj);
                            *z3.get_unchecked_mut(i1) = *u
                                .get_unchecked(((i3 + 1) * mm2 + (i2 + 1)) * mm1 + i1 + irj)
                                + *u.get_unchecked(((i3 + 1) * mm2 + i2) * mm1 + i1 + irj)
                                + *z1.get_unchecked(i1);
                        }
                        for i1 in 0..mm1 - 1 {
                            *u.get_unchecked_mut(
                                ((i3 << 1) * n2 + (i2 << 1)) * n1 + (i1 << 1) + irk,
                            ) += *u.get_unchecked((i3 * mm2 + i2) * mm1 + i1 + irj);
                            *u.get_unchecked_mut(
                                ((i3 << 1) * n2 + (i2 << 1)) * n1 + (i1 << 1) + 1 + irk,
                            ) += 0.5
                                * (*u.get_unchecked((i3 * mm2 + i2) * mm1 + i1 + 1 + irj)
                                    + *u.get_unchecked((i3 * mm2 + i2) * mm1 + i1 + irj));
                        }
                        for i1 in 0..mm1 - 1 {
                            *u.get_unchecked_mut(
                                ((i3 << 1) * n2 + ((i2 << 1) + 1)) * n1 + (i1 << 1) + irk,
                            ) += 0.5 * *z1.get_unchecked(i1);
                            *u.get_unchecked_mut(
                                ((i3 << 1) * n2 + ((i2 << 1) + 1)) * n1 + ((i1 << 1) + 1) + irk,
                            ) += 0.25 * (*z1.get_unchecked(i1) + *z1.get_unchecked(i1 + 1));
                        }
                        for i1 in 0..mm1 - 1 {
                            *u.get_unchecked_mut(
                                (((i3 << 1) + 1) * n2 + (i2 << 1)) * n1 + (i1 << 1) + irk,
                            ) += 0.5 * *z2.get_unchecked(i1);
                            *u.get_unchecked_mut(
                                (((i3 << 1) + 1) * n2 + (i2 << 1)) * n1 + ((i1 << 1) + 1) + irk,
                            ) += 0.25 * (*z2.get_unchecked(i1) + *z2.get_unchecked(i1 + 1));
                        }
                        for i1 in 0..mm1 - 1 {
                            *u.get_unchecked_mut(
                                (((i3 << 1) + 1) * n2 + ((i2 << 1) + 1)) * n1 + (i1 << 1) + irk,
                            ) += 0.25 * *z3.get_unchecked(i1);
                            *u.get_unchecked_mut(
                                (((i3 << 1) + 1) * n2 + ((i2 << 1) + 1)) * n1
                                    + ((i1 << 1) + 1)
                                    + irk,
                            ) += 0.125 * (*z3.get_unchecked(i1) + *z3.get_unchecked(i1 + 1));
                        }
                    }
                } else {
                    for i1 in 0..mm1 {
                        z1[i1] = u[(i3 * mm2 + (i2 + 1)) * mm1 + i1 + irj]
                            + u[(i3 * mm2 + i2) * mm1 + i1 + irj];
                        z2[i1] = u[((i3 + 1) * mm2 + i2) * mm1 + i1 + irj]
                            + u[(i3 * mm2 + i2) * mm1 + i1 + irj];
                        z3[i1] = u[((i3 + 1) * mm2 + (i2 + 1)) * mm1 + i1 + irj]
                            + u[((i3 + 1) * mm2 + i2) * mm1 + i1 + irj]
                            + z1[i1];
                    }
                    for i1 in 0..mm1 - 1 {
                        u[((i3 << 1) * n2 + (i2 << 1)) * n1 + (i1 << 1) + irk] +=
                            u[(i3 * mm2 + i2) * mm1 + i1 + irj];
                        u[((i3 << 1) * n2 + (i2 << 1)) * n1 + (i1 << 1) + 1 + irk] += 0.5
                            * (u[(i3 * mm2 + i2) * mm1 + i1 + 1 + irj]
                                + u[(i3 * mm2 + i2) * mm1 + i1 + irj]);
                    }
                    for i1 in 0..mm1 - 1 {
                        u[((i3 << 1) * n2 + ((i2 << 1) + 1)) * n1 + (i1 << 1) + irk] +=
                            0.5 * z1[i1];
                        u[((i3 << 1) * n2 + ((i2 << 1) + 1)) * n1 + ((i1 << 1) + 1) + irk] +=
                            0.25 * (z1[i1] + z1[i1 + 1]);
                    }
                    for i1 in 0..mm1 - 1 {
                        u[(((i3 << 1) + 1) * n2 + (i2 << 1)) * n1 + (i1 << 1) + irk] +=
                            0.5 * z2[i1];
                        u[(((i3 << 1) + 1) * n2 + (i2 << 1)) * n1 + ((i1 << 1) + 1) + irk] +=
                            0.25 * (z2[i1] + z2[i1 + 1]);
                    }
                    for i1 in 0..mm1 - 1 {
                        u[(((i3 << 1) + 1) * n2 + ((i2 << 1) + 1)) * n1 + (i1 << 1) + irk] +=
                            0.25 * z3[i1];
                        u[(((i3 << 1) + 1) * n2 + ((i2 << 1) + 1)) * n1 + ((i1 << 1) + 1) + irk] +=
                            0.125 * (z3[i1] + z3[i1 + 1]);
                    }
                }
            }
        }
    } else {
        if n1 == 3 {
            d1 = 2;
            t1 = 1;
        } else {
            d1 = 1;
            t1 = 0;
        }
        if n2 == 3 {
            d2 = 2;
            t2 = 1;
        } else {
            d2 = 1;
            t2 = 0;
        }
        if n3 == 3 {
            d3 = 2;
            t3 = 1;
        } else {
            d3 = 1;
            t3 = 0;
        }
        for i3 in d3..mm3 {
            if UNSAFE {
                unsafe {
                    for i2 in d2..mm2 {
                        for i1 in d1..mm1 {
                            *u.get_unchecked_mut(
                                (((i3 << 1) - d3 - 1) * n2 + ((i2 << 1) - d2 - 1)) * n1 + (i1 << 1)
                                    - d1
                                    - 1
                                    + irk,
                            ) += *u.get_unchecked(((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj);
                        }
                        for i1 in 1..mm1 {
                            *u.get_unchecked_mut(
                                (((i3 << 1) - d3 - 1) * n2 + ((i2 << 1) - d2 - 1)) * n1 + (i1 << 1)
                                    - t1
                                    - 1
                                    + irk,
                            ) += 0.5
                                * (*u.get_unchecked(((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 + irj)
                                    + *u.get_unchecked(
                                        ((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj,
                                    ));
                        }
                    }
                    for i2 in 1..mm2 {
                        for i1 in d1..mm1 {
                            *u.get_unchecked_mut(
                                (((i3 << 1) - d3 - 1) * n2 + ((i2 << 1) - t2 - 1)) * n1 + (i1 << 1)
                                    - d1
                                    - 1
                                    + irk,
                            ) += 0.5
                                * (*u.get_unchecked(((i3 - 1) * mm2 + i2) * mm1 + i1 - 1 + irj)
                                    + *u.get_unchecked(
                                        ((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj,
                                    ));
                        }
                        for i1 in 1..mm1 {
                            *u.get_unchecked_mut(
                                (((i3 << 1) - d3 - 1) * n2 + ((i2 << 1) - t2 - 1)) * n1 + (i1 << 1)
                                    - t1
                                    - 1
                                    + irk,
                            ) += 0.25
                                * (*u.get_unchecked(((i3 - 1) * mm2 + i2) * mm1 + i1 + irj)
                                    + *u.get_unchecked(
                                        ((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 + irj,
                                    )
                                    + *u.get_unchecked(((i3 - 1) * mm2 + i2) * mm1 + i1 - 1 + irj)
                                    + *u.get_unchecked(
                                        ((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj,
                                    ));
                        }
                    }
                }
            } else {
                for i2 in d2..mm2 {
                    for i1 in d1..mm1 {
                        u[(((i3 << 1) - d3 - 1) * n2 + ((i2 << 1) - d2 - 1)) * n1 + (i1 << 1)
                            - d1
                            - 1
                            + irk] += u[((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj];
                    }
                    for i1 in 1..mm1 {
                        u[(((i3 << 1) - d3 - 1) * n2 + ((i2 << 1) - d2 - 1)) * n1 + (i1 << 1)
                            - t1
                            - 1
                            + irk] += 0.5
                            * (u[((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 + irj]
                                + u[((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj]);
                    }
                }
                for i2 in 1..mm2 {
                    for i1 in d1..mm1 {
                        u[(((i3 << 1) - d3 - 1) * n2 + ((i2 << 1) - t2 - 1)) * n1 + (i1 << 1)
                            - d1
                            - 1
                            + irk] += 0.5
                            * (u[((i3 - 1) * mm2 + i2) * mm1 + i1 - 1 + irj]
                                + u[((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj]);
                    }
                    for i1 in 1..mm1 {
                        u[(((i3 << 1) - d3 - 1) * n2 + ((i2 << 1) - t2 - 1)) * n1 + (i1 << 1)
                            - t1
                            - 1
                            + irk] += 0.25
                            * (u[((i3 - 1) * mm2 + i2) * mm1 + i1 + irj]
                                + u[((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 + irj]
                                + u[((i3 - 1) * mm2 + i2) * mm1 + i1 - 1 + irj]
                                + u[((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj]);
                    }
                }
            }
        }
        for i3 in 1..mm3 {
            if UNSAFE {
                unsafe {
                    for i2 in d2..mm2 {
                        for i1 in d1..mm1 {
                            *u.get_unchecked_mut(
                                (((i3 << 1) - t3 - 1) * n2 + ((i2 << 1) - d2 - 1)) * n1 + (i1 << 1)
                                    - d1
                                    - 1
                                    + irk,
                            ) += 0.5
                                * (*u.get_unchecked((i3 * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj)
                                    + *u.get_unchecked(
                                        ((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj,
                                    ));
                        }
                        for i1 in 1..mm1 {
                            *u.get_unchecked_mut(
                                (((i3 << 1) - t3 - 1) * n2 + ((i2 << 1) - d2 - 1)) * n1 + (i1 << 1)
                                    - t1
                                    - 1
                                    + irk,
                            ) += 0.25
                                * (*u.get_unchecked((i3 * mm2 + (i2 - 1)) * mm1 + i1 + irj)
                                    + *u.get_unchecked((i3 * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj)
                                    + *u.get_unchecked(
                                        ((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 + irj,
                                    )
                                    + *u.get_unchecked(
                                        ((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj,
                                    ));
                        }
                    }
                    for i2 in 1..mm2 {
                        for i1 in d1..mm1 {
                            *u.get_unchecked_mut(
                                (((i3 << 1) - t3 - 1) * n2 + ((i2 << 1) - t2 - 1)) * n1 + (i1 << 1)
                                    - d1
                                    - 1
                                    + irk,
                            ) += 0.25
                                * (*u.get_unchecked((i3 * mm2 + i2) * mm1 + i1 - 1 + irj)
                                    + *u.get_unchecked((i3 * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj)
                                    + *u.get_unchecked(((i3 - 1) * mm2 + i2) * mm1 + i1 - 1 + irj)
                                    + *u.get_unchecked(
                                        ((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj,
                                    ));
                        }
                        for i1 in 1..mm1 {
                            *u.get_unchecked_mut(
                                (((i3 << 1) - t3 - 1) * n2 + ((i2 << 1) - t2 - 1)) * n1 + (i1 << 1)
                                    - t1
                                    - 1
                                    + irk,
                            ) += 0.125
                                * (*u.get_unchecked((i3 * mm2 + i2) * mm1 + i1 + irj)
                                    + *u.get_unchecked((i3 * mm2 + (i2 - 1)) * mm1 + i1 + irj)
                                    + *u.get_unchecked((i3 * mm2 + i2) * mm1 + i1 - 1 + irj)
                                    + *u.get_unchecked((i3 * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj)
                                    + *u.get_unchecked(((i3 - 1) * mm2 + i2) * mm1 + i1 + irj)
                                    + *u.get_unchecked(
                                        ((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 + irj,
                                    )
                                    + *u.get_unchecked(((i3 - 1) * mm2 + i2) * mm1 + i1 - 1 + irj)
                                    + *u.get_unchecked(
                                        ((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj,
                                    ));
                        }
                    }
                }
            } else {
                for i2 in d2..mm2 {
                    for i1 in d1..mm1 {
                        u[(((i3 << 1) - t3 - 1) * n2 + ((i2 << 1) - d2 - 1)) * n1 + (i1 << 1)
                            - d1
                            - 1
                            + irk] += 0.5
                            * (u[(i3 * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj]
                                + u[((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj]);
                    }
                    for i1 in 1..mm1 {
                        u[(((i3 << 1) - t3 - 1) * n2 + ((i2 << 1) - d2 - 1)) * n1 + (i1 << 1)
                            - t1
                            - 1
                            + irk] += 0.25
                            * (u[(i3 * mm2 + (i2 - 1)) * mm1 + i1 + irj]
                                + u[(i3 * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj]
                                + u[((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 + irj]
                                + u[((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj]);
                    }
                }
                for i2 in 1..mm2 {
                    for i1 in d1..mm1 {
                        u[(((i3 << 1) - t3 - 1) * n2 + ((i2 << 1) - t2 - 1)) * n1 + (i1 << 1)
                            - d1
                            - 1
                            + irk] += 0.25
                            * (u[(i3 * mm2 + i2) * mm1 + i1 - 1 + irj]
                                + u[(i3 * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj]
                                + u[((i3 - 1) * mm2 + i2) * mm1 + i1 - 1 + irj]
                                + u[((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj]);
                    }
                    for i1 in 1..mm1 {
                        u[(((i3 << 1) - t3 - 1) * n2 + ((i2 << 1) - t2 - 1)) * n1 + (i1 << 1)
                            - t1
                            - 1
                            + irk] += 0.125
                            * (u[(i3 * mm2 + i2) * mm1 + i1 + irj]
                                + u[(i3 * mm2 + (i2 - 1)) * mm1 + i1 + irj]
                                + u[(i3 * mm2 + i2) * mm1 + i1 - 1 + irj]
                                + u[(i3 * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj]
                                + u[((i3 - 1) * mm2 + i2) * mm1 + i1 + irj]
                                + u[((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 + irj]
                                + u[((i3 - 1) * mm2 + i2) * mm1 + i1 - 1 + irj]
                                + u[((i3 - 1) * mm2 + (i2 - 1)) * mm1 + i1 - 1 + irj]);
                    }
                }
            }
        }
    }
    if TIMERS {
        timers.stop(T_INTERP);
    }

    if DEBUG_VEC[0] >= 1 {
        rep_nrm(
            u,
            mm1,
            mm2,
            mm3,
            String::from("z: inter"),
            nx,
            ny,
            nz,
            k - 1,
            irj,
            timers,
        );
        rep_nrm(
            u,
            n1,
            n2,
            n3,
            String::from("u: inter"),
            nx,
            ny,
            nz,
            k,
            irk,
            timers,
        );
    }
    if DEBUG_VEC[5] >= k {
        showall(u, mm1, mm2, mm3, irj);
        showall(u, n1, n2, n3, irk);
    }
}

/*
 * --------------------------------------------------------------------
 * multigrid v-cycle routine
 * --------------------------------------------------------------------
 */
fn mg3_p(
    u: &mut [f64],
    v: &mut [f64],
    r: &mut [f64],
    a: &[f64],
    c: &[f64],
    n1: usize,
    n2: usize,
    n3: usize,
    lb: usize,
    m1: &[usize],
    m2: &[usize],
    m3: &[usize],
    nx: &[usize],
    ny: &[usize],
    nz: &[usize],
    timers: &mut Timer,
    ir: &mut [usize],
) {
    let mut j: usize;

    /*
     * --------------------------------------------------------------------
     * down cycle.
     * restrict the residual from the find grid to the coarse
     * -------------------------------------------------------------------
     */
    for k in (lb + 1..LT_DEFAULT + 1).rev() {
        j = k - 1;
        rprj3(
            ir[k],
            ir[j],
            m1[k],
            m2[k],
            m3[k],
            &mut r[..],
            m1[j],
            m2[j],
            m3[j],
            nx,
            ny,
            nz,
            k,
            timers,
        );
    }

    let k: usize = lb;
    /*
     * --------------------------------------------------------------------
     * compute an approximate solution on the coarsest grid
     * --------------------------------------------------------------------
     */

    zero3(&mut u[ir[k]..], m1[k], m2[k], m3[k]);
    psinv(
        &r[ir[k]..],
        &mut u[ir[k]..],
        m1[k],
        m2[k],
        m3[k],
        nx,
        ny,
        nz,
        c,
        k,
        timers,
    );

    for k in lb + 1..LT_DEFAULT {
        j = k - 1;
        /*
         * --------------------------------------------------------------------
         * prolongate from level k-1  to k
         * -------------------------------------------------------------------
         */
        zero3(&mut u[ir[k]..], m1[k], m2[k], m3[k]);
        interp(
            ir[j],
            ir[k],
            m1[j],
            m2[j],
            m3[j],
            &mut u[..],
            m1[k],
            m2[k],
            m3[k],
            nx,
            ny,
            nz,
            k,
            timers,
        );

        /*
         * --------------------------------------------------------------------
         * compute residual for level k
         * --------------------------------------------------------------------
         */
        resid_2(
            &mut u[ir[k]..],
            &mut r[ir[k]..],
            m1[k],
            m2[k],
            m3[k],
            nx,
            ny,
            nz,
            a,
            k,
            timers,
        );
        /*
         * --------------------------------------------------------------------
         * apply smoother
         * --------------------------------------------------------------------
         */
        psinv(
            &r[ir[k]..],
            &mut u[ir[k]..],
            m1[k],
            m2[k],
            m3[k],
            nx,
            ny,
            nz,
            c,
            k,
            timers,
        );
    }

    j = LT_DEFAULT - 1;
    interp(
        ir[j],
        0,
        m1[j],
        m2[j],
        m3[j],
        &mut u[..],
        n1,
        n2,
        n3,
        nx,
        ny,
        nz,
        LT_DEFAULT,
        timers,
    );
    resid(
        &mut u[..],
        &mut v[..],
        &mut r[..],
        n1,
        n2,
        n3,
        nx,
        ny,
        nz,
        a,
        LT_DEFAULT,
        timers,
    );
    psinv(
        &mut r[..],
        &mut u[..],
        n1,
        n2,
        n3,
        nx,
        ny,
        nz,
        c,
        LT_DEFAULT,
        timers,
    );
}

fn norm2u3(
    r: &mut [f64],
    n1: usize,
    n2: usize,
    n3: usize,
    rnm2: &mut f64,
    rnmu: &mut f64,
    nx: usize,
    ny: usize,
    nz: usize,
    timers: &mut Timer,
) {
    let (mut s, mut a): (f64, f64);

    let dn: f64;

    if TIMERS {
        timers.start(T_NORM2);
    }
    dn = (nx * ny * nz) as f64;

    s = 0.0;
    *rnmu = 0.0;

    for i3 in 1..n3 - 1 {
        for i2 in 1..n2 - 1 {
            for i1 in 1..n1 - 1 {
                let i321: usize = i1 + n1 * (i2 + n2 * i3);
                s = s + r[i321] * r[i321];
                a = r[i321].abs();
                if a > *rnmu {
                    *rnmu = a;
                }
            }
        }
    }

    *rnm2 = f64::sqrt(s / dn);
    if TIMERS {
        timers.stop(T_NORM2);
    }
}

/*
 * ---------------------------------------------------------------------
 * power raises an integer, disguised as a double
 * precision real, to an integer power
 * ---------------------------------------------------------------------
 */
fn power(a: f64, n: usize) -> f64 {
    let mut power: f64 = 1.0;
    let mut nj: usize = n;
    let mut aj: f64 = a;

    while nj != 0 {
        if (nj % 2) == 1 {
            _ = randlc(&mut power, aj);
        }
        let aux_aj: f64 = aj;
        _ = randlc(&mut aj, aux_aj);
        nj = nj >> 1;
    }
    return power;
}

/*
 * --------------------------------------------------------------------
 * psinv applies an approximate inverse as smoother: u = u + Cr
 *
 * this  implementation costs  15A + 4M per result, where
 * A and M denote the costs of Addition and Multiplication.
 * presuming coefficient c(3) is zero (the NPB assumes this,
 * but it is thus not a general case), 2A + 1M may be eliminated,
 * resulting in 13A + 3M.
 * note that this vectorizes, and is also fine for cache
 * based machines.
 * --------------------------------------------------------------------
 */
fn psinv(
    r: &[f64],
    u: &mut [f64],
    n1: usize,
    n2: usize,
    n3: usize,
    nx: &[usize],
    ny: &[usize],
    nz: &[usize],
    c: &[f64],
    k: usize,
    timers: &mut Timer,
) {
    let mut r1: [f64; M] = [0.0; M];
    let mut r2: [f64; M] = [0.0; M];
    if TIMERS {
        timers.start(T_PSINV);
    }
    for i3 in 1..n3 - 1 {
        for i2 in 1..n2 - 1 {
            if UNSAFE {
                unsafe {
                    for i1 in 0..n1 {
                        *r1.get_unchecked_mut(i1) = *r
                            .get_unchecked((i3 * n2 + (i2 - 1)) * n1 + i1)
                            + *r.get_unchecked((i3 * n2 + (i2 + 1)) * n1 + i1)
                            + *r.get_unchecked(((i3 - 1) * n2 + i2) * n1 + i1)
                            + *r.get_unchecked(((i3 + 1) * n2 + i2) * n1 + i1);
                        *r2.get_unchecked_mut(i1) = *r
                            .get_unchecked(((i3 - 1) * n2 + (i2 - 1)) * n1 + i1)
                            + *r.get_unchecked(((i3 - 1) * n2 + (i2 + 1)) * n1 + i1)
                            + *r.get_unchecked(((i3 + 1) * n2 + (i2 - 1)) * n1 + i1)
                            + *r.get_unchecked(((i3 + 1) * n2 + (i2 + 1)) * n1 + i1);
                    }
                    for i1 in 1..n1 - 1 {
                        *u.get_unchecked_mut((i3 * n2 + i2) * n1 + i1) += *c.get_unchecked(0)
                            * *r.get_unchecked((i3 * n2 + i2) * n1 + i1)
                            + *c.get_unchecked(1)
                                * (*r.get_unchecked((i3 * n2 + i2) * n1 + i1 - 1)
                                    + *r.get_unchecked((i3 * n2 + i2) * n1 + i1 + 1)
                                    + *r1.get_unchecked(i1))
                            + *c.get_unchecked(2)
                                * (*r2.get_unchecked(i1)
                                    + *r1.get_unchecked(i1 - 1)
                                    + *r1.get_unchecked(i1 + 1));
                    }
                }
            } else {
                for i1 in 0..n1 {
                    r1[i1] = r[(i3 * n2 + (i2 - 1)) * n1 + i1]
                        + r[(i3 * n2 + (i2 + 1)) * n1 + i1]
                        + r[((i3 - 1) * n2 + i2) * n1 + i1]
                        + r[((i3 + 1) * n2 + i2) * n1 + i1];
                    r2[i1] = r[((i3 - 1) * n2 + (i2 - 1)) * n1 + i1]
                        + r[((i3 - 1) * n2 + (i2 + 1)) * n1 + i1]
                        + r[((i3 + 1) * n2 + (i2 - 1)) * n1 + i1]
                        + r[((i3 + 1) * n2 + (i2 + 1)) * n1 + i1];
                }
                for i1 in 1..n1 - 1 {
                    u[(i3 * n2 + i2) * n1 + i1] += c[0] * r[(i3 * n2 + i2) * n1 + i1]
                        + c[1]
                            * (r[(i3 * n2 + i2) * n1 + i1 - 1]
                                + r[(i3 * n2 + i2) * n1 + i1 + 1]
                                + r1[i1])
                        + c[2] * (r2[i1] + r1[i1 - 1] + r1[i1 + 1]);
                    /*
                    	* --------------------------------------------------------------------
                    	* assume c(3) = 0    (enable line below if c(3) not= 0)
                    	* --------------------------------------------------------------------
                    	* > + c(3) * ( r2(i1-1) + r2(i1+1) )
                    	* --------------------------------------------------------------------
                    	*/
                }
            }
        }
    }
    if TIMERS {
        timers.stop(T_PSINV);
    }

    /*
     * --------------------------------------------------------------------
     * exchange boundary points
     * --------------------------------------------------------------------
     */
    comm3(u, n1, n2, n3, timers);

    if DEBUG_VEC[0] >= 1 {
        rep_nrm(
            u,
            n1,
            n2,
            n3,
            String::from("   psinv"),
            nx,
            ny,
            nz,
            k,
            0,
            timers,
        );
    }

    if DEBUG_VEC[3] >= k {
        showall(u, n1, n2, n3, 0);
    }
}

/*
 * ---------------------------------------------------------------------
 * report on norm
 * ---------------------------------------------------------------------
 */
fn rep_nrm(
    u: &mut [f64],
    n1: usize,
    n2: usize,
    n3: usize,
    title: String,
    nx: &[usize],
    ny: &[usize],
    nz: &[usize],
    kk: usize,
    k: usize,
    timers: &mut Timer,
) {
    let (mut rnm2, mut rnmu): (f64, f64) = (0.0, 0.0);
    norm2u3(
        &mut u[k..],
        n1,
        n2,
        n3,
        &mut rnm2,
        &mut rnmu,
        nx[kk],
        ny[kk],
        nz[kk],
        timers,
    );
    print!(
        " Level {} in {}: norms ={:21.14e} {:21.14e}\n",
        kk, title, rnm2, rnmu
    );
}

/*
 * --------------------------------------------------------------------
 * resid computes the residual: r = v - Au
 *
 * this  implementation costs  15A + 4M per result, where
 * A and M denote the costs of addition (or subtraction) and
 * multiplication, respectively.
 * presuming coefficient a(1) is zero (the NPB assumes this,
 * but it is thus not a general case), 3A + 1M may be eliminated,
 * resulting in 12A + 3M.
 * note that this vectorizes, and is also fine for cache
 * based machines.
 * --------------------------------------------------------------------
 */
fn resid(
    u: &mut [f64],
    v: &[f64],
    r: &mut [f64],
    n1: usize,
    n2: usize,
    n3: usize,
    nx: &[usize],
    ny: &[usize],
    nz: &[usize],
    a: &[f64],
    k: usize,
    timers: &mut Timer,
) {
    let mut u1: [f64; M] = [0.0; M];
    let mut u2: [f64; M] = [0.0; M];

    if TIMERS {
        timers.start(T_RESID);
    }
    for i3 in 1..n3 - 1 {
        for i2 in 1..n2 - 1 {
            if UNSAFE {
                unsafe {
                    for i1 in 0..n1 {
                        *u1.get_unchecked_mut(i1) = *u
                            .get_unchecked((i3 * n2 + (i2 - 1)) * n1 + i1)
                            + *u.get_unchecked((i3 * n2 + (i2 + 1)) * n1 + i1)
                            + *u.get_unchecked(((i3 - 1) * n2 + i2) * n1 + i1)
                            + *u.get_unchecked(((i3 + 1) * n2 + i2) * n1 + i1);

                        *u2.get_unchecked_mut(i1) = u[((i3 - 1) * n2 + (i2 - 1)) * n1 + i1]
                            + *u.get_unchecked(((i3 - 1) * n2 + (i2 + 1)) * n1 + i1)
                            + *u.get_unchecked(((i3 + 1) * n2 + (i2 - 1)) * n1 + i1)
                            + *u.get_unchecked(((i3 + 1) * n2 + (i2 + 1)) * n1 + i1);
                    }
                    for i1 in 1..n1 - 1 {
                        *r.get_unchecked_mut((i3 * n2 + i2) * n1 + i1) = *v
                            .get_unchecked((i3 * n2 + i2) * n1 + i1)
                            - *a.get_unchecked(0) * *u.get_unchecked((i3 * n2 + i2) * n1 + i1)
                            - *a.get_unchecked(2)
                                * (*u2.get_unchecked(i1)
                                    + *u1.get_unchecked(i1 - 1)
                                    + *u1.get_unchecked(i1 + 1))
                            - *a.get_unchecked(3)
                                * (*u2.get_unchecked(i1 - 1) + *u2.get_unchecked(i1 + 1));
                    }
                }
            } else {
                for i1 in 0..n1 {
                    u1[i1] = u[(i3 * n2 + (i2 - 1)) * n1 + i1]
                        + u[(i3 * n2 + (i2 + 1)) * n1 + i1]
                        + u[((i3 - 1) * n2 + i2) * n1 + i1]
                        + u[((i3 + 1) * n2 + i2) * n1 + i1];

                    u2[i1] = u[((i3 - 1) * n2 + (i2 - 1)) * n1 + i1]
                        + u[((i3 - 1) * n2 + (i2 + 1)) * n1 + i1]
                        + u[((i3 + 1) * n2 + (i2 - 1)) * n1 + i1]
                        + u[((i3 + 1) * n2 + (i2 + 1)) * n1 + i1];
                }
                for i1 in 1..n1 - 1 {
                    /*
                    	* ---------------------------------------------------------------------
                    	* assume a(1) = 0 (enable 2 lines below if a(1) not= 0)
                    	* ---------------------------------------------------------------------
                    	* > - a(1) * ( u(i1-1,i2,i3) + u(i1+1,i2,i3)
                    	* > + u1(i1) )
                    	* ---------------------------------------------------------------------
                    	*/
                    r[(i3 * n2 + i2) * n1 + i1] = v[(i3 * n2 + i2) * n1 + i1]
                        - a[0] * u[(i3 * n2 + i2) * n1 + i1]
                        - a[2] * (u2[i1] + u1[i1 - 1] + u1[i1 + 1])
                        - a[3] * (u2[i1 - 1] + u2[i1 + 1]);
                }
            }
        }
    }
    if TIMERS {
        timers.stop(T_RESID);
    }

    /*
     * --------------------------------------------------------------------
     * exchange boundary data
     * --------------------------------------------------------------------
     */
    comm3(r, n1, n2, n3, timers);

    if DEBUG_VEC[0] >= 1 {
        rep_nrm(
            r,
            n1,
            n2,
            n3,
            String::from("   resid"),
            &nx[..],
            &ny[..],
            &nz[..],
            k,
            0,
            timers,
        );
    }
    if DEBUG_VEC[2] >= k {
        showall(r, n1, n2, n3, 0);
    }
}

fn resid_2(
    u: &mut [f64],
    r: &mut [f64],
    n1: usize,
    n2: usize,
    n3: usize,
    nx: &[usize],
    ny: &[usize],
    nz: &[usize],
    a: &[f64],
    k: usize,
    timers: &mut Timer,
) {
    let mut u1: [f64; M] = [0.0; M];
    let mut u2: [f64; M] = [0.0; M];

    if TIMERS {
        timers.start(T_RESID);
    }
    for i3 in 1..n3 - 1 {
        for i2 in 1..n2 - 1 {
            if UNSAFE {
                unsafe {
                    for i1 in 0..n1 {
                        *u1.get_unchecked_mut(i1) = *u
                            .get_unchecked((i3 * n2 + (i2 - 1)) * n1 + i1)
                            + *u.get_unchecked((i3 * n2 + (i2 + 1)) * n1 + i1)
                            + *u.get_unchecked(((i3 - 1) * n2 + i2) * n1 + i1)
                            + *u.get_unchecked(((i3 + 1) * n2 + i2) * n1 + i1);

                        *u2.get_unchecked_mut(i1) = *u
                            .get_unchecked(((i3 - 1) * n2 + (i2 - 1)) * n1 + i1)
                            + *u.get_unchecked(((i3 - 1) * n2 + (i2 + 1)) * n1 + i1)
                            + *u.get_unchecked(((i3 + 1) * n2 + (i2 - 1)) * n1 + i1)
                            + *u.get_unchecked(((i3 + 1) * n2 + (i2 + 1)) * n1 + i1);
                    }
                    for i1 in 1..n1 - 1 {
                        *r.get_unchecked_mut((i3 * n2 + i2) * n1 + i1) = *r
                            .get_unchecked((i3 * n2 + i2) * n1 + i1)
                            - *a.get_unchecked(0) * *u.get_unchecked((i3 * n2 + i2) * n1 + i1)
                            - *a.get_unchecked(2)
                                * (*u2.get_unchecked(i1)
                                    + *u1.get_unchecked(i1 - 1)
                                    + *u1.get_unchecked(i1 + 1))
                            - *a.get_unchecked(3)
                                * (*u2.get_unchecked(i1 - 1) + *u2.get_unchecked(i1 + 1));
                    }
                }
            } else {
                for i1 in 0..n1 {
                    u1[i1] = u[(i3 * n2 + (i2 - 1)) * n1 + i1]
                        + u[(i3 * n2 + (i2 + 1)) * n1 + i1]
                        + u[((i3 - 1) * n2 + i2) * n1 + i1]
                        + u[((i3 + 1) * n2 + i2) * n1 + i1];

                    u2[i1] = u[((i3 - 1) * n2 + (i2 - 1)) * n1 + i1]
                        + u[((i3 - 1) * n2 + (i2 + 1)) * n1 + i1]
                        + u[((i3 + 1) * n2 + (i2 - 1)) * n1 + i1]
                        + u[((i3 + 1) * n2 + (i2 + 1)) * n1 + i1];
                }
                for i1 in 1..n1 - 1 {
                    /*
                    	* ---------------------------------------------------------------------
                    	* assume a(1) = 0 (enable 2 lines below if a(1) not= 0)
                    	* ---------------------------------------------------------------------
                    	* > - a(1) * ( u(i1-1,i2,i3) + u(i1+1,i2,i3)
                    	* > + u1(i1) )
                    	* ---------------------------------------------------------------------
                    	*/
                    r[(i3 * n2 + i2) * n1 + i1] = r[(i3 * n2 + i2) * n1 + i1]
                        - a[0] * u[(i3 * n2 + i2) * n1 + i1]
                        - a[2] * (u2[i1] + u1[i1 - 1] + u1[i1 + 1])
                        - a[3] * (u2[i1 - 1] + u2[i1 + 1]);
                }
            }
        }
    }
    if TIMERS {
        timers.stop(T_RESID);
    }

    /*
     * --------------------------------------------------------------------
     * exchange boundary data
     * --------------------------------------------------------------------
     */
    comm3(r, n1, n2, n3, timers);

    if DEBUG_VEC[0] >= 1 {
        rep_nrm(
            r,
            n1,
            n2,
            n3,
            String::from("   resid"),
            &nx[..],
            &ny[..],
            &nz[..],
            k - 1,
            0,
            timers,
        );
    }
    if DEBUG_VEC[2] >= k {
        showall(r, n1, n2, n3, 0);
    }
}

/*
 * --------------------------------------------------------------------
 * rprj3 projects onto the next coarser grid,
 * using a trilinear finite element projection: s = r' = P r
 *
 * this  implementation costs 20A + 4M per result, where
 * A and M denote the costs of addition and multiplication.
 * note that this vectorizes, and is also fine for cache
 * based machines.
 * --------------------------------------------------------------------
 */
fn rprj3(
    irk: usize,
    irj: usize,
    m1k: usize,
    m2k: usize,
    m3k: usize,
    r: &mut [f64],
    m1j: usize,
    m2j: usize,
    m3j: usize,
    nx: &[usize],
    ny: &[usize],
    nz: &[usize],
    k: usize,
    timers: &mut Timer,
) {
    let (mut i3, mut i2, mut i1, d1, d2, d3): (usize, usize, usize, usize, usize, usize);
    let (mut x2, mut y2): (f64, f64);
    let mut x1: [f64; M] = [0.0; M];
    let mut y1: [f64; M] = [0.0; M];

    if TIMERS {
        timers.start(T_RPRJ3);
    }
    if m1k == 3 {
        d1 = 2;
    } else {
        d1 = 1;
    }
    if m2k == 3 {
        d2 = 2;
    } else {
        d2 = 1;
    }
    if m3k == 3 {
        d3 = 2;
    } else {
        d3 = 1;
    }
    for j3 in 1..m3j - 1 {
        i3 = (j3 << 1) - d3;
        for j2 in 1..m2j - 1 {
            i2 = (j2 << 1) - d2;
            if UNSAFE {
                unsafe {
                    for j1 in 1..m1j {
                        i1 = (j1 << 1) - d1;
                        *x1.get_unchecked_mut(i1) = *r
                            .get_unchecked(((i3 + 1) * m2k + i2) * m1k + i1 + irk)
                            + *r.get_unchecked(((i3 + 1) * m2k + (i2 + 2)) * m1k + i1 + irk)
                            + *r.get_unchecked((i3 * m2k + (i2 + 1)) * m1k + i1 + irk)
                            + *r.get_unchecked(((i3 + 2) * m2k + (i2 + 1)) * m1k + i1 + irk);

                        *y1.get_unchecked_mut(i1) = *r
                            .get_unchecked((i3 * m2k + i2) * m1k + i1 + irk)
                            + *r.get_unchecked(((i3 + 2) * m2k + i2) * m1k + i1 + irk)
                            + *r.get_unchecked((i3 * m2k + (i2 + 2)) * m1k + i1 + irk)
                            + *r.get_unchecked(((i3 + 2) * m2k + (i2 + 2)) * m1k + i1 + irk);
                    }
                    for j1 in 1..m1j - 1 {
                        i1 = (j1 << 1) - d1;
                        y2 = *r.get_unchecked((i3 * m2k + i2) * m1k + i1 + 1 + irk)
                            + *r.get_unchecked(((i3 + 2) * m2k + i2) * m1k + i1 + 1 + irk)
                            + *r.get_unchecked((i3 * m2k + (i2 + 2)) * m1k + i1 + 1 + irk)
                            + *r.get_unchecked(((i3 + 2) * m2k + (i2 + 2)) * m1k + i1 + 1 + irk);

                        x2 = *r.get_unchecked(((i3 + 1) * m2k + i2) * m1k + i1 + 1 + irk)
                            + *r.get_unchecked(((i3 + 1) * m2k + (i2 + 2)) * m1k + i1 + 1 + irk)
                            + *r.get_unchecked((i3 * m2k + (i2 + 1)) * m1k + i1 + 1 + irk)
                            + *r.get_unchecked(((i3 + 2) * m2k + (i2 + 1)) * m1k + i1 + 1 + irk);
                        *r.get_unchecked_mut((j3 * m2j + j2) * m1j + j1 + irj) = 0.5
                            * *r.get_unchecked(((i3 + 1) * m2k + (i2 + 1)) * m1k + i1 + 1 + irk)
                            + 0.25
                                * (*r.get_unchecked(((i3 + 1) * m2k + (i2 + 1)) * m1k + i1 + irk)
                                    + *r.get_unchecked(
                                        ((i3 + 1) * m2k + (i2 + 1)) * m1k + i1 + 2 + irk,
                                    )
                                    + x2)
                            + 0.125 * (*x1.get_unchecked(i1) + *x1.get_unchecked(i1 + 2) + y2)
                            + 0.0625 * (*y1.get_unchecked(i1) + *y1.get_unchecked(i1 + 2));
                    }
                }
            } else {
                for j1 in 1..m1j {
                    i1 = (j1 << 1) - d1;
                    x1[i1] = r[((i3 + 1) * m2k + i2) * m1k + i1 + irk]
                        + r[((i3 + 1) * m2k + (i2 + 2)) * m1k + i1 + irk]
                        + r[(i3 * m2k + (i2 + 1)) * m1k + i1 + irk]
                        + r[((i3 + 2) * m2k + (i2 + 1)) * m1k + i1 + irk];

                    y1[i1] = r[(i3 * m2k + i2) * m1k + i1 + irk]
                        + r[((i3 + 2) * m2k + i2) * m1k + i1 + irk]
                        + r[(i3 * m2k + (i2 + 2)) * m1k + i1 + irk]
                        + r[((i3 + 2) * m2k + (i2 + 2)) * m1k + i1 + irk];
                }
                for j1 in 1..m1j - 1 {
                    i1 = (j1 << 1) - d1;
                    y2 = r[(i3 * m2k + i2) * m1k + i1 + 1 + irk]
                        + r[((i3 + 2) * m2k + i2) * m1k + i1 + 1 + irk]
                        + r[(i3 * m2k + (i2 + 2)) * m1k + i1 + 1 + irk]
                        + r[((i3 + 2) * m2k + (i2 + 2)) * m1k + i1 + 1 + irk];

                    x2 = r[((i3 + 1) * m2k + i2) * m1k + i1 + 1 + irk]
                        + r[((i3 + 1) * m2k + (i2 + 2)) * m1k + i1 + 1 + irk]
                        + r[(i3 * m2k + (i2 + 1)) * m1k + i1 + 1 + irk]
                        + r[((i3 + 2) * m2k + (i2 + 1)) * m1k + i1 + 1 + irk];
                    r[(j3 * m2j + j2) * m1j + j1 + irj] = 0.5
                        * r[((i3 + 1) * m2k + (i2 + 1)) * m1k + i1 + 1 + irk]
                        + 0.25
                            * (r[((i3 + 1) * m2k + (i2 + 1)) * m1k + i1 + irk]
                                + r[((i3 + 1) * m2k + (i2 + 1)) * m1k + i1 + 2 + irk]
                                + x2)
                        + 0.125 * (x1[i1] + x1[i1 + 2] + y2)
                        + 0.0625 * (y1[i1] + y1[i1 + 2]);
                }
            }
        }
    }
    if TIMERS {
        timers.stop(T_RPRJ3);
    }

    comm3(&mut r[irj..], m1j, m2j, m3j, timers);

    if DEBUG_VEC[0] >= 1 {
        rep_nrm(
            r,
            m1j,
            m2j,
            m3j,
            String::from("   rprj3"),
            nx,
            ny,
            nz,
            k - 1,
            irj,
            timers,
        );
    }

    if DEBUG_VEC[4] >= k {
        showall(r, m1j, m2j, m3j, irj);
    }
}

fn zero3(z: &mut [f64], n1: usize, n2: usize, n3: usize) {
    for i3 in 0..n3 {
        for i2 in 0..n2 {
            for i1 in 0..n1 {
                if UNSAFE {
                    unsafe {
                        *z.get_unchecked_mut(i3 * n2 * n1 + i2 * n1 + i1) = 0.0;
                    }
                } else {
                    z[i3 * n2 * n1 + i2 * n1 + i1] = 0.0;
                }
            }
        }
    }
}

fn showall(z: &mut [f64], n1: usize, n2: usize, n3: usize, n: usize) {
    let m1: usize = if n1 < 18 { n1 } else { 18 };
    let m2: usize = if n2 < 14 { n2 } else { 14 };
    let m3: usize = if n3 < 18 { n3 } else { 18 };
    print!("\n");
    for i3 in 0..m3 {
        for i2 in 0..m2 {
            for i1 in 0..m1 {
                print!("{:>6.3}", z[i3 * n2 * n1 + i2 * n1 + i1 + n]);
            }
            print!("\n");
        }
        print!(" - - - - - - - \n");
    }
    print!("\n");
}

/*
 * ---------------------------------------------------------------------
 * zran3 loads +1 at ten randomly chosen points,
 * loads -1 at a different ten random points,
 * and zero elsewhere.
 * ---------------------------------------------------------------------
 */
fn zran3(
    z: &mut [f64],
    n1: usize,
    n2: usize,
    n3: usize,
    nx: usize,
    ny: usize,
    is1: usize,
    is2: usize,
    is3: usize,
    ie1: usize,
    ie2: usize,
    ie3: usize,
    timers: &mut Timer,
) {
    let (mut i0, m0, m1): (usize, usize, usize);
    let (mut i1, d1, _e1, e2, e3): (usize, usize, usize, usize, usize);
    let (mut xx, mut x0, mut x1, a1, a2, ai): (f64, f64, f64, f64, f64, f64);
    let mut best: f64;
    let i: usize;
    let mut ten: [[f64; MM]; 2] = [[0.0; MM]; 2];
    let mut j1: [[usize; MM]; 2] = [[0; MM]; 2];
    let mut j2: [[usize; MM]; 2] = [[0; MM]; 2];
    let mut j3: [[usize; MM]; 2] = [[0; MM]; 2];
    let mut jg: [[[usize; 4]; MM]; 2] = [[[0; 4]; MM]; 2];

    a1 = power(A, nx);
    a2 = power(A, nx * ny);

    zero3(z, n1, n2, n3);

    i = is1 - 2 + nx * (is2 - 2 + ny * (is3 - 2));

    ai = power(A, i);
    d1 = ie1 - is1 + 1;
    _e1 = ie1 - is1 + 2;
    e2 = ie2 - is2 + 2;
    e3 = ie3 - is3 + 2;
    x0 = X;
    randlc(&mut x0, ai);

    for i3 in 1..e3 {
        x1 = x0;
        for i2 in 1..e2 {
            xx = x1;
            vranlc(
                d1 as i32,
                &mut xx,
                A,
                &mut z[((i3 * (n2 * n1)) + (i2 * n1) + 1)..],
            );
            randlc(&mut x1, a1);
        }
        randlc(&mut x0, a2);
    }

    /*
     * ---------------------------------------------------------------------
     * each processor looks for twenty candidates
     * ---------------------------------------------------------------------
     */
    for i in 0..MM {
        ten[0][i] = 1.0;
    }
    for i3 in 1..n3 - 1 {
        for i2 in 1..n2 - 1 {
            for i1 in 1..n1 - 1 {
                if z[(i3 * n2 + i2) * n1 + i1] > ten[1][0] {
                    ten[1][0] = z[(i3 * n2 + i2) * n1 + i1];
                    j1[1][0] = i1;
                    j2[1][0] = i2;
                    j3[1][0] = i3;
                    bubble(&mut ten, &mut j1, &mut j2, &mut j3, MM, 1);
                }
                if z[(i3 * n2 + i2) * n1 + i1] < ten[0][0] {
                    ten[0][0] = z[(i3 * n2 + i2) * n1 + i1];
                    j1[0][0] = i1;
                    j2[0][0] = i2;
                    j3[0][0] = i3;
                    bubble(&mut ten, &mut j1, &mut j2, &mut j3, MM, 0);
                }
            }
        }
    }

    i1 = MM - 1;
    i0 = MM - 1;
    for i in (0..MM).rev() {
        best = 0.0;
        if best < ten[1][i1] {
            jg[1][i][0] = 0;
            jg[1][i][1] = is1 - 2 + j1[1][i1];
            jg[1][i][2] = is2 - 2 + j2[1][i1];
            jg[1][i][3] = is3 - 2 + j3[1][i1];
            if i1 != 0 {
                i1 = i1 - 1;
            }
        } else {
            jg[1][i][0] = 0;
            jg[1][i][1] = 0;
            jg[1][i][2] = 0;
            jg[1][i][3] = 0;
        }
        best = 1.0;
        if best > ten[0][i0] {
            jg[0][i][0] = 0;
            jg[0][i][1] = is1 - 2 + j1[0][i0];
            jg[0][i][2] = is2 - 2 + j2[0][i0];
            jg[0][i][3] = is3 - 2 + j3[0][i0];
            if i0 != 0 {
                i0 = i0 - 1;
            }
        } else {
            jg[0][i][0] = 0;
            jg[0][i][1] = 0;
            jg[0][i][2] = 0;
            jg[0][i][3] = 0;
        }
    }
    m1 = 0;
    m0 = 0;

    for i in 0..n1 * n2 * n3 {
        z[i] = 0.0;
    }

    for i in (m0..MM).rev() {
        z[(jg[0][i][3] * n2 * n1) + (jg[0][i][2] * n1) + jg[0][i][1]] = -1.0;
    }
    for i in (m1..MM).rev() {
        z[(jg[1][i][3] * n2 * n1) + (jg[1][i][2] * n1) + jg[1][i][1]] = 1.0;
    }

    comm3(z, n1, n2, n3, timers);
}

const fn setup() -> (
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    [usize; MAXLEVEL + 1],
    [usize; MAXLEVEL + 1],
    [usize; MAXLEVEL + 1],
    [usize; MAXLEVEL + 1],
    [usize; MAXLEVEL + 1],
    [usize; MAXLEVEL + 1],
) {
    let mut nx = [0; MAXLEVEL + 1];
    let mut ny = [0; MAXLEVEL + 1];
    let mut nz = [0; MAXLEVEL + 1];
    let mut m1 = [0; MAXLEVEL + 1];
    let mut m2 = [0; MAXLEVEL + 1];
    let mut m3 = [0; MAXLEVEL + 1];

    nx[LT_DEFAULT] = NX_DEFAULT;
    ny[LT_DEFAULT] = NY_DEFAULT;
    nz[LT_DEFAULT] = NZ_DEFAULT;

    let mut mi: [[usize; 3]; MAXLEVEL + 1] = [[0; 3]; MAXLEVEL + 1];
    let mut ng: [[usize; 3]; MAXLEVEL + 1] = [[0; 3]; MAXLEVEL + 1];

    ng[LT_DEFAULT][0] = nx[LT_DEFAULT];
    ng[LT_DEFAULT][1] = ny[LT_DEFAULT];
    ng[LT_DEFAULT][2] = nz[LT_DEFAULT];

    let mut ax = 0;
    while ax < 3 {
        let mut k = LT_DEFAULT - 1;
        while k > 0 {
            ng[k][ax] = ng[k + 1][ax] >> 1;
            k -= 1;
        }
        ax += 1;
    }

    let mut k = LT_DEFAULT;
    while k > 0 {
        nx[k] = ng[k][0];
        ny[k] = ng[k][1];
        nz[k] = ng[k][2];
        k -= 1;
    }

    k = LT_DEFAULT;
    while k > 0 {
        let mut ax = 0;
        while ax < 3 {
            mi[k][ax] = 2 + ng[k][ax];
            ax += 1;
        }
        m1[k] = mi[k][0];
        m2[k] = mi[k][1];
        m3[k] = mi[k][2];
        k -= 1;
    }

    let is1 = 2 + ng[LT_DEFAULT][0] - ng[LT_DEFAULT][0];
    let ie1 = 1 + ng[LT_DEFAULT][0];
    let n1 = 3 + ie1 - is1;
    let is2 = 2 + ng[LT_DEFAULT][1] - ng[LT_DEFAULT][1];
    let ie2 = 1 + ng[LT_DEFAULT][1];
    let n2 = 3 + ie2 - is2;
    let is3 = 2 + ng[LT_DEFAULT][2] - ng[LT_DEFAULT][2];
    let ie3 = 1 + ng[LT_DEFAULT][2];
    let n3 = 3 + ie3 - is3;

    (
        n1, n2, n3, is1, ie1, is2, ie2, is3, ie3, nx, ny, nz, m1, m2, m3,
    )
}
