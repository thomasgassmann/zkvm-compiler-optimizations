#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

use npb_common::print_results::*;
use npb_common::timers::*;

#[cfg(class = "Z")]
mod params {
    pub const CLASS: char = 'Z';
    pub const PROBLEM_SIZE: usize = 5;
    pub const DT_DEFAULT: f64 = 0.010;
    pub const NITER_DEFAULT: i32 = 1;
}

#[cfg(class = "S")]
mod params {
    pub const CLASS: char = 'S';
    pub const PROBLEM_SIZE: usize = 12;
    pub const DT_DEFAULT: f64 = 0.010;
    pub const NITER_DEFAULT: i32 = 60;
}

#[cfg(class = "W")]
mod params {
    pub const CLASS: char = 'W';
    pub const PROBLEM_SIZE: usize = 24;
    pub const DT_DEFAULT: f64 = 0.0008;
    pub const NITER_DEFAULT: i32 = 200;
}

#[cfg(class = "A")]
mod params {
    pub const CLASS: char = 'A';
    pub const PROBLEM_SIZE: usize = 64;
    pub const DT_DEFAULT: f64 = 0.0008;
    pub const NITER_DEFAULT: i32 = 200;
}

#[cfg(class = "B")]
mod params {
    pub const CLASS: char = 'B';
    pub const PROBLEM_SIZE: usize = 102;
    pub const DT_DEFAULT: f64 = 0.0003;
    pub const NITER_DEFAULT: i32 = 200;
}

#[cfg(class = "C")]
mod params {
    pub const CLASS: char = 'C';
    pub const PROBLEM_SIZE: usize = 162;
    pub const DT_DEFAULT: f64 = 0.0001;
    pub const NITER_DEFAULT: i32 = 200;
}

#[cfg(class = "D")]
mod params {
    pub const CLASS: char = 'D';
    pub const PROBLEM_SIZE: usize = 408;
    pub const DT_DEFAULT: f64 = 0.00002;
    pub const NITER_DEFAULT: i32 = 250;
}

