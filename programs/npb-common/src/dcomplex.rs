#[derive(Clone, Copy)]
pub struct Dcomplex {
    pub real: f64,
    pub imag: f64,
}

#[allow(dead_code)]
impl Dcomplex {
    #[inline(always)]
    pub const fn dcomplex_create(real: &f64, imag: &f64) -> Dcomplex {
        Dcomplex {
            real: *real,
            imag: *imag,
        }
    }

    #[inline(always)]
    pub const fn dcomplex_add(a: &Dcomplex, b: &Dcomplex) -> Dcomplex {
        Dcomplex {
            real: (a.real + b.real),
            imag: (a.imag + b.imag),
        }
    }

    #[inline(always)]
    pub const fn dcomplex_sub(a: &Dcomplex, b: &Dcomplex) -> Dcomplex {
        Dcomplex {
            real: (a.real - b.real),
            imag: (a.imag - b.imag),
        }
    }

    #[inline(always)]
    pub const fn dcomplex_mul(a: &Dcomplex, b: &Dcomplex) -> Dcomplex {
        Dcomplex {
            real: ((a.real * b.real) - (a.imag * b.imag)),
            imag: ((a.real * b.imag) + (a.imag * b.real)),
        }
    }

    #[inline(always)]
    pub const fn dcomplex_mul2(a: &Dcomplex, b: &f64) -> Dcomplex {
        Dcomplex {
            real: (a.real * *b),
            imag: (a.imag * *b),
        }
    }

    #[inline(always)]
    pub const fn dcomplex_div(z1: &Dcomplex, z2: &Dcomplex) -> Dcomplex {
        let divisor: f64 = z2.real * z2.real + z2.imag * z2.imag;
        Dcomplex {
            real: ((z1.real * z2.real + z1.imag * z2.imag) / divisor),
            imag: ((z1.imag * z2.real - z1.real * z2.imag) / divisor),
        }
    }

    #[inline(always)]
    pub const fn dcomplex_div2(a: &Dcomplex, b: &f64) -> Dcomplex {
        Dcomplex {
            real: a.real / *b,
            imag: a.imag / *b,
        }
    }

    #[inline(always)]
    pub fn dcomplex_abs(x: &Dcomplex) -> f64 {
        f64::sqrt((x.real * x.real) + (x.imag * x.imag))
    }

    #[inline(always)]
    pub const fn dconjg(x: &Dcomplex) -> Dcomplex {
        Dcomplex {
            real: x.real,
            imag: (-x.imag),
        }
    }
}