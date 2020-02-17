use ddreal::DDReal;
use num::{Float, FromPrimitive};

/*
- Ref: Gamma / Error Functions, Takuya OOURA
    - http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf.html
    - http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf-j.html
    - http://www.kurims.kyoto-u.ac.jp/~ooura/papers/jsiam95.pdf
    - http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf.tar.gz
    - http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf.zip
- Ref: jStat - JavaScript Statistical Library
    - https://github.com/jstat/jstat
- Ref: Boost Math
    - https://www.boost.org/doc/libs/1_72_0/libs/math/doc/html/index.html
    - https://www.boost.org/doc/libs/1_72_0/libs/math/doc/html/special.html
    - https://www.boost.org/doc/libs/1_72_0/boost/math/special_functions/lanczos.hpp
        - https://www.boost.org/doc/libs/1_72_0/libs/math/doc/html/math_toolkit/lanczos.html
    - https://www.boost.org/doc/libs/1_72_0/boost/math/special_functions/gamma.hpp
        - https://www.boost.org/doc/libs/1_72_0/libs/math/doc/html/math_toolkit/sf_gamma/tgamma.html
        - https://www.boost.org/doc/libs/1_72_0/libs/math/doc/html/math_toolkit/sf_gamma/lgamma.html
    - https://www.boost.org/doc/libs/1_72_0/boost/math/special_functions/beta.hpp
        - https://www.boost.org/doc/libs/1_72_0/libs/math/doc/html/math_toolkit/sf_beta/beta_function.html
        - https://www.boost.org/doc/libs/1_72_0/libs/math/doc/html/math_toolkit/sf_beta/ibeta_function.html
        - https://www.boost.org/doc/libs/1_72_0/libs/math/doc/html/math_toolkit/sf_beta/ibeta_inv_function.html
    - https://www.boost.org/doc/libs/1_72_0/boost/math/special_functions/erf.hpp
        - https://www.boost.org/doc/libs/1_72_0/libs/math/doc/html/math_toolkit/sf_erf/error_function.html
        - https://www.boost.org/doc/libs/1_72_0/libs/math/doc/html/math_toolkit/sf_erf/error_inv.html
*/

pub trait SpecialFloat: Float + FromPrimitive {
    fn eloconst() -> Self;
    fn gammaln(self) -> Self;
    fn ln1mexp(self) -> Self {
        if self.is_zero() || self.is_sign_positive() {
            Self::nan()
        } else if self.to_f64().unwrap() > -std::f64::consts::LN_2 {
            (-(self.exp_m1())).ln()
        } else {
            (-(self.exp())).ln_1p()
        }
    }
    fn ln1pexp(self) -> Self {
        if self.is_sign_negative() {
            self.exp().ln_1p()
        } else {
            self + (-self).exp().ln_1p()
        }
    }
    fn enlogit(self) -> Self {
        if self <= Self::zero() {
            return Self::neg_infinity();
        }
        if self >= Self::one() {
            return Self::infinity();
        }
        self.ln() - (-self).ln_1p()
    }
    fn delogit(self) -> Self {
        if self.is_sign_positive() {
            let nexp = (-self).exp();
            (nexp + Self::one()).recip()
        } else {
            let pexp = self.exp();
            pexp / (pexp + Self::one())
        }
    }
    fn logdelogit(self) -> Self {
        if self.is_sign_positive() {
            -((-self).exp().ln_1p())
        } else {
            self - self.exp().ln_1p()
        }
    }
    fn expenlogit(self) -> Self {
        if self.is_sign_positive() {
            return Self::infinity();
        }
        self - self.ln1mexp()
    }
}

// ln(1-exp(x))

// ln(1+exp(x))
#[allow(dead_code)]

