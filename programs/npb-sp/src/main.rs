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
    pub const DT_DEFAULT: f64 = 0.015;
    pub const NITER_DEFAULT: i32 = 1;
}

#[cfg(class = "S")]
mod params {
    pub const CLASS: char = 'S';
    pub const PROBLEM_SIZE: usize = 12;
    pub const DT_DEFAULT: f64 = 0.015;
    pub const NITER_DEFAULT: i32 = 100;
}

#[cfg(class = "W")]
mod params {
    pub const CLASS: char = 'W';
    pub const PROBLEM_SIZE: usize = 36;
    pub const DT_DEFAULT: f64 = 0.0015;
    pub const NITER_DEFAULT: i32 = 400;
}

#[cfg(class = "A")]
mod params {
    pub const CLASS: char = 'A';
    pub const PROBLEM_SIZE: usize = 64;
    pub const DT_DEFAULT: f64 = 0.0015;
    pub const NITER_DEFAULT: i32 = 400;
}

#[cfg(class = "B")]
mod params {
    pub const CLASS: char = 'B';
    pub const PROBLEM_SIZE: usize = 102;
    pub const DT_DEFAULT: f64 = 0.001;
    pub const NITER_DEFAULT: i32 = 400;
}

#[cfg(class = "C")]
mod params {
    pub const CLASS: char = 'C';
    pub const PROBLEM_SIZE: usize = 162;
    pub const DT_DEFAULT: f64 = 0.00067;
    pub const NITER_DEFAULT: i32 = 400;
}

#[cfg(class = "D")]
mod params {
    pub const CLASS: char = 'D';
    pub const PROBLEM_SIZE: usize = 408;
    pub const DT_DEFAULT: f64 = 0.0003;
    pub const NITER_DEFAULT: i32 = 500;
}

