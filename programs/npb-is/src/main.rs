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
    pub const TOTAL_KEYS_LOG_2: i32 = 10;
    pub const MAX_KEY_LOG_2: i32 = 8;
    pub const NUM_BUCKETS_LOG_2: i32 = 5;
    pub type IntType = i32;
    pub const TOTAL_KEYS: i32 = 1 << TOTAL_KEYS_LOG_2;
    pub const TEST_INDEX_ARRAY: &[IntType] = &[10, 100, 500, 900, 1023];
    pub const TEST_RANK_ARRAY: &[IntType] = &[307, 716, 458, 235, 309];
}

#[cfg(class = "S")]
mod params {
    pub const CLASS: char = 'S';
    pub const TOTAL_KEYS_LOG_2: i32 = 16;
    pub const MAX_KEY_LOG_2: i32 = 11;
    pub const NUM_BUCKETS_LOG_2: i32 = 9;
    pub type IntType = i32;
    pub const TOTAL_KEYS: i32 = 1 << TOTAL_KEYS_LOG_2;
    pub const TEST_INDEX_ARRAY: &[IntType] = &[48427, 17148, 23627, 62548, 4431];
    pub const TEST_RANK_ARRAY: &[IntType] = &[0, 18, 346, 64917, 65463];
}

#[cfg(class = "W")]
mod params {
    pub const CLASS: char = 'W';
    pub const TOTAL_KEYS_LOG_2: i32 = 20;
    pub const MAX_KEY_LOG_2: i32 = 16;
    pub const NUM_BUCKETS_LOG_2: i32 = 10;
    pub type IntType = i32;
    pub const TOTAL_KEYS: i32 = 1 << TOTAL_KEYS_LOG_2;
    pub const TEST_INDEX_ARRAY: &[IntType] = &[357773, 934767, 875723, 898999, 404505];
    pub const TEST_RANK_ARRAY: &[IntType] = &[1249, 11698, 1039987, 1043896, 1048018];
}

#[cfg(class = "A")]
mod params {
    pub const CLASS: char = 'A';
    pub const TOTAL_KEYS_LOG_2: i32 = 23;
    pub const MAX_KEY_LOG_2: i32 = 19;
    pub const NUM_BUCKETS_LOG_2: i32 = 10;
    pub type IntType = i32;
    pub const TOTAL_KEYS: i32 = 1 << TOTAL_KEYS_LOG_2;
    pub const TEST_INDEX_ARRAY: &[IntType] = &[2112377, 662041, 5336171, 3642833, 4250760];
    pub const TEST_RANK_ARRAY: &[IntType] = &[104, 17523, 123928, 8288932, 8388264];
}

#[cfg(class = "B")]
mod params {
    pub const CLASS: char = 'B';
    pub const TOTAL_KEYS_LOG_2: i32 = 25;
    pub const MAX_KEY_LOG_2: i32 = 21;
    pub const NUM_BUCKETS_LOG_2: i32 = 10;
    pub type IntType = i32;
    pub const TOTAL_KEYS: i32 = 1 << TOTAL_KEYS_LOG_2;
    pub const TEST_INDEX_ARRAY: &[IntType] = &[41869, 812306, 5102857, 18232239, 26860214];
    pub const TEST_RANK_ARRAY: &[IntType] = &[33422937, 10244, 59149, 33135281, 99];
}

#[cfg(class = "C")]
mod params {
    pub const CLASS: char = 'C';
    pub const TOTAL_KEYS_LOG_2: i32 = 27;
    pub const MAX_KEY_LOG_2: i32 = 23;
    pub const NUM_BUCKETS_LOG_2: i32 = 10;
    pub type IntType = i32;
    pub const TOTAL_KEYS: i32 = 1 << TOTAL_KEYS_LOG_2;
    pub const TEST_INDEX_ARRAY: &[IntType] = &[44172927, 72999161, 74326391, 129606274, 21736814];
    pub const TEST_RANK_ARRAY: &[IntType] = &[61147, 882988, 266290, 133997595, 133525895];
}