#[cfg(class = "E")]
mod params {
    pub const CLASS: char = 'E';
    pub const PROBLEM_SIZE: usize = 1020;
    pub const DT_DEFAULT: f64 = 0.4e-5;
    pub const NITER_DEFAULT: i32 = 250;
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
    //Never used
    pub const CLASS: char = 'U';
    pub const PROBLEM_SIZE: usize = 1;
    pub const DT_DEFAULT: f64 = 1.0;
    pub const NITER_DEFAULT: i32 = 1;
    compile_error!(
        "\n\n\
		Must set a class at compilation time by setting RUSTFLAGS\n\
		class options for BT are: {S, W, A, B, C, D, E}\n\
		For example:\n\
		RUSTFLAGS='--cfg class=\"A\" ' cargo build --release --bin bt\n\n\n\
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

pub const IMAX: usize = PROBLEM_SIZE;
pub const JMAX: usize = PROBLEM_SIZE;
pub const KMAX: usize = PROBLEM_SIZE;
pub const IMAXP: usize = IMAX / 2 * 2;
pub const JMAXP: usize = JMAX / 2 * 2;
pub const AA: usize = 0;
pub const BB: usize = 1;
pub const CC: usize = 2;
pub const BLOCK_SIZE: i32 = 5;
pub const T_TOTAL: usize = 1;
pub const T_RHSX: usize = 2;
pub const T_RHSY: usize = 3;
pub const T_RHSZ: usize = 4;
pub const T_RHS: usize = 5;
pub const T_XSOLVE: usize = 6;
pub const T_YSOLVE: usize = 7;
pub const T_ZSOLVE: usize = 8;
pub const T_RDIS1: usize = 9;
pub const T_RDIS2: usize = 10;
pub const T_ADD: usize = 11;
pub const T_LAST: usize = 11;

pub const EPSILON: f64 = 1.0e-08;
pub const GRID_POINTS: &[usize] = &[PROBLEM_SIZE, PROBLEM_SIZE, PROBLEM_SIZE];
pub const C1: f64 = 1.4;
pub const C2: f64 = 0.4;
pub const C3: f64 = 0.1;
pub const C4: f64 = 1.0;
pub const C5: f64 = 1.4;
pub const C1C2: f64 = C1 * C2;
pub const C1C5: f64 = C1 * C5;
pub const C3C4: f64 = C3 * C4;
pub const DNXM1: f64 = 1.0 / (GRID_POINTS[0] - 1) as f64;
pub const DNYM1: f64 = 1.0 / (GRID_POINTS[1] - 1) as f64;
pub const DNZM1: f64 = 1.0 / (GRID_POINTS[2] - 1) as f64;
pub const TX1: f64 = 1.0 / (DNXM1 * DNXM1);
pub const TX2: f64 = 1.0 / (2.0 * DNXM1);
pub const TX3: f64 = 1.0 / DNXM1;
pub const TY1: f64 = 1.0 / (DNYM1 * DNYM1);
pub const TY2: f64 = 1.0 / (2.0 * DNYM1);
pub const TY3: f64 = 1.0 / DNYM1;
pub const TZ1: f64 = 1.0 / (DNZM1 * DNZM1);
pub const TZ2: f64 = 1.0 / (2.0 * DNZM1);
pub const TZ3: f64 = 1.0 / DNZM1;
pub const DX1: f64 = 0.75;
pub const DX2: f64 = 0.75;
pub const DX3: f64 = 0.75;
pub const DX4: f64 = 0.75;
pub const DX5: f64 = 0.75;
pub const DY1: f64 = 0.75;
pub const DY2: f64 = 0.75;
pub const DY3: f64 = 0.75;
pub const DY4: f64 = 0.75;
pub const DY5: f64 = 0.75;
pub const DZ1: f64 = 1.0;
pub const DZ2: f64 = 1.0;
pub const DZ3: f64 = 1.0;
pub const DZ4: f64 = 1.0;
pub const DZ5: f64 = 1.0;
pub const DX1TX1: f64 = DX1 * TX1;
pub const DX2TX1: f64 = DX2 * TX1;
pub const DX3TX1: f64 = DX3 * TX1;
pub const DX4TX1: f64 = DX4 * TX1;
pub const DX5TX1: f64 = DX5 * TX1;
pub const DY1TY1: f64 = DY1 * TY1;
pub const DY2TY1: f64 = DY2 * TY1;
pub const DY3TY1: f64 = DY3 * TY1;
pub const DY4TY1: f64 = DY4 * TY1;
pub const DY5TY1: f64 = DY5 * TY1;
pub const DZ1TZ1: f64 = DZ1 * TZ1;
pub const DZ2TZ1: f64 = DZ2 * TZ1;
pub const DZ3TZ1: f64 = DZ3 * TZ1;
pub const DZ4TZ1: f64 = DZ4 * TZ1;
pub const DZ5TZ1: f64 = DZ5 * TZ1;
pub const C3C4TX3: f64 = C3C4 * TX3;
pub const C3C4TY3: f64 = C3C4 * TY3;
pub const C3C4TZ3: f64 = C3C4 * TZ3;
pub const DSSP: f64 = {
    let (x, y);
    if DY1 > DZ1 {
        x = DY1
    } else {
        x = DZ1
    }
    if x > DX1 {
        y = x * 0.25
    } else {
        y = DX1 * 0.25
    }
    y
};
pub const DTDSSP: f64 = DT_DEFAULT * DSSP;
pub const C1345: f64 = C1C5 * C3C4;
pub const CONZ1: f64 = 1.0 - C1C5;
pub const CON43: f64 = 4.0 / 3.0;
pub const CON16: f64 = 1.0 / 6.0;
pub const COMZ1: f64 = DTDSSP;
pub const COMZ4: f64 = 4.0 * DTDSSP;
pub const COMZ5: f64 = 5.0 * DTDSSP;
pub const COMZ6: f64 = 6.0 * DTDSSP;
pub const C2IV: f64 = 2.5;
pub const XXCON1: f64 = C3C4TX3 * CON43 * TX3;
pub const XXCON2: f64 = C3C4TX3 * TX3;
pub const XXCON3: f64 = C3C4TX3 * CONZ1 * TX3;
pub const XXCON4: f64 = C3C4TX3 * CON16 * TX3;
pub const XXCON5: f64 = C3C4TX3 * C1C5 * TX3;
pub const YYCON1: f64 = C3C4TY3 * CON43 * TY3;
pub const YYCON2: f64 = C3C4TY3 * TY3;
pub const YYCON3: f64 = C3C4TY3 * CONZ1 * TY3;
pub const YYCON4: f64 = C3C4TY3 * CON16 * TY3;
pub const YYCON5: f64 = C3C4TY3 * C1C5 * TY3;
pub const ZZCON1: f64 = C3C4TZ3 * CON43 * TZ3;
pub const ZZCON2: f64 = C3C4TZ3 * TZ3;
pub const ZZCON3: f64 = C3C4TZ3 * CONZ1 * TZ3;
pub const ZZCON4: f64 = C3C4TZ3 * CON16 * TZ3;
pub const ZZCON5: f64 = C3C4TZ3 * C1C5 * TZ3;
pub const C4DSSP: f64 = 4.0 * DSSP;
pub const C5DSSP: f64 = 5.0 * DSSP;
pub const DTTX1: f64 = DT_DEFAULT * TX1;
pub const DTTX2: f64 = DT_DEFAULT * TX2;
pub const DTTY1: f64 = DT_DEFAULT * TY1;
pub const DTTY2: f64 = DT_DEFAULT * TY2;
pub const DTTZ1: f64 = DT_DEFAULT * TZ1;
pub const DTTZ2: f64 = DT_DEFAULT * TZ2;
pub const C2DTTX1: f64 = 2.0 * DTTX1;
pub const C2DTTY1: f64 = 2.0 * DTTY1;
pub const C2DTTZ1: f64 = 2.0 * DTTZ1;

/* bt */
fn main() {
    #[cfg(feature = "risc0")]
    let n_iter: i32 = risc0_zkvm::guest::env::read();
    #[cfg(feature = "sp1")]
    let n_iter: i32 = sp1_zkvm::io::read();

    let mut us: Vec<[[f64; IMAXP + 1]; JMAXP + 1]> = vec![[[0.0; IMAXP + 1]; JMAXP + 1]; KMAX];
    let mut vs: Vec<[[f64; IMAXP + 1]; JMAXP + 1]> = vec![[[0.0; IMAXP + 1]; JMAXP + 1]; KMAX];
    let mut ws: Vec<[[f64; IMAXP + 1]; JMAXP + 1]> = vec![[[0.0; IMAXP + 1]; JMAXP + 1]; KMAX];
    let mut qs: Vec<[[f64; IMAXP + 1]; JMAXP + 1]> = vec![[[0.0; IMAXP + 1]; JMAXP + 1]; KMAX];
    let mut rho_i: Vec<[[f64; IMAXP + 1]; JMAXP + 1]> = vec![[[0.0; IMAXP + 1]; JMAXP + 1]; KMAX];
    let mut square: Vec<[[f64; IMAXP + 1]; JMAXP + 1]> = vec![[[0.0; IMAXP + 1]; JMAXP + 1]; KMAX];
    let mut forcing: Vec<[[[f64; 5]; IMAXP + 1]; JMAXP + 1]> =
        vec![[[[0.0; 5]; IMAXP + 1]; JMAXP + 1]; KMAX];
    let mut u: Vec<[[[f64; 5]; IMAXP + 1]; JMAXP + 1]> =
        vec![[[[0.0; 5]; IMAXP + 1]; JMAXP + 1]; KMAX];
    let mut rhs: Vec<[[[f64; 5]; IMAXP + 1]; JMAXP + 1]> =
        vec![[[[0.0; 5]; IMAXP + 1]; JMAXP + 1]; KMAX];
    let mut cuf: Vec<f64> = vec![0.0; PROBLEM_SIZE + 1];
    let mut q: Vec<f64> = vec![0.0; PROBLEM_SIZE + 1];
    let mut ue: Vec<[f64; PROBLEM_SIZE + 1]> = vec![[0.0; PROBLEM_SIZE + 1]; 5];
    let mut buf: Vec<[f64; PROBLEM_SIZE + 1]> = vec![[0.0; PROBLEM_SIZE + 1]; 5];
    let mut fjac: Vec<[[f64; 5]; 5]> = vec![[[0.0; 5]; 5]; PROBLEM_SIZE + 1];
    let mut njac: Vec<[[f64; 5]; 5]> = vec![[[0.0; 5]; 5]; PROBLEM_SIZE + 1];
    let mut lhs: Vec<[[[f64; 5]; 5]; 3]> = vec![[[[0.0; 5]; 5]; 3]; PROBLEM_SIZE + 1];
    let mut ce: Vec<[f64; 5]> = vec![[0.0; 5]; 13];
    let mut verified: i8 = 0;

    println!(" Using compiled defaults");
    let mut timers = Timer::new();
    for i in 1..T_LAST + 1 {
        timers.clear(i);
    }

    println!("\n\n NAS Parallel Benchmarks 4.1 Serial Rust version - BT Benchmark\n");
    println!(
        " Size: {} {} {}",
        GRID_POINTS[0], GRID_POINTS[1], GRID_POINTS[2]
    );
    println!(" Iterations: {}    dt: {}", n_iter, DT_DEFAULT);
    println!("");

    /* - - - - - - - - - - SET CONSTANTS - - - - - - - - - - */
    ce[0][0] = 2.0;
    ce[1][0] = 0.0;
    ce[2][0] = 0.0;
    ce[3][0] = 4.0;
    ce[4][0] = 5.0;
    ce[5][0] = 3.0;
    ce[6][0] = 0.5;
    ce[7][0] = 0.02;
    ce[8][0] = 0.01;
    ce[9][0] = 0.03;
    ce[10][0] = 0.5;
    ce[11][0] = 0.4;
    ce[12][0] = 0.3;
    /* */
    ce[0][1] = 1.0;
    ce[1][1] = 0.0;
    ce[2][1] = 0.0;
    ce[3][1] = 0.0;
    ce[4][1] = 1.0;
    ce[5][1] = 2.0;
    ce[6][1] = 3.0;
    ce[7][1] = 0.01;
    ce[8][1] = 0.03;
    ce[9][1] = 0.02;
    ce[10][1] = 0.4;
    ce[11][1] = 0.3;
    ce[12][1] = 0.5;
    /* */
    ce[0][2] = 2.0;
    ce[1][2] = 2.0;
    ce[2][2] = 0.0;
    ce[3][2] = 0.0;
    ce[4][2] = 0.0;
    ce[5][2] = 2.0;
    ce[6][2] = 3.0;
    ce[7][2] = 0.04;
    ce[8][2] = 0.03;
    ce[9][2] = 0.05;
    ce[10][2] = 0.3;
    ce[11][2] = 0.5;
    ce[12][2] = 0.4;
    /* */
    ce[0][3] = 2.0;
    ce[1][3] = 2.0;
    ce[2][3] = 0.0;
    ce[3][3] = 0.0;
    ce[4][3] = 0.0;
    ce[5][3] = 2.0;
    ce[6][3] = 3.0;
    ce[7][3] = 0.03;
    ce[8][3] = 0.05;
    ce[9][3] = 0.04;
    ce[10][3] = 0.2;
    ce[11][3] = 0.1;
    ce[12][3] = 0.3;
    /* */
    ce[0][4] = 5.0;
    ce[1][4] = 4.0;
    ce[2][4] = 3.0;
    ce[3][4] = 2.0;
    ce[4][4] = 0.1;
    ce[5][4] = 0.4;
    ce[6][4] = 0.3;
    ce[7][4] = 0.05;
    ce[8][4] = 0.04;
    ce[9][4] = 0.03;
    ce[10][4] = 0.1;
    ce[11][4] = 0.3;
    ce[12][4] = 0.2;
    /* - - - - - - - - - - END SET CONSTANTS - - - - - - - - - - */

    initialize(&mut u[..], &ce[..]);

    exact_rhs(
        &mut forcing[..],
        &ce[..],
        &mut ue[..],
        &mut buf[..],
        &mut cuf[..],
        &mut q[..],
    );

    /*
     * ---------------------------------------------------------------------
     * do one time step to touch all code, and reinitialize
     * ---------------------------------------------------------------------
     */
    adi(
        &mut rhs[..],
        &mut lhs[..],
        &mut rho_i[..],
        &mut us[..],
        &mut vs[..],
        &mut ws[..],
        &mut square[..],
        &mut qs[..],
        &mut u[..],
        &forcing[..],
        &mut fjac[..],
        &mut njac[..],
        &mut timers,
    );
    initialize(&mut u[..], &ce[..]);
    for i in 1..T_LAST + 1 {
        timers.clear(i);
    }
    timers.start(1);
    for step in 1..n_iter + 1 {
        if step % 20 == 0 || step == 1 {
            println!(" Time step {}", step);
        }
        adi(
            &mut rhs[..],
            &mut lhs[..],
            &mut rho_i[..],
            &mut us[..],
            &mut vs[..],
            &mut ws[..],
            &mut square[..],
            &mut qs[..],
            &mut u[..],
            &forcing[..],
            &mut fjac[..],
            &mut njac[..],
            &mut timers,
        );
    }
    timers.stop(1);
    let mut tmax: f64 = timers.read(1).as_secs_f64();
    verify(
        &mut verified,
        &u[..],
        &ce[..],
        &mut rhs[..],
        &mut rho_i[..],
        &mut us[..],
        &mut vs[..],
        &mut ws[..],
        &mut square[..],
        &mut qs[..],
        &forcing[..],
        &mut timers,
    );
    let n3 = (GRID_POINTS[0] * GRID_POINTS[1] * GRID_POINTS[2]) as f64;
    let navg = (GRID_POINTS[0] + GRID_POINTS[1] + GRID_POINTS[2]) as f64 / 3.0;
    let mops;
    if tmax != 0.0 {
        mops = 1.0e-6
            * n_iter as f64
            * (3478.8 * n3 - 17655.7 * (navg * navg) + 28023.7 * navg)
            / tmax;
    } else {
        mops = 0.0;
    }

    let info = PrintInfo {
        name: String::from("BT"),
        class: CLASS.to_string(),
        size: (GRID_POINTS[0], GRID_POINTS[1], GRID_POINTS[2]),
        num_iter: n_iter,
        time: tmax,
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
        let mut t_names: Vec<String> = vec![String::new(); T_LAST + 1];
        t_names[T_TOTAL] = String::from("total");
        t_names[T_RHSX] = String::from("rhsx");
        t_names[T_RHSY] = String::from("rhsy");
        t_names[T_RHSZ] = String::from("rhsz");
        t_names[T_RHS] = String::from("rhs");
        t_names[T_XSOLVE] = String::from("xsolve");
        t_names[T_YSOLVE] = String::from("ysolve");
        t_names[T_ZSOLVE] = String::from("zsolve");
        t_names[T_RDIS1] = String::from("redist1");
        t_names[T_RDIS2] = String::from("redist2");
        t_names[T_ADD] = String::from("add");

        let mut trecs: [f64; T_LAST + 1] = [0.0; T_LAST + 1];
        for i in 1..T_LAST + 1 {
            trecs[i] = timers.read(i).as_secs_f64();
        }
        if tmax == 0.0 {
            tmax = 1.0;
        }
        println!("  SECTION   Time (secs)");
        for i in 1..T_LAST + 1 {
            let mut t;
            println!(
                "  {:<8}:{:>9.3}  ({:>6.2}%)",
                t_names[i],
                trecs[i],
                trecs[i] * 100.0 / tmax
            );
            if i == T_RHS {
                t = trecs[T_RHSX] + trecs[T_RHSY] + trecs[T_RHSZ];
                println!(
                    "    --> {:>8}:{:>9.3}  ({:>6.2}%)",
                    "sub-rhs",
                    t,
                    t * 100.0 / tmax
                );
                t = trecs[T_RHS] - t;
                println!(
                    "    --> {:>8}:{:>9.3}  ({:>6.2}%)",
                    "rest-rhs",
                    t,
                    t * 100.0 / tmax
                );
            } else if i == T_ZSOLVE {
                t = trecs[T_ZSOLVE] - trecs[T_RDIS1] - trecs[T_RDIS2];
                println!(
                    "    --> {:>8}:{:>9.3}  ({:>6.2}%)",
                    "sub-zsol",
                    t,
                    t * 100.0 / tmax
                );
            } else if i == T_RDIS2 {
                t = trecs[T_RDIS1] + trecs[T_RDIS2];
                println!(
                    "    --> {:>8}:{:>9.3}  ({:>6.2}%)",
                    "redist",
                    t,
                    t * 100.0 / tmax
                );
            }
        }
    }

    #[cfg(feature = "sp1")]
    sp1_zkvm::io::commit(&verified);
    #[cfg(feature = "risc0")]
    risc0_zkvm::guest::env::commit(&verified);
}

fn add(
    u: &mut [[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    rhs: &[[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    timers: &mut Timer,
) {
    if TIMERS {
        timers.start(T_ADD);
    }
    u.iter_mut().zip(rhs.iter()).for_each(|(u, rhs)| {
        u.iter_mut().zip(rhs.iter()).for_each(|(u, rhs)| {
            u.iter_mut().zip(rhs.iter()).for_each(|(u, rhs)| {
                u.iter_mut().zip(rhs.iter()).for_each(|(u, rhs)| {
                    *u += *rhs;
                });
            });
        });
    });
    if TIMERS {
        timers.stop(T_ADD);
    }
}

fn adi(
    rhs: &mut [[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    lhs: &mut [[[[f64; 5]; 5]; 3]],
    rho_i: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    us: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    vs: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    ws: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    square: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    qs: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    u: &mut [[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    forcing: &[[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    fjac: &mut [[[f64; 5]; 5]],
    njac: &mut [[[f64; 5]; 5]],
    timers: &mut Timer,
) {
    compute_rhs(
        &mut rhs[..],
        &mut rho_i[..],
        &mut us[..],
        &mut vs[..],
        &mut ws[..],
        &mut square[..],
        &mut qs[..],
        &u[..],
        &forcing[..],
        timers,
    );
    x_solve(
        &rho_i[..],
        &mut fjac[..],
        &mut njac[..],
        &mut lhs[..],
        &mut rhs[..],
        &u[..],
        &qs[..],
        &square[..],
        timers,
    );
    y_solve(
        &rho_i[..],
        &mut fjac[..],
        &mut njac[..],
        &mut lhs[..],
        &mut rhs[..],
        &u[..],
        &qs[..],
        &square[..],
        timers,
    );
    z_solve(
        &mut fjac[..],
        &mut njac[..],
        &mut lhs[..],
        &mut rhs[..],
        &u[..],
        &qs[..],
        &square[..],
        timers,
    );
    add(&mut u[..], &rhs[..], timers);
}

fn binvcrhs(
    lhs: &mut [[[[f64; 5]; 5]; 3]],
    i1: usize,
    j1: usize,
    i2: usize,
    j2: usize,
    rhs: &mut [f64],
) {
    let mut pivot = 1.00 / lhs[i1][j1][0][0];
    lhs[i1][j1][1][0] = lhs[i1][j1][1][0] * pivot;
    lhs[i1][j1][2][0] = lhs[i1][j1][2][0] * pivot;
    lhs[i1][j1][3][0] = lhs[i1][j1][3][0] * pivot;
    lhs[i1][j1][4][0] = lhs[i1][j1][4][0] * pivot;
    lhs[i2][j2][0][0] = lhs[i2][j2][0][0] * pivot;
    lhs[i2][j2][1][0] = lhs[i2][j2][1][0] * pivot;
    lhs[i2][j2][2][0] = lhs[i2][j2][2][0] * pivot;
    lhs[i2][j2][3][0] = lhs[i2][j2][3][0] * pivot;
    lhs[i2][j2][4][0] = lhs[i2][j2][4][0] * pivot;
    rhs[0] = rhs[0] * pivot;
    /* */
    let mut coeff = lhs[i1][j1][0][1];
    lhs[i1][j1][1][1] = lhs[i1][j1][1][1] - coeff * lhs[i1][j1][1][0];
    lhs[i1][j1][2][1] = lhs[i1][j1][2][1] - coeff * lhs[i1][j1][2][0];
    lhs[i1][j1][3][1] = lhs[i1][j1][3][1] - coeff * lhs[i1][j1][3][0];
    lhs[i1][j1][4][1] = lhs[i1][j1][4][1] - coeff * lhs[i1][j1][4][0];
    lhs[i2][j2][0][1] = lhs[i2][j2][0][1] - coeff * lhs[i2][j2][0][0];
    lhs[i2][j2][1][1] = lhs[i2][j2][1][1] - coeff * lhs[i2][j2][1][0];
    lhs[i2][j2][2][1] = lhs[i2][j2][2][1] - coeff * lhs[i2][j2][2][0];
    lhs[i2][j2][3][1] = lhs[i2][j2][3][1] - coeff * lhs[i2][j2][3][0];
    lhs[i2][j2][4][1] = lhs[i2][j2][4][1] - coeff * lhs[i2][j2][4][0];
    rhs[1] = rhs[1] - coeff * rhs[0];
    /* */
    coeff = lhs[i1][j1][0][2];
    lhs[i1][j1][1][2] = lhs[i1][j1][1][2] - coeff * lhs[i1][j1][1][0];
    lhs[i1][j1][2][2] = lhs[i1][j1][2][2] - coeff * lhs[i1][j1][2][0];
    lhs[i1][j1][3][2] = lhs[i1][j1][3][2] - coeff * lhs[i1][j1][3][0];
    lhs[i1][j1][4][2] = lhs[i1][j1][4][2] - coeff * lhs[i1][j1][4][0];
    lhs[i2][j2][0][2] = lhs[i2][j2][0][2] - coeff * lhs[i2][j2][0][0];
    lhs[i2][j2][1][2] = lhs[i2][j2][1][2] - coeff * lhs[i2][j2][1][0];
    lhs[i2][j2][2][2] = lhs[i2][j2][2][2] - coeff * lhs[i2][j2][2][0];
    lhs[i2][j2][3][2] = lhs[i2][j2][3][2] - coeff * lhs[i2][j2][3][0];
    lhs[i2][j2][4][2] = lhs[i2][j2][4][2] - coeff * lhs[i2][j2][4][0];
    rhs[2] = rhs[2] - coeff * rhs[0];
    /* */
    coeff = lhs[i1][j1][0][3];
    lhs[i1][j1][1][3] = lhs[i1][j1][1][3] - coeff * lhs[i1][j1][1][0];
    lhs[i1][j1][2][3] = lhs[i1][j1][2][3] - coeff * lhs[i1][j1][2][0];
    lhs[i1][j1][3][3] = lhs[i1][j1][3][3] - coeff * lhs[i1][j1][3][0];
    lhs[i1][j1][4][3] = lhs[i1][j1][4][3] - coeff * lhs[i1][j1][4][0];
    lhs[i2][j2][0][3] = lhs[i2][j2][0][3] - coeff * lhs[i2][j2][0][0];
    lhs[i2][j2][1][3] = lhs[i2][j2][1][3] - coeff * lhs[i2][j2][1][0];
    lhs[i2][j2][2][3] = lhs[i2][j2][2][3] - coeff * lhs[i2][j2][2][0];
    lhs[i2][j2][3][3] = lhs[i2][j2][3][3] - coeff * lhs[i2][j2][3][0];
    lhs[i2][j2][4][3] = lhs[i2][j2][4][3] - coeff * lhs[i2][j2][4][0];
    rhs[3] = rhs[3] - coeff * rhs[0];
    /* */
    coeff = lhs[i1][j1][0][4];
    lhs[i1][j1][1][4] = lhs[i1][j1][1][4] - coeff * lhs[i1][j1][1][0];
    lhs[i1][j1][2][4] = lhs[i1][j1][2][4] - coeff * lhs[i1][j1][2][0];
    lhs[i1][j1][3][4] = lhs[i1][j1][3][4] - coeff * lhs[i1][j1][3][0];
    lhs[i1][j1][4][4] = lhs[i1][j1][4][4] - coeff * lhs[i1][j1][4][0];
    lhs[i2][j2][0][4] = lhs[i2][j2][0][4] - coeff * lhs[i2][j2][0][0];
    lhs[i2][j2][1][4] = lhs[i2][j2][1][4] - coeff * lhs[i2][j2][1][0];
    lhs[i2][j2][2][4] = lhs[i2][j2][2][4] - coeff * lhs[i2][j2][2][0];
    lhs[i2][j2][3][4] = lhs[i2][j2][3][4] - coeff * lhs[i2][j2][3][0];
    lhs[i2][j2][4][4] = lhs[i2][j2][4][4] - coeff * lhs[i2][j2][4][0];
    rhs[4] = rhs[4] - coeff * rhs[0];
    /* */
    pivot = 1.00 / lhs[i1][j1][1][1];
    lhs[i1][j1][2][1] = lhs[i1][j1][2][1] * pivot;
    lhs[i1][j1][3][1] = lhs[i1][j1][3][1] * pivot;
    lhs[i1][j1][4][1] = lhs[i1][j1][4][1] * pivot;
    lhs[i2][j2][0][1] = lhs[i2][j2][0][1] * pivot;
    lhs[i2][j2][1][1] = lhs[i2][j2][1][1] * pivot;
    lhs[i2][j2][2][1] = lhs[i2][j2][2][1] * pivot;
    lhs[i2][j2][3][1] = lhs[i2][j2][3][1] * pivot;
    lhs[i2][j2][4][1] = lhs[i2][j2][4][1] * pivot;
    rhs[1] = rhs[1] * pivot;
    /* */
    coeff = lhs[i1][j1][1][0];
    lhs[i1][j1][2][0] = lhs[i1][j1][2][0] - coeff * lhs[i1][j1][2][1];
    lhs[i1][j1][3][0] = lhs[i1][j1][3][0] - coeff * lhs[i1][j1][3][1];
    lhs[i1][j1][4][0] = lhs[i1][j1][4][0] - coeff * lhs[i1][j1][4][1];
    lhs[i2][j2][0][0] = lhs[i2][j2][0][0] - coeff * lhs[i2][j2][0][1];
    lhs[i2][j2][1][0] = lhs[i2][j2][1][0] - coeff * lhs[i2][j2][1][1];
    lhs[i2][j2][2][0] = lhs[i2][j2][2][0] - coeff * lhs[i2][j2][2][1];
    lhs[i2][j2][3][0] = lhs[i2][j2][3][0] - coeff * lhs[i2][j2][3][1];
    lhs[i2][j2][4][0] = lhs[i2][j2][4][0] - coeff * lhs[i2][j2][4][1];
    rhs[0] = rhs[0] - coeff * rhs[1];
    /* */
    coeff = lhs[i1][j1][1][2];
    lhs[i1][j1][2][2] = lhs[i1][j1][2][2] - coeff * lhs[i1][j1][2][1];
    lhs[i1][j1][3][2] = lhs[i1][j1][3][2] - coeff * lhs[i1][j1][3][1];
    lhs[i1][j1][4][2] = lhs[i1][j1][4][2] - coeff * lhs[i1][j1][4][1];
    lhs[i2][j2][0][2] = lhs[i2][j2][0][2] - coeff * lhs[i2][j2][0][1];
    lhs[i2][j2][1][2] = lhs[i2][j2][1][2] - coeff * lhs[i2][j2][1][1];
    lhs[i2][j2][2][2] = lhs[i2][j2][2][2] - coeff * lhs[i2][j2][2][1];
    lhs[i2][j2][3][2] = lhs[i2][j2][3][2] - coeff * lhs[i2][j2][3][1];
    lhs[i2][j2][4][2] = lhs[i2][j2][4][2] - coeff * lhs[i2][j2][4][1];
    rhs[2] = rhs[2] - coeff * rhs[1];
    /* */
    coeff = lhs[i1][j1][1][3];
    lhs[i1][j1][2][3] = lhs[i1][j1][2][3] - coeff * lhs[i1][j1][2][1];
    lhs[i1][j1][3][3] = lhs[i1][j1][3][3] - coeff * lhs[i1][j1][3][1];
    lhs[i1][j1][4][3] = lhs[i1][j1][4][3] - coeff * lhs[i1][j1][4][1];
    lhs[i2][j2][0][3] = lhs[i2][j2][0][3] - coeff * lhs[i2][j2][0][1];
    lhs[i2][j2][1][3] = lhs[i2][j2][1][3] - coeff * lhs[i2][j2][1][1];
    lhs[i2][j2][2][3] = lhs[i2][j2][2][3] - coeff * lhs[i2][j2][2][1];
    lhs[i2][j2][3][3] = lhs[i2][j2][3][3] - coeff * lhs[i2][j2][3][1];
    lhs[i2][j2][4][3] = lhs[i2][j2][4][3] - coeff * lhs[i2][j2][4][1];
    rhs[3] = rhs[3] - coeff * rhs[1];
    /* */
    coeff = lhs[i1][j1][1][4];
    lhs[i1][j1][2][4] = lhs[i1][j1][2][4] - coeff * lhs[i1][j1][2][1];
    lhs[i1][j1][3][4] = lhs[i1][j1][3][4] - coeff * lhs[i1][j1][3][1];
    lhs[i1][j1][4][4] = lhs[i1][j1][4][4] - coeff * lhs[i1][j1][4][1];
    lhs[i2][j2][0][4] = lhs[i2][j2][0][4] - coeff * lhs[i2][j2][0][1];
    lhs[i2][j2][1][4] = lhs[i2][j2][1][4] - coeff * lhs[i2][j2][1][1];
    lhs[i2][j2][2][4] = lhs[i2][j2][2][4] - coeff * lhs[i2][j2][2][1];
    lhs[i2][j2][3][4] = lhs[i2][j2][3][4] - coeff * lhs[i2][j2][3][1];
    lhs[i2][j2][4][4] = lhs[i2][j2][4][4] - coeff * lhs[i2][j2][4][1];
    rhs[4] = rhs[4] - coeff * rhs[1];
    /* */
    pivot = 1.00 / lhs[i1][j1][2][2];
    lhs[i1][j1][3][2] = lhs[i1][j1][3][2] * pivot;
    lhs[i1][j1][4][2] = lhs[i1][j1][4][2] * pivot;
    lhs[i2][j2][0][2] = lhs[i2][j2][0][2] * pivot;
    lhs[i2][j2][1][2] = lhs[i2][j2][1][2] * pivot;
    lhs[i2][j2][2][2] = lhs[i2][j2][2][2] * pivot;
    lhs[i2][j2][3][2] = lhs[i2][j2][3][2] * pivot;
    lhs[i2][j2][4][2] = lhs[i2][j2][4][2] * pivot;
    rhs[2] = rhs[2] * pivot;
    /* */
    coeff = lhs[i1][j1][2][0];
    lhs[i1][j1][3][0] = lhs[i1][j1][3][0] - coeff * lhs[i1][j1][3][2];
    lhs[i1][j1][4][0] = lhs[i1][j1][4][0] - coeff * lhs[i1][j1][4][2];
    lhs[i2][j2][0][0] = lhs[i2][j2][0][0] - coeff * lhs[i2][j2][0][2];
    lhs[i2][j2][1][0] = lhs[i2][j2][1][0] - coeff * lhs[i2][j2][1][2];
    lhs[i2][j2][2][0] = lhs[i2][j2][2][0] - coeff * lhs[i2][j2][2][2];
    lhs[i2][j2][3][0] = lhs[i2][j2][3][0] - coeff * lhs[i2][j2][3][2];
    lhs[i2][j2][4][0] = lhs[i2][j2][4][0] - coeff * lhs[i2][j2][4][2];
    rhs[0] = rhs[0] - coeff * rhs[2];
    /* */
    coeff = lhs[i1][j1][2][1];
    lhs[i1][j1][3][1] = lhs[i1][j1][3][1] - coeff * lhs[i1][j1][3][2];
    lhs[i1][j1][4][1] = lhs[i1][j1][4][1] - coeff * lhs[i1][j1][4][2];
    lhs[i2][j2][0][1] = lhs[i2][j2][0][1] - coeff * lhs[i2][j2][0][2];
    lhs[i2][j2][1][1] = lhs[i2][j2][1][1] - coeff * lhs[i2][j2][1][2];
    lhs[i2][j2][2][1] = lhs[i2][j2][2][1] - coeff * lhs[i2][j2][2][2];
    lhs[i2][j2][3][1] = lhs[i2][j2][3][1] - coeff * lhs[i2][j2][3][2];
    lhs[i2][j2][4][1] = lhs[i2][j2][4][1] - coeff * lhs[i2][j2][4][2];
    rhs[1] = rhs[1] - coeff * rhs[2];
    /* */
    coeff = lhs[i1][j1][2][3];
    lhs[i1][j1][3][3] = lhs[i1][j1][3][3] - coeff * lhs[i1][j1][3][2];
    lhs[i1][j1][4][3] = lhs[i1][j1][4][3] - coeff * lhs[i1][j1][4][2];
    lhs[i2][j2][0][3] = lhs[i2][j2][0][3] - coeff * lhs[i2][j2][0][2];
    lhs[i2][j2][1][3] = lhs[i2][j2][1][3] - coeff * lhs[i2][j2][1][2];
    lhs[i2][j2][2][3] = lhs[i2][j2][2][3] - coeff * lhs[i2][j2][2][2];
    lhs[i2][j2][3][3] = lhs[i2][j2][3][3] - coeff * lhs[i2][j2][3][2];
    lhs[i2][j2][4][3] = lhs[i2][j2][4][3] - coeff * lhs[i2][j2][4][2];
    rhs[3] = rhs[3] - coeff * rhs[2];
    /* */
    coeff = lhs[i1][j1][2][4];
    lhs[i1][j1][3][4] = lhs[i1][j1][3][4] - coeff * lhs[i1][j1][3][2];
    lhs[i1][j1][4][4] = lhs[i1][j1][4][4] - coeff * lhs[i1][j1][4][2];
    lhs[i2][j2][0][4] = lhs[i2][j2][0][4] - coeff * lhs[i2][j2][0][2];
    lhs[i2][j2][1][4] = lhs[i2][j2][1][4] - coeff * lhs[i2][j2][1][2];
    lhs[i2][j2][2][4] = lhs[i2][j2][2][4] - coeff * lhs[i2][j2][2][2];
    lhs[i2][j2][3][4] = lhs[i2][j2][3][4] - coeff * lhs[i2][j2][3][2];
    lhs[i2][j2][4][4] = lhs[i2][j2][4][4] - coeff * lhs[i2][j2][4][2];
    rhs[4] = rhs[4] - coeff * rhs[2];
    /* */
    pivot = 1.00 / lhs[i1][j1][3][3];
    lhs[i1][j1][4][3] = lhs[i1][j1][4][3] * pivot;
    lhs[i2][j2][0][3] = lhs[i2][j2][0][3] * pivot;
    lhs[i2][j2][1][3] = lhs[i2][j2][1][3] * pivot;
    lhs[i2][j2][2][3] = lhs[i2][j2][2][3] * pivot;
    lhs[i2][j2][3][3] = lhs[i2][j2][3][3] * pivot;
    lhs[i2][j2][4][3] = lhs[i2][j2][4][3] * pivot;
    rhs[3] = rhs[3] * pivot;
    /* */
    coeff = lhs[i1][j1][3][0];
    lhs[i1][j1][4][0] = lhs[i1][j1][4][0] - coeff * lhs[i1][j1][4][3];
    lhs[i2][j2][0][0] = lhs[i2][j2][0][0] - coeff * lhs[i2][j2][0][3];
    lhs[i2][j2][1][0] = lhs[i2][j2][1][0] - coeff * lhs[i2][j2][1][3];
    lhs[i2][j2][2][0] = lhs[i2][j2][2][0] - coeff * lhs[i2][j2][2][3];
    lhs[i2][j2][3][0] = lhs[i2][j2][3][0] - coeff * lhs[i2][j2][3][3];
    lhs[i2][j2][4][0] = lhs[i2][j2][4][0] - coeff * lhs[i2][j2][4][3];
    rhs[0] = rhs[0] - coeff * rhs[3];
    /* */
    coeff = lhs[i1][j1][3][1];
    lhs[i1][j1][4][1] = lhs[i1][j1][4][1] - coeff * lhs[i1][j1][4][3];
    lhs[i2][j2][0][1] = lhs[i2][j2][0][1] - coeff * lhs[i2][j2][0][3];
    lhs[i2][j2][1][1] = lhs[i2][j2][1][1] - coeff * lhs[i2][j2][1][3];
    lhs[i2][j2][2][1] = lhs[i2][j2][2][1] - coeff * lhs[i2][j2][2][3];
    lhs[i2][j2][3][1] = lhs[i2][j2][3][1] - coeff * lhs[i2][j2][3][3];
    lhs[i2][j2][4][1] = lhs[i2][j2][4][1] - coeff * lhs[i2][j2][4][3];
    rhs[1] = rhs[1] - coeff * rhs[3];
    /* */
    coeff = lhs[i1][j1][3][2];
    lhs[i1][j1][4][2] = lhs[i1][j1][4][2] - coeff * lhs[i1][j1][4][3];
    lhs[i2][j2][0][2] = lhs[i2][j2][0][2] - coeff * lhs[i2][j2][0][3];
    lhs[i2][j2][1][2] = lhs[i2][j2][1][2] - coeff * lhs[i2][j2][1][3];
    lhs[i2][j2][2][2] = lhs[i2][j2][2][2] - coeff * lhs[i2][j2][2][3];
    lhs[i2][j2][3][2] = lhs[i2][j2][3][2] - coeff * lhs[i2][j2][3][3];
    lhs[i2][j2][4][2] = lhs[i2][j2][4][2] - coeff * lhs[i2][j2][4][3];
    rhs[2] = rhs[2] - coeff * rhs[3];
    /* */
    coeff = lhs[i1][j1][3][4];
    lhs[i1][j1][4][4] = lhs[i1][j1][4][4] - coeff * lhs[i1][j1][4][3];
    lhs[i2][j2][0][4] = lhs[i2][j2][0][4] - coeff * lhs[i2][j2][0][3];
    lhs[i2][j2][1][4] = lhs[i2][j2][1][4] - coeff * lhs[i2][j2][1][3];
    lhs[i2][j2][2][4] = lhs[i2][j2][2][4] - coeff * lhs[i2][j2][2][3];
    lhs[i2][j2][3][4] = lhs[i2][j2][3][4] - coeff * lhs[i2][j2][3][3];
    lhs[i2][j2][4][4] = lhs[i2][j2][4][4] - coeff * lhs[i2][j2][4][3];
    rhs[4] = rhs[4] - coeff * rhs[3];
    /* */
    pivot = 1.00 / lhs[i1][j1][4][4];
    lhs[i2][j2][0][4] = lhs[i2][j2][0][4] * pivot;
    lhs[i2][j2][1][4] = lhs[i2][j2][1][4] * pivot;
    lhs[i2][j2][2][4] = lhs[i2][j2][2][4] * pivot;
    lhs[i2][j2][3][4] = lhs[i2][j2][3][4] * pivot;
    lhs[i2][j2][4][4] = lhs[i2][j2][4][4] * pivot;
    rhs[4] = rhs[4] * pivot;
    /* */
    coeff = lhs[i1][j1][4][0];
    lhs[i2][j2][0][0] = lhs[i2][j2][0][0] - coeff * lhs[i2][j2][0][4];
    lhs[i2][j2][1][0] = lhs[i2][j2][1][0] - coeff * lhs[i2][j2][1][4];
    lhs[i2][j2][2][0] = lhs[i2][j2][2][0] - coeff * lhs[i2][j2][2][4];
    lhs[i2][j2][3][0] = lhs[i2][j2][3][0] - coeff * lhs[i2][j2][3][4];
    lhs[i2][j2][4][0] = lhs[i2][j2][4][0] - coeff * lhs[i2][j2][4][4];
    rhs[0] = rhs[0] - coeff * rhs[4];
    /* */
    coeff = lhs[i1][j1][4][1];
    lhs[i2][j2][0][1] = lhs[i2][j2][0][1] - coeff * lhs[i2][j2][0][4];
    lhs[i2][j2][1][1] = lhs[i2][j2][1][1] - coeff * lhs[i2][j2][1][4];
    lhs[i2][j2][2][1] = lhs[i2][j2][2][1] - coeff * lhs[i2][j2][2][4];
    lhs[i2][j2][3][1] = lhs[i2][j2][3][1] - coeff * lhs[i2][j2][3][4];
    lhs[i2][j2][4][1] = lhs[i2][j2][4][1] - coeff * lhs[i2][j2][4][4];
    rhs[1] = rhs[1] - coeff * rhs[4];
    /* */
    coeff = lhs[i1][j1][4][2];
    lhs[i2][j2][0][2] = lhs[i2][j2][0][2] - coeff * lhs[i2][j2][0][4];
    lhs[i2][j2][1][2] = lhs[i2][j2][1][2] - coeff * lhs[i2][j2][1][4];
    lhs[i2][j2][2][2] = lhs[i2][j2][2][2] - coeff * lhs[i2][j2][2][4];
    lhs[i2][j2][3][2] = lhs[i2][j2][3][2] - coeff * lhs[i2][j2][3][4];
    lhs[i2][j2][4][2] = lhs[i2][j2][4][2] - coeff * lhs[i2][j2][4][4];
    rhs[2] = rhs[2] - coeff * rhs[4];
    /* */
    coeff = lhs[i1][j1][4][3];
    lhs[i2][j2][0][3] = lhs[i2][j2][0][3] - coeff * lhs[i2][j2][0][4];
    lhs[i2][j2][1][3] = lhs[i2][j2][1][3] - coeff * lhs[i2][j2][1][4];
    lhs[i2][j2][2][3] = lhs[i2][j2][2][3] - coeff * lhs[i2][j2][2][4];
    lhs[i2][j2][3][3] = lhs[i2][j2][3][3] - coeff * lhs[i2][j2][3][4];
    lhs[i2][j2][4][3] = lhs[i2][j2][4][3] - coeff * lhs[i2][j2][4][4];
    rhs[3] = rhs[3] - coeff * rhs[4];
}

fn binvrhs(lhs: &mut [[f64; 5]], rhs: &mut [f64]) {
    let mut pivot = 1.00 / lhs[0][0];
    lhs[1][0] = lhs[1][0] * pivot;
    lhs[2][0] = lhs[2][0] * pivot;
    lhs[3][0] = lhs[3][0] * pivot;
    lhs[4][0] = lhs[4][0] * pivot;
    rhs[0] = rhs[0] * pivot;
    /* */
    let mut coeff = lhs[0][1];
    lhs[1][1] = lhs[1][1] - coeff * lhs[1][0];
    lhs[2][1] = lhs[2][1] - coeff * lhs[2][0];
    lhs[3][1] = lhs[3][1] - coeff * lhs[3][0];
    lhs[4][1] = lhs[4][1] - coeff * lhs[4][0];
    rhs[1] = rhs[1] - coeff * rhs[0];
    /* */
    coeff = lhs[0][2];
    lhs[1][2] = lhs[1][2] - coeff * lhs[1][0];
    lhs[2][2] = lhs[2][2] - coeff * lhs[2][0];
    lhs[3][2] = lhs[3][2] - coeff * lhs[3][0];
    lhs[4][2] = lhs[4][2] - coeff * lhs[4][0];
    rhs[2] = rhs[2] - coeff * rhs[0];
    /* */
    coeff = lhs[0][3];
    lhs[1][3] = lhs[1][3] - coeff * lhs[1][0];
    lhs[2][3] = lhs[2][3] - coeff * lhs[2][0];
    lhs[3][3] = lhs[3][3] - coeff * lhs[3][0];
    lhs[4][3] = lhs[4][3] - coeff * lhs[4][0];
    rhs[3] = rhs[3] - coeff * rhs[0];
    /* */
    coeff = lhs[0][4];
    lhs[1][4] = lhs[1][4] - coeff * lhs[1][0];
    lhs[2][4] = lhs[2][4] - coeff * lhs[2][0];
    lhs[3][4] = lhs[3][4] - coeff * lhs[3][0];
    lhs[4][4] = lhs[4][4] - coeff * lhs[4][0];
    rhs[4] = rhs[4] - coeff * rhs[0];
    /* */
    pivot = 1.00 / lhs[1][1];
    lhs[2][1] = lhs[2][1] * pivot;
    lhs[3][1] = lhs[3][1] * pivot;
    lhs[4][1] = lhs[4][1] * pivot;
    rhs[1] = rhs[1] * pivot;
    /* */
    coeff = lhs[1][0];
    lhs[2][0] = lhs[2][0] - coeff * lhs[2][1];
    lhs[3][0] = lhs[3][0] - coeff * lhs[3][1];
    lhs[4][0] = lhs[4][0] - coeff * lhs[4][1];
    rhs[0] = rhs[0] - coeff * rhs[1];
    /* */
    coeff = lhs[1][2];
    lhs[2][2] = lhs[2][2] - coeff * lhs[2][1];
    lhs[3][2] = lhs[3][2] - coeff * lhs[3][1];
    lhs[4][2] = lhs[4][2] - coeff * lhs[4][1];
    rhs[2] = rhs[2] - coeff * rhs[1];
    /* */
    coeff = lhs[1][3];
    lhs[2][3] = lhs[2][3] - coeff * lhs[2][1];
    lhs[3][3] = lhs[3][3] - coeff * lhs[3][1];
    lhs[4][3] = lhs[4][3] - coeff * lhs[4][1];
    rhs[3] = rhs[3] - coeff * rhs[1];
    /* */
    coeff = lhs[1][4];
    lhs[2][4] = lhs[2][4] - coeff * lhs[2][1];
    lhs[3][4] = lhs[3][4] - coeff * lhs[3][1];
    lhs[4][4] = lhs[4][4] - coeff * lhs[4][1];
    rhs[4] = rhs[4] - coeff * rhs[1];
    /* */
    pivot = 1.00 / lhs[2][2];
    lhs[3][2] = lhs[3][2] * pivot;
    lhs[4][2] = lhs[4][2] * pivot;
    rhs[2] = rhs[2] * pivot;
    /* */
    coeff = lhs[2][0];
    lhs[3][0] = lhs[3][0] - coeff * lhs[3][2];
    lhs[4][0] = lhs[4][0] - coeff * lhs[4][2];
    rhs[0] = rhs[0] - coeff * rhs[2];
    /* */
    coeff = lhs[2][1];
    lhs[3][1] = lhs[3][1] - coeff * lhs[3][2];
    lhs[4][1] = lhs[4][1] - coeff * lhs[4][2];
    rhs[1] = rhs[1] - coeff * rhs[2];
    /* */
    coeff = lhs[2][3];
    lhs[3][3] = lhs[3][3] - coeff * lhs[3][2];
    lhs[4][3] = lhs[4][3] - coeff * lhs[4][2];
    rhs[3] = rhs[3] - coeff * rhs[2];
    /* */
    coeff = lhs[2][4];
    lhs[3][4] = lhs[3][4] - coeff * lhs[3][2];
    lhs[4][4] = lhs[4][4] - coeff * lhs[4][2];
    rhs[4] = rhs[4] - coeff * rhs[2];
    /* */
    pivot = 1.00 / lhs[3][3];
    lhs[4][3] = lhs[4][3] * pivot;
    rhs[3] = rhs[3] * pivot;
    /* */
    coeff = lhs[3][0];
    lhs[4][0] = lhs[4][0] - coeff * lhs[4][3];
    rhs[0] = rhs[0] - coeff * rhs[3];
    /* */
    coeff = lhs[3][1];
    lhs[4][1] = lhs[4][1] - coeff * lhs[4][3];
    rhs[1] = rhs[1] - coeff * rhs[3];
    /* */
    coeff = lhs[3][2];
    lhs[4][2] = lhs[4][2] - coeff * lhs[4][3];
    rhs[2] = rhs[2] - coeff * rhs[3];
    /* */
    coeff = lhs[3][4];
    lhs[4][4] = lhs[4][4] - coeff * lhs[4][3];
    rhs[4] = rhs[4] - coeff * rhs[3];
    /* */
    pivot = 1.00 / lhs[4][4];
    rhs[4] = rhs[4] * pivot;
    /* */
    coeff = lhs[4][0];
    rhs[0] = rhs[0] - coeff * rhs[4];
    /* */
    coeff = lhs[4][1];
    rhs[1] = rhs[1] - coeff * rhs[4];
    /* */
    coeff = lhs[4][2];
    rhs[2] = rhs[2] - coeff * rhs[4];
    /* */
    coeff = lhs[4][3];
    rhs[3] = rhs[3] - coeff * rhs[4];
}

fn compute_rhs(
    rhs: &mut [[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    rho_i: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    us: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    vs: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    ws: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    square: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    qs: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    u: &[[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    forcing: &[[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    timers: &mut Timer,
) {
    if TIMERS {
        timers.start(T_RHS);
    }
    /*
     * ---------------------------------------------------------------------
     * compute the reciprocal of density, and the kinetic energy,
     * and the speed of sound.
     * ---------------------------------------------------------------------
     */
    for k in 0..GRID_POINTS[2] {
        for j in 0..GRID_POINTS[1] {
            for i in 0..GRID_POINTS[0] {
                let rho_inv: f64 = 1.0 / u[k][j][i][0];
                rho_i[k][j][i] = rho_inv;
                us[k][j][i] = u[k][j][i][1] * rho_inv;
                vs[k][j][i] = u[k][j][i][2] * rho_inv;
                ws[k][j][i] = u[k][j][i][3] * rho_inv;
                square[k][j][i] = 0.5
                    * (u[k][j][i][1] * u[k][j][i][1]
                        + u[k][j][i][2] * u[k][j][i][2]
                        + u[k][j][i][3] * u[k][j][i][3])
                    * rho_inv;
                qs[k][j][i] = square[k][j][i] * rho_inv;
            }
        }
    }
    /*
     * ---------------------------------------------------------------------
     * copy the exact forcing term to the right hand side; because
     * this forcing term is known, we can store it on the whole grid
     * including the boundary
     * ---------------------------------------------------------------------
     */
    rhs.iter_mut()
        .zip(forcing.iter())
        .for_each(|(rhs, forcing)| {
            rhs.iter_mut()
                .zip(forcing.iter())
                .for_each(|(rhs, forcing)| {
                    rhs.iter_mut()
                        .zip(forcing.iter())
                        .for_each(|(rhs, forcing)| {
                            rhs.iter_mut()
                                .zip(forcing.iter())
                                .for_each(|(rhs, forcing)| {
                                    *rhs = *forcing;
                                });
                        });
                });
        });
    if TIMERS {
        timers.start(T_RHSX);
    }
    /*
     * ---------------------------------------------------------------------
     * compute xi-direction fluxes
     * ---------------------------------------------------------------------
     */
    for k in 1..GRID_POINTS[2] - 1 {
        for j in 1..GRID_POINTS[1] - 1 {
            for i in 1..GRID_POINTS[0] - 1 {
                let uijk = us[k][j][i];
                let up1 = us[k][j][i + 1];
                let um1 = us[k][j][i - 1];
                rhs[k][j][i][0] = rhs[k][j][i][0]
                    + DX1TX1 * (u[k][j][i + 1][0] - 2.0 * u[k][j][i][0] + u[k][j][i - 1][0])
                    - TX2 * (u[k][j][i + 1][1] - u[k][j][i - 1][1]);
                rhs[k][j][i][1] = rhs[k][j][i][1]
                    + DX2TX1 * (u[k][j][i + 1][1] - 2.0 * u[k][j][i][1] + u[k][j][i - 1][1])
                    + XXCON2 * CON43 * (up1 - 2.0 * uijk + um1)
                    - TX2
                        * (u[k][j][i + 1][1] * up1 - u[k][j][i - 1][1] * um1
                            + (u[k][j][i + 1][4] - square[k][j][i + 1] - u[k][j][i - 1][4]
                                + square[k][j][i - 1])
                                * C2);
                rhs[k][j][i][2] = rhs[k][j][i][2]
                    + DX3TX1 * (u[k][j][i + 1][2] - 2.0 * u[k][j][i][2] + u[k][j][i - 1][2])
                    + XXCON2 * (vs[k][j][i + 1] - 2.0 * vs[k][j][i] + vs[k][j][i - 1])
                    - TX2 * (u[k][j][i + 1][2] * up1 - u[k][j][i - 1][2] * um1);
                rhs[k][j][i][3] = rhs[k][j][i][3]
                    + DX4TX1 * (u[k][j][i + 1][3] - 2.0 * u[k][j][i][3] + u[k][j][i - 1][3])
                    + XXCON2 * (ws[k][j][i + 1] - 2.0 * ws[k][j][i] + ws[k][j][i - 1])
                    - TX2 * (u[k][j][i + 1][3] * up1 - u[k][j][i - 1][3] * um1);
                rhs[k][j][i][4] = rhs[k][j][i][4]
                    + DX5TX1 * (u[k][j][i + 1][4] - 2.0 * u[k][j][i][4] + u[k][j][i - 1][4])
                    + XXCON3 * (qs[k][j][i + 1] - 2.0 * qs[k][j][i] + qs[k][j][i - 1])
                    + XXCON4 * (up1 * up1 - 2.0 * uijk * uijk + um1 * um1)
                    + XXCON5
                        * (u[k][j][i + 1][4] * rho_i[k][j][i + 1]
                            - 2.0 * u[k][j][i][4] * rho_i[k][j][i]
                            + u[k][j][i - 1][4] * rho_i[k][j][i - 1])
                    - TX2
                        * ((C1 * u[k][j][i + 1][4] - C2 * square[k][j][i + 1]) * up1
                            - (C1 * u[k][j][i - 1][4] - C2 * square[k][j][i - 1]) * um1);
            }
        }
        /*
         * ---------------------------------------------------------------------
         * add fourth order xi-direction dissipation
         * ---------------------------------------------------------------------
         */
        for j in 1..GRID_POINTS[1] - 1 {
            let mut i = 1;
            for m in 0..5 {
                rhs[k][j][i][m] = rhs[k][j][i][m]
                    - DSSP * (5.0 * u[k][j][i][m] - 4.0 * u[k][j][i + 1][m] + u[k][j][i + 2][m]);
            }
            i = 2;
            for m in 0..5 {
                rhs[k][j][i][m] = rhs[k][j][i][m]
                    - DSSP
                        * (-4.0 * u[k][j][i - 1][m] + 6.0 * u[k][j][i][m]
                            - 4.0 * u[k][j][i + 1][m]
                            + u[k][j][i + 2][m]);
            }
        }
        for j in 1..GRID_POINTS[1] - 1 {
            for i in 3..GRID_POINTS[0] - 3 {
                for m in 0..5 {
                    rhs[k][j][i][m] = rhs[k][j][i][m]
                        - DSSP
                            * (u[k][j][i - 2][m] - 4.0 * u[k][j][i - 1][m] + 6.0 * u[k][j][i][m]
                                - 4.0 * u[k][j][i + 1][m]
                                + u[k][j][i + 2][m]);
                }
            }
        }
        for j in 1..GRID_POINTS[1] - 1 {
            let mut i = GRID_POINTS[0] - 3;
            for m in 0..5 {
                rhs[k][j][i][m] = rhs[k][j][i][m]
                    - DSSP
                        * (u[k][j][i - 2][m] - 4.0 * u[k][j][i - 1][m] + 6.0 * u[k][j][i][m]
                            - 4.0 * u[k][j][i + 1][m]);
            }
            i = GRID_POINTS[0] - 2;
            for m in 0..5 {
                rhs[k][j][i][m] = rhs[k][j][i][m]
                    - DSSP * (u[k][j][i - 2][m] - 4.0 * u[k][j][i - 1][m] + 5.0 * u[k][j][i][m]);
            }
        }
    }
    if TIMERS {
        timers.stop(T_RHSX);
    }
    if TIMERS {
        timers.start(T_RHSY);
    }
    /*
     * ---------------------------------------------------------------------
     * compute eta-direction fluxes
     * ---------------------------------------------------------------------
     */
    for k in 1..GRID_POINTS[2] - 1 {
        for j in 1..GRID_POINTS[1] - 1 {
            for i in 1..GRID_POINTS[0] - 1 {
                let vijk = vs[k][j][i];
                let vp1 = vs[k][j + 1][i];
                let vm1 = vs[k][j - 1][i];
                rhs[k][j][i][0] = rhs[k][j][i][0]
                    + DY1TY1 * (u[k][j + 1][i][0] - 2.0 * u[k][j][i][0] + u[k][j - 1][i][0])
                    - TY2 * (u[k][j + 1][i][2] - u[k][j - 1][i][2]);
                rhs[k][j][i][1] = rhs[k][j][i][1]
                    + DY2TY1 * (u[k][j + 1][i][1] - 2.0 * u[k][j][i][1] + u[k][j - 1][i][1])
                    + YYCON2 * (us[k][j + 1][i] - 2.0 * us[k][j][i] + us[k][j - 1][i])
                    - TY2 * (u[k][j + 1][i][1] * vp1 - u[k][j - 1][i][1] * vm1);
                rhs[k][j][i][2] = rhs[k][j][i][2]
                    + DY3TY1 * (u[k][j + 1][i][2] - 2.0 * u[k][j][i][2] + u[k][j - 1][i][2])
                    + YYCON2 * CON43 * (vp1 - 2.0 * vijk + vm1)
                    - TY2
                        * (u[k][j + 1][i][2] * vp1 - u[k][j - 1][i][2] * vm1
                            + (u[k][j + 1][i][4] - square[k][j + 1][i] - u[k][j - 1][i][4]
                                + square[k][j - 1][i])
                                * C2);
                rhs[k][j][i][3] = rhs[k][j][i][3]
                    + DY4TY1 * (u[k][j + 1][i][3] - 2.0 * u[k][j][i][3] + u[k][j - 1][i][3])
                    + YYCON2 * (ws[k][j + 1][i] - 2.0 * ws[k][j][i] + ws[k][j - 1][i])
                    - TY2 * (u[k][j + 1][i][3] * vp1 - u[k][j - 1][i][3] * vm1);
                rhs[k][j][i][4] = rhs[k][j][i][4]
                    + DY5TY1 * (u[k][j + 1][i][4] - 2.0 * u[k][j][i][4] + u[k][j - 1][i][4])
                    + YYCON3 * (qs[k][j + 1][i] - 2.0 * qs[k][j][i] + qs[k][j - 1][i])
                    + YYCON4 * (vp1 * vp1 - 2.0 * vijk * vijk + vm1 * vm1)
                    + YYCON5
                        * (u[k][j + 1][i][4] * rho_i[k][j + 1][i]
                            - 2.0 * u[k][j][i][4] * rho_i[k][j][i]
                            + u[k][j - 1][i][4] * rho_i[k][j - 1][i])
                    - TY2
                        * ((C1 * u[k][j + 1][i][4] - C2 * square[k][j + 1][i]) * vp1
                            - (C1 * u[k][j - 1][i][4] - C2 * square[k][j - 1][i]) * vm1);
            }
        }
        /*
         * ---------------------------------------------------------------------
         * add fourth order eta-direction dissipation
         * ---------------------------------------------------------------------
         */
        let mut j = 1;
        for i in 1..GRID_POINTS[0] - 1 {
            for m in 0..5 {
                rhs[k][j][i][m] = rhs[k][j][i][m]
                    - DSSP * (5.0 * u[k][j][i][m] - 4.0 * u[k][j + 1][i][m] + u[k][j + 2][i][m]);
            }
        }
        j = 2;
        for i in 1..GRID_POINTS[0] - 1 {
            for m in 0..5 {
                rhs[k][j][i][m] = rhs[k][j][i][m]
                    - DSSP
                        * (-4.0 * u[k][j - 1][i][m] + 6.0 * u[k][j][i][m]
                            - 4.0 * u[k][j + 1][i][m]
                            + u[k][j + 2][i][m]);
            }
        }
        for j in 3..GRID_POINTS[1] - 3 {
            for i in 1..GRID_POINTS[0] - 1 {
                for m in 0..5 {
                    rhs[k][j][i][m] = rhs[k][j][i][m]
                        - DSSP
                            * (u[k][j - 2][i][m] - 4.0 * u[k][j - 1][i][m] + 6.0 * u[k][j][i][m]
                                - 4.0 * u[k][j + 1][i][m]
                                + u[k][j + 2][i][m]);
                }
            }
        }
        j = GRID_POINTS[1] - 3;
        for i in 1..GRID_POINTS[0] - 1 {
            for m in 0..5 {
                rhs[k][j][i][m] = rhs[k][j][i][m]
                    - DSSP
                        * (u[k][j - 2][i][m] - 4.0 * u[k][j - 1][i][m] + 6.0 * u[k][j][i][m]
                            - 4.0 * u[k][j + 1][i][m]);
            }
        }
        j = GRID_POINTS[1] - 2;
        for i in 1..GRID_POINTS[0] - 1 {
            for m in 0..5 {
                rhs[k][j][i][m] = rhs[k][j][i][m]
                    - DSSP * (u[k][j - 2][i][m] - 4.0 * u[k][j - 1][i][m] + 5.0 * u[k][j][i][m]);
            }
        }
    }
    if TIMERS {
        timers.stop(T_RHSY);
    }
    if TIMERS {
        timers.start(T_RHSZ);
    }
    /*
     * ---------------------------------------------------------------------
     * compute zeta-direction fluxes
     * ---------------------------------------------------------------------
     */
    for k in 1..GRID_POINTS[2] - 1 {
        for j in 1..GRID_POINTS[1] - 1 {
            for i in 1..GRID_POINTS[0] - 1 {
                let wijk = ws[k][j][i];
                let wp1 = ws[k + 1][j][i];
                let wm1 = ws[k - 1][j][i];
                rhs[k][j][i][0] = rhs[k][j][i][0]
                    + DZ1TZ1 * (u[k + 1][j][i][0] - 2.0 * u[k][j][i][0] + u[k - 1][j][i][0])
                    - TZ2 * (u[k + 1][j][i][3] - u[k - 1][j][i][3]);
                rhs[k][j][i][1] = rhs[k][j][i][1]
                    + DZ2TZ1 * (u[k + 1][j][i][1] - 2.0 * u[k][j][i][1] + u[k - 1][j][i][1])
                    + ZZCON2 * (us[k + 1][j][i] - 2.0 * us[k][j][i] + us[k - 1][j][i])
                    - TZ2 * (u[k + 1][j][i][1] * wp1 - u[k - 1][j][i][1] * wm1);
                rhs[k][j][i][2] = rhs[k][j][i][2]
                    + DZ3TZ1 * (u[k + 1][j][i][2] - 2.0 * u[k][j][i][2] + u[k - 1][j][i][2])
                    + ZZCON2 * (vs[k + 1][j][i] - 2.0 * vs[k][j][i] + vs[k - 1][j][i])
                    - TZ2 * (u[k + 1][j][i][2] * wp1 - u[k - 1][j][i][2] * wm1);
                rhs[k][j][i][3] = rhs[k][j][i][3]
                    + DZ4TZ1 * (u[k + 1][j][i][3] - 2.0 * u[k][j][i][3] + u[k - 1][j][i][3])
                    + ZZCON2 * CON43 * (wp1 - 2.0 * wijk + wm1)
                    - TZ2
                        * (u[k + 1][j][i][3] * wp1 - u[k - 1][j][i][3] * wm1
                            + (u[k + 1][j][i][4] - square[k + 1][j][i] - u[k - 1][j][i][4]
                                + square[k - 1][j][i])
                                * C2);
                rhs[k][j][i][4] = rhs[k][j][i][4]
                    + DZ5TZ1 * (u[k + 1][j][i][4] - 2.0 * u[k][j][i][4] + u[k - 1][j][i][4])
                    + ZZCON3 * (qs[k + 1][j][i] - 2.0 * qs[k][j][i] + qs[k - 1][j][i])
                    + ZZCON4 * (wp1 * wp1 - 2.0 * wijk * wijk + wm1 * wm1)
                    + ZZCON5
                        * (u[k + 1][j][i][4] * rho_i[k + 1][j][i]
                            - 2.0 * u[k][j][i][4] * rho_i[k][j][i]
                            + u[k - 1][j][i][4] * rho_i[k - 1][j][i])
                    - TZ2
                        * ((C1 * u[k + 1][j][i][4] - C2 * square[k + 1][j][i]) * wp1
                            - (C1 * u[k - 1][j][i][4] - C2 * square[k - 1][j][i]) * wm1);
            }
        }
    }
    /*
     * ---------------------------------------------------------------------
     * add fourth order zeta-direction dissipation
     * ---------------------------------------------------------------------
     */
    let mut k = 1;
    for j in 1..GRID_POINTS[1] - 1 {
        for i in 1..GRID_POINTS[0] - 1 {
            for m in 0..5 {
                rhs[k][j][i][m] = rhs[k][j][i][m]
                    - DSSP * (5.0 * u[k][j][i][m] - 4.0 * u[k + 1][j][i][m] + u[k + 2][j][i][m]);
            }
        }
    }
    k = 2;
    for j in 1..GRID_POINTS[1] - 1 {
        for i in 1..GRID_POINTS[0] - 1 {
            for m in 0..5 {
                rhs[k][j][i][m] = rhs[k][j][i][m]
                    - DSSP
                        * (-4.0 * u[k - 1][j][i][m] + 6.0 * u[k][j][i][m]
                            - 4.0 * u[k + 1][j][i][m]
                            + u[k + 2][j][i][m]);
            }
        }
    }
    for k in 3..GRID_POINTS[2] - 3 {
        for j in 1..GRID_POINTS[1] - 1 {
            for i in 1..GRID_POINTS[0] - 1 {
                for m in 0..5 {
                    rhs[k][j][i][m] = rhs[k][j][i][m]
                        - DSSP
                            * (u[k - 2][j][i][m] - 4.0 * u[k - 1][j][i][m] + 6.0 * u[k][j][i][m]
                                - 4.0 * u[k + 1][j][i][m]
                                + u[k + 2][j][i][m]);
                }
            }
        }
    }
    k = GRID_POINTS[2] - 3;
    for j in 1..GRID_POINTS[1] - 1 {
        for i in 1..GRID_POINTS[0] - 1 {
            for m in 0..5 {
                rhs[k][j][i][m] = rhs[k][j][i][m]
                    - DSSP
                        * (u[k - 2][j][i][m] - 4.0 * u[k - 1][j][i][m] + 6.0 * u[k][j][i][m]
                            - 4.0 * u[k + 1][j][i][m]);
            }
        }
    }
    k = GRID_POINTS[2] - 2;
    for j in 1..GRID_POINTS[1] - 1 {
        for i in 1..GRID_POINTS[0] - 1 {
            for m in 0..5 {
                rhs[k][j][i][m] = rhs[k][j][i][m]
                    - DSSP * (u[k - 2][j][i][m] - 4.0 * u[k - 1][j][i][m] + 5.0 * u[k][j][i][m]);
            }
        }
    }
    if TIMERS {
        timers.stop(T_RHSZ);
    }
    rhs.iter_mut().for_each(|rhs| {
        rhs.iter_mut().for_each(|rhs| {
            rhs.iter_mut().for_each(|rhs| {
                rhs.iter_mut().for_each(|rhs| *rhs *= DT_DEFAULT);
            });
        });
    });
    if TIMERS {
        timers.stop(T_RHS);
    }
}

/*
 * ---------------------------------------------------------------------
 * this function computes the norm of the difference between the
 * computed solution and the exact solution
 * ---------------------------------------------------------------------
 */
fn error_norm(rms: &mut [f64], u: &[[[[f64; 5]; IMAXP + 1]; JMAXP + 1]], ce: &[[f64; 5]]) {
    let mut u_exact: [f64; 5] = [0.0; 5];
    rms.iter_mut().for_each(|rms| *rms = 0.0);
    for k in 0..GRID_POINTS[2] {
        let zeta = k as f64 * DNZM1;
        for j in 0..GRID_POINTS[1] {
            let eta = j as f64 * DNYM1;
            for i in 0..GRID_POINTS[0] {
                let xi = i as f64 * DNXM1;
                exact_solution(xi, eta, zeta, &mut u_exact[..], &ce[..]);
                for m in 0..5 {
                    let add = u[k][j][i][m] - u_exact[m];
                    rms[m] = rms[m] + add * add;
                }
            }
        }
    }
    for m in 0..5 {
        for d in 0..3 {
            rms[m] = rms[m] / (GRID_POINTS[d] - 2) as f64;
        }
        rms[m] = f64::sqrt(rms[m]);
    }
}

fn exact_rhs(
    forcing: &mut [[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    ce: &[[f64; 5]],
    ue: &mut [[f64; PROBLEM_SIZE + 1]],
    buf: &mut [[f64; PROBLEM_SIZE + 1]],
    cuf: &mut [f64],
    q: &mut [f64],
) {
    let mut zeta: f64;
    let mut eta: f64;
    let mut xi: f64;
    let mut dtpp: f64;
    let mut dtemp: [f64; 5] = [0.0; 5];
    let mut i;
    let mut j;
    let mut k;
    let mut im1;
    let mut ip1;
    let mut jm1;
    let mut jp1;
    let mut km1;
    let mut kp1;

    /*
     * ---------------------------------------------------------------------
     * initialize
     * ---------------------------------------------------------------------
     */
    forcing.iter_mut().for_each(|f| {
        f.iter_mut().for_each(|f| {
            f.iter_mut().for_each(|f| {
                f.iter_mut().for_each(|f| *f = 0.0);
            });
        });
    });
    /*
     * ---------------------------------------------------------------------
     * xi-direction flux differences
     * ---------------------------------------------------------------------
     */
    for k in 1..GRID_POINTS[2] - 1 {
        zeta = k as f64 * DNZM1;
        for j in 1..GRID_POINTS[1] - 1 {
            eta = j as f64 * DNYM1;
            for i in 0..GRID_POINTS[0] {
                xi = i as f64 * DNXM1;
                exact_solution(xi, eta, zeta, &mut dtemp[..], &ce[..]);
                for m in 0..5 {
                    ue[m][i] = dtemp[m];
                }
                dtpp = 1.0 / dtemp[0];
                for m in 1..5 {
                    buf[m][i] = dtpp * dtemp[m];
                }
                cuf[i] = buf[1][i] * buf[1][i];
                buf[0][i] = cuf[i] + buf[2][i] * buf[2][i] + buf[3][i] * buf[3][i];
                q[i] = 0.5 * (buf[1][i] * ue[1][i] + buf[2][i] * ue[2][i] + buf[3][i] * ue[3][i]);
            }
            for i in 1..GRID_POINTS[0] - 1 {
                im1 = i - 1;
                ip1 = i + 1;
                forcing[k][j][i][0] = forcing[k][j][i][0] - TX2 * (ue[1][ip1] - ue[1][im1])
                    + DX1TX1 * (ue[0][ip1] - 2.0 * ue[0][i] + ue[0][im1]);
                forcing[k][j][i][1] = forcing[k][j][i][1]
                    - TX2
                        * ((ue[1][ip1] * buf[1][ip1] + C2 * (ue[4][ip1] - q[ip1]))
                            - (ue[1][im1] * buf[1][im1] + C2 * (ue[4][im1] - q[im1])))
                    + XXCON1 * (buf[1][ip1] - 2.0 * buf[1][i] + buf[1][im1])
                    + DX2TX1 * (ue[1][ip1] - 2.0 * ue[1][i] + ue[1][im1]);
                forcing[k][j][i][2] = forcing[k][j][i][2]
                    - TX2 * (ue[2][ip1] * buf[1][ip1] - ue[2][im1] * buf[1][im1])
                    + XXCON2 * (buf[2][ip1] - 2.0 * buf[2][i] + buf[2][im1])
                    + DX3TX1 * (ue[2][ip1] - 2.0 * ue[2][i] + ue[2][im1]);
                forcing[k][j][i][3] = forcing[k][j][i][3]
                    - TX2 * (ue[3][ip1] * buf[1][ip1] - ue[3][im1] * buf[1][im1])
                    + XXCON2 * (buf[3][ip1] - 2.0 * buf[3][i] + buf[3][im1])
                    + DX4TX1 * (ue[3][ip1] - 2.0 * ue[3][i] + ue[3][im1]);
                forcing[k][j][i][4] = forcing[k][j][i][4]
                    - TX2
                        * (buf[1][ip1] * (C1 * ue[4][ip1] - C2 * q[ip1])
                            - buf[1][im1] * (C1 * ue[4][im1] - C2 * q[im1]))
                    + 0.5 * XXCON3 * (buf[0][ip1] - 2.0 * buf[0][i] + buf[0][im1])
                    + XXCON4 * (cuf[ip1] - 2.0 * cuf[i] + cuf[im1])
                    + XXCON5 * (buf[4][ip1] - 2.0 * buf[4][i] + buf[4][im1])
                    + DX5TX1 * (ue[4][ip1] - 2.0 * ue[4][i] + ue[4][im1]);
            }
            /*
             * ---------------------------------------------------------------------
             * fourth-order dissipation
             * ---------------------------------------------------------------------
             */
            for m in 0..5 {
                i = 1;
                forcing[k][j][i][m] = forcing[k][j][i][m]
                    - DSSP * (5.0 * ue[m][i] - 4.0 * ue[m][i + 1] + ue[m][i + 2]);
                i = 2;
                forcing[k][j][i][m] = forcing[k][j][i][m]
                    - DSSP
                        * (-4.0 * ue[m][i - 1] + 6.0 * ue[m][i] - 4.0 * ue[m][i + 1]
                            + ue[m][i + 2]);
            }
            for i in 3..GRID_POINTS[0] - 3 {
                for m in 0..5 {
                    forcing[k][j][i][m] = forcing[k][j][i][m]
                        - DSSP
                            * (ue[m][i - 2] - 4.0 * ue[m][i - 1] + 6.0 * ue[m][i]
                                - 4.0 * ue[m][i + 1]
                                + ue[m][i + 2]);
                }
            }
            for m in 0..5 {
                i = GRID_POINTS[0] - 3;
                forcing[k][j][i][m] = forcing[k][j][i][m]
                    - DSSP
                        * (ue[m][i - 2] - 4.0 * ue[m][i - 1] + 6.0 * ue[m][i] - 4.0 * ue[m][i + 1]);
                i = GRID_POINTS[0] - 2;
                forcing[k][j][i][m] = forcing[k][j][i][m]
                    - DSSP * (ue[m][i - 2] - 4.0 * ue[m][i - 1] + 5.0 * ue[m][i]);
            }
        }
    }
    /*
     * ---------------------------------------------------------------------
     * eta-direction flux differences
     * ---------------------------------------------------------------------
     */
    for k in 1..GRID_POINTS[2] - 1 {
        zeta = k as f64 * DNZM1;
        for i in 1..GRID_POINTS[0] - 1 {
            xi = i as f64 * DNXM1;
            for j in 0..GRID_POINTS[1] {
                eta = j as f64 * DNYM1;
                exact_solution(xi, eta, zeta, &mut dtemp[..], &ce[..]);
                for m in 0..5 {
                    ue[m][j] = dtemp[m];
                }
                dtpp = 1.0 / dtemp[0];
                for m in 1..5 {
                    buf[m][j] = dtpp * dtemp[m];
                }
                cuf[j] = buf[2][j] * buf[2][j];
                buf[0][j] = cuf[j] + buf[1][j] * buf[1][j] + buf[3][j] * buf[3][j];
                q[j] = 0.5 * (buf[1][j] * ue[1][j] + buf[2][j] * ue[2][j] + buf[3][j] * ue[3][j]);
            }
            for j in 1..GRID_POINTS[1] - 1 {
                jm1 = j - 1;
                jp1 = j + 1;
                forcing[k][j][i][0] = forcing[k][j][i][0] - TY2 * (ue[2][jp1] - ue[2][jm1])
                    + DY1TY1 * (ue[0][jp1] - 2.0 * ue[0][j] + ue[0][jm1]);
                forcing[k][j][i][1] = forcing[k][j][i][1]
                    - TY2 * (ue[1][jp1] * buf[2][jp1] - ue[1][jm1] * buf[2][jm1])
                    + YYCON2 * (buf[1][jp1] - 2.0 * buf[1][j] + buf[1][jm1])
                    + DY2TY1 * (ue[1][jp1] - 2.0 * ue[1][j] + ue[1][jm1]);
                forcing[k][j][i][2] = forcing[k][j][i][2]
                    - TY2
                        * ((ue[2][jp1] * buf[2][jp1] + C2 * (ue[4][jp1] - q[jp1]))
                            - (ue[2][jm1] * buf[2][jm1] + C2 * (ue[4][jm1] - q[jm1])))
                    + YYCON1 * (buf[2][jp1] - 2.0 * buf[2][j] + buf[2][jm1])
                    + DY3TY1 * (ue[2][jp1] - 2.0 * ue[2][j] + ue[2][jm1]);
                forcing[k][j][i][3] = forcing[k][j][i][3]
                    - TY2 * (ue[3][jp1] * buf[2][jp1] - ue[3][jm1] * buf[2][jm1])
                    + YYCON2 * (buf[3][jp1] - 2.0 * buf[3][j] + buf[3][jm1])
                    + DY4TY1 * (ue[3][jp1] - 2.0 * ue[3][j] + ue[3][jm1]);
                forcing[k][j][i][4] = forcing[k][j][i][4]
                    - TY2
                        * (buf[2][jp1] * (C1 * ue[4][jp1] - C2 * q[jp1])
                            - buf[2][jm1] * (C1 * ue[4][jm1] - C2 * q[jm1]))
                    + 0.5 * YYCON3 * (buf[0][jp1] - 2.0 * buf[0][j] + buf[0][jm1])
                    + YYCON4 * (cuf[jp1] - 2.0 * cuf[j] + cuf[jm1])
                    + YYCON5 * (buf[4][jp1] - 2.0 * buf[4][j] + buf[4][jm1])
                    + DY5TY1 * (ue[4][jp1] - 2.0 * ue[4][j] + ue[4][jm1]);
            }
            /*
             * ---------------------------------------------------------------------
             * fourth-order dissipation
             * ---------------------------------------------------------------------
             */
            for m in 0..5 {
                j = 1;
                forcing[k][j][i][m] = forcing[k][j][i][m]
                    - DSSP * (5.0 * ue[m][j] - 4.0 * ue[m][j + 1] + ue[m][j + 2]);
                j = 2;
                forcing[k][j][i][m] = forcing[k][j][i][m]
                    - DSSP
                        * (-4.0 * ue[m][j - 1] + 6.0 * ue[m][j] - 4.0 * ue[m][j + 1]
                            + ue[m][j + 2]);
            }
            for j in 3..GRID_POINTS[1] - 3 {
                for m in 0..5 {
                    forcing[k][j][i][m] = forcing[k][j][i][m]
                        - DSSP
                            * (ue[m][j - 2] - 4.0 * ue[m][j - 1] + 6.0 * ue[m][j]
                                - 4.0 * ue[m][j + 1]
                                + ue[m][j + 2]);
                }
            }
            for m in 0..5 {
                j = GRID_POINTS[1] - 3;
                forcing[k][j][i][m] = forcing[k][j][i][m]
                    - DSSP
                        * (ue[m][j - 2] - 4.0 * ue[m][j - 1] + 6.0 * ue[m][j] - 4.0 * ue[m][j + 1]);
                j = GRID_POINTS[1] - 2;
                forcing[k][j][i][m] = forcing[k][j][i][m]
                    - DSSP * (ue[m][j - 2] - 4.0 * ue[m][j - 1] + 5.0 * ue[m][j]);
            }
        }
    }
    /*
     * ---------------------------------------------------------------------
     * zeta-direction flux differences
     * ---------------------------------------------------------------------
     */
    for j in 1..GRID_POINTS[1] - 1 {
        eta = j as f64 * DNYM1;
        for i in 1..GRID_POINTS[0] - 1 {
            xi = i as f64 * DNXM1;
            for k in 0..GRID_POINTS[2] {
                zeta = k as f64 * DNZM1;
                exact_solution(xi, eta, zeta, &mut dtemp[..], &ce[..]);
                for m in 0..5 {
                    ue[m][k] = dtemp[m];
                }
                dtpp = 1.0 / dtemp[0];
                for m in 1..5 {
                    buf[m][k] = dtpp * dtemp[m];
                }
                cuf[k] = buf[3][k] * buf[3][k];
                buf[0][k] = cuf[k] + buf[1][k] * buf[1][k] + buf[2][k] * buf[2][k];
                q[k] = 0.5 * (buf[1][k] * ue[1][k] + buf[2][k] * ue[2][k] + buf[3][k] * ue[3][k]);
            }
            for k in 1..GRID_POINTS[2] - 1 {
                km1 = k - 1;
                kp1 = k + 1;
                forcing[k][j][i][0] = forcing[k][j][i][0] - TZ2 * (ue[3][kp1] - ue[3][km1])
                    + DZ1TZ1 * (ue[0][kp1] - 2.0 * ue[0][k] + ue[0][km1]);
                forcing[k][j][i][1] = forcing[k][j][i][1]
                    - TZ2 * (ue[1][kp1] * buf[3][kp1] - ue[1][km1] * buf[3][km1])
                    + ZZCON2 * (buf[1][kp1] - 2.0 * buf[1][k] + buf[1][km1])
                    + DZ2TZ1 * (ue[1][kp1] - 2.0 * ue[1][k] + ue[1][km1]);
                forcing[k][j][i][2] = forcing[k][j][i][2]
                    - TZ2 * (ue[2][kp1] * buf[3][kp1] - ue[2][km1] * buf[3][km1])
                    + ZZCON2 * (buf[2][kp1] - 2.0 * buf[2][k] + buf[2][km1])
                    + DZ3TZ1 * (ue[2][kp1] - 2.0 * ue[2][k] + ue[2][km1]);
                forcing[k][j][i][3] = forcing[k][j][i][3]
                    - TZ2
                        * ((ue[3][kp1] * buf[3][kp1] + C2 * (ue[4][kp1] - q[kp1]))
                            - (ue[3][km1] * buf[3][km1] + C2 * (ue[4][km1] - q[km1])))
                    + ZZCON1 * (buf[3][kp1] - 2.0 * buf[3][k] + buf[3][km1])
                    + DZ4TZ1 * (ue[3][kp1] - 2.0 * ue[3][k] + ue[3][km1]);
                forcing[k][j][i][4] = forcing[k][j][i][4]
                    - TZ2
                        * (buf[3][kp1] * (C1 * ue[4][kp1] - C2 * q[kp1])
                            - buf[3][km1] * (C1 * ue[4][km1] - C2 * q[km1]))
                    + 0.5 * ZZCON3 * (buf[0][kp1] - 2.0 * buf[0][k] + buf[0][km1])
                    + ZZCON4 * (cuf[kp1] - 2.0 * cuf[k] + cuf[km1])
                    + ZZCON5 * (buf[4][kp1] - 2.0 * buf[4][k] + buf[4][km1])
                    + DZ5TZ1 * (ue[4][kp1] - 2.0 * ue[4][k] + ue[4][km1]);
            }
            /*
             * ---------------------------------------------------------------------
             * fourth-order dissipation
             * ---------------------------------------------------------------------
             */
            for m in 0..5 {
                k = 1;
                forcing[k][j][i][m] = forcing[k][j][i][m]
                    - DSSP * (5.0 * ue[m][k] - 4.0 * ue[m][k + 1] + ue[m][k + 2]);
                k = 2;
                forcing[k][j][i][m] = forcing[k][j][i][m]
                    - DSSP
                        * (-4.0 * ue[m][k - 1] + 6.0 * ue[m][k] - 4.0 * ue[m][k + 1]
                            + ue[m][k + 2]);
            }
            for k in 3..GRID_POINTS[2] - 3 {
                for m in 0..5 {
                    forcing[k][j][i][m] = forcing[k][j][i][m]
                        - DSSP
                            * (ue[m][k - 2] - 4.0 * ue[m][k - 1] + 6.0 * ue[m][k]
                                - 4.0 * ue[m][k + 1]
                                + ue[m][k + 2]);
                }
            }
            for m in 0..5 {
                k = GRID_POINTS[2] - 3;
                forcing[k][j][i][m] = forcing[k][j][i][m]
                    - DSSP
                        * (ue[m][k - 2] - 4.0 * ue[m][k - 1] + 6.0 * ue[m][k] - 4.0 * ue[m][k + 1]);
                k = GRID_POINTS[2] - 2;
                forcing[k][j][i][m] = forcing[k][j][i][m]
                    - DSSP * (ue[m][k - 2] - 4.0 * ue[m][k - 1] + 5.0 * ue[m][k]);
            }
        }
    }
    /*
     * ---------------------------------------------------------------------
     * now change the sign of the forcing function
     * ---------------------------------------------------------------------
     */
    forcing.iter_mut().for_each(|f| {
        f.iter_mut().for_each(|f| {
            f.iter_mut().for_each(|f| {
                f.iter_mut().for_each(|f| *f = -*f);
            });
        });
    });
}

fn exact_solution(xi: f64, eta: f64, zeta: f64, dtemp: &mut [f64], ce: &[[f64; 5]]) {
    for m in 0..5 {
        dtemp[m] = ce[0][m]
            + xi * (ce[1][m] + xi * (ce[4][m] + xi * (ce[7][m] + xi * ce[10][m])))
            + eta * (ce[2][m] + eta * (ce[5][m] + eta * (ce[8][m] + eta * ce[11][m])))
            + zeta * (ce[3][m] + zeta * (ce[6][m] + zeta * (ce[9][m] + zeta * ce[12][m])));
    }
}

fn exact_solution_3d(
    xi: f64,
    eta: f64,
    zeta: f64,
    i: usize,
    j: usize,
    dtemp: &mut [[[f64; 5]; 3]],
    ce: &[[f64; 5]],
) {
    for m in 0..5 {
        dtemp[i][j][m] = ce[0][m]
            + xi * (ce[1][m] + xi * (ce[4][m] + xi * (ce[7][m] + xi * ce[10][m])))
            + eta * (ce[2][m] + eta * (ce[5][m] + eta * (ce[8][m] + eta * ce[11][m])))
            + zeta * (ce[3][m] + zeta * (ce[6][m] + zeta * (ce[9][m] + zeta * ce[12][m])));
    }
}

fn initialize(u: &mut [[[[f64; 5]; IMAXP + 1]; JMAXP + 1]], ce: &[[f64; 5]]) {
    /*
     * ---------------------------------------------------------------------
     * later (in compute_rhs) we compute 1/u for every element. a few of
     * the corner elements are not used, but it convenient (and faster)
     * to compute the whole thing with a simple loop. make sure those
     * values are nonzero by initializing the whole thing here.
     * ---------------------------------------------------------------------
     */
    u.iter_mut().for_each(|u| {
        u.iter_mut().for_each(|u| {
            u.iter_mut().for_each(|u| {
                u.iter_mut().for_each(|u| {
                    *u = 1.0;
                });
            });
        });
    });
    /*
     * ---------------------------------------------------------------------
     * first store the "interpolated" values everywhere on the grid
     * ---------------------------------------------------------------------
     */
    let mut zeta: f64;
    let mut eta: f64;
    let mut xi: f64;
    let mut pxi: f64;
    let mut peta: f64;
    let mut pzeta: f64;
    let mut pface: [[[f64; 5]; 3]; 2] = [[[0.0; 5]; 3]; 2];
    let mut temp: [f64; 5] = [0.0; 5];
    for k in 0..GRID_POINTS[2] {
        zeta = k as f64 * DNZM1;
        for j in 0..GRID_POINTS[1] {
            eta = j as f64 * DNYM1;
            for i in 0..GRID_POINTS[0] {
                xi = i as f64 * DNXM1;
                for ix in 0..2 as usize {
                    exact_solution_3d(ix as f64, eta, zeta, ix, 0, &mut pface[..], &ce[..]);
                }
                for iy in 0..2 as usize {
                    exact_solution_3d(xi, iy as f64, zeta, iy, 1, &mut pface[..], &ce[..]);
                }
                for iz in 0..2 as usize {
                    exact_solution_3d(xi, eta, iz as f64, iz, 2, &mut pface[..], &ce[..]);
                }
                for m in 0..5 {
                    pxi = xi * pface[1][0][m] + (1.0 - xi) * pface[0][0][m];
                    peta = eta * pface[1][1][m] + (1.0 - eta) * pface[0][1][m];
                    pzeta = zeta * pface[1][2][m] + (1.0 - zeta) * pface[0][2][m];
                    u[k][j][i][m] = pxi + peta + pzeta - pxi * peta - pxi * pzeta - peta * pzeta
                        + pxi * peta * pzeta;
                }
            }
        }
    }
    /*
     * ---------------------------------------------------------------------
     * now store the exact values on the boundaries
     * ---------------------------------------------------------------------
     * west face
     * ---------------------------------------------------------------------
     */
    let mut i: usize = 0;
    let mut xi: f64 = 0.0;
    for k in 0..GRID_POINTS[2] {
        zeta = k as f64 * DNZM1;
        for j in 0..GRID_POINTS[1] {
            eta = j as f64 * DNYM1;
            exact_solution(xi, eta, zeta, &mut temp[..], &ce[..]);
            for m in 0..5 {
                u[k][j][i][m] = temp[m];
            }
        }
    }
    /*
     * ---------------------------------------------------------------------
     * east face
     * ---------------------------------------------------------------------
     */
    i = GRID_POINTS[0] - 1;
    xi = 1.0;
    for k in 0..GRID_POINTS[2] {
        zeta = k as f64 * DNZM1;
        for j in 0..GRID_POINTS[1] {
            eta = j as f64 * DNYM1;
            exact_solution(xi, eta, zeta, &mut temp[..], &ce[..]);
            for m in 0..5 {
                u[k][j][i][m] = temp[m];
            }
        }
    }
    /*
     * ---------------------------------------------------------------------
     * south face
     * ---------------------------------------------------------------------
     */
    let mut j: usize = 0;
    eta = 0.0;
    for k in 0..GRID_POINTS[2] {
        zeta = k as f64 * DNZM1;
        for i in 0..GRID_POINTS[0] {
            xi = i as f64 * DNXM1;
            exact_solution(xi, eta, zeta, &mut temp[..], &ce[..]);
            for m in 0..5 {
                u[k][j][i][m] = temp[m];
            }
        }
    }
    /*
     * ---------------------------------------------------------------------
     * north face
     * ---------------------------------------------------------------------
     */
    j = GRID_POINTS[1] - 1;
    eta = 1.0;
    for k in 0..GRID_POINTS[2] {
        zeta = k as f64 * DNZM1;
        for i in 0..GRID_POINTS[0] {
            xi = i as f64 * DNXM1;
            exact_solution(xi, eta, zeta, &mut temp[..], &ce[..]);
            for m in 0..5 {
                u[k][j][i][m] = temp[m];
            }
        }
    }
    /*
     * ---------------------------------------------------------------------
     * bottom face
     * ---------------------------------------------------------------------
     */
    let mut k: usize = 0;
    zeta = 0.0;
    for j in 0..GRID_POINTS[1] {
        eta = j as f64 * DNYM1;
        for i in 0..GRID_POINTS[0] {
            xi = i as f64 * DNXM1;
            exact_solution(xi, eta, zeta, &mut temp[..], &ce[..]);
            for m in 0..5 {
                u[k][j][i][m] = temp[m];
            }
        }
    }
    /*
     * ---------------------------------------------------------------------
     * top face
     * ---------------------------------------------------------------------
     */
    k = GRID_POINTS[2] - 1;
    zeta = 1.0;
    for j in 0..GRID_POINTS[1] {
        eta = j as f64 * DNYM1;
        for i in 0..GRID_POINTS[0] {
            xi = i as f64 * DNXM1;
            exact_solution(xi, eta, zeta, &mut temp[..], &ce[..]);
            for m in 0..5 {
                u[k][j][i][m] = temp[m];
            }
        }
    }
}

fn lhsinit(lhs: &mut [[[[f64; 5]; 5]; 3]], size: usize) {
    let i: usize = size;
    /*
     * ---------------------------------------------------------------------
     * zero the whole left hand side for starters
     * ---------------------------------------------------------------------
     */
    for m in 0..5 {
        for n in 0..5 {
            lhs[0][0][n][m] = 0.0;
            lhs[0][1][n][m] = 0.0;
            lhs[0][2][n][m] = 0.0;
            lhs[i][0][n][m] = 0.0;
            lhs[i][1][n][m] = 0.0;
            lhs[i][2][n][m] = 0.0;
        }
    }
    /*
     * ---------------------------------------------------------------------
     * next, set all diagonal values to 1. This is overkill, but convenient
     * ---------------------------------------------------------------------
     */
    for m in 0..5 {
        lhs[0][1][m][m] = 1.0;
        lhs[i][1][m][m] = 1.0;
    }
}

/*
 * ---------------------------------------------------------------------
 * subtracts a(i,j,k) X b(i,j,k) from c(i,j,k)
 * ---------------------------------------------------------------------
 */
fn matmul_sub(
    lhs: &mut [[[[f64; 5]; 5]; 3]],
    i1: usize,
    j1: usize,
    i2: usize,
    j2: usize,
    i3: usize,
    j3: usize,
) {
    lhs[i3][j3][0][0] = lhs[i3][j3][0][0]
        - lhs[i1][j1][0][0] * lhs[i2][j2][0][0]
        - lhs[i1][j1][1][0] * lhs[i2][j2][0][1]
        - lhs[i1][j1][2][0] * lhs[i2][j2][0][2]
        - lhs[i1][j1][3][0] * lhs[i2][j2][0][3]
        - lhs[i1][j1][4][0] * lhs[i2][j2][0][4];
    lhs[i3][j3][0][1] = lhs[i3][j3][0][1]
        - lhs[i1][j1][0][1] * lhs[i2][j2][0][0]
        - lhs[i1][j1][1][1] * lhs[i2][j2][0][1]
        - lhs[i1][j1][2][1] * lhs[i2][j2][0][2]
        - lhs[i1][j1][3][1] * lhs[i2][j2][0][3]
        - lhs[i1][j1][4][1] * lhs[i2][j2][0][4];
    lhs[i3][j3][0][2] = lhs[i3][j3][0][2]
        - lhs[i1][j1][0][2] * lhs[i2][j2][0][0]
        - lhs[i1][j1][1][2] * lhs[i2][j2][0][1]
        - lhs[i1][j1][2][2] * lhs[i2][j2][0][2]
        - lhs[i1][j1][3][2] * lhs[i2][j2][0][3]
        - lhs[i1][j1][4][2] * lhs[i2][j2][0][4];
    lhs[i3][j3][0][3] = lhs[i3][j3][0][3]
        - lhs[i1][j1][0][3] * lhs[i2][j2][0][0]
        - lhs[i1][j1][1][3] * lhs[i2][j2][0][1]
        - lhs[i1][j1][2][3] * lhs[i2][j2][0][2]
        - lhs[i1][j1][3][3] * lhs[i2][j2][0][3]
        - lhs[i1][j1][4][3] * lhs[i2][j2][0][4];
    lhs[i3][j3][0][4] = lhs[i3][j3][0][4]
        - lhs[i1][j1][0][4] * lhs[i2][j2][0][0]
        - lhs[i1][j1][1][4] * lhs[i2][j2][0][1]
        - lhs[i1][j1][2][4] * lhs[i2][j2][0][2]
        - lhs[i1][j1][3][4] * lhs[i2][j2][0][3]
        - lhs[i1][j1][4][4] * lhs[i2][j2][0][4];
    lhs[i3][j3][1][0] = lhs[i3][j3][1][0]
        - lhs[i1][j1][0][0] * lhs[i2][j2][1][0]
        - lhs[i1][j1][1][0] * lhs[i2][j2][1][1]
        - lhs[i1][j1][2][0] * lhs[i2][j2][1][2]
        - lhs[i1][j1][3][0] * lhs[i2][j2][1][3]
        - lhs[i1][j1][4][0] * lhs[i2][j2][1][4];
    lhs[i3][j3][1][1] = lhs[i3][j3][1][1]
        - lhs[i1][j1][0][1] * lhs[i2][j2][1][0]
        - lhs[i1][j1][1][1] * lhs[i2][j2][1][1]
        - lhs[i1][j1][2][1] * lhs[i2][j2][1][2]
        - lhs[i1][j1][3][1] * lhs[i2][j2][1][3]
        - lhs[i1][j1][4][1] * lhs[i2][j2][1][4];
    lhs[i3][j3][1][2] = lhs[i3][j3][1][2]
        - lhs[i1][j1][0][2] * lhs[i2][j2][1][0]
        - lhs[i1][j1][1][2] * lhs[i2][j2][1][1]
        - lhs[i1][j1][2][2] * lhs[i2][j2][1][2]
        - lhs[i1][j1][3][2] * lhs[i2][j2][1][3]
        - lhs[i1][j1][4][2] * lhs[i2][j2][1][4];
    lhs[i3][j3][1][3] = lhs[i3][j3][1][3]
        - lhs[i1][j1][0][3] * lhs[i2][j2][1][0]
        - lhs[i1][j1][1][3] * lhs[i2][j2][1][1]
        - lhs[i1][j1][2][3] * lhs[i2][j2][1][2]
        - lhs[i1][j1][3][3] * lhs[i2][j2][1][3]
        - lhs[i1][j1][4][3] * lhs[i2][j2][1][4];
    lhs[i3][j3][1][4] = lhs[i3][j3][1][4]
        - lhs[i1][j1][0][4] * lhs[i2][j2][1][0]
        - lhs[i1][j1][1][4] * lhs[i2][j2][1][1]
        - lhs[i1][j1][2][4] * lhs[i2][j2][1][2]
        - lhs[i1][j1][3][4] * lhs[i2][j2][1][3]
        - lhs[i1][j1][4][4] * lhs[i2][j2][1][4];
    lhs[i3][j3][2][0] = lhs[i3][j3][2][0]
        - lhs[i1][j1][0][0] * lhs[i2][j2][2][0]
        - lhs[i1][j1][1][0] * lhs[i2][j2][2][1]
        - lhs[i1][j1][2][0] * lhs[i2][j2][2][2]
        - lhs[i1][j1][3][0] * lhs[i2][j2][2][3]
        - lhs[i1][j1][4][0] * lhs[i2][j2][2][4];
    lhs[i3][j3][2][1] = lhs[i3][j3][2][1]
        - lhs[i1][j1][0][1] * lhs[i2][j2][2][0]
        - lhs[i1][j1][1][1] * lhs[i2][j2][2][1]
        - lhs[i1][j1][2][1] * lhs[i2][j2][2][2]
        - lhs[i1][j1][3][1] * lhs[i2][j2][2][3]
        - lhs[i1][j1][4][1] * lhs[i2][j2][2][4];
    lhs[i3][j3][2][2] = lhs[i3][j3][2][2]
        - lhs[i1][j1][0][2] * lhs[i2][j2][2][0]
        - lhs[i1][j1][1][2] * lhs[i2][j2][2][1]
        - lhs[i1][j1][2][2] * lhs[i2][j2][2][2]
        - lhs[i1][j1][3][2] * lhs[i2][j2][2][3]
        - lhs[i1][j1][4][2] * lhs[i2][j2][2][4];
    lhs[i3][j3][2][3] = lhs[i3][j3][2][3]
        - lhs[i1][j1][0][3] * lhs[i2][j2][2][0]
        - lhs[i1][j1][1][3] * lhs[i2][j2][2][1]
        - lhs[i1][j1][2][3] * lhs[i2][j2][2][2]
        - lhs[i1][j1][3][3] * lhs[i2][j2][2][3]
        - lhs[i1][j1][4][3] * lhs[i2][j2][2][4];
    lhs[i3][j3][2][4] = lhs[i3][j3][2][4]
        - lhs[i1][j1][0][4] * lhs[i2][j2][2][0]
        - lhs[i1][j1][1][4] * lhs[i2][j2][2][1]
        - lhs[i1][j1][2][4] * lhs[i2][j2][2][2]
        - lhs[i1][j1][3][4] * lhs[i2][j2][2][3]
        - lhs[i1][j1][4][4] * lhs[i2][j2][2][4];
    lhs[i3][j3][3][0] = lhs[i3][j3][3][0]
        - lhs[i1][j1][0][0] * lhs[i2][j2][3][0]
        - lhs[i1][j1][1][0] * lhs[i2][j2][3][1]
        - lhs[i1][j1][2][0] * lhs[i2][j2][3][2]
        - lhs[i1][j1][3][0] * lhs[i2][j2][3][3]
        - lhs[i1][j1][4][0] * lhs[i2][j2][3][4];
    lhs[i3][j3][3][1] = lhs[i3][j3][3][1]
        - lhs[i1][j1][0][1] * lhs[i2][j2][3][0]
        - lhs[i1][j1][1][1] * lhs[i2][j2][3][1]
        - lhs[i1][j1][2][1] * lhs[i2][j2][3][2]
        - lhs[i1][j1][3][1] * lhs[i2][j2][3][3]
        - lhs[i1][j1][4][1] * lhs[i2][j2][3][4];
    lhs[i3][j3][3][2] = lhs[i3][j3][3][2]
        - lhs[i1][j1][0][2] * lhs[i2][j2][3][0]
        - lhs[i1][j1][1][2] * lhs[i2][j2][3][1]
        - lhs[i1][j1][2][2] * lhs[i2][j2][3][2]
        - lhs[i1][j1][3][2] * lhs[i2][j2][3][3]
        - lhs[i1][j1][4][2] * lhs[i2][j2][3][4];
    lhs[i3][j3][3][3] = lhs[i3][j3][3][3]
        - lhs[i1][j1][0][3] * lhs[i2][j2][3][0]
        - lhs[i1][j1][1][3] * lhs[i2][j2][3][1]
        - lhs[i1][j1][2][3] * lhs[i2][j2][3][2]
        - lhs[i1][j1][3][3] * lhs[i2][j2][3][3]
        - lhs[i1][j1][4][3] * lhs[i2][j2][3][4];
    lhs[i3][j3][3][4] = lhs[i3][j3][3][4]
        - lhs[i1][j1][0][4] * lhs[i2][j2][3][0]
        - lhs[i1][j1][1][4] * lhs[i2][j2][3][1]
        - lhs[i1][j1][2][4] * lhs[i2][j2][3][2]
        - lhs[i1][j1][3][4] * lhs[i2][j2][3][3]
        - lhs[i1][j1][4][4] * lhs[i2][j2][3][4];
    lhs[i3][j3][4][0] = lhs[i3][j3][4][0]
        - lhs[i1][j1][0][0] * lhs[i2][j2][4][0]
        - lhs[i1][j1][1][0] * lhs[i2][j2][4][1]
        - lhs[i1][j1][2][0] * lhs[i2][j2][4][2]
        - lhs[i1][j1][3][0] * lhs[i2][j2][4][3]
        - lhs[i1][j1][4][0] * lhs[i2][j2][4][4];
    lhs[i3][j3][4][1] = lhs[i3][j3][4][1]
        - lhs[i1][j1][0][1] * lhs[i2][j2][4][0]
        - lhs[i1][j1][1][1] * lhs[i2][j2][4][1]
        - lhs[i1][j1][2][1] * lhs[i2][j2][4][2]
        - lhs[i1][j1][3][1] * lhs[i2][j2][4][3]
        - lhs[i1][j1][4][1] * lhs[i2][j2][4][4];
    lhs[i3][j3][4][2] = lhs[i3][j3][4][2]
        - lhs[i1][j1][0][2] * lhs[i2][j2][4][0]
        - lhs[i1][j1][1][2] * lhs[i2][j2][4][1]
        - lhs[i1][j1][2][2] * lhs[i2][j2][4][2]
        - lhs[i1][j1][3][2] * lhs[i2][j2][4][3]
        - lhs[i1][j1][4][2] * lhs[i2][j2][4][4];
    lhs[i3][j3][4][3] = lhs[i3][j3][4][3]
        - lhs[i1][j1][0][3] * lhs[i2][j2][4][0]
        - lhs[i1][j1][1][3] * lhs[i2][j2][4][1]
        - lhs[i1][j1][2][3] * lhs[i2][j2][4][2]
        - lhs[i1][j1][3][3] * lhs[i2][j2][4][3]
        - lhs[i1][j1][4][3] * lhs[i2][j2][4][4];
    lhs[i3][j3][4][4] = lhs[i3][j3][4][4]
        - lhs[i1][j1][0][4] * lhs[i2][j2][4][0]
        - lhs[i1][j1][1][4] * lhs[i2][j2][4][1]
        - lhs[i1][j1][2][4] * lhs[i2][j2][4][2]
        - lhs[i1][j1][3][4] * lhs[i2][j2][4][3]
        - lhs[i1][j1][4][4] * lhs[i2][j2][4][4];
}

/*
 * ---------------------------------------------------------------------
 * subtracts bvec=bvec - ablock*avec
 * ---------------------------------------------------------------------
 */
fn matvec_sub(
    lhs: &mut [[f64; 5]],
    rhs: &mut [[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    i1: usize,
    j1: usize,
    k1: usize,
    i2: usize,
    j2: usize,
    k2: usize,
) {
    /*
     * ---------------------------------------------------------------------
     * rhs[kc][jc][ic][i] = rhs[kc][jc][ic][i] - lhs[ia][ablock][0][i]*
     * ---------------------------------------------------------------------
     */
    rhs[i2][j2][k2][0] = rhs[i2][j2][k2][0]
        - lhs[0][0] * rhs[i1][j1][k1][0]
        - lhs[1][0] * rhs[i1][j1][k1][1]
        - lhs[2][0] * rhs[i1][j1][k1][2]
        - lhs[3][0] * rhs[i1][j1][k1][3]
        - lhs[4][0] * rhs[i1][j1][k1][4];
    rhs[i2][j2][k2][1] = rhs[i2][j2][k2][1]
        - lhs[0][1] * rhs[i1][j1][k1][0]
        - lhs[1][1] * rhs[i1][j1][k1][1]
        - lhs[2][1] * rhs[i1][j1][k1][2]
        - lhs[3][1] * rhs[i1][j1][k1][3]
        - lhs[4][1] * rhs[i1][j1][k1][4];
    rhs[i2][j2][k2][2] = rhs[i2][j2][k2][2]
        - lhs[0][2] * rhs[i1][j1][k1][0]
        - lhs[1][2] * rhs[i1][j1][k1][1]
        - lhs[2][2] * rhs[i1][j1][k1][2]
        - lhs[3][2] * rhs[i1][j1][k1][3]
        - lhs[4][2] * rhs[i1][j1][k1][4];
    rhs[i2][j2][k2][3] = rhs[i2][j2][k2][3]
        - lhs[0][3] * rhs[i1][j1][k1][0]
        - lhs[1][3] * rhs[i1][j1][k1][1]
        - lhs[2][3] * rhs[i1][j1][k1][2]
        - lhs[3][3] * rhs[i1][j1][k1][3]
        - lhs[4][3] * rhs[i1][j1][k1][4];
    rhs[i2][j2][k2][4] = rhs[i2][j2][k2][4]
        - lhs[0][4] * rhs[i1][j1][k1][0]
        - lhs[1][4] * rhs[i1][j1][k1][1]
        - lhs[2][4] * rhs[i1][j1][k1][2]
        - lhs[3][4] * rhs[i1][j1][k1][3]
        - lhs[4][4] * rhs[i1][j1][k1][4];
}

fn rhs_norm(rms: &mut [f64], rhs: &[[[[f64; 5]; IMAXP + 1]; JMAXP + 1]]) {
    rms.iter_mut().for_each(|rms| *rms = 0.0);
    for k in 1..GRID_POINTS[2] - 1 {
        for j in 1..GRID_POINTS[1] - 1 {
            for i in 1..GRID_POINTS[0] - 1 {
                for m in 0..5 {
                    let add = rhs[k][j][i][m];
                    rms[m] = rms[m] + add * add;
                }
            }
        }
    }
    for m in 0..5 {
        for d in 0..3 {
            rms[m] = rms[m] / (GRID_POINTS[d] - 2) as f64;
        }
        rms[m] = f64::sqrt(rms[m]);
    }
}

/*
 * ---------------------------------------------------------------------
 * verification routine
 * ---------------------------------------------------------------------
 */
fn verify(
    verified: &mut i8,
    u: &[[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    ce: &[[f64; 5]],
    rhs: &mut [[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    rho_i: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    us: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    vs: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    ws: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    square: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    qs: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    forcing: &[[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    timers: &mut Timer,
) {
    let mut xcrref: [f64; 5] = [1.0; 5];
    let mut xceref: [f64; 5] = [1.0; 5];
    let mut xcrdif: [f64; 5] = [0.0; 5];
    let mut xcedif: [f64; 5] = [0.0; 5];
    let mut xce: [f64; 5] = [0.0; 5];
    let mut xcr: [f64; 5] = [0.0; 5];
    let mut dtref: f64 = 0.0;
    /*
     * ---------------------------------------------------------------------
     * compute the error norm and the residual norm, and exit if not printing
     * ---------------------------------------------------------------------
     */
    error_norm(&mut xce[..], &u[..], &ce[..]);
    compute_rhs(
        &mut rhs[..],
        &mut rho_i[..],
        &mut us[..],
        &mut vs[..],
        &mut ws[..],
        &mut square[..],
        &mut qs[..],
        &u[..],
        &forcing[..],
        timers,
    );
    rhs_norm(&mut xcr[..], &rhs[..]);
    xcr.iter_mut().for_each(|xcr| *xcr /= DT_DEFAULT);
    *verified = 1;
    /*
     * ---------------------------------------------------------------------
     * reference data for 12X12X12 grids after 60 time steps, with DT = 1.0e-02
     * ---------------------------------------------------------------------
     */
    if CLASS == 'S' {
        dtref = 1.0e-2;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual.
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 1.7034283709541311e-01;
        xcrref[1] = 1.2975252070034097e-02;
        xcrref[2] = 3.2527926989486055e-02;
        xcrref[3] = 2.6436421275166801e-02;
        xcrref[4] = 1.9211784131744430e-01;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error.
         * ---------------------------------------------------------------------
         */
        xceref[0] = 4.9976913345811579e-04;
        xceref[1] = 4.5195666782961927e-05;
        xceref[2] = 7.3973765172921357e-05;
        xceref[3] = 7.3821238632439731e-05;
        xceref[4] = 8.9269630987491446e-04;
    /*
     * ---------------------------------------------------------------------
     * reference data for 24X24X24 grids after 200 time steps, with DT = 0.8d-3
     * ---------------------------------------------------------------------
     */
    } else if CLASS == 'W' {
        dtref = 0.8e-3;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual.
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 0.1125590409344e+03;
        xcrref[1] = 0.1180007595731e+02;
        xcrref[2] = 0.2710329767846e+02;
        xcrref[3] = 0.2469174937669e+02;
        xcrref[4] = 0.2638427874317e+03;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error.
         * ---------------------------------------------------------------------
         */
        xceref[0] = 0.4419655736008e+01;
        xceref[1] = 0.4638531260002e+00;
        xceref[2] = 0.1011551749967e+01;
        xceref[3] = 0.9235878729944e+00;
        xceref[4] = 0.1018045837718e+02;
    /*
     * ---------------------------------------------------------------------
     * reference data for 64X64X64 grids after 200 time steps, with DT = 0.8d-3
     * ---------------------------------------------------------------------
     */
    } else if CLASS == 'A' {
        dtref = 0.8e-3;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual.
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 1.0806346714637264e+02;
        xcrref[1] = 1.1319730901220813e+01;
        xcrref[2] = 2.5974354511582465e+01;
        xcrref[3] = 2.3665622544678910e+01;
        xcrref[4] = 2.5278963211748344e+02;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error.
         * ---------------------------------------------------------------------
         */
        xceref[0] = 4.2348416040525025e+00;
        xceref[1] = 4.4390282496995698e-01;
        xceref[2] = 9.6692480136345650e-01;
        xceref[3] = 8.8302063039765474e-01;
        xceref[4] = 9.7379901770829278e+00;
    /*
     * ---------------------------------------------------------------------
     * reference data for 102X102X102 grids after 200 time steps,
     * with DT = 3.0e-04
     * ---------------------------------------------------------------------
     */
    } else if CLASS == 'B' {
        dtref = 3.0e-4;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual.
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 1.4233597229287254e+03;
        xcrref[1] = 9.9330522590150238e+01;
        xcrref[2] = 3.5646025644535285e+02;
        xcrref[3] = 3.2485447959084092e+02;
        xcrref[4] = 3.2707541254659363e+03;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error.
         * ---------------------------------------------------------------------
         */
        xceref[0] = 5.2969847140936856e+01;
        xceref[1] = 4.4632896115670668e+00;
        xceref[2] = 1.3122573342210174e+01;
        xceref[3] = 1.2006925323559144e+01;
        xceref[4] = 1.2459576151035986e+02;
    /*
     * ---------------------------------------------------------------------
     * reference data for 162X162X162 grids after 200 time steps,
     * with DT = 1.0e-04
     * ---------------------------------------------------------------------
     */
    } else if CLASS == 'C' {
        dtref = 1.0e-4;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual.
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 0.62398116551764615e+04;
        xcrref[1] = 0.50793239190423964e+03;
        xcrref[2] = 0.15423530093013596e+04;
        xcrref[3] = 0.13302387929291190e+04;
        xcrref[4] = 0.11604087428436455e+05;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error.
         * ---------------------------------------------------------------------
         */
        xceref[0] = 0.16462008369091265e+03;
        xceref[1] = 0.11497107903824313e+02;
        xceref[2] = 0.41207446207461508e+02;
        xceref[3] = 0.37087651059694167e+02;
        xceref[4] = 0.36211053051841265e+03;
    /*
     * ---------------------------------------------------------------------
     * reference data for 408x408x408 grids after 250 time steps,
     * with DT = 0.2e-04
     * ---------------------------------------------------------------------
     */
    } else if CLASS == 'D' {
        dtref = 0.2e-4;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual.
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 0.2533188551738e+05;
        xcrref[1] = 0.2346393716980e+04;
        xcrref[2] = 0.6294554366904e+04;
        xcrref[3] = 0.5352565376030e+04;
        xcrref[4] = 0.3905864038618e+05;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error.
         * ---------------------------------------------------------------------
         */
        xceref[0] = 0.3100009377557e+03;
        xceref[1] = 0.2424086324913e+02;
        xceref[2] = 0.7782212022645e+02;
        xceref[3] = 0.6835623860116e+02;
        xceref[4] = 0.6065737200368e+03;
    /*
     * ---------------------------------------------------------------------
     * reference data for 1020x1020x1020 grids after 250 time steps,
     * with DT = 0.4e-05
     * ---------------------------------------------------------------------
     */
    } else if CLASS == 'E' {
        dtref = 0.4e-5;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual.
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 0.9795372484517e+05;
        xcrref[1] = 0.9739814511521e+04;
        xcrref[2] = 0.2467606342965e+05;
        xcrref[3] = 0.2092419572860e+05;
        xcrref[4] = 0.1392138856939e+06;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error.
         * ---------------------------------------------------------------------
         */
        xceref[0] = 0.4327562208414e+03;
        xceref[1] = 0.3699051964887e+02;
        xceref[2] = 0.1089845040954e+03;
        xceref[3] = 0.9462517622043e+02;
        xceref[4] = 0.7765512765309e+03;
    } else if CLASS == 'Z' {
        dtref = 1.0e-2;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual.
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 1.0200813443985e+4;
        xcrref[1] = 8.4913657272573e+2;
        xcrref[2] = 2.5393982433952e+3;
        xcrref[3] = 2.1563994809843e+3;
        xcrref[4] = 1.9342537700702e+4;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error.
         * ---------------------------------------------------------------------
         */
        xceref[0] = 2.8482460825060e+2;
        xceref[1] = 1.9931321628550e+1;
        xceref[2] = 7.2543515384947e+1;
        xceref[3] = 6.4604135220732e+1;
        xceref[4] = 6.4098557617690e+2;
    } else {
        *verified = 0;
    }
    /*
     * ---------------------------------------------------------------------
     * verification test for residuals if gridsize is one of
     * the defined grid sizes above (*class_npb != 'U')
     * ---------------------------------------------------------------------
     * compute the difference of solution values and the known reference values.
     * ---------------------------------------------------------------------
     */
    for m in 0..5 {
        xcrdif[m] = ((xcr[m] - xcrref[m]) / xcrref[m]).abs();
        xcedif[m] = ((xce[m] - xceref[m]) / xceref[m]).abs();
    }
    /*
     * ---------------------------------------------------------------------
     * output the comparison of computed results to known cases.
     * ---------------------------------------------------------------------
     */
    if CLASS != 'U' {
        println!(" Verification being performed for class_npb {}", CLASS);
        println!(" accuracy setting for epsilon = {:>20.13e}", EPSILON);
        *verified = {
            let x;
            if (DT_DEFAULT - dtref).abs() <= EPSILON {
                x = 1
            } else {
                x = 0
            }
            x
        };
        if *verified == 0 {
            println!(" DT does not match the reference value of {:>15.8e}", dtref);
        }
        println!(" Comparison of RMS-norms of residual");
        for m in 0..5 {
            if xcrdif[m] <= EPSILON {
                println!(
                    "          {:>2} {:>20.13e} {:>20.13e} {:>20.13e}",
                    m + 1,
                    xcr[m],
                    xcrref[m],
                    xcrdif[m]
                );
            } else {
                *verified = 0;
                println!(
                    " FAILURE: {:>2} {:>20.13e} {:>20.13e} {:>20.13e}",
                    m + 1,
                    xcr[m],
                    xcrref[m],
                    xcrdif[m]
                );
            }
        }
        print!(" Comparison of RMS-norms of solution error\n");
        for m in 0..5 {
            if xcedif[m] <= EPSILON {
                println!(
                    "          {:>2} {:>20.13e} {:>20.13e} {:>20.13e}",
                    m + 1,
                    xce[m],
                    xceref[m],
                    xcedif[m]
                );
            } else {
                *verified = 0;
                println!(
                    " FAILURE: {:>2} {:>20.13e} {:>20.13e} {:>20.13e}",
                    m + 1,
                    xce[m],
                    xceref[m],
                    xcedif[m]
                );
            }
        }
        if *verified == 1 {
            println!(" Verification Successful");
        } else {
            println!(" Verification failed");
        }
    }
}

fn x_solve(
    rho_i: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    fjac: &mut [[[f64; 5]; 5]],
    njac: &mut [[[f64; 5]; 5]],
    lhs: &mut [[[[f64; 5]; 5]; 3]],
    rhs: &mut [[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    u: &[[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    qs: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    square: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    timers: &mut Timer,
) {
    if TIMERS {
        timers.start(T_XSOLVE);
    }
    /*
     * ---------------------------------------------------------------------
     * this function computes the left hand side in the xi-direction
     * ---------------------------------------------------------------------
     */
    let isize = GRID_POINTS[0] - 1;
    /*
     * ---------------------------------------------------------------------
     * determine a (labeled f) and n jacobians
     * ---------------------------------------------------------------------
     */
    for k in 1..GRID_POINTS[2] - 1 {
        for j in 1..GRID_POINTS[1] - 1 {
            for i in 0..isize + 1 {
                let tmp1 = rho_i[k][j][i];
                let tmp2 = tmp1 * tmp1;
                let tmp3 = tmp1 * tmp2;
                fjac[i][0][0] = 0.0;
                fjac[i][1][0] = 1.0;
                fjac[i][2][0] = 0.0;
                fjac[i][3][0] = 0.0;
                fjac[i][4][0] = 0.0;
                fjac[i][0][1] = -(u[k][j][i][1] * tmp2 * u[k][j][i][1]) + C2 * qs[k][j][i];
                fjac[i][1][1] = (2.0 - C2) * (u[k][j][i][1] / u[k][j][i][0]);
                fjac[i][2][1] = -C2 * (u[k][j][i][2] * tmp1);
                fjac[i][3][1] = -C2 * (u[k][j][i][3] * tmp1);
                fjac[i][4][1] = C2;
                fjac[i][0][2] = -(u[k][j][i][1] * u[k][j][i][2]) * tmp2;
                fjac[i][1][2] = u[k][j][i][2] * tmp1;
                fjac[i][2][2] = u[k][j][i][1] * tmp1;
                fjac[i][3][2] = 0.0;
                fjac[i][4][2] = 0.0;
                fjac[i][0][3] = -(u[k][j][i][1] * u[k][j][i][3]) * tmp2;
                fjac[i][1][3] = u[k][j][i][3] * tmp1;
                fjac[i][2][3] = 0.0;
                fjac[i][3][3] = u[k][j][i][1] * tmp1;
                fjac[i][4][3] = 0.0;
                fjac[i][0][4] =
                    (C2 * 2.0 * square[k][j][i] - C1 * u[k][j][i][4]) * (u[k][j][i][1] * tmp2);
                fjac[i][1][4] = C1 * u[k][j][i][4] * tmp1
                    - C2 * (u[k][j][i][1] * u[k][j][i][1] * tmp2 + qs[k][j][i]);
                fjac[i][2][4] = -C2 * (u[k][j][i][2] * u[k][j][i][1]) * tmp2;
                fjac[i][3][4] = -C2 * (u[k][j][i][3] * u[k][j][i][1]) * tmp2;
                fjac[i][4][4] = C1 * (u[k][j][i][1] * tmp1);
                njac[i][0][0] = 0.0;
                njac[i][1][0] = 0.0;
                njac[i][2][0] = 0.0;
                njac[i][3][0] = 0.0;
                njac[i][4][0] = 0.0;
                njac[i][0][1] = -CON43 * C3C4 * tmp2 * u[k][j][i][1];
                njac[i][1][1] = CON43 * C3C4 * tmp1;
                njac[i][2][1] = 0.0;
                njac[i][3][1] = 0.0;
                njac[i][4][1] = 0.0;
                njac[i][0][2] = -C3C4 * tmp2 * u[k][j][i][2];
                njac[i][1][2] = 0.0;
                njac[i][2][2] = C3C4 * tmp1;
                njac[i][3][2] = 0.0;
                njac[i][4][2] = 0.0;
                njac[i][0][3] = -C3C4 * tmp2 * u[k][j][i][3];
                njac[i][1][3] = 0.0;
                njac[i][2][3] = 0.0;
                njac[i][3][3] = C3C4 * tmp1;
                njac[i][4][3] = 0.0;
                njac[i][0][4] = -(CON43 * C3C4 - C1345) * tmp3 * (u[k][j][i][1] * u[k][j][i][1])
                    - (C3C4 - C1345) * tmp3 * (u[k][j][i][2] * u[k][j][i][2])
                    - (C3C4 - C1345) * tmp3 * (u[k][j][i][3] * u[k][j][i][3])
                    - C1345 * tmp2 * u[k][j][i][4];
                njac[i][1][4] = (CON43 * C3C4 - C1345) * tmp2 * u[k][j][i][1];
                njac[i][2][4] = (C3C4 - C1345) * tmp2 * u[k][j][i][2];
                njac[i][3][4] = (C3C4 - C1345) * tmp2 * u[k][j][i][3];
                njac[i][4][4] = (C1345) * tmp1;
            }
            /*
             * ---------------------------------------------------------------------
             * now jacobians set, so form left hand side in x direction
             * ---------------------------------------------------------------------
             */
            lhsinit(&mut lhs[..], isize);
            for i in 1..isize {
                let tmp1 = DT_DEFAULT * TX1;
                let tmp2 = DT_DEFAULT * TX2;
                lhs[i][AA][0][0] =
                    -tmp2 * fjac[i - 1][0][0] - tmp1 * njac[i - 1][0][0] - tmp1 * DX1;
                lhs[i][AA][1][0] = -tmp2 * fjac[i - 1][1][0] - tmp1 * njac[i - 1][1][0];
                lhs[i][AA][2][0] = -tmp2 * fjac[i - 1][2][0] - tmp1 * njac[i - 1][2][0];
                lhs[i][AA][3][0] = -tmp2 * fjac[i - 1][3][0] - tmp1 * njac[i - 1][3][0];
                lhs[i][AA][4][0] = -tmp2 * fjac[i - 1][4][0] - tmp1 * njac[i - 1][4][0];
                lhs[i][AA][0][1] = -tmp2 * fjac[i - 1][0][1] - tmp1 * njac[i - 1][0][1];
                lhs[i][AA][1][1] =
                    -tmp2 * fjac[i - 1][1][1] - tmp1 * njac[i - 1][1][1] - tmp1 * DX2;
                lhs[i][AA][2][1] = -tmp2 * fjac[i - 1][2][1] - tmp1 * njac[i - 1][2][1];
                lhs[i][AA][3][1] = -tmp2 * fjac[i - 1][3][1] - tmp1 * njac[i - 1][3][1];
                lhs[i][AA][4][1] = -tmp2 * fjac[i - 1][4][1] - tmp1 * njac[i - 1][4][1];
                lhs[i][AA][0][2] = -tmp2 * fjac[i - 1][0][2] - tmp1 * njac[i - 1][0][2];
                lhs[i][AA][1][2] = -tmp2 * fjac[i - 1][1][2] - tmp1 * njac[i - 1][1][2];
                lhs[i][AA][2][2] =
                    -tmp2 * fjac[i - 1][2][2] - tmp1 * njac[i - 1][2][2] - tmp1 * DX3;
                lhs[i][AA][3][2] = -tmp2 * fjac[i - 1][3][2] - tmp1 * njac[i - 1][3][2];
                lhs[i][AA][4][2] = -tmp2 * fjac[i - 1][4][2] - tmp1 * njac[i - 1][4][2];
                lhs[i][AA][0][3] = -tmp2 * fjac[i - 1][0][3] - tmp1 * njac[i - 1][0][3];
                lhs[i][AA][1][3] = -tmp2 * fjac[i - 1][1][3] - tmp1 * njac[i - 1][1][3];
                lhs[i][AA][2][3] = -tmp2 * fjac[i - 1][2][3] - tmp1 * njac[i - 1][2][3];
                lhs[i][AA][3][3] =
                    -tmp2 * fjac[i - 1][3][3] - tmp1 * njac[i - 1][3][3] - tmp1 * DX4;
                lhs[i][AA][4][3] = -tmp2 * fjac[i - 1][4][3] - tmp1 * njac[i - 1][4][3];
                lhs[i][AA][0][4] = -tmp2 * fjac[i - 1][0][4] - tmp1 * njac[i - 1][0][4];
                lhs[i][AA][1][4] = -tmp2 * fjac[i - 1][1][4] - tmp1 * njac[i - 1][1][4];
                lhs[i][AA][2][4] = -tmp2 * fjac[i - 1][2][4] - tmp1 * njac[i - 1][2][4];
                lhs[i][AA][3][4] = -tmp2 * fjac[i - 1][3][4] - tmp1 * njac[i - 1][3][4];
                lhs[i][AA][4][4] =
                    -tmp2 * fjac[i - 1][4][4] - tmp1 * njac[i - 1][4][4] - tmp1 * DX5;
                lhs[i][BB][0][0] = 1.0 + tmp1 * 2.0 * njac[i][0][0] + tmp1 * 2.0 * DX1;
                lhs[i][BB][1][0] = tmp1 * 2.0 * njac[i][1][0];
                lhs[i][BB][2][0] = tmp1 * 2.0 * njac[i][2][0];
                lhs[i][BB][3][0] = tmp1 * 2.0 * njac[i][3][0];
                lhs[i][BB][4][0] = tmp1 * 2.0 * njac[i][4][0];
                lhs[i][BB][0][1] = tmp1 * 2.0 * njac[i][0][1];
                lhs[i][BB][1][1] = 1.0 + tmp1 * 2.0 * njac[i][1][1] + tmp1 * 2.0 * DX2;
                lhs[i][BB][2][1] = tmp1 * 2.0 * njac[i][2][1];
                lhs[i][BB][3][1] = tmp1 * 2.0 * njac[i][3][1];
                lhs[i][BB][4][1] = tmp1 * 2.0 * njac[i][4][1];
                lhs[i][BB][0][2] = tmp1 * 2.0 * njac[i][0][2];
                lhs[i][BB][1][2] = tmp1 * 2.0 * njac[i][1][2];
                lhs[i][BB][2][2] = 1.0 + tmp1 * 2.0 * njac[i][2][2] + tmp1 * 2.0 * DX3;
                lhs[i][BB][3][2] = tmp1 * 2.0 * njac[i][3][2];
                lhs[i][BB][4][2] = tmp1 * 2.0 * njac[i][4][2];
                lhs[i][BB][0][3] = tmp1 * 2.0 * njac[i][0][3];
                lhs[i][BB][1][3] = tmp1 * 2.0 * njac[i][1][3];
                lhs[i][BB][2][3] = tmp1 * 2.0 * njac[i][2][3];
                lhs[i][BB][3][3] = 1.0 + tmp1 * 2.0 * njac[i][3][3] + tmp1 * 2.0 * DX4;
                lhs[i][BB][4][3] = tmp1 * 2.0 * njac[i][4][3];
                lhs[i][BB][0][4] = tmp1 * 2.0 * njac[i][0][4];
                lhs[i][BB][1][4] = tmp1 * 2.0 * njac[i][1][4];
                lhs[i][BB][2][4] = tmp1 * 2.0 * njac[i][2][4];
                lhs[i][BB][3][4] = tmp1 * 2.0 * njac[i][3][4];
                lhs[i][BB][4][4] = 1.0 + tmp1 * 2.0 * njac[i][4][4] + tmp1 * 2.0 * DX5;
                lhs[i][CC][0][0] = tmp2 * fjac[i + 1][0][0] - tmp1 * njac[i + 1][0][0] - tmp1 * DX1;
                lhs[i][CC][1][0] = tmp2 * fjac[i + 1][1][0] - tmp1 * njac[i + 1][1][0];
                lhs[i][CC][2][0] = tmp2 * fjac[i + 1][2][0] - tmp1 * njac[i + 1][2][0];
                lhs[i][CC][3][0] = tmp2 * fjac[i + 1][3][0] - tmp1 * njac[i + 1][3][0];
                lhs[i][CC][4][0] = tmp2 * fjac[i + 1][4][0] - tmp1 * njac[i + 1][4][0];
                lhs[i][CC][0][1] = tmp2 * fjac[i + 1][0][1] - tmp1 * njac[i + 1][0][1];
                lhs[i][CC][1][1] = tmp2 * fjac[i + 1][1][1] - tmp1 * njac[i + 1][1][1] - tmp1 * DX2;
                lhs[i][CC][2][1] = tmp2 * fjac[i + 1][2][1] - tmp1 * njac[i + 1][2][1];
                lhs[i][CC][3][1] = tmp2 * fjac[i + 1][3][1] - tmp1 * njac[i + 1][3][1];
                lhs[i][CC][4][1] = tmp2 * fjac[i + 1][4][1] - tmp1 * njac[i + 1][4][1];
                lhs[i][CC][0][2] = tmp2 * fjac[i + 1][0][2] - tmp1 * njac[i + 1][0][2];
                lhs[i][CC][1][2] = tmp2 * fjac[i + 1][1][2] - tmp1 * njac[i + 1][1][2];
                lhs[i][CC][2][2] = tmp2 * fjac[i + 1][2][2] - tmp1 * njac[i + 1][2][2] - tmp1 * DX3;
                lhs[i][CC][3][2] = tmp2 * fjac[i + 1][3][2] - tmp1 * njac[i + 1][3][2];
                lhs[i][CC][4][2] = tmp2 * fjac[i + 1][4][2] - tmp1 * njac[i + 1][4][2];
                lhs[i][CC][0][3] = tmp2 * fjac[i + 1][0][3] - tmp1 * njac[i + 1][0][3];
                lhs[i][CC][1][3] = tmp2 * fjac[i + 1][1][3] - tmp1 * njac[i + 1][1][3];
                lhs[i][CC][2][3] = tmp2 * fjac[i + 1][2][3] - tmp1 * njac[i + 1][2][3];
                lhs[i][CC][3][3] = tmp2 * fjac[i + 1][3][3] - tmp1 * njac[i + 1][3][3] - tmp1 * DX4;
                lhs[i][CC][4][3] = tmp2 * fjac[i + 1][4][3] - tmp1 * njac[i + 1][4][3];
                lhs[i][CC][0][4] = tmp2 * fjac[i + 1][0][4] - tmp1 * njac[i + 1][0][4];
                lhs[i][CC][1][4] = tmp2 * fjac[i + 1][1][4] - tmp1 * njac[i + 1][1][4];
                lhs[i][CC][2][4] = tmp2 * fjac[i + 1][2][4] - tmp1 * njac[i + 1][2][4];
                lhs[i][CC][3][4] = tmp2 * fjac[i + 1][3][4] - tmp1 * njac[i + 1][3][4];
                lhs[i][CC][4][4] = tmp2 * fjac[i + 1][4][4] - tmp1 * njac[i + 1][4][4] - tmp1 * DX5;
            }
            /*
             * ---------------------------------------------------------------------
             * performs guaussian elimination on this cell.
             *
             * assumes that unpacking routines for non-first cells
             * preload C' and rhs' from previous cell.
             *
             * assumed send happens outside this routine, but that
             * c'(IMAX) and rhs'(IMAX) will be sent to next cell
             * ---------------------------------------------------------------------
             * outer most do loops - sweeping in i direction
             * ---------------------------------------------------------------------
             * multiply c(0,j,k) by b_inverse and copy back to c
             * multiply rhs(0) by b_inverse(0) and copy to rhs
             * ---------------------------------------------------------------------
             */
            binvcrhs(&mut lhs[..], 0, BB, 0, CC, &mut rhs[k][j][0][..]);
            /*
             * ---------------------------------------------------------------------
             * begin inner most do loop
             * do all the elements of the cell unless last
             * ---------------------------------------------------------------------
             */
            for i in 1..isize {
                /*
                 * -------------------------------------------------------------------
                 * rhs(i) = rhs(i) - A*rhs(i-1)
                 * -------------------------------------------------------------------
                 */
                matvec_sub(&mut lhs[i][AA][..], &mut rhs[..], k, j, i - 1, k, j, i);

                /*
                 * -------------------------------------------------------------------
                 * B(i) = B(i) - C(i-1)*A(i)
                 * -------------------------------------------------------------------
                 */
                matmul_sub(&mut lhs[..], i, AA, i - 1, CC, i, BB);
                /*
                 * -------------------------------------------------------------------
                 * multiply c(i,j,k) by b_inverse and copy back to c
                 * multiply rhs(1,j,k) by b_inverse(1,j,k) and copy to rhs
                 * -------------------------------------------------------------------
                 */
                binvcrhs(&mut lhs[..], i, BB, i, CC, &mut rhs[k][j][i][..]);
            }
            /*
             * ---------------------------------------------------------------------
             * rhs(isize) = rhs(isize) - A*rhs(isize-1)
             * ---------------------------------------------------------------------
             */
            matvec_sub(
                &mut lhs[isize][AA][..],
                &mut rhs[..],
                k,
                j,
                isize - 1,
                k,
                j,
                isize,
            );
            /*
             * ---------------------------------------------------------------------
             * B(isize) = B(isize) - C(isize-1)*A(isize)
             * ---------------------------------------------------------------------
             */
            matmul_sub(&mut lhs[..], isize, AA, isize - 1, CC, isize, BB);
            /*
             * ---------------------------------------------------------------------
             * multiply rhs() by b_inverse() and copy to rhs
             * ---------------------------------------------------------------------
             */
            binvrhs(&mut lhs[isize][BB][..], &mut rhs[k][j][isize][..]);
            /*
             * ---------------------------------------------------------------------
             * back solve: if last cell, then generate U(isize)=rhs(isize)
             * else assume U(isize) is loaded in un pack backsub_info
             * so just use it
             * after u(istart) will be sent to next cell
             * ---------------------------------------------------------------------
             */
            for i in (0..isize).rev() {
                for m in 0..BLOCK_SIZE as usize {
                    for n in 0..BLOCK_SIZE as usize {
                        rhs[k][j][i][m] = rhs[k][j][i][m] - lhs[i][CC][n][m] * rhs[k][j][i + 1][n];
                    }
                }
            }
        }
    }
    if TIMERS {
        timers.stop(T_XSOLVE);
    }
}

fn y_solve(
    rho_i: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    fjac: &mut [[[f64; 5]; 5]],
    njac: &mut [[[f64; 5]; 5]],
    lhs: &mut [[[[f64; 5]; 5]; 3]],
    rhs: &mut [[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    u: &[[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    qs: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    square: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    timers: &mut Timer,
) {
    if TIMERS {
        timers.start(T_YSOLVE);
    }
    /*
     * ---------------------------------------------------------------------
     * this function computes the left hand side for the three y-factors
     * ---------------------------------------------------------------------
     */
    let jsize = GRID_POINTS[1] - 1;
    /*
     * ---------------------------------------------------------------------
     * compute the indices for storing the tri-diagonal matrix;
     * determine a (labeled f) and n jacobians for cell c
     * ---------------------------------------------------------------------
     */
    for k in 1..GRID_POINTS[2] - 1 {
        for i in 1..GRID_POINTS[0] - 1 {
            for j in 0..jsize + 1 {
                let tmp1 = rho_i[k][j][i];
                let tmp2 = tmp1 * tmp1;
                let tmp3 = tmp1 * tmp2;
                fjac[j][0][0] = 0.0;
                fjac[j][1][0] = 0.0;
                fjac[j][2][0] = 1.0;
                fjac[j][3][0] = 0.0;
                fjac[j][4][0] = 0.0;
                fjac[j][0][1] = -(u[k][j][i][1] * u[k][j][i][2]) * tmp2;
                fjac[j][1][1] = u[k][j][i][2] * tmp1;
                fjac[j][2][1] = u[k][j][i][1] * tmp1;
                fjac[j][3][1] = 0.0;
                fjac[j][4][1] = 0.0;
                fjac[j][0][2] = -(u[k][j][i][2] * u[k][j][i][2] * tmp2) + C2 * qs[k][j][i];
                fjac[j][1][2] = -C2 * u[k][j][i][1] * tmp1;
                fjac[j][2][2] = (2.0 - C2) * u[k][j][i][2] * tmp1;
                fjac[j][3][2] = -C2 * u[k][j][i][3] * tmp1;
                fjac[j][4][2] = C2;
                fjac[j][0][3] = -(u[k][j][i][2] * u[k][j][i][3]) * tmp2;
                fjac[j][1][3] = 0.0;
                fjac[j][2][3] = u[k][j][i][3] * tmp1;
                fjac[j][3][3] = u[k][j][i][2] * tmp1;
                fjac[j][4][3] = 0.0;
                fjac[j][0][4] =
                    (C2 * 2.0 * square[k][j][i] - C1 * u[k][j][i][4]) * u[k][j][i][2] * tmp2;
                fjac[j][1][4] = -C2 * u[k][j][i][1] * u[k][j][i][2] * tmp2;
                fjac[j][2][4] = C1 * u[k][j][i][4] * tmp1
                    - C2 * (qs[k][j][i] + u[k][j][i][2] * u[k][j][i][2] * tmp2);
                fjac[j][3][4] = -C2 * (u[k][j][i][2] * u[k][j][i][3]) * tmp2;
                fjac[j][4][4] = C1 * u[k][j][i][2] * tmp1;
                njac[j][0][0] = 0.0;
                njac[j][1][0] = 0.0;
                njac[j][2][0] = 0.0;
                njac[j][3][0] = 0.0;
                njac[j][4][0] = 0.0;
                njac[j][0][1] = -C3C4 * tmp2 * u[k][j][i][1];
                njac[j][1][1] = C3C4 * tmp1;
                njac[j][2][1] = 0.0;
                njac[j][3][1] = 0.0;
                njac[j][4][1] = 0.0;
                njac[j][0][2] = -CON43 * C3C4 * tmp2 * u[k][j][i][2];
                njac[j][1][2] = 0.0;
                njac[j][2][2] = CON43 * C3C4 * tmp1;
                njac[j][3][2] = 0.0;
                njac[j][4][2] = 0.0;
                njac[j][0][3] = -C3C4 * tmp2 * u[k][j][i][3];
                njac[j][1][3] = 0.0;
                njac[j][2][3] = 0.0;
                njac[j][3][3] = C3C4 * tmp1;
                njac[j][4][3] = 0.0;
                njac[j][0][4] = -(C3C4 - C1345) * tmp3 * (u[k][j][i][1] * u[k][j][i][1])
                    - (CON43 * C3C4 - C1345) * tmp3 * (u[k][j][i][2] * u[k][j][i][2])
                    - (C3C4 - C1345) * tmp3 * (u[k][j][i][3] * u[k][j][i][3])
                    - C1345 * tmp2 * u[k][j][i][4];
                njac[j][1][4] = (C3C4 - C1345) * tmp2 * u[k][j][i][1];
                njac[j][2][4] = (CON43 * C3C4 - C1345) * tmp2 * u[k][j][i][2];
                njac[j][3][4] = (C3C4 - C1345) * tmp2 * u[k][j][i][3];
                njac[j][4][4] = (C1345) * tmp1;
            }
            /*
             * ---------------------------------------------------------------------
             * now joacobians set, so form left hand side in y direction
             * ---------------------------------------------------------------------
             */
            lhsinit(&mut lhs[..], jsize);
            for j in 1..jsize {
                let tmp1 = DT_DEFAULT * TY1;
                let tmp2 = DT_DEFAULT * TY2;
                lhs[j][AA][0][0] =
                    -tmp2 * fjac[j - 1][0][0] - tmp1 * njac[j - 1][0][0] - tmp1 * DY1;
                lhs[j][AA][1][0] = -tmp2 * fjac[j - 1][1][0] - tmp1 * njac[j - 1][1][0];
                lhs[j][AA][2][0] = -tmp2 * fjac[j - 1][2][0] - tmp1 * njac[j - 1][2][0];
                lhs[j][AA][3][0] = -tmp2 * fjac[j - 1][3][0] - tmp1 * njac[j - 1][3][0];
                lhs[j][AA][4][0] = -tmp2 * fjac[j - 1][4][0] - tmp1 * njac[j - 1][4][0];
                lhs[j][AA][0][1] = -tmp2 * fjac[j - 1][0][1] - tmp1 * njac[j - 1][0][1];
                lhs[j][AA][1][1] =
                    -tmp2 * fjac[j - 1][1][1] - tmp1 * njac[j - 1][1][1] - tmp1 * DY2;
                lhs[j][AA][2][1] = -tmp2 * fjac[j - 1][2][1] - tmp1 * njac[j - 1][2][1];
                lhs[j][AA][3][1] = -tmp2 * fjac[j - 1][3][1] - tmp1 * njac[j - 1][3][1];
                lhs[j][AA][4][1] = -tmp2 * fjac[j - 1][4][1] - tmp1 * njac[j - 1][4][1];
                lhs[j][AA][0][2] = -tmp2 * fjac[j - 1][0][2] - tmp1 * njac[j - 1][0][2];
                lhs[j][AA][1][2] = -tmp2 * fjac[j - 1][1][2] - tmp1 * njac[j - 1][1][2];
                lhs[j][AA][2][2] =
                    -tmp2 * fjac[j - 1][2][2] - tmp1 * njac[j - 1][2][2] - tmp1 * DY3;
                lhs[j][AA][3][2] = -tmp2 * fjac[j - 1][3][2] - tmp1 * njac[j - 1][3][2];
                lhs[j][AA][4][2] = -tmp2 * fjac[j - 1][4][2] - tmp1 * njac[j - 1][4][2];
                lhs[j][AA][0][3] = -tmp2 * fjac[j - 1][0][3] - tmp1 * njac[j - 1][0][3];
                lhs[j][AA][1][3] = -tmp2 * fjac[j - 1][1][3] - tmp1 * njac[j - 1][1][3];
                lhs[j][AA][2][3] = -tmp2 * fjac[j - 1][2][3] - tmp1 * njac[j - 1][2][3];
                lhs[j][AA][3][3] =
                    -tmp2 * fjac[j - 1][3][3] - tmp1 * njac[j - 1][3][3] - tmp1 * DY4;
                lhs[j][AA][4][3] = -tmp2 * fjac[j - 1][4][3] - tmp1 * njac[j - 1][4][3];
                lhs[j][AA][0][4] = -tmp2 * fjac[j - 1][0][4] - tmp1 * njac[j - 1][0][4];
                lhs[j][AA][1][4] = -tmp2 * fjac[j - 1][1][4] - tmp1 * njac[j - 1][1][4];
                lhs[j][AA][2][4] = -tmp2 * fjac[j - 1][2][4] - tmp1 * njac[j - 1][2][4];
                lhs[j][AA][3][4] = -tmp2 * fjac[j - 1][3][4] - tmp1 * njac[j - 1][3][4];
                lhs[j][AA][4][4] =
                    -tmp2 * fjac[j - 1][4][4] - tmp1 * njac[j - 1][4][4] - tmp1 * DY5;
                lhs[j][BB][0][0] = 1.0 + tmp1 * 2.0 * njac[j][0][0] + tmp1 * 2.0 * DY1;
                lhs[j][BB][1][0] = tmp1 * 2.0 * njac[j][1][0];
                lhs[j][BB][2][0] = tmp1 * 2.0 * njac[j][2][0];
                lhs[j][BB][3][0] = tmp1 * 2.0 * njac[j][3][0];
                lhs[j][BB][4][0] = tmp1 * 2.0 * njac[j][4][0];
                lhs[j][BB][0][1] = tmp1 * 2.0 * njac[j][0][1];
                lhs[j][BB][1][1] = 1.0 + tmp1 * 2.0 * njac[j][1][1] + tmp1 * 2.0 * DY2;
                lhs[j][BB][2][1] = tmp1 * 2.0 * njac[j][2][1];
                lhs[j][BB][3][1] = tmp1 * 2.0 * njac[j][3][1];
                lhs[j][BB][4][1] = tmp1 * 2.0 * njac[j][4][1];
                lhs[j][BB][0][2] = tmp1 * 2.0 * njac[j][0][2];
                lhs[j][BB][1][2] = tmp1 * 2.0 * njac[j][1][2];
                lhs[j][BB][2][2] = 1.0 + tmp1 * 2.0 * njac[j][2][2] + tmp1 * 2.0 * DY3;
                lhs[j][BB][3][2] = tmp1 * 2.0 * njac[j][3][2];
                lhs[j][BB][4][2] = tmp1 * 2.0 * njac[j][4][2];
                lhs[j][BB][0][3] = tmp1 * 2.0 * njac[j][0][3];
                lhs[j][BB][1][3] = tmp1 * 2.0 * njac[j][1][3];
                lhs[j][BB][2][3] = tmp1 * 2.0 * njac[j][2][3];
                lhs[j][BB][3][3] = 1.0 + tmp1 * 2.0 * njac[j][3][3] + tmp1 * 2.0 * DY4;
                lhs[j][BB][4][3] = tmp1 * 2.0 * njac[j][4][3];
                lhs[j][BB][0][4] = tmp1 * 2.0 * njac[j][0][4];
                lhs[j][BB][1][4] = tmp1 * 2.0 * njac[j][1][4];
                lhs[j][BB][2][4] = tmp1 * 2.0 * njac[j][2][4];
                lhs[j][BB][3][4] = tmp1 * 2.0 * njac[j][3][4];
                lhs[j][BB][4][4] = 1.0 + tmp1 * 2.0 * njac[j][4][4] + tmp1 * 2.0 * DY5;
                lhs[j][CC][0][0] = tmp2 * fjac[j + 1][0][0] - tmp1 * njac[j + 1][0][0] - tmp1 * DY1;
                lhs[j][CC][1][0] = tmp2 * fjac[j + 1][1][0] - tmp1 * njac[j + 1][1][0];
                lhs[j][CC][2][0] = tmp2 * fjac[j + 1][2][0] - tmp1 * njac[j + 1][2][0];
                lhs[j][CC][3][0] = tmp2 * fjac[j + 1][3][0] - tmp1 * njac[j + 1][3][0];
                lhs[j][CC][4][0] = tmp2 * fjac[j + 1][4][0] - tmp1 * njac[j + 1][4][0];
                lhs[j][CC][0][1] = tmp2 * fjac[j + 1][0][1] - tmp1 * njac[j + 1][0][1];
                lhs[j][CC][1][1] = tmp2 * fjac[j + 1][1][1] - tmp1 * njac[j + 1][1][1] - tmp1 * DY2;
                lhs[j][CC][2][1] = tmp2 * fjac[j + 1][2][1] - tmp1 * njac[j + 1][2][1];
                lhs[j][CC][3][1] = tmp2 * fjac[j + 1][3][1] - tmp1 * njac[j + 1][3][1];
                lhs[j][CC][4][1] = tmp2 * fjac[j + 1][4][1] - tmp1 * njac[j + 1][4][1];
                lhs[j][CC][0][2] = tmp2 * fjac[j + 1][0][2] - tmp1 * njac[j + 1][0][2];
                lhs[j][CC][1][2] = tmp2 * fjac[j + 1][1][2] - tmp1 * njac[j + 1][1][2];
                lhs[j][CC][2][2] = tmp2 * fjac[j + 1][2][2] - tmp1 * njac[j + 1][2][2] - tmp1 * DY3;
                lhs[j][CC][3][2] = tmp2 * fjac[j + 1][3][2] - tmp1 * njac[j + 1][3][2];
                lhs[j][CC][4][2] = tmp2 * fjac[j + 1][4][2] - tmp1 * njac[j + 1][4][2];
                lhs[j][CC][0][3] = tmp2 * fjac[j + 1][0][3] - tmp1 * njac[j + 1][0][3];
                lhs[j][CC][1][3] = tmp2 * fjac[j + 1][1][3] - tmp1 * njac[j + 1][1][3];
                lhs[j][CC][2][3] = tmp2 * fjac[j + 1][2][3] - tmp1 * njac[j + 1][2][3];
                lhs[j][CC][3][3] = tmp2 * fjac[j + 1][3][3] - tmp1 * njac[j + 1][3][3] - tmp1 * DY4;
                lhs[j][CC][4][3] = tmp2 * fjac[j + 1][4][3] - tmp1 * njac[j + 1][4][3];
                lhs[j][CC][0][4] = tmp2 * fjac[j + 1][0][4] - tmp1 * njac[j + 1][0][4];
                lhs[j][CC][1][4] = tmp2 * fjac[j + 1][1][4] - tmp1 * njac[j + 1][1][4];
                lhs[j][CC][2][4] = tmp2 * fjac[j + 1][2][4] - tmp1 * njac[j + 1][2][4];
                lhs[j][CC][3][4] = tmp2 * fjac[j + 1][3][4] - tmp1 * njac[j + 1][3][4];
                lhs[j][CC][4][4] = tmp2 * fjac[j + 1][4][4] - tmp1 * njac[j + 1][4][4] - tmp1 * DY5;
            }
            /*
             * ---------------------------------------------------------------------
             * performs guaussian elimination on this cell.
             *
             * assumes that unpacking routines for non-first cells
             * preload c' and rhs' from previous cell.
             *
             * assumed send happens outside this routine, but that
             * c'(JMAX) and rhs'(JMAX) will be sent to next cell
             * ---------------------------------------------------------------------
             * multiply c(i,0,k) by b_inverse and copy back to c
             * multiply rhs(0) by b_inverse(0) and copy to rhs
             * ---------------------------------------------------------------------
             */
            binvcrhs(&mut lhs[..], 0, BB, 0, CC, &mut rhs[k][0][i][..]);
            /*
             * ---------------------------------------------------------------------
             * begin inner most do loop
             * do all the elements of the cell unless last
             * ---------------------------------------------------------------------
             */
            for j in 1..jsize {
                /*
                 * -------------------------------------------------------------------
                 * subtract A*lhs_vector(j-1) from lhs_vector(j)
                 *
                 * rhs(j) = rhs(j) - A*rhs(j-1)
                 * -------------------------------------------------------------------
                 */
                matvec_sub(&mut lhs[j][AA][..], &mut rhs[..], k, j - 1, i, k, j, i);
                /*
                 * -------------------------------------------------------------------
                 * B(j) = B(j) - C(j-1)*A(j)
                 * -------------------------------------------------------------------
                 */
                matmul_sub(&mut lhs[..], j, AA, j - 1, CC, j, BB);
                /*
                 * -------------------------------------------------------------------
                 * multiply c(i,j,k) by b_inverse and copy back to c
                 * multiply rhs(i,1,k) by b_inverse(i,1,k) and copy to rhs
                 * -------------------------------------------------------------------
                 */
                binvcrhs(&mut lhs[..], j, BB, j, CC, &mut rhs[k][j][i][..]);
            }
            /*
             * ---------------------------------------------------------------------
             * rhs(jsize) = rhs(jsize) - A*rhs(jsize-1)
             * ---------------------------------------------------------------------
             */
            matvec_sub(
                &mut lhs[jsize][AA][..],
                &mut rhs[..],
                k,
                jsize - 1,
                i,
                k,
                jsize,
                i,
            );
            /*
             * ---------------------------------------------------------------------
             * B(jsize) = B(jsize) - C(jsize-1)*A(jsize)
             * matmul_sub(aa,i,jsize,k,c,
             * $ cc,i,jsize-1,k,c,bb,i,jsize,k)
             * ---------------------------------------------------------------------
             */
            matmul_sub(&mut lhs[..], jsize, AA, jsize - 1, CC, jsize, BB);
            /*
             * ---------------------------------------------------------------------
             * multiply rhs(jsize) by b_inverse(jsize) and copy to rhs
             * ---------------------------------------------------------------------
             */
            binvrhs(&mut lhs[jsize][BB][..], &mut rhs[k][jsize][i][..]);
            /*
             * ---------------------------------------------------------------------
             * back solve: if last cell, then generate U(jsize)=rhs(jsize)
             * else assume U(jsize) is loaded in un pack backsub_info
             * so just use it
             * after u(jstart) will be sent to next cell
             * ---------------------------------------------------------------------
             */
            for j in (0..jsize).rev() {
                for m in 0..BLOCK_SIZE as usize {
                    for n in 0..BLOCK_SIZE as usize {
                        rhs[k][j][i][m] = rhs[k][j][i][m] - lhs[j][CC][n][m] * rhs[k][j + 1][i][n];
                    }
                }
            }
        }
    }
    if TIMERS {
        timers.stop(T_YSOLVE);
    }
}

fn z_solve(
    fjac: &mut [[[f64; 5]; 5]],
    njac: &mut [[[f64; 5]; 5]],
    lhs: &mut [[[[f64; 5]; 5]; 3]],
    rhs: &mut [[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    u: &[[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    qs: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    square: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    timers: &mut Timer,
) {
    if TIMERS {
        timers.start(T_ZSOLVE);
    }
    /*
     * ---------------------------------------------------------------------
     * this function computes the left hand side for the three z-factors
     * ---------------------------------------------------------------------
     */
    let ksize = GRID_POINTS[2] - 1;
    /*
     * ---------------------------------------------------------------------
     * compute the indices for storing the block-diagonal matrix;
     * determine c (labeled f) and s jacobians
     * ---------------------------------------------------------------------
     */
    for j in 1..GRID_POINTS[1] - 1 {
        for i in 1..GRID_POINTS[0] - 1 {
            for k in 0..ksize + 1 {
                let tmp1 = 1.0 / u[k][j][i][0];
                let tmp2 = tmp1 * tmp1;
                let tmp3 = tmp1 * tmp2;
                fjac[k][0][0] = 0.0;
                fjac[k][1][0] = 0.0;
                fjac[k][2][0] = 0.0;
                fjac[k][3][0] = 1.0;
                fjac[k][4][0] = 0.0;
                fjac[k][0][1] = -(u[k][j][i][1] * u[k][j][i][3]) * tmp2;
                fjac[k][1][1] = u[k][j][i][3] * tmp1;
                fjac[k][2][1] = 0.0;
                fjac[k][3][1] = u[k][j][i][1] * tmp1;
                fjac[k][4][1] = 0.0;
                fjac[k][0][2] = -(u[k][j][i][2] * u[k][j][i][3]) * tmp2;
                fjac[k][1][2] = 0.0;
                fjac[k][2][2] = u[k][j][i][3] * tmp1;
                fjac[k][3][2] = u[k][j][i][2] * tmp1;
                fjac[k][4][2] = 0.0;
                fjac[k][0][3] = -(u[k][j][i][3] * u[k][j][i][3] * tmp2) + C2 * qs[k][j][i];
                fjac[k][1][3] = -C2 * u[k][j][i][1] * tmp1;
                fjac[k][2][3] = -C2 * u[k][j][i][2] * tmp1;
                fjac[k][3][3] = (2.0 - C2) * u[k][j][i][3] * tmp1;
                fjac[k][4][3] = C2;
                fjac[k][0][4] =
                    (C2 * 2.0 * square[k][j][i] - C1 * u[k][j][i][4]) * u[k][j][i][3] * tmp2;
                fjac[k][1][4] = -C2 * (u[k][j][i][1] * u[k][j][i][3]) * tmp2;
                fjac[k][2][4] = -C2 * (u[k][j][i][2] * u[k][j][i][3]) * tmp2;
                fjac[k][3][4] = C1 * (u[k][j][i][4] * tmp1)
                    - C2 * (qs[k][j][i] + u[k][j][i][3] * u[k][j][i][3] * tmp2);
                fjac[k][4][4] = C1 * u[k][j][i][3] * tmp1;
                njac[k][0][0] = 0.0;
                njac[k][1][0] = 0.0;
                njac[k][2][0] = 0.0;
                njac[k][3][0] = 0.0;
                njac[k][4][0] = 0.0;
                njac[k][0][1] = -C3C4 * tmp2 * u[k][j][i][1];
                njac[k][1][1] = C3C4 * tmp1;
                njac[k][2][1] = 0.0;
                njac[k][3][1] = 0.0;
                njac[k][4][1] = 0.0;
                njac[k][0][2] = -C3C4 * tmp2 * u[k][j][i][2];
                njac[k][1][2] = 0.0;
                njac[k][2][2] = C3C4 * tmp1;
                njac[k][3][2] = 0.0;
                njac[k][4][2] = 0.0;
                njac[k][0][3] = -CON43 * C3C4 * tmp2 * u[k][j][i][3];
                njac[k][1][3] = 0.0;
                njac[k][2][3] = 0.0;
                njac[k][3][3] = CON43 * C3 * C4 * tmp1;
                njac[k][4][3] = 0.0;
                njac[k][0][4] = -(C3C4 - C1345) * tmp3 * (u[k][j][i][1] * u[k][j][i][1])
                    - (C3C4 - C1345) * tmp3 * (u[k][j][i][2] * u[k][j][i][2])
                    - (CON43 * C3C4 - C1345) * tmp3 * (u[k][j][i][3] * u[k][j][i][3])
                    - C1345 * tmp2 * u[k][j][i][4];
                njac[k][1][4] = (C3C4 - C1345) * tmp2 * u[k][j][i][1];
                njac[k][2][4] = (C3C4 - C1345) * tmp2 * u[k][j][i][2];
                njac[k][3][4] = (CON43 * C3C4 - C1345) * tmp2 * u[k][j][i][3];
                njac[k][4][4] = (C1345) * tmp1;
            }
            /*
             * ---------------------------------------------------------------------
             * now jacobians set, so form left hand side in z direction
             * ---------------------------------------------------------------------
             */
            lhsinit(&mut lhs[..], ksize);
            for k in 1..ksize {
                let tmp1 = DT_DEFAULT * TZ1;
                let tmp2 = DT_DEFAULT * TZ2;
                lhs[k][AA][0][0] =
                    -tmp2 * fjac[k - 1][0][0] - tmp1 * njac[k - 1][0][0] - tmp1 * DZ1;
                lhs[k][AA][1][0] = -tmp2 * fjac[k - 1][1][0] - tmp1 * njac[k - 1][1][0];
                lhs[k][AA][2][0] = -tmp2 * fjac[k - 1][2][0] - tmp1 * njac[k - 1][2][0];
                lhs[k][AA][3][0] = -tmp2 * fjac[k - 1][3][0] - tmp1 * njac[k - 1][3][0];
                lhs[k][AA][4][0] = -tmp2 * fjac[k - 1][4][0] - tmp1 * njac[k - 1][4][0];
                lhs[k][AA][0][1] = -tmp2 * fjac[k - 1][0][1] - tmp1 * njac[k - 1][0][1];
                lhs[k][AA][1][1] =
                    -tmp2 * fjac[k - 1][1][1] - tmp1 * njac[k - 1][1][1] - tmp1 * DZ2;
                lhs[k][AA][2][1] = -tmp2 * fjac[k - 1][2][1] - tmp1 * njac[k - 1][2][1];
                lhs[k][AA][3][1] = -tmp2 * fjac[k - 1][3][1] - tmp1 * njac[k - 1][3][1];
                lhs[k][AA][4][1] = -tmp2 * fjac[k - 1][4][1] - tmp1 * njac[k - 1][4][1];
                lhs[k][AA][0][2] = -tmp2 * fjac[k - 1][0][2] - tmp1 * njac[k - 1][0][2];
                lhs[k][AA][1][2] = -tmp2 * fjac[k - 1][1][2] - tmp1 * njac[k - 1][1][2];
                lhs[k][AA][2][2] =
                    -tmp2 * fjac[k - 1][2][2] - tmp1 * njac[k - 1][2][2] - tmp1 * DZ3;
                lhs[k][AA][3][2] = -tmp2 * fjac[k - 1][3][2] - tmp1 * njac[k - 1][3][2];
                lhs[k][AA][4][2] = -tmp2 * fjac[k - 1][4][2] - tmp1 * njac[k - 1][4][2];
                lhs[k][AA][0][3] = -tmp2 * fjac[k - 1][0][3] - tmp1 * njac[k - 1][0][3];
                lhs[k][AA][1][3] = -tmp2 * fjac[k - 1][1][3] - tmp1 * njac[k - 1][1][3];
                lhs[k][AA][2][3] = -tmp2 * fjac[k - 1][2][3] - tmp1 * njac[k - 1][2][3];
                lhs[k][AA][3][3] =
                    -tmp2 * fjac[k - 1][3][3] - tmp1 * njac[k - 1][3][3] - tmp1 * DZ4;
                lhs[k][AA][4][3] = -tmp2 * fjac[k - 1][4][3] - tmp1 * njac[k - 1][4][3];
                lhs[k][AA][0][4] = -tmp2 * fjac[k - 1][0][4] - tmp1 * njac[k - 1][0][4];
                lhs[k][AA][1][4] = -tmp2 * fjac[k - 1][1][4] - tmp1 * njac[k - 1][1][4];
                lhs[k][AA][2][4] = -tmp2 * fjac[k - 1][2][4] - tmp1 * njac[k - 1][2][4];
                lhs[k][AA][3][4] = -tmp2 * fjac[k - 1][3][4] - tmp1 * njac[k - 1][3][4];
                lhs[k][AA][4][4] =
                    -tmp2 * fjac[k - 1][4][4] - tmp1 * njac[k - 1][4][4] - tmp1 * DZ5;
                lhs[k][BB][0][0] = 1.0 + tmp1 * 2.0 * njac[k][0][0] + tmp1 * 2.0 * DZ1;
                lhs[k][BB][1][0] = tmp1 * 2.0 * njac[k][1][0];
                lhs[k][BB][2][0] = tmp1 * 2.0 * njac[k][2][0];
                lhs[k][BB][3][0] = tmp1 * 2.0 * njac[k][3][0];
                lhs[k][BB][4][0] = tmp1 * 2.0 * njac[k][4][0];
                lhs[k][BB][0][1] = tmp1 * 2.0 * njac[k][0][1];
                lhs[k][BB][1][1] = 1.0 + tmp1 * 2.0 * njac[k][1][1] + tmp1 * 2.0 * DZ2;
                lhs[k][BB][2][1] = tmp1 * 2.0 * njac[k][2][1];
                lhs[k][BB][3][1] = tmp1 * 2.0 * njac[k][3][1];
                lhs[k][BB][4][1] = tmp1 * 2.0 * njac[k][4][1];
                lhs[k][BB][0][2] = tmp1 * 2.0 * njac[k][0][2];
                lhs[k][BB][1][2] = tmp1 * 2.0 * njac[k][1][2];
                lhs[k][BB][2][2] = 1.0 + tmp1 * 2.0 * njac[k][2][2] + tmp1 * 2.0 * DZ3;
                lhs[k][BB][3][2] = tmp1 * 2.0 * njac[k][3][2];
                lhs[k][BB][4][2] = tmp1 * 2.0 * njac[k][4][2];
                lhs[k][BB][0][3] = tmp1 * 2.0 * njac[k][0][3];
                lhs[k][BB][1][3] = tmp1 * 2.0 * njac[k][1][3];
                lhs[k][BB][2][3] = tmp1 * 2.0 * njac[k][2][3];
                lhs[k][BB][3][3] = 1.0 + tmp1 * 2.0 * njac[k][3][3] + tmp1 * 2.0 * DZ4;
                lhs[k][BB][4][3] = tmp1 * 2.0 * njac[k][4][3];
                lhs[k][BB][0][4] = tmp1 * 2.0 * njac[k][0][4];
                lhs[k][BB][1][4] = tmp1 * 2.0 * njac[k][1][4];
                lhs[k][BB][2][4] = tmp1 * 2.0 * njac[k][2][4];
                lhs[k][BB][3][4] = tmp1 * 2.0 * njac[k][3][4];
                lhs[k][BB][4][4] = 1.0 + tmp1 * 2.0 * njac[k][4][4] + tmp1 * 2.0 * DZ5;
                lhs[k][CC][0][0] = tmp2 * fjac[k + 1][0][0] - tmp1 * njac[k + 1][0][0] - tmp1 * DZ1;
                lhs[k][CC][1][0] = tmp2 * fjac[k + 1][1][0] - tmp1 * njac[k + 1][1][0];
                lhs[k][CC][2][0] = tmp2 * fjac[k + 1][2][0] - tmp1 * njac[k + 1][2][0];
                lhs[k][CC][3][0] = tmp2 * fjac[k + 1][3][0] - tmp1 * njac[k + 1][3][0];
                lhs[k][CC][4][0] = tmp2 * fjac[k + 1][4][0] - tmp1 * njac[k + 1][4][0];
                lhs[k][CC][0][1] = tmp2 * fjac[k + 1][0][1] - tmp1 * njac[k + 1][0][1];
                lhs[k][CC][1][1] = tmp2 * fjac[k + 1][1][1] - tmp1 * njac[k + 1][1][1] - tmp1 * DZ2;
                lhs[k][CC][2][1] = tmp2 * fjac[k + 1][2][1] - tmp1 * njac[k + 1][2][1];
                lhs[k][CC][3][1] = tmp2 * fjac[k + 1][3][1] - tmp1 * njac[k + 1][3][1];
                lhs[k][CC][4][1] = tmp2 * fjac[k + 1][4][1] - tmp1 * njac[k + 1][4][1];
                lhs[k][CC][0][2] = tmp2 * fjac[k + 1][0][2] - tmp1 * njac[k + 1][0][2];
                lhs[k][CC][1][2] = tmp2 * fjac[k + 1][1][2] - tmp1 * njac[k + 1][1][2];
                lhs[k][CC][2][2] = tmp2 * fjac[k + 1][2][2] - tmp1 * njac[k + 1][2][2] - tmp1 * DZ3;
                lhs[k][CC][3][2] = tmp2 * fjac[k + 1][3][2] - tmp1 * njac[k + 1][3][2];
                lhs[k][CC][4][2] = tmp2 * fjac[k + 1][4][2] - tmp1 * njac[k + 1][4][2];
                lhs[k][CC][0][3] = tmp2 * fjac[k + 1][0][3] - tmp1 * njac[k + 1][0][3];
                lhs[k][CC][1][3] = tmp2 * fjac[k + 1][1][3] - tmp1 * njac[k + 1][1][3];
                lhs[k][CC][2][3] = tmp2 * fjac[k + 1][2][3] - tmp1 * njac[k + 1][2][3];
                lhs[k][CC][3][3] = tmp2 * fjac[k + 1][3][3] - tmp1 * njac[k + 1][3][3] - tmp1 * DZ4;
                lhs[k][CC][4][3] = tmp2 * fjac[k + 1][4][3] - tmp1 * njac[k + 1][4][3];
                lhs[k][CC][0][4] = tmp2 * fjac[k + 1][0][4] - tmp1 * njac[k + 1][0][4];
                lhs[k][CC][1][4] = tmp2 * fjac[k + 1][1][4] - tmp1 * njac[k + 1][1][4];
                lhs[k][CC][2][4] = tmp2 * fjac[k + 1][2][4] - tmp1 * njac[k + 1][2][4];
                lhs[k][CC][3][4] = tmp2 * fjac[k + 1][3][4] - tmp1 * njac[k + 1][3][4];
                lhs[k][CC][4][4] = tmp2 * fjac[k + 1][4][4] - tmp1 * njac[k + 1][4][4] - tmp1 * DZ5;
            }
            /*
             * ---------------------------------------------------------------------
             * performs guaussian elimination on this cell.
             *
             * assumes that unpacking routines for non-first cells
             * preload c' and rhs' from previous cell.
             *
             * assumed send happens outside this routine, but that
             * c'(KMAX) and rhs'(KMAX) will be sent to next cell.
             * ---------------------------------------------------------------------
             * outer most do loops - sweeping in i direction
             * ---------------------------------------------------------------------
             * multiply c(i,j,0) by b_inverse and copy back to c
             * multiply rhs(0) by b_inverse(0) and copy to rhs
             * ---------------------------------------------------------------------
             */
            binvcrhs(&mut lhs[..], 0, BB, 0, CC, &mut rhs[0][j][i][..]);
            /*
             * ---------------------------------------------------------------------
             * begin inner most do loop
             * do all the elements of the cell unless last
             * ---------------------------------------------------------------------
             */
            for k in 1..ksize {
                /*
                 * -------------------------------------------------------------------
                 * subtract A*lhs_vector(k-1) from lhs_vector(k)
                 *
                 * rhs(k) = rhs(k) - A*rhs(k-1)
                 * -------------------------------------------------------------------
                 */
                matvec_sub(&mut lhs[k][AA][..], &mut rhs[..], k - 1, j, i, k, j, i);
                /*
                 * -------------------------------------------------------------------
                 * B(k) = B(k) - C(k-1)*A(k)
                 * matmul_sub(aa,i,j,k,c,cc,i,j,k-1,c,bb,i,j,k)
                 * --------------------------------------------------------------------
                 */
                matmul_sub(&mut lhs[..], k, AA, k - 1, CC, k, BB);
                /*
                 * -------------------------------------------------------------------
                 * multiply c(i,j,k) by b_inverse and copy back to c
                 * multiply rhs(i,j,1) by b_inverse(i,j,1) and copy to rhs
                 * -------------------------------------------------------------------
                 */
                binvcrhs(&mut lhs[..], k, BB, k, CC, &mut rhs[k][j][i][..]);
            }
            /*
             * ---------------------------------------------------------------------
             * now finish up special cases for last cell
             * ---------------------------------------------------------------------
             * rhs(ksize) = rhs(ksize) - A*rhs(ksize-1)
             * ---------------------------------------------------------------------
             */
            matvec_sub(
                &mut lhs[ksize][AA][..],
                &mut rhs[..],
                ksize - 1,
                j,
                i,
                ksize,
                j,
                i,
            );
            /*
             * ---------------------------------------------------------------------
             * B(ksize) = B(ksize) - C(ksize-1)*A(ksize)
             * matmul_sub(aa,i,j,ksize,c,
             * $ cc,i,j,ksize-1,c,bb,i,j,ksize)
             * ---------------------------------------------------------------------
             */
            matmul_sub(&mut lhs[..], ksize, AA, ksize - 1, CC, ksize, BB);
            /*
             * ---------------------------------------------------------------------
             * multiply rhs(ksize) by b_inverse(ksize) and copy to rhs
             * ---------------------------------------------------------------------
             */
            binvrhs(&mut lhs[ksize][BB][..], &mut rhs[ksize][j][i][..]);
            /*
             * ---------------------------------------------------------------------
             * back solve: if last cell, then generate U(ksize)=rhs(ksize)
             * else assume U(ksize) is loaded in un pack backsub_info
             * so just use it
             * after u(kstart) will be sent to next cell
             * ---------------------------------------------------------------------
             */
            for k in (0..ksize).rev() {
                for m in 0..BLOCK_SIZE as usize {
                    for n in 0..BLOCK_SIZE as usize {
                        rhs[k][j][i][m] = rhs[k][j][i][m] - lhs[k][CC][n][m] * rhs[k + 1][j][i][n];
                    }
                }
            }
        }
    }
    if TIMERS {
        timers.stop(T_ZSOLVE);
    }
}