#[cfg(class = "E")]
mod params {
    pub const CLASS: char = 'E';
    pub const PROBLEM_SIZE: usize = 1020;
    pub const DT_DEFAULT: f64 = 0.0001;
    pub const NITER_DEFAULT: i32 = 500;
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
    pub const PROBLEM_SIZE: usize = 3;
    pub const DT_DEFAULT: f64 = 1.0;
    pub const NITER_DEFAULT: i32 = 1;
    compile_error!(
        "\n\n\
		Must set a class at compilation time by setting RUSTFLAGS\n\
		class options for SP are: {S, W, A, B, C, D, E}\n\
		For example:\n\
		RUSTFLAGS='--cfg class=\"A\" ' cargo build --release --bin sp\n\n\n\
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
pub const T_TXINVR: usize = 11;
pub const T_PINVR: usize = 12;
pub const T_NINVR: usize = 13;
pub const T_TZETAR: usize = 14;
pub const T_ADD: usize = 15;
pub const T_LAST: usize = 15;

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
pub const DXMAX: f64 = {
    let x;
    if DX3 > DX4 {
        x = DX3
    } else {
        x = DX4
    }
    x
};
pub const DYMAX: f64 = {
    let x;
    if DY2 > DY4 {
        x = DY2
    } else {
        x = DY4
    }
    x
};
pub const DZMAX: f64 = {
    let x;
    if DZ2 > DZ3 {
        x = DZ2
    } else {
        x = DZ3
    }
    x
};
pub const NX2: usize = GRID_POINTS[0] - 2;
pub const NY2: usize = GRID_POINTS[1] - 2;
pub const NZ2: usize = GRID_POINTS[2] - 2;
/* sp */
fn main() {
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
    let mut cuf: Vec<f64> = vec![0.0; PROBLEM_SIZE];
    let mut q: Vec<f64> = vec![0.0; PROBLEM_SIZE];
    let mut ue: Vec<[f64; PROBLEM_SIZE]> = vec![[0.0; PROBLEM_SIZE]; 5];
    let mut buf: Vec<[f64; PROBLEM_SIZE]> = vec![[0.0; PROBLEM_SIZE]; 5];
    let mut speed: Vec<[[f64; IMAXP + 1]; JMAXP + 1]> = vec![[[0.0; IMAXP + 1]; JMAXP + 1]; KMAX];
    let mut cv: Vec<f64> = vec![0.0; PROBLEM_SIZE];
    let mut rhon: Vec<f64> = vec![0.0; PROBLEM_SIZE];
    let mut rhos: Vec<f64> = vec![0.0; PROBLEM_SIZE];
    let mut rhoq: Vec<f64> = vec![0.0; PROBLEM_SIZE];
    let mut lhsp: Vec<[[f64; 5]; IMAXP + 1]> = vec![[[0.0; 5]; IMAXP + 1]; IMAXP + 1];
    let mut lhsm: Vec<[[f64; 5]; IMAXP + 1]> = vec![[[0.0; 5]; IMAXP + 1]; IMAXP + 1];
    let mut lhs: Vec<[[f64; 5]; IMAXP + 1]> = vec![[[0.0; 5]; IMAXP + 1]; IMAXP + 1];
    let mut ce: Vec<[f64; 5]> = vec![[0.0; 5]; 13];
    let mut verified: i8 = 0;

    println!(" Using compiled defaults");
    let mut timers = Timer::new();
    for i in 1..T_LAST + 1 {
        timers.clear(i);
    }

    println!("\n\n NAS Parallel Benchmarks 4.1 Serial Rust version - SP Benchmark\n");
    println!(
        " Size: {} {} {}",
        GRID_POINTS[0], GRID_POINTS[1], GRID_POINTS[2]
    );
    println!(" Iterations: {}    dt: {}", NITER_DEFAULT, DT_DEFAULT);
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
    /* */
    let bt: f64 = f64::sqrt(0.5);
    /* - - - - - - - - - - END SET CONSTANTS - - - - - - - - - - */

    exact_rhs(
        &mut forcing[..],
        &ce[..],
        &mut ue[..],
        &mut buf[..],
        &mut cuf[..],
        &mut q[..],
    );

    initialize(&mut u[..], &ce[..]);

    /*
     * ---------------------------------------------------------------------
     * do one time step to touch all code, and reinitialize
     * ---------------------------------------------------------------------
     */
    adi(
        &mut rhs[..],
        &mut lhs[..],
        &mut lhsp[..],
        &mut lhsm[..],
        &mut rho_i[..],
        &mut rhon[..],
        &mut rhoq[..],
        &mut rhos[..],
        &mut cv[..],
        &mut us[..],
        &mut vs[..],
        &mut ws[..],
        &mut square[..],
        &mut qs[..],
        &mut u[..],
        &forcing[..],
        &mut speed[..],
        bt,
        &mut timers,
    );
    initialize(&mut u[..], &ce[..]);
    for i in 1..T_LAST + 1 {
        timers.clear(i);
    }
    timers.start(1);
    for step in 1..NITER_DEFAULT + 1 {
        if step % 20 == 0 || step == 1 {
            println!(" Time step {}", step);
        }
        adi(
            &mut rhs[..],
            &mut lhs[..],
            &mut lhsp[..],
            &mut lhsm[..],
            &mut rho_i[..],
            &mut rhon[..],
            &mut rhoq[..],
            &mut rhos[..],
            &mut cv[..],
            &mut us[..],
            &mut vs[..],
            &mut ws[..],
            &mut square[..],
            &mut qs[..],
            &mut u[..],
            &forcing[..],
            &mut speed[..],
            bt,
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
        &mut speed[..],
        &mut timers,
    );
    let n3 = (GRID_POINTS[0] * GRID_POINTS[1] * GRID_POINTS[2]) as f64;
    let t = (GRID_POINTS[0] + GRID_POINTS[1] + GRID_POINTS[2]) as f64 / 3.0;
    let mops;
    if tmax != 0.0 {
        mops = (881.174 * n3 - 4683.91 * (t * t) + 11484.5 * t - 19272.4) * NITER_DEFAULT as f64
            / (tmax * 1000000.0);
    } else {
        mops = 0.0;
    }

    let info = PrintInfo {
        name: String::from("SP"),
        class: CLASS.to_string(),
        size: (GRID_POINTS[0], GRID_POINTS[1], GRID_POINTS[2]),
        num_iter: NITER_DEFAULT,
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
        t_names[T_TZETAR] = String::from("tzetar");
        t_names[T_NINVR] = String::from("ninvr");
        t_names[T_PINVR] = String::from("pinvr");
        t_names[T_TXINVR] = String::from("txinvr");
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
}

/*
 * ---------------------------------------------------------------------
 * addition of update to the vector u
 * ---------------------------------------------------------------------
 */
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
    lhs: &mut [[[f64; 5]; IMAXP + 1]],
    lhsp: &mut [[[f64; 5]; IMAXP + 1]],
    lhsm: &mut [[[f64; 5]; IMAXP + 1]],
    rho_i: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    rhon: &mut [f64],
    rhoq: &mut [f64],
    rhos: &mut [f64],
    cv: &mut [f64],
    us: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    vs: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    ws: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    square: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    qs: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    u: &mut [[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    forcing: &[[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    speed: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
    bt: f64,
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
        &mut speed[..],
        timers,
    );
    txinvr(
        &mut rhs[..],
        &rho_i[..],
        &us[..],
        &vs[..],
        &ws[..],
        &qs[..],
        &speed[..],
        bt,
        timers,
    );
    x_solve(
        &mut lhs[..],
        &mut rhs[..],
        &mut lhsp[..],
        &mut lhsm[..],
        &mut cv[..],
        &mut rhon[..],
        &speed[..],
        &us[..],
        &rho_i[..],
        bt,
        timers,
    );
    y_solve(
        &mut lhs[..],
        &mut rhs[..],
        &mut lhsp[..],
        &mut lhsm[..],
        &mut cv[..],
        &mut rhoq[..],
        &speed[..],
        &mut vs[..],
        &rho_i[..],
        bt,
        timers,
    );
    z_solve(
        &mut lhs[..],
        &mut rhs[..],
        &mut lhsp[..],
        &mut lhsm[..],
        &mut cv[..],
        &mut rhos[..],
        &speed[..],
        &mut ws[..],
        &mut us[..],
        &mut vs[..],
        &mut qs[..],
        &u[..],
        &rho_i[..],
        bt,
        timers,
    );
    add(&mut u[..], &rhs[..], timers);
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
    speed: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
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
                /*
                 * ---------------------------------------------------------------------
                 * (don't need speed and ainx until the lhs computation)
                 * ---------------------------------------------------------------------
                 */
                let aux = C1C2 * rho_inv * (u[k][j][i][4] - square[k][j][i]);
                speed[k][j][i] = f64::sqrt(aux);
            }
        }
    }
    /*
     * ---------------------------------------------------------------------
     * copy the exact forcing term to the right hand side;  because
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
    /*
     * ---------------------------------------------------------------------
     * compute xi-direction fluxes
     * ---------------------------------------------------------------------
     */
    if TIMERS {
        timers.start(T_RHSX);
    }
    for k in 1..NZ2 + 1 {
        for j in 1..NY2 + 1 {
            for i in 1..NX2 + 1 {
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
        for j in 1..NY2 + 1 {
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
        for j in 1..NY2 + 1 {
            for i in 3..NX2 + 1 - 2 {
                for m in 0..5 {
                    rhs[k][j][i][m] = rhs[k][j][i][m]
                        - DSSP
                            * (u[k][j][i - 2][m] - 4.0 * u[k][j][i - 1][m] + 6.0 * u[k][j][i][m]
                                - 4.0 * u[k][j][i + 1][m]
                                + u[k][j][i + 2][m]);
                }
            }
        }
        for j in 1..NY2 + 1 {
            let mut i = NX2 - 1;
            for m in 0..5 {
                rhs[k][j][i][m] = rhs[k][j][i][m]
                    - DSSP
                        * (u[k][j][i - 2][m] - 4.0 * u[k][j][i - 1][m] + 6.0 * u[k][j][i][m]
                            - 4.0 * u[k][j][i + 1][m]);
            }
            i = NX2;
            for m in 0..5 {
                rhs[k][j][i][m] = rhs[k][j][i][m]
                    - DSSP * (u[k][j][i - 2][m] - 4.0 * u[k][j][i - 1][m] + 5.0 * u[k][j][i][m]);
            }
        }
    }
    if TIMERS {
        timers.stop(T_RHSX);
    }
    /*
     * ---------------------------------------------------------------------
     * compute eta-direction fluxes
     * ---------------------------------------------------------------------
     */
    if TIMERS {
        timers.start(T_RHSY);
    }
    for k in 1..NZ2 + 1 {
        for j in 1..NY2 + 1 {
            for i in 1..NX2 + 1 {
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
        for i in 1..NX2 + 1 {
            for m in 0..5 {
                rhs[k][j][i][m] = rhs[k][j][i][m]
                    - DSSP * (5.0 * u[k][j][i][m] - 4.0 * u[k][j + 1][i][m] + u[k][j + 2][i][m]);
            }
        }
        j = 2;
        for i in 1..NX2 + 1 {
            for m in 0..5 {
                rhs[k][j][i][m] = rhs[k][j][i][m]
                    - DSSP
                        * (-4.0 * u[k][j - 1][i][m] + 6.0 * u[k][j][i][m]
                            - 4.0 * u[k][j + 1][i][m]
                            + u[k][j + 2][i][m]);
            }
        }
        for j in 3..NY2 - 1 {
            for i in 1..NX2 + 1 {
                for m in 0..5 {
                    rhs[k][j][i][m] = rhs[k][j][i][m]
                        - DSSP
                            * (u[k][j - 2][i][m] - 4.0 * u[k][j - 1][i][m] + 6.0 * u[k][j][i][m]
                                - 4.0 * u[k][j + 1][i][m]
                                + u[k][j + 2][i][m]);
                }
            }
        }
        j = NY2 - 1;
        for i in 1..NX2 + 1 {
            for m in 0..5 {
                rhs[k][j][i][m] = rhs[k][j][i][m]
                    - DSSP
                        * (u[k][j - 2][i][m] - 4.0 * u[k][j - 1][i][m] + 6.0 * u[k][j][i][m]
                            - 4.0 * u[k][j + 1][i][m]);
            }
        }
        j = NY2;
        for i in 1..NX2 + 1 {
            for m in 0..5 {
                rhs[k][j][i][m] = rhs[k][j][i][m]
                    - DSSP * (u[k][j - 2][i][m] - 4.0 * u[k][j - 1][i][m] + 5.0 * u[k][j][i][m]);
            }
        }
    }
    if TIMERS {
        timers.stop(T_RHSY);
    }
    /*
     * ---------------------------------------------------------------------
     * compute zeta-direction fluxes
     * ---------------------------------------------------------------------
     */
    if TIMERS {
        timers.start(T_RHSZ);
    }
    for k in 1..NZ2 + 1 {
        for j in 1..NY2 + 1 {
            for i in 1..NX2 + 1 {
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
    for j in 1..NY2 + 1 {
        for i in 1..NX2 + 1 {
            for m in 0..5 {
                rhs[k][j][i][m] = rhs[k][j][i][m]
                    - DSSP * (5.0 * u[k][j][i][m] - 4.0 * u[k + 1][j][i][m] + u[k + 2][j][i][m]);
            }
        }
    }
    k = 2;
    for j in 1..NY2 + 1 {
        for i in 1..NX2 + 1 {
            for m in 0..5 {
                rhs[k][j][i][m] = rhs[k][j][i][m]
                    - DSSP
                        * (-4.0 * u[k - 1][j][i][m] + 6.0 * u[k][j][i][m]
                            - 4.0 * u[k + 1][j][i][m]
                            + u[k + 2][j][i][m]);
            }
        }
    }
    for k in 3..NZ2 + 1 - 2 {
        for j in 1..NY2 + 1 {
            for i in 1..NX2 + 1 {
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
    k = NZ2 - 1;
    for j in 1..NY2 + 1 {
        for i in 1..NX2 + 1 {
            for m in 0..5 {
                rhs[k][j][i][m] = rhs[k][j][i][m]
                    - DSSP
                        * (u[k - 2][j][i][m] - 4.0 * u[k - 1][j][i][m] + 6.0 * u[k][j][i][m]
                            - 4.0 * u[k + 1][j][i][m]);
            }
        }
    }
    k = NZ2;
    for j in 1..NY2 + 1 {
        for i in 1..NX2 + 1 {
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

/*
 * ---------------------------------------------------------------------
 * compute the right hand side based on exact solution
 * ---------------------------------------------------------------------
 */
fn exact_rhs(
    forcing: &mut [[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    ce: &[[f64; 5]],
    ue: &mut [[f64; PROBLEM_SIZE]],
    buf: &mut [[f64; PROBLEM_SIZE]],
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
            for m in 0..5 {
                for i in 3..GRID_POINTS[0] + 1 - 4 {
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
            for m in 0..5 {
                for j in 3..GRID_POINTS[1] + 1 - 4 {
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
            for m in 0..5 {
                for k in 3..GRID_POINTS[2] + 1 - 4 {
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
                u[0] = 1.0;
                u[1] = 0.0;
                u[2] = 0.0;
                u[3] = 0.0;
                u[4] = 1.0;
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
    let mut xi: f64 = 0.0;
    let mut i: usize = 0;
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
    xi = 1.0;
    i = GRID_POINTS[0] - 1;
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
    eta = 0.0;
    let mut j = 0;
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
    eta = 1.0;
    j = GRID_POINTS[1] - 1;
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
    zeta = 0.0;
    let mut k = 0;
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
    zeta = 1.0;
    k = GRID_POINTS[2] - 1;
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

fn lhsinit(
    ni: usize,
    nj: usize,
    lhs: &mut [[[f64; 5]; IMAXP + 1]],
    lhsp: &mut [[[f64; 5]; IMAXP + 1]],
    lhsm: &mut [[[f64; 5]; IMAXP + 1]],
) {
    /*
     * ---------------------------------------------------------------------
     * zap the whole left hand side for starters
     * set all diagonal values to 1. This is overkill, but convenient
     * ---------------------------------------------------------------------
     */
    for j in 1..nj + 1 {
        for m in 0..5 {
            lhs[j][0][m] = 0.0;
            lhsp[j][0][m] = 0.0;
            lhsm[j][0][m] = 0.0;
            lhs[j][ni][m] = 0.0;
            lhsp[j][ni][m] = 0.0;
            lhsm[j][ni][m] = 0.0;
        }
        lhs[j][0][2] = 1.0;
        lhsp[j][0][2] = 1.0;
        lhsm[j][0][2] = 1.0;
        lhs[j][ni][2] = 1.0;
        lhsp[j][ni][2] = 1.0;
        lhsm[j][ni][2] = 1.0;
    }
}

fn lhsinitj(
    nj: usize,
    ni: usize,
    lhs: &mut [[[f64; 5]; IMAXP + 1]],
    lhsp: &mut [[[f64; 5]; IMAXP + 1]],
    lhsm: &mut [[[f64; 5]; IMAXP + 1]],
) {
    /*
     * ---------------------------------------------------------------------
     * zap the whole left hand side for starters
     * set all diagonal values to 1. This is overkill, but convenient
     * ---------------------------------------------------------------------
     */
    for i in 1..ni + 1 {
        for m in 0..5 {
            lhs[0][i][m] = 0.0;
            lhsp[0][i][m] = 0.0;
            lhsm[0][i][m] = 0.0;
            lhs[nj][i][m] = 0.0;
            lhsp[nj][i][m] = 0.0;
            lhsm[nj][i][m] = 0.0;
        }
        lhs[0][i][2] = 1.0;
        lhsp[0][i][2] = 1.0;
        lhsm[0][i][2] = 1.0;
        lhs[nj][i][2] = 1.0;
        lhsp[nj][i][2] = 1.0;
        lhsm[nj][i][2] = 1.0;
    }
}

/*
 * ---------------------------------------------------------------------
 * block-diagonal matrix-vector multiplication
 * ---------------------------------------------------------------------
 */
fn ninvr(rhs: &mut [[[[f64; 5]; IMAXP + 1]; JMAXP + 1]], bt: f64, timers: &mut Timer) {
    let (mut r1, mut r2, mut r3, mut r4, mut r5, mut t1, mut t2): (
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
    );
    if TIMERS {
        timers.start(T_NINVR);
    }
    for k in 1..NZ2 + 1 {
        for j in 1..NY2 + 1 {
            for i in 1..NX2 + 1 {
                r1 = rhs[k][j][i][0];
                r2 = rhs[k][j][i][1];
                r3 = rhs[k][j][i][2];
                r4 = rhs[k][j][i][3];
                r5 = rhs[k][j][i][4];
                t1 = bt * r3;
                t2 = 0.5 * (r4 + r5);
                rhs[k][j][i][0] = -r2;
                rhs[k][j][i][1] = r1;
                rhs[k][j][i][2] = bt * (r4 - r5);
                rhs[k][j][i][3] = -t1 + t2;
                rhs[k][j][i][4] = t1 + t2;
            }
        }
    }
    if TIMERS {
        timers.stop(T_NINVR);
    }
}

/*
 * ---------------------------------------------------------------------
 * block-diagonal matrix-vector multiplication
 * ---------------------------------------------------------------------
 */
fn pinvr(rhs: &mut [[[[f64; 5]; IMAXP + 1]; JMAXP + 1]], bt: f64, timers: &mut Timer) {
    let (mut r1, mut r2, mut r3, mut r4, mut r5, mut t1, mut t2): (
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
    );
    if TIMERS {
        timers.start(T_PINVR);
    }
    for k in 1..NZ2 + 1 {
        for j in 1..NY2 + 1 {
            for i in 1..NX2 + 1 {
                r1 = rhs[k][j][i][0];
                r2 = rhs[k][j][i][1];
                r3 = rhs[k][j][i][2];
                r4 = rhs[k][j][i][3];
                r5 = rhs[k][j][i][4];
                t1 = bt * r1;
                t2 = 0.5 * (r4 + r5);
                rhs[k][j][i][0] = bt * (r4 - r5);
                rhs[k][j][i][1] = -r3;
                rhs[k][j][i][2] = r2;
                rhs[k][j][i][3] = -t1 + t2;
                rhs[k][j][i][4] = t1 + t2;
            }
        }
    }
    if TIMERS {
        timers.stop(T_PINVR);
    }
}

fn rhs_norm(rms: &mut [f64], rhs: &[[[[f64; 5]; IMAXP + 1]; JMAXP + 1]]) {
    rms.iter_mut().for_each(|rms| *rms = 0.0);
    for k in 1..NZ2 + 1 {
        for j in 1..NY2 + 1 {
            for i in 1..NX2 + 1 {
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
 * block-diagonal matrix-vector multiplication
 * ---------------------------------------------------------------------
 */
fn txinvr(
    rhs: &mut [[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    rho_i: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    us: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    vs: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    ws: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    qs: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    speed: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    bt: f64,
    timers: &mut Timer,
) {
    let (mut t1, mut t2, mut t3, mut ac, mut ru1, mut uu, mut vv, mut ww): (
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
    );
    let (mut r1, mut r2, mut r3, mut r4, mut r5, mut ac2inv): (f64, f64, f64, f64, f64, f64);
    if TIMERS {
        timers.start(T_TXINVR);
    }
    for k in 1..NZ2 + 1 {
        for j in 1..NY2 + 1 {
            for i in 1..NX2 + 1 {
                ru1 = rho_i[k][j][i];
                uu = us[k][j][i];
                vv = vs[k][j][i];
                ww = ws[k][j][i];
                ac = speed[k][j][i];
                ac2inv = ac * ac;
                r1 = rhs[k][j][i][0];
                r2 = rhs[k][j][i][1];
                r3 = rhs[k][j][i][2];
                r4 = rhs[k][j][i][3];
                r5 = rhs[k][j][i][4];
                t1 = C2 / ac2inv * (qs[k][j][i] * r1 - uu * r2 - vv * r3 - ww * r4 + r5);
                t2 = bt * ru1 * (uu * r1 - r2);
                t3 = (bt * ru1 * ac) * t1;
                rhs[k][j][i][0] = r1 - t1;
                rhs[k][j][i][1] = -ru1 * (ww * r1 - r4);
                rhs[k][j][i][2] = ru1 * (vv * r1 - r3);
                rhs[k][j][i][3] = -t2 + t3;
                rhs[k][j][i][4] = t2 + t3;
            }
        }
    }
    if TIMERS {
        timers.stop(T_TXINVR);
    }
}

/*
 * ---------------------------------------------------------------------
 * block-diagonal matrix-vector multiplication
 * ---------------------------------------------------------------------
 */
fn tzetar(
    rhs: &mut [[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    us: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    vs: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    ws: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    speed: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    qs: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    u: &[[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    bt: f64,
    timers: &mut Timer,
) {
    let (mut r1, mut r2, mut r4, mut r5, mut t1, mut t2, mut t3): (
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
    );
    let (mut ac, mut xvel, mut yvel, mut zvel, mut btuz, mut ac2u, mut uzik1): (
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
    );
    if TIMERS {
        timers.start(T_TZETAR);
    }
    for k in 1..NZ2 + 1 {
        for j in 1..NY2 + 1 {
            for i in 1..NX2 + 1 {
                xvel = us[k][j][i];
                yvel = vs[k][j][i];
                zvel = ws[k][j][i];
                ac = speed[k][j][i];
                ac2u = ac * ac;
                r1 = rhs[k][j][i][0];
                r2 = rhs[k][j][i][1];
                r4 = rhs[k][j][i][3];
                r5 = rhs[k][j][i][4];
                uzik1 = u[k][j][i][0];
                btuz = bt * uzik1;
                t1 = btuz / ac * (r4 + r5);
                t2 = rhs[k][j][i][2] + t1;
                t3 = btuz * (r4 - r5);
                rhs[k][j][i][0] = t2;
                rhs[k][j][i][1] = -uzik1 * r2 + xvel * t2;
                rhs[k][j][i][2] = uzik1 * r1 + yvel * t2;
                rhs[k][j][i][3] = zvel * t2 + t3;
                rhs[k][j][i][4] = uzik1 * (-xvel * r2 + yvel * r1)
                    + qs[k][j][i] * t2
                    + C2IV * ac2u * t1
                    + zvel * t3;
            }
        }
    }
    if TIMERS {
        timers.stop(T_TZETAR);
    }
}

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
    speed: &mut [[[f64; IMAXP + 1]; JMAXP + 1]],
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
        &mut speed[..],
        timers,
    );
    rhs_norm(&mut xcr[..], &rhs[..]);
    xcr.iter_mut().for_each(|xcr| *xcr /= DT_DEFAULT);
    *verified = 1;
    /*
     * ---------------------------------------------------------------------
     * reference data for 12X12X12 grids after 100 time steps, with DT = 1.50d-02
     * ---------------------------------------------------------------------
     */
    if CLASS == 'S' {
        dtref = 1.5e-2;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 2.7470315451339479e-02;
        xcrref[1] = 1.0360746705285417e-02;
        xcrref[2] = 1.6235745065095532e-02;
        xcrref[3] = 1.5840557224455615e-02;
        xcrref[4] = 3.4849040609362460e-02;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error
         * ---------------------------------------------------------------------
         */
        xceref[0] = 2.7289258557377227e-05;
        xceref[1] = 1.0364446640837285e-05;
        xceref[2] = 1.6154798287166471e-05;
        xceref[3] = 1.5750704994480102e-05;
        xceref[4] = 3.4177666183390531e-05;
    /*
     * ---------------------------------------------------------------------
     * reference data for 36X36X36 grids after 400 time steps, with DT = 1.5d-03
     * ---------------------------------------------------------------------
     */
    } else if CLASS == 'Z' {
        dtref = 1.5e-2;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 8.8451710971449e3;
        xcrref[1] = 6.6404060453931e2;
        xcrref[2] = 2.1603141015600e3;
        xcrref[3] = 1.8248565085675e3;
        xcrref[4] = 1.7081321457454e4;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error
         * ---------------------------------------------------------------------
         */
        xceref[0] = 2.5633162320473e2;
        xceref[1] = 1.6016029939348e1;
        xceref[2] = 6.3757992199873e1;
        xceref[3] = 5.6095772950892e1;
        xceref[4] = 5.8097874515389e2;
    } else if CLASS == 'W' {
        dtref = 1.5e-3;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 0.1893253733584e-02;
        xcrref[1] = 0.1717075447775e-03;
        xcrref[2] = 0.2778153350936e-03;
        xcrref[3] = 0.2887475409984e-03;
        xcrref[4] = 0.3143611161242e-02;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error
         * ---------------------------------------------------------------------
         */
        xceref[0] = 0.7542088599534e-04;
        xceref[1] = 0.6512852253086e-05;
        xceref[2] = 0.1049092285688e-04;
        xceref[3] = 0.1128838671535e-04;
        xceref[4] = 0.1212845639773e-03;
    /*
     * ---------------------------------------------------------------------
     * reference data for 64X64X64 grids after 400 time steps, with DT = 1.5d-03
     * ---------------------------------------------------------------------
     */
    } else if CLASS == 'A' {
        dtref = 1.5e-3;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual.
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 2.4799822399300195;
        xcrref[1] = 1.1276337964368832;
        xcrref[2] = 1.5028977888770491;
        xcrref[3] = 1.4217816211695179;
        xcrref[4] = 2.1292113035138280;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error.
         * ---------------------------------------------------------------------
         */
        xceref[0] = 1.0900140297820550e-04;
        xceref[1] = 3.7343951769282091e-05;
        xceref[2] = 5.0092785406541633e-05;
        xceref[3] = 4.7671093939528255e-05;
        xceref[4] = 1.3621613399213001e-04;
    /*
     * ---------------------------------------------------------------------
     * reference data for 102X102X102 grids after 400 time steps,
     * with DT = 1.0d-03
     * ---------------------------------------------------------------------
     */
    } else if CLASS == 'B' {
        dtref = 1.0e-3;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 0.6903293579998e+02;
        xcrref[1] = 0.3095134488084e+02;
        xcrref[2] = 0.4103336647017e+02;
        xcrref[3] = 0.3864769009604e+02;
        xcrref[4] = 0.5643482272596e+02;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error
         * ---------------------------------------------------------------------
         */
        xceref[0] = 0.9810006190188e-02;
        xceref[1] = 0.1022827905670e-02;
        xceref[2] = 0.1720597911692e-02;
        xceref[3] = 0.1694479428231e-02;
        xceref[4] = 0.1847456263981e-01;
    /*
     * ---------------------------------------------------------------------
     * reference data for 162X162X162 grids after 400 time steps,
     * with DT = 0.67d-03
     * ---------------------------------------------------------------------
     */
    } else if CLASS == 'C' {
        dtref = 0.67e-3;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 0.5881691581829e+03;
        xcrref[1] = 0.2454417603569e+03;
        xcrref[2] = 0.3293829191851e+03;
        xcrref[3] = 0.3081924971891e+03;
        xcrref[4] = 0.4597223799176e+03;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error
         * ---------------------------------------------------------------------
         */
        xceref[0] = 0.2598120500183e+00;
        xceref[1] = 0.2590888922315e-01;
        xceref[2] = 0.5132886416320e-01;
        xceref[3] = 0.4806073419454e-01;
        xceref[4] = 0.5483377491301e+00;
    /*
     * ---------------------------------------------------------------------
     * reference data for 408X408X408 grids after 500 time steps,
     * with DT = 0.3d-03
     * ---------------------------------------------------------------------
     */
    } else if CLASS == 'D' {
        dtref = 0.30e-3;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 0.1044696216887e+05;
        xcrref[1] = 0.3204427762578e+04;
        xcrref[2] = 0.4648680733032e+04;
        xcrref[3] = 0.4238923283697e+04;
        xcrref[4] = 0.7588412036136e+04;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error
         * ---------------------------------------------------------------------
         */
        xceref[0] = 0.5089471423669e+01;
        xceref[1] = 0.5323514855894e+00;
        xceref[2] = 0.1187051008971e+01;
        xceref[3] = 0.1083734951938e+01;
        xceref[4] = 0.1164108338568e+02;
    /*
     * ---------------------------------------------------------------------
     * reference data for 1020X1020X1020 grids after 500 time steps,
     * with DT = 0.1d-03
     * ---------------------------------------------------------------------
     */
    } else if CLASS == 'E' {
        dtref = 0.10e-3;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of residual
         * ---------------------------------------------------------------------
         */
        xcrref[0] = 0.6255387422609e+05;
        xcrref[1] = 0.1495317020012e+05;
        xcrref[2] = 0.2347595750586e+05;
        xcrref[3] = 0.2091099783534e+05;
        xcrref[4] = 0.4770412841218e+05;
        /*
         * ---------------------------------------------------------------------
         * reference values of RMS-norms of solution error
         * ---------------------------------------------------------------------
         */
        xceref[0] = 0.6742735164909e+02;
        xceref[1] = 0.5390656036938e+01;
        xceref[2] = 0.1680647196477e+02;
        xceref[3] = 0.1536963126457e+02;
        xceref[4] = 0.1575330146156e+03;
    } else {
        *verified = 0;
    }
    /*
     * ---------------------------------------------------------------------
     * verification test for residuals if gridsize is one of
     * the defined grid sizes above (class .ne. 'U')
     * ---------------------------------------------------------------------
     * compute the difference of solution values and the known reference values
     * ---------------------------------------------------------------------
     */
    for m in 0..5 {
        xcrdif[m] = ((xcr[m] - xcrref[m]) / xcrref[m]).abs();
        xcedif[m] = ((xce[m] - xceref[m]) / xceref[m]).abs();
    }
    /*
     * ---------------------------------------------------------------------
     * output the comparison of computed results to known cases
     * ---------------------------------------------------------------------
     */
    if CLASS != 'U' {
        println!(" Verification being performed for class_npb {}", CLASS);
        println!(" accuracy setting for epsilon = {:.1e}", EPSILON);
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
            println!(" DT does not match the reference value of {}", dtref);
        }
        println!(" Comparison of RMS-norms of residual");
        for m in 0..5 {
            if xcrdif[m] <= EPSILON {
                println!(
                    "          {:>2} {:>20.13e}{:>20.13e}{:>20.13e}",
                    m + 1,
                    xcr[m],
                    xcrref[m],
                    xcrdif[m]
                );
            } else {
                *verified = 0;
                println!(
                    " FAILURE: {:>2} {:>20.13e}{:>20.13e}{:>20.13e}",
                    m + 1,
                    xcr[m],
                    xcrref[m],
                    xcrdif[m]
                );
            }
        }
        println!(" Comparison of RMS-norms of solution error");
        for m in 0..5 {
            if xcedif[m] <= EPSILON {
                println!(
                    "          {:>2} {:>20.13e}{:>20.13e}{:>20.13e}",
                    m + 1,
                    xce[m],
                    xceref[m],
                    xcedif[m]
                );
            } else {
                *verified = 0;
                println!(
                    " FAILURE: {:>2} {:>20.13e}{:>20.13e}{:>20.13e}",
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

/*
 * ---------------------------------------------------------------------
 * this function performs the solution of the approximate factorization
 * step in the x-direction for all five matrix components
 * simultaneously. the thomas algorithm is employed to solve the
 * systems for the x-lines. boundary conditions are non-periodic
 * ---------------------------------------------------------------------
 */
fn x_solve(
    lhs: &mut [[[f64; 5]; IMAXP + 1]],
    rhs: &mut [[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    lhsp: &mut [[[f64; 5]; IMAXP + 1]],
    lhsm: &mut [[[f64; 5]; IMAXP + 1]],
    cv: &mut [f64],
    rhon: &mut [f64],
    speed: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    us: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    rho_i: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    bt: f64,
    timers: &mut Timer,
) {
    let (mut ru1, mut fac1, mut fac2): (f64, f64, f64);
    if TIMERS {
        timers.start(T_XSOLVE);
    }
    for k in 1..NZ2 + 1 {
        lhsinit(NX2 + 1, NY2, &mut lhs[..], &mut lhsp[..], &mut lhsm[..]);
        /*
         * ---------------------------------------------------------------------
         * computes the left hand side for the three x-factors
         * ---------------------------------------------------------------------
         * first fill the lhs for the u-eigenvalue
         * ---------------------------------------------------------------------
         */
        for j in 1..NY2 + 1 {
            for i in 0..GRID_POINTS[0] {
                ru1 = C3C4 * rho_i[k][j][i];
                cv[i] = us[k][j][i];
                rhon[i] = f64::max(
                    f64::max(DX2 + CON43 * ru1, DX5 + C1C5 * ru1),
                    f64::max(DXMAX + ru1, DX1),
                );
            }
            for i in 1..NX2 + 1 {
                lhs[j][i][0] = 0.0;
                lhs[j][i][1] = -DTTX2 * cv[i - 1] - DTTX1 * rhon[i - 1];
                lhs[j][i][2] = 1.0 + C2DTTX1 * rhon[i];
                lhs[j][i][3] = DTTX2 * cv[i + 1] - DTTX1 * rhon[i + 1];
                lhs[j][i][4] = 0.0;
            }
        }
        /*
         * ---------------------------------------------------------------------
         * add fourth order dissipation
         * ---------------------------------------------------------------------
         */
        for j in 1..NY2 + 1 {
            let i: usize = 1;
            lhs[j][i][2] = lhs[j][i][2] + COMZ5;
            lhs[j][i][3] = lhs[j][i][3] - COMZ4;
            lhs[j][i][4] = lhs[j][i][4] + COMZ1;
            lhs[j][i + 1][1] = lhs[j][i + 1][1] - COMZ4;
            lhs[j][i + 1][2] = lhs[j][i + 1][2] + COMZ6;
            lhs[j][i + 1][3] = lhs[j][i + 1][3] - COMZ4;
            lhs[j][i + 1][4] = lhs[j][i + 1][4] + COMZ1;
        }
        for j in 1..NY2 + 1 {
            for i in 3..GRID_POINTS[0] - 3 {
                lhs[j][i][0] = lhs[j][i][0] + COMZ1;
                lhs[j][i][1] = lhs[j][i][1] - COMZ4;
                lhs[j][i][2] = lhs[j][i][2] + COMZ6;
                lhs[j][i][3] = lhs[j][i][3] - COMZ4;
                lhs[j][i][4] = lhs[j][i][4] + COMZ1;
            }
        }
        for j in 1..NY2 + 1 {
            let i: usize = GRID_POINTS[0] - 3;
            lhs[j][i][0] = lhs[j][i][0] + COMZ1;
            lhs[j][i][1] = lhs[j][i][1] - COMZ4;
            lhs[j][i][2] = lhs[j][i][2] + COMZ6;
            lhs[j][i][3] = lhs[j][i][3] - COMZ4;
            lhs[j][i + 1][0] = lhs[j][i + 1][0] + COMZ1;
            lhs[j][i + 1][1] = lhs[j][i + 1][1] - COMZ4;
            lhs[j][i + 1][2] = lhs[j][i + 1][2] + COMZ5;
        }
        /*
         * ---------------------------------------------------------------------
         * subsequently, fill the other factors (u+c), (u-c) by adding to
         * the first
         * ---------------------------------------------------------------------
         */
        for j in 1..NY2 + 1 {
            for i in 1..NX2 + 1 {
                lhsp[j][i][0] = lhs[j][i][0];
                lhsp[j][i][1] = lhs[j][i][1] - DTTX2 * speed[k][j][i - 1];
                lhsp[j][i][2] = lhs[j][i][2];
                lhsp[j][i][3] = lhs[j][i][3] + DTTX2 * speed[k][j][i + 1];
                lhsp[j][i][4] = lhs[j][i][4];
                lhsm[j][i][0] = lhs[j][i][0];
                lhsm[j][i][1] = lhs[j][i][1] + DTTX2 * speed[k][j][i - 1];
                lhsm[j][i][2] = lhs[j][i][2];
                lhsm[j][i][3] = lhs[j][i][3] - DTTX2 * speed[k][j][i + 1];
                lhsm[j][i][4] = lhs[j][i][4];
            }
        }
        /*
         * ---------------------------------------------------------------------
         * FORWARD ELIMINATION
         * ---------------------------------------------------------------------
         * perform the thomas algorithm; first, FORWARD ELIMINATION
         * ---------------------------------------------------------------------
         */
        for j in 1..NY2 + 1 {
            for i in 0..GRID_POINTS[0] - 2 {
                let i1 = i + 1;
                let i2 = i + 2;
                fac1 = 1.0 / lhs[j][i][2];
                lhs[j][i][3] = fac1 * lhs[j][i][3];
                lhs[j][i][4] = fac1 * lhs[j][i][4];
                for m in 0..3 {
                    rhs[k][j][i][m] = fac1 * rhs[k][j][i][m];
                }
                lhs[j][i1][2] = lhs[j][i1][2] - lhs[j][i1][1] * lhs[j][i][3];
                lhs[j][i1][3] = lhs[j][i1][3] - lhs[j][i1][1] * lhs[j][i][4];
                for m in 0..3 {
                    rhs[k][j][i1][m] = rhs[k][j][i1][m] - lhs[j][i1][1] * rhs[k][j][i][m];
                }
                lhs[j][i2][1] = lhs[j][i2][1] - lhs[j][i2][0] * lhs[j][i][3];
                lhs[j][i2][2] = lhs[j][i2][2] - lhs[j][i2][0] * lhs[j][i][4];
                for m in 0..3 {
                    rhs[k][j][i2][m] = rhs[k][j][i2][m] - lhs[j][i2][0] * rhs[k][j][i][m];
                }
            }
        }
        /*
         * ---------------------------------------------------------------------
         * the last two rows in this grid block are a bit different,
         * since they do not have two more rows available for the
         * elimination of off-diagonal entries
         * ---------------------------------------------------------------------
         */
        for j in 1..NY2 + 1 {
            let i = GRID_POINTS[0] - 2;
            let i1 = GRID_POINTS[0] - 1;
            fac1 = 1.0 / lhs[j][i][2];
            lhs[j][i][3] = fac1 * lhs[j][i][3];
            lhs[j][i][4] = fac1 * lhs[j][i][4];
            for m in 0..3 {
                rhs[k][j][i][m] = fac1 * rhs[k][j][i][m];
            }
            lhs[j][i1][2] = lhs[j][i1][2] - lhs[j][i1][1] * lhs[j][i][3];
            lhs[j][i1][3] = lhs[j][i1][3] - lhs[j][i1][1] * lhs[j][i][4];
            for m in 0..3 {
                rhs[k][j][i1][m] = rhs[k][j][i1][m] - lhs[j][i1][1] * rhs[k][j][i][m];
            }
            /*
             * ---------------------------------------------------------------------
             * scale the last row immediately
             * ---------------------------------------------------------------------
             */
            fac2 = 1.0 / lhs[j][i1][2];
            for m in 0..3 {
                rhs[k][j][i1][m] = fac2 * rhs[k][j][i1][m];
            }
        }
        /*
         * ---------------------------------------------------------------------
         * do the u+c and the u-c factors
         * ---------------------------------------------------------------------
         */
        for j in 1..NY2 + 1 {
            for i in 0..GRID_POINTS[0] - 2 {
                let i1 = i + 1;
                let i2 = i + 2;
                let mut m = 3;
                fac1 = 1.0 / lhsp[j][i][2];
                lhsp[j][i][3] = fac1 * lhsp[j][i][3];
                lhsp[j][i][4] = fac1 * lhsp[j][i][4];
                rhs[k][j][i][m] = fac1 * rhs[k][j][i][m];
                lhsp[j][i1][2] = lhsp[j][i1][2] - lhsp[j][i1][1] * lhsp[j][i][3];
                lhsp[j][i1][3] = lhsp[j][i1][3] - lhsp[j][i1][1] * lhsp[j][i][4];
                rhs[k][j][i1][m] = rhs[k][j][i1][m] - lhsp[j][i1][1] * rhs[k][j][i][m];
                lhsp[j][i2][1] = lhsp[j][i2][1] - lhsp[j][i2][0] * lhsp[j][i][3];
                lhsp[j][i2][2] = lhsp[j][i2][2] - lhsp[j][i2][0] * lhsp[j][i][4];
                rhs[k][j][i2][m] = rhs[k][j][i2][m] - lhsp[j][i2][0] * rhs[k][j][i][m];
                m = 4;
                fac1 = 1.0 / lhsm[j][i][2];
                lhsm[j][i][3] = fac1 * lhsm[j][i][3];
                lhsm[j][i][4] = fac1 * lhsm[j][i][4];
                rhs[k][j][i][m] = fac1 * rhs[k][j][i][m];
                lhsm[j][i1][2] = lhsm[j][i1][2] - lhsm[j][i1][1] * lhsm[j][i][3];
                lhsm[j][i1][3] = lhsm[j][i1][3] - lhsm[j][i1][1] * lhsm[j][i][4];
                rhs[k][j][i1][m] = rhs[k][j][i1][m] - lhsm[j][i1][1] * rhs[k][j][i][m];
                lhsm[j][i2][1] = lhsm[j][i2][1] - lhsm[j][i2][0] * lhsm[j][i][3];
                lhsm[j][i2][2] = lhsm[j][i2][2] - lhsm[j][i2][0] * lhsm[j][i][4];
                rhs[k][j][i2][m] = rhs[k][j][i2][m] - lhsm[j][i2][0] * rhs[k][j][i][m];
            }
        }
        /*
         * ---------------------------------------------------------------------
         * and again the last two rows separately
         * ---------------------------------------------------------------------
         */
        for j in 1..NY2 + 1 {
            let i = GRID_POINTS[0] - 2;
            let i1 = GRID_POINTS[0] - 1;
            let mut m = 3;
            fac1 = 1.0 / lhsp[j][i][2];
            lhsp[j][i][3] = fac1 * lhsp[j][i][3];
            lhsp[j][i][4] = fac1 * lhsp[j][i][4];
            rhs[k][j][i][m] = fac1 * rhs[k][j][i][m];
            lhsp[j][i1][2] = lhsp[j][i1][2] - lhsp[j][i1][1] * lhsp[j][i][3];
            lhsp[j][i1][3] = lhsp[j][i1][3] - lhsp[j][i1][1] * lhsp[j][i][4];
            rhs[k][j][i1][m] = rhs[k][j][i1][m] - lhsp[j][i1][1] * rhs[k][j][i][m];
            m = 4;
            fac1 = 1.0 / lhsm[j][i][2];
            lhsm[j][i][3] = fac1 * lhsm[j][i][3];
            lhsm[j][i][4] = fac1 * lhsm[j][i][4];
            rhs[k][j][i][m] = fac1 * rhs[k][j][i][m];
            lhsm[j][i1][2] = lhsm[j][i1][2] - lhsm[j][i1][1] * lhsm[j][i][3];
            lhsm[j][i1][3] = lhsm[j][i1][3] - lhsm[j][i1][1] * lhsm[j][i][4];
            rhs[k][j][i1][m] = rhs[k][j][i1][m] - lhsm[j][i1][1] * rhs[k][j][i][m];
            /*
             * ---------------------------------------------------------------------
             * scale the last row immediately
             * ---------------------------------------------------------------------
             */
            rhs[k][j][i1][3] = rhs[k][j][i1][3] / lhsp[j][i1][2];
            rhs[k][j][i1][4] = rhs[k][j][i1][4] / lhsm[j][i1][2];
        }
        /*
         * ---------------------------------------------------------------------
         * BACKSUBSTITUTION
         * ---------------------------------------------------------------------
         */
        for j in 1..NY2 + 1 {
            let i = GRID_POINTS[0] - 2;
            let i1 = GRID_POINTS[0] - 1;
            for m in 0..3 {
                rhs[k][j][i][m] = rhs[k][j][i][m] - lhs[j][i][3] * rhs[k][j][i1][m];
            }
            rhs[k][j][i][3] = rhs[k][j][i][3] - lhsp[j][i][3] * rhs[k][j][i1][3];
            rhs[k][j][i][4] = rhs[k][j][i][4] - lhsm[j][i][3] * rhs[k][j][i1][4];
        }
        /*
         * ---------------------------------------------------------------------
         * the first three factors
         * ---------------------------------------------------------------------
         */
        for j in 1..NY2 + 1 {
            for i in (0..GRID_POINTS[0] - 2).rev() {
                let i1 = i + 1;
                let i2 = i + 2;
                for m in 0..3 {
                    rhs[k][j][i][m] = rhs[k][j][i][m]
                        - lhs[j][i][3] * rhs[k][j][i1][m]
                        - lhs[j][i][4] * rhs[k][j][i2][m];
                }
                /*
                 * ---------------------------------------------------------------------
                 * and the remaining two
                 * ---------------------------------------------------------------------
                 */
                rhs[k][j][i][3] = rhs[k][j][i][3]
                    - lhsp[j][i][3] * rhs[k][j][i1][3]
                    - lhsp[j][i][4] * rhs[k][j][i2][3];
                rhs[k][j][i][4] = rhs[k][j][i][4]
                    - lhsm[j][i][3] * rhs[k][j][i1][4]
                    - lhsm[j][i][4] * rhs[k][j][i2][4];
            }
        }
    }
    if TIMERS {
        timers.stop(T_XSOLVE);
    }
    /*
     * ---------------------------------------------------------------------
     * do the block-diagonal inversion
     * ---------------------------------------------------------------------
     */
    ninvr(&mut rhs[..], bt, timers);
}

/*
 * ---------------------------------------------------------------------
 * this function performs the solution of the approximate factorization
 * step in the y-direction for all five matrix components
 * simultaneously. the thomas algorithm is employed to solve the
 * systems for the y-lines. boundary conditions are non-periodic
 * ---------------------------------------------------------------------
 */
fn y_solve(
    lhs: &mut [[[f64; 5]; IMAXP + 1]],
    rhs: &mut [[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    lhsp: &mut [[[f64; 5]; IMAXP + 1]],
    lhsm: &mut [[[f64; 5]; IMAXP + 1]],
    cv: &mut [f64],
    rhoq: &mut [f64],
    speed: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    vs: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    rho_i: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    bt: f64,
    timers: &mut Timer,
) {
    let (mut ru1, mut fac1, mut fac2): (f64, f64, f64);
    if TIMERS {
        timers.start(T_YSOLVE);
    }
    for k in 1..GRID_POINTS[2] - 1 {
        lhsinitj(NY2 + 1, NX2, &mut lhs[..], &mut lhsp[..], &mut lhsm[..]);
        /*
         * ---------------------------------------------------------------------
         * computes the left hand side for the three y-factors
         * ---------------------------------------------------------------------
         * first fill the lhs for the u-eigenvalue
         * ---------------------------------------------------------------------
         */
        for i in 1..GRID_POINTS[0] - 1 {
            for j in 0..GRID_POINTS[1] {
                ru1 = C3C4 * rho_i[k][j][i];
                cv[j] = vs[k][j][i];
                rhoq[j] = f64::max(
                    f64::max(DY3 + CON43 * ru1, DY5 + C1C5 * ru1),
                    f64::max(DYMAX + ru1, DY1),
                );
            }
            for j in 1..GRID_POINTS[1] - 1 {
                lhs[j][i][0] = 0.0;
                lhs[j][i][1] = -DTTY2 * cv[j - 1] - DTTY1 * rhoq[j - 1];
                lhs[j][i][2] = 1.0 + C2DTTY1 * rhoq[j];
                lhs[j][i][3] = DTTY2 * cv[j + 1] - DTTY1 * rhoq[j + 1];
                lhs[j][i][4] = 0.0;
            }
        }
        /*
         * ---------------------------------------------------------------------
         * add fourth order dissipation
         * ---------------------------------------------------------------------
         */
        for i in 1..GRID_POINTS[0] - 1 {
            let j = 1;
            lhs[j][i][2] = lhs[j][i][2] + COMZ5;
            lhs[j][i][3] = lhs[j][i][3] - COMZ4;
            lhs[j][i][4] = lhs[j][i][4] + COMZ1;
            lhs[j + 1][i][1] = lhs[j + 1][i][1] - COMZ4;
            lhs[j + 1][i][2] = lhs[j + 1][i][2] + COMZ6;
            lhs[j + 1][i][3] = lhs[j + 1][i][3] - COMZ4;
            lhs[j + 1][i][4] = lhs[j + 1][i][4] + COMZ1;
        }
        for j in 3..GRID_POINTS[1] - 3 {
            for i in 1..GRID_POINTS[0] - 1 {
                lhs[j][i][0] = lhs[j][i][0] + COMZ1;
                lhs[j][i][1] = lhs[j][i][1] - COMZ4;
                lhs[j][i][2] = lhs[j][i][2] + COMZ6;
                lhs[j][i][3] = lhs[j][i][3] - COMZ4;
                lhs[j][i][4] = lhs[j][i][4] + COMZ1;
            }
        }
        for i in 1..GRID_POINTS[0] - 1 {
            let j = GRID_POINTS[1] - 3;
            lhs[j][i][0] = lhs[j][i][0] + COMZ1;
            lhs[j][i][1] = lhs[j][i][1] - COMZ4;
            lhs[j][i][2] = lhs[j][i][2] + COMZ6;
            lhs[j][i][3] = lhs[j][i][3] - COMZ4;
            lhs[j + 1][i][0] = lhs[j + 1][i][0] + COMZ1;
            lhs[j + 1][i][1] = lhs[j + 1][i][1] - COMZ4;
            lhs[j + 1][i][2] = lhs[j + 1][i][2] + COMZ5;
        }
        /*
         * ---------------------------------------------------------------------
         * subsequently, do the other two factors
         * ---------------------------------------------------------------------
         */
        for j in 1..GRID_POINTS[1] - 1 {
            for i in 1..GRID_POINTS[0] - 1 {
                lhsp[j][i][0] = lhs[j][i][0];
                lhsp[j][i][1] = lhs[j][i][1] - DTTY2 * speed[k][j - 1][i];
                lhsp[j][i][2] = lhs[j][i][2];
                lhsp[j][i][3] = lhs[j][i][3] + DTTY2 * speed[k][j + 1][i];
                lhsp[j][i][4] = lhs[j][i][4];
                lhsm[j][i][0] = lhs[j][i][0];
                lhsm[j][i][1] = lhs[j][i][1] + DTTY2 * speed[k][j - 1][i];
                lhsm[j][i][2] = lhs[j][i][2];
                lhsm[j][i][3] = lhs[j][i][3] - DTTY2 * speed[k][j + 1][i];
                lhsm[j][i][4] = lhs[j][i][4];
            }
        }
        /*
         * ---------------------------------------------------------------------
         * FORWARD ELIMINATION
         * ---------------------------------------------------------------------
         */
        for j in 0..GRID_POINTS[1] - 2 {
            let j1 = j + 1;
            let j2 = j + 2;
            for i in 1..GRID_POINTS[0] - 1 {
                fac1 = 1.0 / lhs[j][i][2];
                lhs[j][i][3] = fac1 * lhs[j][i][3];
                lhs[j][i][4] = fac1 * lhs[j][i][4];
                for m in 0..3 {
                    rhs[k][j][i][m] = fac1 * rhs[k][j][i][m];
                }
                lhs[j1][i][2] = lhs[j1][i][2] - lhs[j1][i][1] * lhs[j][i][3];
                lhs[j1][i][3] = lhs[j1][i][3] - lhs[j1][i][1] * lhs[j][i][4];
                for m in 0..3 {
                    rhs[k][j1][i][m] = rhs[k][j1][i][m] - lhs[j1][i][1] * rhs[k][j][i][m];
                }
                lhs[j2][i][1] = lhs[j2][i][1] - lhs[j2][i][0] * lhs[j][i][3];
                lhs[j2][i][2] = lhs[j2][i][2] - lhs[j2][i][0] * lhs[j][i][4];
                for m in 0..3 {
                    rhs[k][j2][i][m] = rhs[k][j2][i][m] - lhs[j2][i][0] * rhs[k][j][i][m];
                }
            }
        }
        /*
         * ---------------------------------------------------------------------
         * the last two rows in this grid block are a bit different,
         * since they do not have two more rows available for the
         * elimination of off-diagonal entries
         * ---------------------------------------------------------------------
         */
        let mut j = GRID_POINTS[1] - 2;
        let mut j1 = GRID_POINTS[1] - 1;
        for i in 1..GRID_POINTS[0] - 1 {
            fac1 = 1.0 / lhs[j][i][2];
            lhs[j][i][3] = fac1 * lhs[j][i][3];
            lhs[j][i][4] = fac1 * lhs[j][i][4];
            for m in 0..3 {
                rhs[k][j][i][m] = fac1 * rhs[k][j][i][m];
            }
            lhs[j1][i][2] = lhs[j1][i][2] - lhs[j1][i][1] * lhs[j][i][3];
            lhs[j1][i][3] = lhs[j1][i][3] - lhs[j1][i][1] * lhs[j][i][4];
            for m in 0..3 {
                rhs[k][j1][i][m] = rhs[k][j1][i][m] - lhs[j1][i][1] * rhs[k][j][i][m];
            }
            /*
             * ---------------------------------------------------------------------
             * scale the last row immediately
             * ---------------------------------------------------------------------
             */
            fac2 = 1.0 / lhs[j1][i][2];
            for m in 0..3 {
                rhs[k][j1][i][m] = fac2 * rhs[k][j1][i][m];
            }
        }
        /*
         * ---------------------------------------------------------------------
         * do the u+c and the u-c factors
         * ---------------------------------------------------------------------
         */
        for j in 0..GRID_POINTS[1] - 2 {
            let j1 = j + 1;
            let j2 = j + 2;
            for i in 1..GRID_POINTS[0] - 1 {
                let mut m = 3;
                fac1 = 1.0 / lhsp[j][i][2];
                lhsp[j][i][3] = fac1 * lhsp[j][i][3];
                lhsp[j][i][4] = fac1 * lhsp[j][i][4];
                rhs[k][j][i][m] = fac1 * rhs[k][j][i][m];
                lhsp[j1][i][2] = lhsp[j1][i][2] - lhsp[j1][i][1] * lhsp[j][i][3];
                lhsp[j1][i][3] = lhsp[j1][i][3] - lhsp[j1][i][1] * lhsp[j][i][4];
                rhs[k][j1][i][m] = rhs[k][j1][i][m] - lhsp[j1][i][1] * rhs[k][j][i][m];
                lhsp[j2][i][1] = lhsp[j2][i][1] - lhsp[j2][i][0] * lhsp[j][i][3];
                lhsp[j2][i][2] = lhsp[j2][i][2] - lhsp[j2][i][0] * lhsp[j][i][4];
                rhs[k][j2][i][m] = rhs[k][j2][i][m] - lhsp[j2][i][0] * rhs[k][j][i][m];
                m = 4;
                fac1 = 1.0 / lhsm[j][i][2];
                lhsm[j][i][3] = fac1 * lhsm[j][i][3];
                lhsm[j][i][4] = fac1 * lhsm[j][i][4];
                rhs[k][j][i][m] = fac1 * rhs[k][j][i][m];
                lhsm[j1][i][2] = lhsm[j1][i][2] - lhsm[j1][i][1] * lhsm[j][i][3];
                lhsm[j1][i][3] = lhsm[j1][i][3] - lhsm[j1][i][1] * lhsm[j][i][4];
                rhs[k][j1][i][m] = rhs[k][j1][i][m] - lhsm[j1][i][1] * rhs[k][j][i][m];
                lhsm[j2][i][1] = lhsm[j2][i][1] - lhsm[j2][i][0] * lhsm[j][i][3];
                lhsm[j2][i][2] = lhsm[j2][i][2] - lhsm[j2][i][0] * lhsm[j][i][4];
                rhs[k][j2][i][m] = rhs[k][j2][i][m] - lhsm[j2][i][0] * rhs[k][j][i][m];
            }
        }
        /*
         * ---------------------------------------------------------------------
         * and again the last two rows separately
         * ---------------------------------------------------------------------
         */
        j = GRID_POINTS[1] - 2;
        j1 = GRID_POINTS[1] - 1;
        for i in 1..GRID_POINTS[0] - 1 {
            let mut m = 3;
            fac1 = 1.0 / lhsp[j][i][2];
            lhsp[j][i][3] = fac1 * lhsp[j][i][3];
            lhsp[j][i][4] = fac1 * lhsp[j][i][4];
            rhs[k][j][i][m] = fac1 * rhs[k][j][i][m];
            lhsp[j1][i][2] = lhsp[j1][i][2] - lhsp[j1][i][1] * lhsp[j][i][3];
            lhsp[j1][i][3] = lhsp[j1][i][3] - lhsp[j1][i][1] * lhsp[j][i][4];
            rhs[k][j1][i][m] = rhs[k][j1][i][m] - lhsp[j1][i][1] * rhs[k][j][i][m];
            m = 4;
            fac1 = 1.0 / lhsm[j][i][2];
            lhsm[j][i][3] = fac1 * lhsm[j][i][3];
            lhsm[j][i][4] = fac1 * lhsm[j][i][4];
            rhs[k][j][i][m] = fac1 * rhs[k][j][i][m];
            lhsm[j1][i][2] = lhsm[j1][i][2] - lhsm[j1][i][1] * lhsm[j][i][3];
            lhsm[j1][i][3] = lhsm[j1][i][3] - lhsm[j1][i][1] * lhsm[j][i][4];
            rhs[k][j1][i][m] = rhs[k][j1][i][m] - lhsm[j1][i][1] * rhs[k][j][i][m];
            /*
             * ---------------------------------------------------------------------
             * scale the last row immediately
             * ---------------------------------------------------------------------
             */
            rhs[k][j1][i][3] = rhs[k][j1][i][3] / lhsp[j1][i][2];
            rhs[k][j1][i][4] = rhs[k][j1][i][4] / lhsm[j1][i][2];
        }
        /*
         * ---------------------------------------------------------------------
         * BACKSUBSTITUTION
         * ---------------------------------------------------------------------
         */
        j = GRID_POINTS[1] - 2;
        j1 = GRID_POINTS[1] - 1;
        for i in 1..GRID_POINTS[0] - 1 {
            for m in 0..3 {
                rhs[k][j][i][m] = rhs[k][j][i][m] - lhs[j][i][3] * rhs[k][j1][i][m];
            }
            rhs[k][j][i][3] = rhs[k][j][i][3] - lhsp[j][i][3] * rhs[k][j1][i][3];
            rhs[k][j][i][4] = rhs[k][j][i][4] - lhsm[j][i][3] * rhs[k][j1][i][4];
        }
        /*
         * ---------------------------------------------------------------------
         * the first three factors
         * ---------------------------------------------------------------------
         */
        for j in (0..GRID_POINTS[1] - 2).rev() {
            j1 = j + 1;
            let j2 = j + 2;
            for i in 1..GRID_POINTS[0] - 1 {
                for m in 0..3 {
                    rhs[k][j][i][m] = rhs[k][j][i][m]
                        - lhs[j][i][3] * rhs[k][j1][i][m]
                        - lhs[j][i][4] * rhs[k][j2][i][m];
                }
                /*
                 * ---------------------------------------------------------------------
                 * and the remaining two
                 * ---------------------------------------------------------------------
                 */
                rhs[k][j][i][3] = rhs[k][j][i][3]
                    - lhsp[j][i][3] * rhs[k][j1][i][3]
                    - lhsp[j][i][4] * rhs[k][j2][i][3];
                rhs[k][j][i][4] = rhs[k][j][i][4]
                    - lhsm[j][i][3] * rhs[k][j1][i][4]
                    - lhsm[j][i][4] * rhs[k][j2][i][4];
            }
        }
    }
    if TIMERS {
        timers.stop(T_YSOLVE);
    }
    pinvr(&mut rhs[..], bt, timers);
}

/*
 * ---------------------------------------------------------------------
 * this function performs the solution of the approximate factorization
 * step in the z-direction for all five matrix components
 * simultaneously. The Thomas algorithm is employed to solve the
 * systems for the z-lines. Boundary conditions are non-periodic
 * ---------------------------------------------------------------------
 */
fn z_solve(
    lhs: &mut [[[f64; 5]; IMAXP + 1]],
    rhs: &mut [[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    lhsp: &mut [[[f64; 5]; IMAXP + 1]],
    lhsm: &mut [[[f64; 5]; IMAXP + 1]],
    cv: &mut [f64],
    rhos: &mut [f64],
    speed: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    ws: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    us: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    vs: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    qs: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    u: &[[[[f64; 5]; IMAXP + 1]; JMAXP + 1]],
    rho_i: &[[[f64; IMAXP + 1]; JMAXP + 1]],
    bt: f64,
    timers: &mut Timer,
) {
    let (mut ru1, mut fac1, mut fac2): (f64, f64, f64);
    if TIMERS {
        timers.start(T_ZSOLVE);
    }
    for j in 1..NY2 + 1 {
        lhsinitj(NZ2 + 1, NX2, &mut lhs[..], &mut lhsp[..], &mut lhsm[..]);
        /*
         * ---------------------------------------------------------------------
         * computes the left hand side for the three z-factors
         * ---------------------------------------------------------------------
         * first fill the lhs for the u-eigenvalue
         * ---------------------------------------------------------------------
         */
        for i in 1..NX2 + 1 {
            for k in 0..NZ2 + 2 {
                ru1 = C3C4 * rho_i[k][j][i];
                cv[k] = ws[k][j][i];
                rhos[k] = f64::max(
                    f64::max(DZ4 + CON43 * ru1, DZ5 + C1C5 * ru1),
                    f64::max(DZMAX + ru1, DZ1),
                );
            }
            for k in 1..NZ2 + 1 {
                lhs[k][i][0] = 0.0;
                lhs[k][i][1] = -DTTZ2 * cv[k - 1] - DTTZ1 * rhos[k - 1];
                lhs[k][i][2] = 1.0 + C2DTTZ1 * rhos[k];
                lhs[k][i][3] = DTTZ2 * cv[k + 1] - DTTZ1 * rhos[k + 1];
                lhs[k][i][4] = 0.0;
            }
        }
        /*
         * ---------------------------------------------------------------------
         * add fourth order dissipation
         * ---------------------------------------------------------------------
         */
        for i in 1..NX2 + 1 {
            let mut k = 1;
            lhs[k][i][2] = lhs[k][i][2] + COMZ5;
            lhs[k][i][3] = lhs[k][i][3] - COMZ4;
            lhs[k][i][4] = lhs[k][i][4] + COMZ1;
            k = 2;
            lhs[k][i][1] = lhs[k][i][1] - COMZ4;
            lhs[k][i][2] = lhs[k][i][2] + COMZ6;
            lhs[k][i][3] = lhs[k][i][3] - COMZ4;
            lhs[k][i][4] = lhs[k][i][4] + COMZ1;
        }
        for k in 3..NZ2 - 1 {
            for i in 1..NX2 + 1 {
                lhs[k][i][0] = lhs[k][i][0] + COMZ1;
                lhs[k][i][1] = lhs[k][i][1] - COMZ4;
                lhs[k][i][2] = lhs[k][i][2] + COMZ6;
                lhs[k][i][3] = lhs[k][i][3] - COMZ4;
                lhs[k][i][4] = lhs[k][i][4] + COMZ1;
            }
        }
        for i in 1..NX2 + 1 {
            let mut k = NZ2 - 1;
            lhs[k][i][0] = lhs[k][i][0] + COMZ1;
            lhs[k][i][1] = lhs[k][i][1] - COMZ4;
            lhs[k][i][2] = lhs[k][i][2] + COMZ6;
            lhs[k][i][3] = lhs[k][i][3] - COMZ4;
            k = NZ2;
            lhs[k][i][0] = lhs[k][i][0] + COMZ1;
            lhs[k][i][1] = lhs[k][i][1] - COMZ4;
            lhs[k][i][2] = lhs[k][i][2] + COMZ5;
        }
        /*
         * ---------------------------------------------------------------------
         * subsequently, fill the other factors (u+c), (u-c)
         * ---------------------------------------------------------------------
         */
        for k in 1..NZ2 + 1 {
            for i in 1..NX2 + 1 {
                lhsp[k][i][0] = lhs[k][i][0];
                lhsp[k][i][1] = lhs[k][i][1] - DTTZ2 * speed[k - 1][j][i];
                lhsp[k][i][2] = lhs[k][i][2];
                lhsp[k][i][3] = lhs[k][i][3] + DTTZ2 * speed[k + 1][j][i];
                lhsp[k][i][4] = lhs[k][i][4];
                lhsm[k][i][0] = lhs[k][i][0];
                lhsm[k][i][1] = lhs[k][i][1] + DTTZ2 * speed[k - 1][j][i];
                lhsm[k][i][2] = lhs[k][i][2];
                lhsm[k][i][3] = lhs[k][i][3] - DTTZ2 * speed[k + 1][j][i];
                lhsm[k][i][4] = lhs[k][i][4];
            }
        }
        /*
         * ---------------------------------------------------------------------
         * FORWARD ELIMINATION
         * ---------------------------------------------------------------------
         */
        for k in 0..GRID_POINTS[2] - 2 {
            let k1 = k + 1;
            let k2 = k + 2;
            for i in 1..NX2 + 1 {
                fac1 = 1.0 / lhs[k][i][2];
                lhs[k][i][3] = fac1 * lhs[k][i][3];
                lhs[k][i][4] = fac1 * lhs[k][i][4];
                for m in 0..3 {
                    rhs[k][j][i][m] = fac1 * rhs[k][j][i][m];
                }
                lhs[k1][i][2] = lhs[k1][i][2] - lhs[k1][i][1] * lhs[k][i][3];
                lhs[k1][i][3] = lhs[k1][i][3] - lhs[k1][i][1] * lhs[k][i][4];
                for m in 0..3 {
                    rhs[k1][j][i][m] = rhs[k1][j][i][m] - lhs[k1][i][1] * rhs[k][j][i][m];
                }
                lhs[k2][i][1] = lhs[k2][i][1] - lhs[k2][i][0] * lhs[k][i][3];
                lhs[k2][i][2] = lhs[k2][i][2] - lhs[k2][i][0] * lhs[k][i][4];
                for m in 0..3 {
                    rhs[k2][j][i][m] = rhs[k2][j][i][m] - lhs[k2][i][0] * rhs[k][j][i][m];
                }
            }
        }
        /*
         * ---------------------------------------------------------------------
         * the last two rows in this grid block are a bit different,
         * since they do not have two more rows available for the
         * elimination of off-diagonal entries
         * ---------------------------------------------------------------------
         */
        let k = GRID_POINTS[2] - 2;
        let k1 = GRID_POINTS[2] - 1;
        for i in 1..NX2 + 1 {
            fac1 = 1.0 / lhs[k][i][2];
            lhs[k][i][3] = fac1 * lhs[k][i][3];
            lhs[k][i][4] = fac1 * lhs[k][i][4];
            for m in 0..3 {
                rhs[k][j][i][m] = fac1 * rhs[k][j][i][m];
            }
            lhs[k1][i][2] = lhs[k1][i][2] - lhs[k1][i][1] * lhs[k][i][3];
            lhs[k1][i][3] = lhs[k1][i][3] - lhs[k1][i][1] * lhs[k][i][4];
            for m in 0..3 {
                rhs[k1][j][i][m] = rhs[k1][j][i][m] - lhs[k1][i][1] * rhs[k][j][i][m];
            }
            /*
             * ---------------------------------------------------------------------
             * scale the last row immediately
             * ---------------------------------------------------------------------
             */
            fac2 = 1.0 / lhs[k1][i][2];
            for m in 0..3 {
                rhs[k1][j][i][m] = fac2 * rhs[k1][j][i][m];
            }
        }
        /*
         * ---------------------------------------------------------------------
         * do the u+c and the u-c factors
         * ---------------------------------------------------------------------
         */
        for k in 0..GRID_POINTS[2] - 2 {
            let k1 = k + 1;
            let k2 = k + 2;
            for i in 1..NX2 + 1 {
                let mut m = 3;
                fac1 = 1.0 / lhsp[k][i][2];
                lhsp[k][i][3] = fac1 * lhsp[k][i][3];
                lhsp[k][i][4] = fac1 * lhsp[k][i][4];
                rhs[k][j][i][m] = fac1 * rhs[k][j][i][m];
                lhsp[k1][i][2] = lhsp[k1][i][2] - lhsp[k1][i][1] * lhsp[k][i][3];
                lhsp[k1][i][3] = lhsp[k1][i][3] - lhsp[k1][i][1] * lhsp[k][i][4];
                rhs[k1][j][i][m] = rhs[k1][j][i][m] - lhsp[k1][i][1] * rhs[k][j][i][m];
                lhsp[k2][i][1] = lhsp[k2][i][1] - lhsp[k2][i][0] * lhsp[k][i][3];
                lhsp[k2][i][2] = lhsp[k2][i][2] - lhsp[k2][i][0] * lhsp[k][i][4];
                rhs[k2][j][i][m] = rhs[k2][j][i][m] - lhsp[k2][i][0] * rhs[k][j][i][m];
                m = 4;
                fac1 = 1.0 / lhsm[k][i][2];
                lhsm[k][i][3] = fac1 * lhsm[k][i][3];
                lhsm[k][i][4] = fac1 * lhsm[k][i][4];
                rhs[k][j][i][m] = fac1 * rhs[k][j][i][m];
                lhsm[k1][i][2] = lhsm[k1][i][2] - lhsm[k1][i][1] * lhsm[k][i][3];
                lhsm[k1][i][3] = lhsm[k1][i][3] - lhsm[k1][i][1] * lhsm[k][i][4];
                rhs[k1][j][i][m] = rhs[k1][j][i][m] - lhsm[k1][i][1] * rhs[k][j][i][m];
                lhsm[k2][i][1] = lhsm[k2][i][1] - lhsm[k2][i][0] * lhsm[k][i][3];
                lhsm[k2][i][2] = lhsm[k2][i][2] - lhsm[k2][i][0] * lhsm[k][i][4];
                rhs[k2][j][i][m] = rhs[k2][j][i][m] - lhsm[k2][i][0] * rhs[k][j][i][m];
            }
        }
        /*
         * ---------------------------------------------------------------------
         * and again the last two rows separately
         * ---------------------------------------------------------------------
         */
        let mut k = GRID_POINTS[2] - 2;
        let mut k1 = GRID_POINTS[2] - 1;
        for i in 1..NX2 + 1 {
            let mut m = 3;
            fac1 = 1.0 / lhsp[k][i][2];
            lhsp[k][i][3] = fac1 * lhsp[k][i][3];
            lhsp[k][i][4] = fac1 * lhsp[k][i][4];
            rhs[k][j][i][m] = fac1 * rhs[k][j][i][m];
            lhsp[k1][i][2] = lhsp[k1][i][2] - lhsp[k1][i][1] * lhsp[k][i][3];
            lhsp[k1][i][3] = lhsp[k1][i][3] - lhsp[k1][i][1] * lhsp[k][i][4];
            rhs[k1][j][i][m] = rhs[k1][j][i][m] - lhsp[k1][i][1] * rhs[k][j][i][m];
            m = 4;
            fac1 = 1.0 / lhsm[k][i][2];
            lhsm[k][i][3] = fac1 * lhsm[k][i][3];
            lhsm[k][i][4] = fac1 * lhsm[k][i][4];
            rhs[k][j][i][m] = fac1 * rhs[k][j][i][m];
            lhsm[k1][i][2] = lhsm[k1][i][2] - lhsm[k1][i][1] * lhsm[k][i][3];
            lhsm[k1][i][3] = lhsm[k1][i][3] - lhsm[k1][i][1] * lhsm[k][i][4];
            rhs[k1][j][i][m] = rhs[k1][j][i][m] - lhsm[k1][i][1] * rhs[k][j][i][m];
            /*
             * ---------------------------------------------------------------------
             * scale the last row immediately (some of this is overkill
             * if this is the last cell)
             * ---------------------------------------------------------------------
             */
            rhs[k1][j][i][3] = rhs[k1][j][i][3] / lhsp[k1][i][2];
            rhs[k1][j][i][4] = rhs[k1][j][i][4] / lhsm[k1][i][2];
        }
        /*
         * ---------------------------------------------------------------------
         * BACKSUBSTITUTION
         * ---------------------------------------------------------------------
         */
        k = GRID_POINTS[2] - 2;
        k1 = GRID_POINTS[2] - 1;
        for i in 1..NX2 + 1 {
            for m in 0..3 {
                rhs[k][j][i][m] = rhs[k][j][i][m] - lhs[k][i][3] * rhs[k1][j][i][m];
            }
            rhs[k][j][i][3] = rhs[k][j][i][3] - lhsp[k][i][3] * rhs[k1][j][i][3];
            rhs[k][j][i][4] = rhs[k][j][i][4] - lhsm[k][i][3] * rhs[k1][j][i][4];
        }
        /*
         * ---------------------------------------------------------------------
         * whether or not this is the last processor, we always have
         * to complete the back-substitution
         * ---------------------------------------------------------------------
         * the first three factors
         * ---------------------------------------------------------------------
         */
        for k in (0..GRID_POINTS[2] - 2).rev() {
            k1 = k + 1;
            let k2 = k + 2;
            for i in 1..NX2 + 1 {
                for m in 0..3 {
                    rhs[k][j][i][m] = rhs[k][j][i][m]
                        - lhs[k][i][3] * rhs[k1][j][i][m]
                        - lhs[k][i][4] * rhs[k2][j][i][m];
                }
                /*
                 * ---------------------------------------------------------------------
                 * and the remaining two
                 * ---------------------------------------------------------------------
                 */
                rhs[k][j][i][3] = rhs[k][j][i][3]
                    - lhsp[k][i][3] * rhs[k1][j][i][3]
                    - lhsp[k][i][4] * rhs[k2][j][i][3];
                rhs[k][j][i][4] = rhs[k][j][i][4]
                    - lhsm[k][i][3] * rhs[k1][j][i][4]
                    - lhsm[k][i][4] * rhs[k2][j][i][4];
            }
        }
    }
    if TIMERS {
        timers.stop(T_ZSOLVE);
    }
    tzetar(
        &mut rhs[..],
        &us[..],
        &vs[..],
        &ws[..],
        &speed[..],
        &qs[..],
        &u[..],
        bt,
        timers,
    );
}