#[cfg(class = "D")]
mod params {
    pub const CLASS: char = 'D';
    pub const TOTAL_KEYS_LOG_2: i64 = 31;
    pub const MAX_KEY_LOG_2: i64 = 27;
    pub const NUM_BUCKETS_LOG_2: i64 = 10;
    pub type IntType = i64;
    pub const TOTAL_KEYS: i64 = 1i64 << TOTAL_KEYS_LOG_2;
    pub const TEST_INDEX_ARRAY: &[IntType] =
        &[1317351170, 995930646, 1157283250, 1503301535, 1453734525];
    pub const TEST_RANK_ARRAY: &[IntType] = &[1, 36538729, 1978098519, 2145192618, 2147425337];
}

#[cfg(not(any(
    class = "S",
    class = "W",
    class = "A",
    class = "B",
    class = "C",
    class = "D",
    class = "Z",
)))]
mod params {
    //Never used
    pub const CLASS: char = 'U';
    pub const TOTAL_KEYS_LOG_2: i32 = 1;
    pub const MAX_KEY_LOG_2: i32 = 1;
    pub const NUM_BUCKETS_LOG_2: i32 = 1;
    pub type IntType = i32;
    pub const TOTAL_KEYS: IntType = 1 << TOTAL_KEYS_LOG_2;
    pub const TEST_INDEX_ARRAY: &[IntType] = &[0];
    pub const TEST_RANK_ARRAY: &[IntType] = &[0];
    compile_error!(
        "\n\n\
		Must set a class at compilation time by setting RUSTFLAGS\n\
		class options for IS are: {S, W, A, B, C, D}\n\
		For example:\n\
		RUSTFLAGS='--cfg class=\"A\" ' cargo build --release --bin is\n\n\n\
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

pub const T_BENCHMARKING: usize = 0;
pub const T_INITIALIZATION: usize = 1;
pub const T_SORTING: usize = 2;
pub const T_TOTAL_EXECUTION: usize = 3;

pub const MAX_KEY: IntType = 1 << MAX_KEY_LOG_2;
pub const NUM_BUCKETS: i32 = 1 << NUM_BUCKETS_LOG_2;
pub const NUM_KEYS: IntType = TOTAL_KEYS;
pub const SIZE_OF_BUFFERS: IntType = NUM_KEYS;

pub const MAX_ITERATIONS: IntType = 1;
pub const TEST_ARRAY_SIZE: usize = 5;

/* is */
fn main() {
    let mut passed_verification: i8 = 0;

    let mut key_array: Vec<IntType> = vec![0; SIZE_OF_BUFFERS as usize + 1];
    let mut key_buff1: Vec<IntType> = vec![0; MAX_KEY as usize];
    let mut key_buff2: Vec<IntType> = vec![0; SIZE_OF_BUFFERS as usize + 1];
    let mut partial_verify_vals: Vec<IntType> = vec![0; TEST_ARRAY_SIZE];

    let mut bucket_size: Vec<Vec<IntType>>;
    let mut bucket_ptrs: Vec<IntType> = vec![0; NUM_BUCKETS as usize];

    let mut timecounter: f64;

    /* Initialize timers */
    let mut timers = Timer::new();
    timers.clear(T_BENCHMARKING);
    if TIMERS {
        timers.clear(T_INITIALIZATION);
        timers.clear(T_SORTING);
        timers.clear(T_TOTAL_EXECUTION);
        timers.start(T_TOTAL_EXECUTION);
    }

    /* Printout initial NPB info */
    print!("\n\n NAS Parallel Benchmarks 4.1 Serial Rust version - IS Benchmark\n\n");
    print!(" Size:  {}  (class {})\n", TOTAL_KEYS as i64, CLASS);
    print!(" Iterations:   {}\n", MAX_ITERATIONS);
    print!("\n");

    if TIMERS {
        timers.start(T_INITIALIZATION)
    }

    /* Generate random number sequence and subsequent keys on all procs */
    create_seq(
        314159265.00,  /* Random number gen seed */
        1220703125.00, /* Random number gen mult */
        &mut key_array[..],
    );

    let num_procs = 1;

    bucket_size = vec![vec![0; NUM_BUCKETS as usize]; num_procs];

    if TIMERS {
        timers.stop(T_INITIALIZATION);
    }

    /* Do one interation for free (i.e., untimed) to guarantee initialization of */
    /* all data and code pages and respective tables */
    rank(
        1,
        &mut key_array[..],
        &mut partial_verify_vals[..],
        &mut key_buff1[..],
        &mut key_buff2[..],
        &mut bucket_size[..],
        &mut bucket_ptrs[..],
        &mut passed_verification,
    );

    /* Start verification counter */
    passed_verification = 0;

    if CLASS != 'S' {
        print!("\n   iteration\n")
    };

    /* Start timer */
    timers.start(T_BENCHMARKING);

    /* This is the main iteration */
    for iteration in 1..MAX_ITERATIONS as IntType + 1 {
        if CLASS != 'S' {
            print!("        {}\n", iteration);
        }
        rank(
            iteration,
            &mut key_array[..],
            &mut partial_verify_vals[..],
            &mut key_buff1[..],
            &mut key_buff2[..],
            &mut bucket_size[..],
            &mut bucket_ptrs[..],
            &mut passed_verification,
        );
    }

    /* End of timing, obtain maximum time of all processors */
    timers.stop(T_BENCHMARKING);
    timecounter = timers.read(T_BENCHMARKING).as_secs_f64();

    /* This tests that keys are in sequence: sorting of last ranked key seq */
    /* occurs here, but is an untimed operation */
    if TIMERS {
        timers.start(T_SORTING)
    }
    full_verify(
        &mut key_array[..],
        &mut bucket_ptrs[..],
        &mut key_buff1[..],
        &mut key_buff2[..],
        &mut passed_verification,
    );
    if TIMERS {
        timers.stop(T_SORTING)
    }

    if TIMERS {
        timers.stop(T_TOTAL_EXECUTION)
    }

    /* The final printout */
    if passed_verification != 5 * MAX_ITERATIONS as i8 + 1 {
        passed_verification = 0;
    } else {
        passed_verification = 1;
    }
    let info = PrintInfo {
        name: String::from("IS"),
        class: CLASS.to_string(),
        size: (TOTAL_KEYS as usize, 0, 0),
        num_iter: MAX_ITERATIONS as i32,
        time: timecounter,
        mops: (MAX_ITERATIONS * TOTAL_KEYS) as f64 / timecounter / 1000000.0,
        operation: String::from("keys ranked"),
        verified: passed_verification,
        num_threads: 1,
        //uns: UNSAFE
    };
    printer(info);

    /* Print additional timers */
    if TIMERS {
        let (mut t_total, mut t_percent): (f64, f64);
        t_total = timers.read(T_TOTAL_EXECUTION).as_secs_f64();
        print!("\nAdditional timers -\n");
        print!(" Total execution: {:>8.3}\n", t_total);
        if t_total == 0.0 {
            t_total = 1.0;
        }
        timecounter = timers.read(T_INITIALIZATION).as_secs_f64();
        t_percent = timecounter / t_total * 100.;
        print!(
            " Initialization : {:>8.3} ({:>5.2}%)\n",
            timecounter, t_percent
        );
        timecounter = timers.read(T_BENCHMARKING).as_secs_f64();
        t_percent = timecounter / t_total * 100.;
        print!(
            " Benchmarking   : {:>8.3} ({:>5.2}%)\n",
            timecounter, t_percent
        );
        timecounter = timers.read(T_SORTING).as_secs_f64();
        t_percent = timecounter / t_total * 100.;
        print!(
            " Sorting        : {:>8.3} ({:>5.2}%)\n",
            timecounter, t_percent
        );
    }
}

/*****************************************************************/
/*************      C  R  E  A  T  E  _  S  E  Q      ************/
/*****************************************************************/
fn create_seq(seed: f64, a: f64, key_array: &mut [IntType]) {
    let (mut x, mut s): (f64, f64);
    let k: IntType = MAX_KEY / 4;

    let (k1, mut k2): (IntType, IntType);
    let an: f64 = a;
    let (myid, num_procs): (IntType, IntType);
    let mq: IntType;

    myid = 0;
    num_procs = 1;

    mq = (NUM_KEYS + num_procs - 1) / num_procs;
    k1 = mq * myid;
    k2 = k1 + mq;
    if k2 > NUM_KEYS {
        k2 = NUM_KEYS;
    }

    s = find_my_seed(myid, num_procs, (NUM_KEYS << 2) as i64, seed, an);

    for i in k1..k2 {
        x = randlc(&mut s, an);
        x += randlc(&mut s, an);
        x += randlc(&mut s, an);
        x += randlc(&mut s, an);
        key_array[i as usize] = (k as f64 * x) as IntType;
    }
}

/*****************************************************************/
/************   F  I  N  D  _  M  Y  _  S  E  E  D    ************/
/************                                         ************/
/************ returns parallel random number seq seed ************/
/*****************************************************************/
fn find_my_seed(
    kn: IntType, /* my processor rank, 0<=kn<=num procs */
    np: IntType, /* np = num procs */
    nn: i64,     /* total num of ran numbers, all procs */
    s: f64,      /* Ran num seed, for ex.: 314159265.00 */
    a: f64,
) -> f64 {
    /* Ran num gen mult, try 1220703125.00 */
    /*
     * Create a random number sequence of total length nn residing
     * on np number of processors.  Each processor will therefore have a
     * subsequence of length nn/np.  This routine returns that random
     * number which is the first random number for the subsequence belonging
     * to processor rank kn, and which is used as seed for proc kn ran # gen.
     */
    let (mut t1, mut t2): (f64, f64);
    let (mq, nq, mut kk, mut ik): (i64, i64, i64, i64);

    if kn == 0 {
        return s;
    }

    mq = ((nn >> 2) + np as i64 - 1) / np as i64;
    nq = (mq << 2) * kn as i64; /* number of rans to be skipped */

    t1 = s;
    t2 = a;
    kk = nq;

    while kk > 1 {
        ik = kk / 2;
        if 2 * ik == kk {
            let aux_t2: f64 = t2;
            randlc(&mut t2, aux_t2);
            kk = ik;
        } else {
            randlc(&mut t1, t2);
            kk = kk - 1;
        }
    }
    randlc(&mut t1, t2);

    return t1;
}

/*****************************************************************/
/*************    F  U  L  L  _  V  E  R  I  F  Y     ************/
/*****************************************************************/
fn full_verify(
    key_array: &mut [IntType],
    bucket_ptrs: &mut [IntType],
    key_buff1: &mut [IntType],
    key_buff2: &mut [IntType],
    passed_verification: &mut i8,
) {
    let mut j: IntType;
    let (mut k, mut k1): (IntType, IntType);

    /* Now, finally, sort the keys: */
    /* Copy keys into work array; keys in key_array will be reassigned. */

    /* Buckets are already sorted. Sorting keys within each bucket */
    for j in 0..NUM_BUCKETS as usize {
        k1 = {
            if j > 0 {
                bucket_ptrs[j - 1]
            } else {
                0
            }
        };
        for i in k1..bucket_ptrs[j] {
            key_buff1[key_buff2[i as usize] as usize] -= 1;
            k = key_buff1[key_buff2[i as usize] as usize];
            key_array[k as usize] = key_buff2[i as usize];
        }
    }

    /* Confirm keys correctly sorted: count incorrectly sorted keys, if any */
    j = 0;
    for i in 1..NUM_KEYS as usize {
        if key_array[i - 1] > key_array[i] {
            j += 1
        }
    }
    if j != 0 {
        print!("Full_verify: number of keys out of sort: {}\n", j as i64);
    } else {
        *passed_verification += 1;
    }
}

/*****************************************************************/
/*************             R  A  N  K             ****************/
/*****************************************************************/
fn rank(
    iteration: IntType,
    key_array: &mut [IntType],
    partial_verify_vals: &mut [IntType],
    key_buff1: &mut [IntType],
    key_buff2: &mut [IntType],
    bucket_size: &mut [Vec<IntType>],
    bucket_ptrs: &mut [IntType],
    passed_verification: &mut i8,
) {
    let mut k: IntType;
    let shift: IntType = MAX_KEY_LOG_2 - NUM_BUCKETS_LOG_2;
    let num_bucket_keys: IntType = (1_i64 << shift as i64) as IntType;

    key_array[iteration as usize] = iteration;
    key_array[(iteration + MAX_ITERATIONS) as usize] = MAX_KEY - iteration;

    /* Determine where the partial verify test keys are, load into */
    /* top of array bucket_size */
    (partial_verify_vals[0..TEST_ARRAY_SIZE])
        .iter_mut()
        .zip(&TEST_INDEX_ARRAY[0..TEST_ARRAY_SIZE])
        .for_each(|(pkv, ti)| {
            *pkv = key_array[*ti as usize];
        });

    let (mut k1, mut k2, mut m): (IntType, IntType, IntType);

    let myid: usize = 0;
    let num_procs: usize = 1;

    /* Bucket sort is known to improve cache performance on some */
    /* cache based systems.  But the actual performance may depend */
    /* on cache size, problem size. */

    /* Initialize */
    for i in 0..NUM_BUCKETS as usize {
        bucket_size[myid][i] = 0
    }

    /* Determine the number of keys in each bucket */
    (key_array[0..NUM_KEYS as usize]).iter().for_each(|ka| {
        if UNSAFE {
            unsafe {
                *bucket_size
                    .get_unchecked_mut(myid)
                    .get_unchecked_mut((ka >> shift) as usize) += 1;
            }
        } else {
            bucket_size[myid][(ka >> shift) as usize] += 1;
        }
    });

    /* Accumulative bucket sizes are the bucket pointers. */
    /* These are global sizes accumulated upon to each bucket */
    bucket_ptrs[0] = 0;
    bucket_ptrs[0] = (&bucket_size[0..myid])
        .iter()
        .map(|bucket_size| bucket_size[0])
        .sum();

    for i in 1..NUM_BUCKETS as usize {
        bucket_ptrs[i] = bucket_ptrs[i - 1];
        bucket_ptrs[i] += (&bucket_size[0..myid])
            .iter()
            .map(|bucket_size| bucket_size[i])
            .sum::<IntType>();
        bucket_ptrs[i] += (&bucket_size[myid..num_procs])
            .iter()
            .map(|bucket_size| bucket_size[i - 1])
            .sum::<IntType>();
    }

    /* Sort into appropriate bucket */
    (key_array[0..NUM_KEYS as usize]).iter().for_each(|k| {
        if UNSAFE {
            unsafe {
                *key_buff2.get_unchecked_mut(
                        *bucket_ptrs.get_unchecked((k >> shift) as usize) as usize,
                    ) = *k;
                *bucket_ptrs.get_unchecked_mut((k >> shift) as usize) += 1;
            }
        } else {
            key_buff2[bucket_ptrs[(k >> shift) as usize] as usize] = *k;
            bucket_ptrs[(k >> shift) as usize] += 1;
        }
    });

    /* The bucket pointers now point to the final accumulated sizes */
    if myid < num_procs - 1 {
        for i in 0..NUM_BUCKETS as usize {
            bucket_ptrs[i] += (&bucket_size[myid + 1..num_procs])
                .iter()
                .map(|bucket_size| bucket_size[i])
                .sum::<IntType>();
        }
    }

    /* Now, buckets are sorted.  We only need to sort keys inside */
    /* each bucket, which can be done in parallel. */
    for i in 0..NUM_BUCKETS as IntType {
        /* Clear the work array section associated with each bucket */
        k1 = i * num_bucket_keys;
        k2 = k1 + num_bucket_keys;
        key_buff1[k1 as usize..k2 as usize].fill(0);
        /* Ranking of all keys occurs in this section: */
        /* In this section, the keys themselves are used as their */
        /* own indexes to determine how many of each there are: their */
        /* individual population */
        m = {
            if i > 0 {
                bucket_ptrs[i as usize - 1]
            } else {
                0
            }
        };

        for k in m..bucket_ptrs[i as usize] {
            if UNSAFE {
                unsafe {
                    *key_buff1.get_unchecked_mut(*key_buff2.get_unchecked(k as usize) as usize) +=
                        1;
                }
            } else {
                key_buff1[key_buff2[k as usize] as usize] += 1;
            }
        }
        /* Now they have individual key population */
        /* To obtain ranks of each key, successively add the individual key */
        /* population, not forgetting to add m, the total of lesser keys, */
        /* to the first key population */
        key_buff1[k1 as usize] += m;
        for k in k1 + 1..k2 {
            key_buff1[k as usize] += key_buff1[k as usize - 1];
        }
    }

    /* This is the partial verify test section */
    /* Observe that test_rank_array vals are */
    /* shifted differently for different cases */
    for i in 0..TEST_ARRAY_SIZE {
        k = partial_verify_vals[i]; /* test vals were put here */
        if 0 < k && k <= NUM_KEYS - 1 {
            let key_rank: IntType = key_buff1[k as usize - 1];
            let mut failed = 0;

            match CLASS {
                'S' => {
                    if i <= 2 {
                        if key_rank != TEST_RANK_ARRAY[i] + iteration {
                            failed = 1;
                        } else {
                            *passed_verification += 1;
                        }
                    } else {
                        if key_rank != TEST_RANK_ARRAY[i] - iteration {
                            failed = 1;
                        } else {
                            *passed_verification += 1;
                        }
                    }
                }
                'W' => {
                    if i < 2 {
                        if key_rank != TEST_RANK_ARRAY[i] + iteration - 2 {
                            failed = 1;
                        } else {
                            *passed_verification += 1;
                        }
                    } else {
                        if key_rank != TEST_RANK_ARRAY[i] - iteration {
                            failed = 1;
                        } else {
                            *passed_verification += 1;
                        }
                    }
                }
                'A' => {
                    if i <= 2 {
                        if key_rank != TEST_RANK_ARRAY[i] + (iteration - 1) {
                            failed = 1;
                        } else {
                            *passed_verification += 1;
                        }
                    } else {
                        if key_rank != TEST_RANK_ARRAY[i] - (iteration - 1) {
                            failed = 1;
                        } else {
                            *passed_verification += 1;
                        }
                    }
                }
                'B' => {
                    if i == 1 || i == 2 || i == 4 {
                        if key_rank != TEST_RANK_ARRAY[i] + iteration {
                            failed = 1;
                        } else {
                            *passed_verification += 1;
                        }
                    } else {
                        if key_rank != TEST_RANK_ARRAY[i] - iteration {
                            failed = 1;
                        } else {
                            *passed_verification += 1;
                        }
                    }
                }
                'C' => {
                    if i <= 2 {
                        if key_rank != TEST_RANK_ARRAY[i] + iteration {
                            failed = 1;
                        } else {
                            *passed_verification += 1;
                        }
                    } else {
                        if key_rank != TEST_RANK_ARRAY[i] - iteration {
                            failed = 1;
                        } else {
                            *passed_verification += 1;
                        }
                    }
                }
                'D' => {
                    if i < 2 {
                        if key_rank != TEST_RANK_ARRAY[i] + iteration {
                            failed = 1;
                        } else {
                            *passed_verification += 1;
                        }
                    } else {
                        if key_rank != TEST_RANK_ARRAY[i] - iteration {
                            failed = 1;
                        } else {
                            *passed_verification += 1;
                        }
                    }
                }
                'Z' => {
                    if i <= 2 {
                        if key_rank != TEST_RANK_ARRAY[i] + iteration {
                            failed = 1;
                        } else {
                            *passed_verification += 1;
                        }
                    } else {
                        if key_rank != TEST_RANK_ARRAY[i] - iteration {
                            failed = 1;
                        } else {
                            *passed_verification += 1;
                        }
                    }
                }
                _ => {}
            }
            if failed == 1 {
                println!(
                    "Failed partial verification: iteration {}, test key {}",
                    iteration, i as i32
                );
            }
        }
    }
}