impl SpecialFloat for f64 {
    fn eloconst() -> Self {
        std::f64::consts::LOG10_E * 400.0
    }
    fn gammaln(self) -> Self {
        let x = self;
        (x - 0.5) * (x + 6.097_507_575_390_685_760_943_755_8e+0).ln() - x
            + ((((((1.124_058_265_716_540_738_343_799_9e-2
                / (x + 5.000_358_988_483_192_554_161_323_7e+0)
                + 5.021_972_270_339_209_072_588_416_8e-1)
                / (x + 3.999_996_630_000_750_893_209_701_6e+0)
                + 2.0962_955_353_894_997_733_869_983e+0)
                / (x + 3.000_000_046_726_524_145_843_161_8e+0)
                + 2.250_230_475_356_181_683_669_585_6e+0)
                / (x + 1.999_999_999_620_102_305_806_517_1e+0)
                + 8.513_708_131_650_341_831_241_165_6e-1)
                / (x + 1.000_000_000_000_655_324_317_056_2e+0)
                + 1.224_259_773_268_799_178_464_597_3e-1)
                / x
                + 5.636_065_618_975_606_496_797_756_4e-3)
                .ln()
            - 1.0
            + 1.0
    }
}

impl SpecialFloat for DDReal {
    fn eloconst() -> Self {
        ddreal::ddconsts::LOG10_E * 400.0
    }
    fn gammaln(self) -> Self {
        const V: DDReal = DDReal {
            x: [1.357812200070394582e1, 6.586738271856839074e-16],
        };
        const AR: DDReal = DDReal {
            x: [3.178238429973489754e-6, 8.844048425878723413e-23],
        };
        const A: [DDReal; 13] = [
            DDReal {
                x: [3.148207028334930086e-4, -5.014885054160961393e-21],
            },
            DDReal {
                x: [1.279374160872298431e-2, 1.881195803054246713e-19],
            },
            DDReal {
                x: [2.787483030602997958e-1, 1.298069106165306386e-17],
            },
            DDReal {
                x: [3.574876395822856878e0, 1.398497777173346160e-16],
            },
            DDReal {
                x: [2.792728042156332435e1, 6.626932123256092213e-16],
            },
            DDReal {
                x: [1.332138465037973845e2, 5.437111746935729167e-15],
            },
            DDReal {
                x: [3.795040519246542203e2, 2.798823517255418949e-15],
            },
            DDReal {
                x: [6.156214999302825390e2, 5.560752947793534183e-14],
            },
            DDReal {
                x: [5.240040086910065611e2, -5.408249003075719577e-14],
            },
            DDReal {
                x: [2.041876620202371271e2, -8.319008907006881530e-15],
            },
            DDReal {
                x: [2.864561977272910909e1, -4.039035693090346268e-16],
            },
            DDReal {
                x: [8.950721014133898867e-1, -3.936403474544812065e-17],
            },
            DDReal {
                x: [1.841086331576126606e-3, -4.297802833254091927e-20],
            },
        ];
        const B: [DDReal; 12] = [
            DDReal {
                x: [1.000000000000000000e0, -1.708253638352812908e-28],
            },
            DDReal {
                x: [2.000000000000000000e0, 1.437253251097736723e-24],
            },
            DDReal {
                x: [3.000000000000000000e0, -1.219301298053216264e-21],
            },
            DDReal {
                x: [4.000000000000000000e0, 3.182289605630538873e-19],
            },
            DDReal {
                x: [5.000000000000000000e0, -3.967234305203110844e-17],
            },
            DDReal {
                x: [6.000000000000002665e0, 3.388956565694340752e-16],
            },
            DDReal {
                x: [6.999999999999837463e0, 6.139707495174003474e-17],
            },
            DDReal {
                x: [8.000000000007190692e0, 8.593944099626351521e-16],
            },
            DDReal {
                x: [8.999999999700067477e0, 7.094444794853377075e-16],
            },
            DDReal {
                x: [1.000000001420500517e1, 6.517927373688808522e-17],
            },
            DDReal {
                x: [1.099999895392011950e1, 1.760151667777773614e-16],
            },
            DDReal {
                x: [1.200023810893419451e1, -1.768963122274351943e-16],
            },
        ];
        let x = self;
        (x - 0.5) * (x + V).ln() - x
            + (((((((((((((A[12] / (x + B[11]) + A[11]) / (x + B[10]) + A[10])
                / (x + B[9])
                + A[9])
                / (x + B[8])
                + A[8])
                / (x + B[7])
                + A[7])
                / (x + B[6])
                + A[6])
                / (x + B[5])
                + A[5])
                / (x + B[4])
                + A[4])
                / (x + B[3])
                + A[3])
                / (x + B[2])
                + A[2])
                / (x + B[1])
                + A[1])
                / (x + B[0])
                + A[0])
                / x
                + AR)
                .ln()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Logit<T: SpecialFloat> {
    v: T,
}

impl<T: SpecialFloat> Logit<T> {
    pub fn _logit(logit: T) -> Logit<T> {
        Logit { v: logit }
    }
    pub fn _px(px: T) -> Logit<T> {
        Logit { v: px.enlogit() }
    }
    pub fn _nx(nx: T) -> Logit<T> {
        Logit { v: -nx.enlogit() }
    }
    pub fn _lnpx(lnpx: T) -> Logit<T> {
        Logit {
            v: lnpx.expenlogit(),
        }
    }
    pub fn _lnnx(lnnx: T) -> Logit<T> {
        Logit {
            v: -lnnx.expenlogit(),
        }
    }
    pub fn logit(&self) -> T {
        self.v
    }
    pub fn px(&self) -> T {
        self.v.delogit()
    }
    pub fn nx(&self) -> T {
        (-self.v).delogit()
    }
    pub fn lnpx(&self) -> T {
        self.v.logdelogit()
    }
    pub fn lnnx(&self) -> T {
        (-self.v).logdelogit()
    }
    pub fn elo(&self) -> f64 {
        (T::eloconst() * self.v).to_f64().unwrap()
    }
    pub fn negate(&self) -> Logit<T> {
        Logit { v: -self.v }
    }
}

pub fn elorating(win: f64, lose: f64) -> f64 {
    std::f64::consts::LOG10_E * 400.0 * (win / lose).ln()
}

pub fn elorating_inv(rdiff: f64) -> f64 {
    1.0 / (1.0 + (-std::f64::consts::LN_10 * 0.0025 * rdiff))
}

// http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf.html
// error function
fn erf_6(x: f64) -> f64 {
    // |x| <= 0.125
    // err = 8.5080e-19
    let x2 = x * x;
    (((((-8.492024351869184700200701e-4 * x2 + 5.223878776856181012778436e-3) * &x2
        - 2.686616984476423776951305e-2)
        * x2
        + 1.128379167066213010234749e-1)
        * x2
        - 3.761263890318336015429662e-1)
        * x2
        + 1.128379167095512573044943e+0)
        * x
}

// http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf.html
// complementary error function
fn erfc_8(x: f64) -> f64 {
    // -infinity < x < infinity
    // err = 3.63856888e-17
    let x2 = x * x;
    (if x < 6.103997330986881994334338e+0 {
        2.0 / ((1.269748999651156838985811e+1 * x).exp() + 1.0)
    } else {
        0.0
    }) + x
        * (-x2).exp()
        * (2.963168851992273778336357e-1 / (x2 + 6.121586444955387580549294e-2)
            + 1.815811251346370699640955e-1 / (x2 + 5.509427800560020848936831e-1)
            + 6.818664514249394930148282e-2 / (x2 + 1.530396620587703969527527e+0)
            + 1.569075431619667090378092e-2 / (x2 + 2.999579523113006340465739e+0)
            + 2.212901166815175728291522e-3 / (x2 + 4.958677771282467011450533e+0)
            + 1.913958130987428643791697e-4 / (x2 + 7.414712510993354068147575e+0)
            + 9.710132840105516234434841e-6 / (x2 + 1.047651043565452375901435e+1)
            + 1.666424471743077527468010e-7 / (x2 + 1.484555573455979566646185e+1))
}

// http://www.kurims.kyoto-u.ac.jp/~ooura/gamerf.html
// scaled complementary error function
fn erfcx_8(x: f64) -> f64 {
    // -infinity < x < infinity
    // err = 3.63856888e-17
    let x2 = x * x;
    (if x < 6.103997330986881994334338e+0 {
        2.0 / ((1.069748999651156838985811e+1 * x).exp() + (-x2).exp())
    } else {
        0.0
    }) + x
        * (2.963168851992273778336357e-1 / (x2 + 6.121586444955387580549294e-2)
            + 1.815811251346370699640955e-1 / (x2 + 5.509427800560020848936831e-1)
            + 6.818664514249394930148282e-2 / (x2 + 1.530396620587703969527527e+0)
            + 1.569075431619667090378092e-2 / (x2 + 2.999579523113006340465739e+0)
            + 2.212901166815175728291522e-3 / (x2 + 4.958677771282467011450533e+0)
            + 1.913958130987428643791697e-4 / (x2 + 7.414712510993354068147575e+0)
            + 9.710132840105516234434841e-6 / (x2 + 1.047651043565452375901435e+1)
            + 1.666424471743077527468010e-7 / (x2 + 1.484555573455979566646185e+1))
}

// complementary error function
// erfc(x) = 1 - erf(x)
pub fn erfc(x: f64) -> f64 {
    erfc_8(x)
}

// scaled complementary error function
// erfcx(x) * exp(-x * x) = erfc(x)
pub fn erfcx(x: f64) -> f64 {
    erfcx_8(x)
}

// error function
pub fn erf(x: f64) -> f64 {
    if x.abs() <= 0.125 {
        erf_6(x)
    } else if x < 0.0 {
        erfc(-x) - 1.0
    } else {
        1.0 - erfc(x)
    }
}

// https://github.com/jstat/jstat/
// Returns the inverse of the complementary error function
pub fn erfcinv(p: f64) -> f64 {
    if p >= 2.0 {
        return -100.0;
    }
    if p <= 0.0 {
        return 100.0;
    }
    let pp = if p < 1.0 { p } else { 2.0 - p };
    let t = (-2.0 * (pp * 0.5).ln()).sqrt();
    let mut x = -0.70711 * ((2.30753 + t * 0.27061) / (1.0 + t * (0.99229 + t * 0.04481)) - t);
    let err = erfc(x) - pp;
    x += err / (1.12837916709551257 * (-x * x).exp() - x * err);
    let err = erfc(x) - pp;
    x += err / (1.12837916709551257 * (-x * x).exp() - x * err);
    return if p < 1.0 { x } else { -x };
}

fn near_ipow2<T: SpecialFloat>(x: T) -> T {
    (-(x.log2().round())).exp2()
}

pub fn betacf<T: SpecialFloat>(x: T, a: T, b: T) -> T {
    let eps = T::epsilon();
    let qab = a + b;
    let qap = a + T::one();
    let qam = a - T::one();
    let mut m0: T = T::one();
    let mut m2: T = T::one();
    let mut m1: T = T::one();
    let mut m3: T = T::one() - x * qab / qap;
    for ni in 1..1024 {
        let n = T::from(ni as u64).unwrap();
        let n2 = n + n;
        let t1 = m1;
        let t3 = m3;
        let d0 = x * n * (b - n) / ((qam + n2) * (a + n2));
        if d0.is_zero() {
            break;
        }
        m0 = m0 * d0;
        m2 = m2 * d0;
        let d1 = T::one() - x * (a + n) * (qab + n) / ((a + n2) * (qap + n2));
        m1 = m0 + t1 * d1;
        m3 = m2 + t3 * d1;
        m0 = m0 + t1;
        m2 = m2 + t3;
        let tx = near_ipow2(m3);
        m0 = m0 * tx;
        m2 = m2 * tx;
        m1 = m1 * tx;
        m3 = m3 * tx;
        let r = m0 * m3;
        if (r - m1 * m2).abs() < (eps * r).abs() {
            break;
        }
    }
    m1 / m3
}

fn betaln<T: SpecialFloat>(a: T, b: T) -> T {
    a.gammaln() + b.gammaln() - (a + b).gammaln()
}

pub fn logitbetaln<T: SpecialFloat>(x: Logit<T>, a: T, b: T) -> T {
    if x.px() < (a + T::one()) / (a + b + (T::one() + T::one())) {
        if x.px().is_zero() {
            return T::neg_infinity();
        }
        // Factors in front of the continued fraction.
        let bt = a * x.lnpx() + b * x.lnnx() - betaln(a, b);
        // Use continued fraction directly.
        bt + (betacf(x.px(), a, b) / a).ln()
    } else {
        if x.nx().is_zero() {
            return T::zero();
        }
        // Factors in front of the continued fraction.
        let bt = a * x.lnpx() + b * x.lnnx() - betaln(a, b);
        // else use continued fraction after making the symmetry transformation.
        (bt + (betacf(x.nx(), b, a) / b).ln()).ln1mexp()
    }
}

// https://github.com/jstat/jstat/
// Returns the inverse of the incomplete beta function
pub fn ibetainvlogit<T: SpecialFloat>(p: T, a: T, b: T) -> Logit<T> {
    if p <= T::zero() {
        return Logit::_px(T::zero());
    }
    if p >= T::one() {
        return Logit::_px(T::one());
    }
    let eps = T::epsilon();
    let two = T::one() + T::one();
    let half = T::one() / two;
    let mut x: Logit<T>;
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();
    let p_f64 = p.to_f64().unwrap();
    if a_f64 >= 1.0 && b_f64 >= 1.0 {
        let pp = if p_f64 < 0.5 { p_f64 } else { 1.0 - p_f64 };
        let t = (-2.0 * pp.ln()).sqrt();
        let u = if p_f64 < 0.5 {
            t - (2.30753 + t * 0.27061) / (1.0 + t * (0.99229 + t * 0.04481))
        } else {
            (2.30753 + t * 0.27061) / (1.0 + t * (0.99229 + t * 0.04481)) - t
        };
        let al = (u * u - 3.0) / 6.0;
        let h = 2.0 / (1.0 / (2.0 * a_f64 - 1.0) + 1.0 / (2.0 * b_f64 - 1.0));
        let w = (u * (al + h).sqrt() / h)
            - (1.0 / (2.0 * b_f64 - 1.0) - 1.0 / (2.0 * a_f64 - 1.0))
                * (al + 5.0 / 6.0 - 2.0 / (3.0 * h));
        x = Logit::_px(a / (a + b * T::from(2.0 * w).unwrap().exp()));
    } else {
        let lna = (a_f64 / (a_f64 + b_f64)).ln();
        let lnb = (b_f64 / (a_f64 + b_f64)).ln();
        let t = (a_f64 * lna).exp() / a_f64;
        let u = (b_f64 * lnb).exp() / b_f64;
        let w = t + u;
        if p_f64 < t / w {
            x = Logit::_lnpx((a * T::from(w).unwrap() * p).ln() / a);
        } else {
            x = Logit::_lnnx((b * T::from(w).unwrap() * (T::one() - p)).ln() / b);
        }
    }
    let afac = betaln(a, b);
    let afacmln2 = afac + two.ln();
    // atanh(2*x-1) = (ln(x)-ln(1-x))/2
    let ph = half * (p.ln() - (-p).ln_1p());
    for j in 0..20 {
        if !x.logit().is_finite() || x.px().is_zero() || x.nx().is_zero() {
            break;
        }
        let lnbeta1 = logitbetaln(x, a, b);
        let lnbeta2 = logitbetaln(x.negate(), b, a);
        let err = half * (lnbeta1 - lnbeta2) - ph;
        let lngrad = x.lnpx() * a + x.lnnx() * b - afacmln2 - lnbeta1 - lnbeta2;
        let grad = lngrad.exp();
        let t = err / grad;
        // println!("j:{} x:{:?} lnbeta1:{:?} lnbeta2:{:?} err:{:?} grad:{:?} p:{:?} a:{:?} b:{:?}",j,x,lnbeta1,lnbeta2,err,grad,p,a,b);
        x = Logit::_logit(x.logit() - t);
        if t.abs() < eps * x.logit().abs() && j > 0 {
            break;
        }
    }
    x
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
