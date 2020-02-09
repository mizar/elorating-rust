// EloRating Calculator

// ln(1-exp(x))
#[allow(dead_code)]
fn ln1mexp(x: f64) -> f64 {
    if x >= 0.0 {
        std::f64::NAN
    } else if x > -std::f64::consts::LN_2 {
        (-(x.exp_m1())).ln()
    } else {
        (-(x.exp())).ln_1p()
    }
}

// ln(1+exp(x))
#[allow(dead_code)]
fn ln1pexp(x: f64) -> f64 {
    if x <= 0.0 {
        x.exp().ln_1p()
    } else {
        x + (-x).exp().ln_1p()
    }
}

fn enlogit(x: f64) -> f64 {
    if x <= 0.0 {
        return std::f64::NEG_INFINITY;
    }
    if x >= 1.0 {
        return std::f64::INFINITY;
    }
    x.ln() - (-x).ln_1p()
}
fn delogit(logit: f64) -> f64 {
    if logit >= 0.0 {
        let nexp = (-logit).exp();
        1.0 / (nexp + 1.0)
    } else {
        let pexp = logit.exp();
        pexp / (pexp + 1.0)
    }
}
fn logdelogit(logit: f64) -> f64 {
    if logit >= 0.0 {
        -((-logit).exp().ln_1p())
    } else {
        logit - logit.exp().ln_1p()
    }
}
fn expenlogit(x: f64) -> f64 {
    if x >= 0.0 {
        return std::f64::INFINITY;
    }
    x - ln1mexp(x)
}

#[derive(Debug, Clone, Copy)]
pub struct Logit { v: f64 }

impl Logit {
    fn _logit(logit: f64) -> Logit {
        Logit { v: logit }
    }
    fn _px(px: f64) -> Logit {
        Logit { v: enlogit(px) }
    }
    fn _nx(nx: f64) -> Logit {
        Logit { v: -enlogit(nx) }
    }
    fn _lnpx(lnpx: f64) -> Logit {
        Logit { v: expenlogit(lnpx) }
    }
    fn _lnnx(lnnx: f64) -> Logit {
        Logit { v: -expenlogit(lnnx) }
    }
    fn px(&self) -> f64 {
        delogit(self.v)
    }
    fn nx(&self) -> f64 {
        delogit(-self.v)
    }
    fn lnpx(&self) -> f64 {
        logdelogit(self.v)
    }
    fn lnnx(&self) -> f64 {
        logdelogit(-self.v)
    }
    fn elo(&self) -> f64 {
        std::f64::consts::LOG10_E * 400.0 * self.v
    }
    fn negate(&self) -> Logit {
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
// logarithm of the gamma function
fn gammaln_6(x: f64) -> f64 {
    // 0 < x < infinity
    // err = 2.09144255e-18
    (x - 0.5) * (x + 6.0975075753906857609437558e+0).ln() - x
        + ((((((1.1240582657165407383437999e-2 / (x + 5.0003589884831925541613237e+0)
            + 5.0219722703392090725884168e-1)
            / (x + 3.9999966300007508932097016e+0)
            + 2.0962955353894997733869983e+0)
            / (x + 3.0000000467265241458431618e+0)
            + 2.2502304753561816836695856e+0)
            / (x + 1.9999999996201023058065171e+0)
            + 8.5137081316503418312411656e-1)
            / (x + 1.0000000000006553243170562e+0)
            + 1.2242597732687991784645973e-1)
            / x
            + 5.6360656189756064967977564e-3)
            .ln()
}

// logarithm of the gamma function
pub fn gammaln(x: f64) -> f64 {
    // to correct gammaln(1.0) == gammaln(2.0) == 0.0
    gammaln_6(x) - 1.0 + 1.0
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

fn near_ipow2(x: f64) -> f64 {
    (-(x.log2().round())).exp2()
}

pub fn betacf(x: f64, a: f64, b: f64) -> f64 {
    let eps = 3e-16;
    let qab = a + b;
    let qap = a + 1.0;
    let qam = a - 1.0;
    let mut m0: f64 = 1.0;
    let mut m2: f64 = 1.0;
    let mut m1: f64 = 1.0;
    let mut m3: f64 = 1.0 - x * qab / qap;
    for ni in 1..1024 {
        let n = ni as f64;
        let n2 = n + n;
        let d0 = x * n * (b - n) / ((qam + n2) * (a + n2));
        let d1 = 1.0 - x * (a + n) * (qab + n) / ((a + n2) * (qap + n2));
        m0 *= d0;
        m2 *= d0;
        let t1 = m1;
        let t3 = m3;
        m1 = m0 + t1 * d1;
        m3 = m2 + t3 * d1;
        m0 += t1;
        m2 += t3;
        let tx = near_ipow2(m3);
        m0 *= tx;
        m2 *= tx;
        m1 *= tx;
        m3 *= tx;
        let r = m0 * m3;
        if d0 == 0.0 || (r - m1 * m2).abs() < (eps * r).abs() {
            /*
            println!(
                "x:{} a:{} b:{} n:{} m0:{} m1:{} m2:{} m3:{}",
                x, a, b, n, m0, m1, m2, m3
            );
            */
            break;
        }
    }
    m1 / m3
}

// https://github.com/jstat/jstat/
// Evaluates the continued fraction for incomplete beta function by modified
// Lentz's method.
pub fn betacf_jstat(x: f64, a: f64, b: f64) -> f64 {
    let fpmin = 1e-30;
    let eps = 3e-7;
    let qab = a + b;
    let qap = a + 1.0;
    let qam = a - 1.0;
    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;

    // These q's will be used in factors that occur in the coefficients
    if d.abs() < fpmin {
        d = fpmin;
    }
    d = 1.0 / d;
    let mut h = d;

    for mi in 1..101 {
        let m = mi as f64;
        let m2 = m + m;
        let aa = m * (b - m) * x / ((qam + m2) * (a + m2));
        // One step (the even one) of the recurrence
        d = 1.0 + aa * d;
        if d.abs() < fpmin {
            d = fpmin;
        }
        c = 1.0 + aa / c;
        if c.abs() < fpmin {
            c = fpmin;
        }
        d = 1.0 / d;
        h *= d * c;
        let aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2));
        // Next step of the recurrence (the odd one)
        d = 1.0 + aa * d;
        if d.abs() < fpmin {
            d = fpmin;
        }
        c = 1.0 + aa / c;
        if c.abs() < fpmin {
            c = fpmin;
        }
        d = 1.0 / d;
        let del = d * c;
        h *= del;
        if (del - 1.0).abs() < eps {
            break;
        }
    }
    h
}

fn betaln(a: f64, b: f64) -> f64 {
    gammaln(a) + gammaln(b) - gammaln(a + b)
}

// https://github.com/jstat/jstat/
// Returns the incomplete beta function I_x(a,b)
fn ibeta(x: f64, a: f64, b: f64) -> f64 {
    // Factors in front of the continued fraction.
    let bt = if x == 0.0 || x == 1.0 {
        0.0
    } else {
        (a * x.ln() + b * (1.0 - x).ln() - betaln(a, b)).exp()
    };
    if x < 0.0 || x > 1.0 {
        std::f64::NAN
    } else if x < (a + 1.0) / (a + b + 2.0) {
        // Use continued fraction directly.
        bt * betacf_jstat(x, a, b) / a
    } else {
        // else use continued fraction after making the symmetry transformation.
        1.0 - bt * betacf_jstat(1.0 - x, b, a) / b
    }
}

fn logitbetaln(x: &Logit, a: f64, b: f64) -> f64 {
    // Factors in front of the continued fraction.
    let bt = if x.px() == 0.0 || x.nx() == 0.0 {
        std::f64::NEG_INFINITY
    } else {
        a * x.lnpx() + b * x.lnnx() - betaln(a, b)
    };
    if x.px() < (a + 1.0) / (a + b + 2.0) {
        // Use continued fraction directly.
        bt + (betacf(x.px(), a, b) / a).ln()
    } else {
        // else use continued fraction after making the symmetry transformation.
        ln1mexp(bt + (betacf(x.nx(), b, a) / b).ln())
    }
}

// https://github.com/jstat/jstat/
// Returns the inverse of the incomplete beta function
pub fn ibetainvlogit(p: f64, a: f64, b: f64) -> Logit {
    if p <= 0.0 {
        return Logit::_px(0.0);
    }
    if p >= 1.0 {
        return Logit::_px(1.0);
    }
    let mut x: Logit;
    if a >= 1.0 && b >= 1.0 {
        let pp = if p < 0.5 { p } else { 1.0 - p };
        let t = (-2.0 * pp.ln()).sqrt();
        let u = if p < 0.5 {
            t - (2.30753 + t * 0.27061) / (1.0 + t * (0.99229 + t * 0.04481))
        } else {
            (2.30753 + t * 0.27061) / (1.0 + t * (0.99229 + t * 0.04481)) - t
        };
        let al = (u * u - 3.0) / 6.0;
        let h = 2.0 / (1.0 / (2.0 * a - 1.0) + 1.0 / (2.0 * b - 1.0));
        let w = (u * (al + h).sqrt() / h)
            - (1.0 / (2.0 * b - 1.0) - 1.0 / (2.0 * a - 1.0)) * (al + 5.0 / 6.0 - 2.0 / (3.0 * h));
        x = Logit::_px(a / (a + b * (2.0 * w).exp()));
    } else {
        let lna = (a / (a + b)).ln();
        let lnb = (b / (a + b)).ln();
        let t = (a * lna).exp() / a;
        let u = (b * lnb).exp() / b;
        let w = t + u;
        if p < t / w {
            x = Logit::_lnpx((a * w * p).ln() / a);
        } else {
            x = Logit::_lnnx((b * w * (1.0 - p)).ln() / b);
        }
    }
    let eps = 1e-14_f64;
    let afac = betaln(a, b);
    let afacmln2 = afac + std::f64::consts::LN_2;
    // atanh(2*x-1) = (ln(x)-ln(1-x))/2
    let ph = 0.5 * (p.ln() - (-p).ln_1p());
    for j in 0..20 {
        if !x.v.is_finite() || x.px() == 0.0 || x.nx() == 0.0 {
            break;
        }
        let lnbeta1 = logitbetaln(&x, a, b);
        let lnbeta2 = logitbetaln(&x.negate(), b, a);
        let err = 0.5 * (lnbeta1 - lnbeta2) - ph;
        let lngrad = x.lnpx() * a + x.lnnx() * b - afacmln2 - lnbeta1 - lnbeta2;
        let grad = lngrad.exp();
        let t = err / grad;
        //println!("j:{} x:{:?} lnbeta1:{} lnbeta2:{} err:{} grad:{} p:{} a:{} b:{}",j,x,lnbeta1,lnbeta2,err,grad,p,a,b);
        x = Logit::_logit(x.v - t);
        if t.abs() < eps * x.v.abs() && j > 0 {
            break;
        }
    }
    x
}

// https://github.com/jstat/jstat/
// Returns the inverse of the incomplete beta function
fn ibetainv(p: f64, a: f64, b: f64) -> f64 {
    let eps = 1e-8_f64;
    let a1 = a - 1.0;
    let b1 = b - 1.0;
    if p <= 0.0 {
        return 0.0;
    }
    if p >= 1.0 {
        return 1.0;
    }
    let mut x;
    if a >= 1.0 && b >= 1.0 {
        let pp = if p < 0.5 { p } else { 1.0 - p };
        let t = (-2.0 * pp.ln()).sqrt();
        x = (2.30753 + t * 0.27061) / (1.0 + t * (0.99229 + t * 0.04481)) - t;
        if p < 0.5 {
            x = -x;
        }
        let al = (x * x - 3.0) / 6.0;
        let h = 2.0 / (1.0 / (2.0 * a - 1.0) + 1.0 / (2.0 * b - 1.0));
        let w = (x * (al + h).sqrt() / h)
            - (1.0 / (2.0 * b - 1.0) - 1.0 / (2.0 * a - 1.0)) * (al + 5.0 / 6.0 - 2.0 / (3.0 * h));
        x = a / (a + b * (2.0 * w).exp());
    } else {
        let lna = (a / (a + b)).ln();
        let lnb = (b / (a + b)).ln();
        let t = (a * lna).exp() / a;
        let u = (b * lnb).exp() / b;
        let w = t + u;
        if p < t / w {
            x = (a * w * p).powf(1.0 / a);
        } else {
            x = 1.0 - (b * w * (1.0 - p)).powf(1.0 / b);
        }
    }
    let afac = -betaln(a, b);
    for j in 0..10 {
        if x == 0.0 || x == 1.0 {
            return x;
        }
        let err = ibeta(x, a, b) - p;
        let t = (a1 * x.ln() + b1 * (1.0 - x).ln() + afac).exp();
        let u = err / t;
        let t = u / (1.0 - 0.5 * f64::min(1.0, u * (a1 / x - b1 / (1.0 - x))));
        x -= t;
        if x <= 0.0 {
            x = 0.5 * (x + t);
        }
        if x >= 1.0 {
            x = 0.5 * (x + t + 1.0);
        }
        if t.abs() < eps * x && j > 0 {
            break;
        }
    }
    x
}

pub struct RatingRange {
    pub lower_w: f64,
    pub lower_l: f64,
    pub upper_w: f64,
    pub upper_l: f64,
    pub lowerrate: f64,
    pub upperrate: f64,
}

// https://github.com/tibigame/HandicappedRook/tree/master/tool
pub fn clooper_pearson(w: f64, l: f64, y: f64) -> RatingRange {
    let alpha_l = 0.5 * y;
    let l1 = l + 1.0;
    let w1 = w + 1.0;
    let (lower_w, lower_l, lowerrate): (f64, f64, f64) = {
        let lower_w = ibetainv(alpha_l, w, l1);
        (lower_w, 1.0 - lower_w, elorating(lower_w, 1.0 - lower_w))
    };
    let (upper_w, upper_l, upperrate): (f64, f64, f64) = {
        let upper_l = ibetainv(alpha_l, l, w1);
        (1.0 - upper_l, upper_l, elorating(1.0 - upper_l, upper_l))
    };
    RatingRange {
        lower_w: if lower_w.is_nan() { 0.0 } else { lower_w },
        lower_l: if lower_l.is_nan() { 1.0 } else { lower_l },
        upper_w: if upper_w.is_nan() { 1.0 } else { upper_w },
        upper_l: if upper_l.is_nan() { 0.0 } else { upper_l },
        lowerrate: if lowerrate.is_nan() {
            std::f64::NEG_INFINITY
        } else {
            lowerrate
        },
        upperrate: if upperrate.is_nan() {
            std::f64::INFINITY
        } else {
            upperrate
        },
    }
}

fn write_elorating_table(pvec: Vec<f64>, win: f64, draw: f64, lose: f64) {
    let games = win + draw + lose;
    let win_hdraw = win + 0.5 * draw;
    let lose_hdraw = lose + 0.5 * draw;

    println!("Win-Draw-Lose: {}-{}-{}", win, draw, lose);
    println!("Games: {}", games);
    println!("EloRating median: {:+.2}", elorating(win_hdraw, lose_hdraw));
    println!("WinRate: {:.2}%", win_hdraw / games * 100.0);
    println!("DrawRate: {:.2}%", draw / games * 100.0);

    println!();
    println!("   σ   (            %            ) |         EloRating estimated range         |  Δrange |  error_low  error_upp");
    println!("-----------------------------------+-------------------------------------------+---------+------------------------");

    for p in pvec {
        /*
        if y != 0.001 { continue; }
        let rating_range = clooper_pearson(win_hdraw, lose_hdraw, y);
        println!(
            "{:5.3}σ ({:10.7}%) | {:+21.14} ({:6.2}%) ~ {:+21.14} ({:6.2}%) | {:7.2}",
            erfcinv(y) * std::f64::consts::SQRT_2,
            100.0 * (1.0 - y),
            rating_range.lowerrate,
            rating_range.lower_w * 100.0,
            rating_range.upperrate,
            rating_range.upper_w * 100.0,
            rating_range.upperrate - rating_range.lowerrate,
        );
        */
        let lower = ibetainvlogit(p, win_hdraw, lose_hdraw + 1.0);
        let upper = ibetainvlogit(p, lose_hdraw, win_hdraw + 1.0).negate();
        println!(
            "{:5.3}σ ({:10.7}%, {:11.8}%) | {:+9.2} ({:6.2}%) ~ {:+9.2} ({:6.2}%) | {:7.2} | {:+10.3e} {:+10.3e}",
            erfcinv(p * 2.0) * std::f64::consts::SQRT_2,
            100.0 * (1.0 - p * 2.0),
            100.0 * p,
            lower.elo(),
            lower.px() * 100.0,
            upper.elo(),
            upper.px() * 100.0,
            upper.elo() - lower.elo(),
            logitbetaln(&lower, win_hdraw, lose_hdraw + 1.0).exp() - p,
            logitbetaln(&upper.negate(), lose_hdraw, win_hdraw + 1.0).exp() - p,
        );
    }
}

fn main() {
    let (win, draw, lose): (f64, f64, f64) = (1.0e14, 1.0, 0.0e13);

    let pvec: Vec<f64> = vec![
        0.375,
        // 0.382_924_922_548 ~= 0.5σ
        0.5 * erfc(0.5 * std::f64::consts::FRAC_1_SQRT_2),
        0.250,
        // 0.682_689_492_137 ~= 1.0σ
        0.5 * erfc(1.0 * std::f64::consts::FRAC_1_SQRT_2),
        0.125,
        0.100,
        // 0.866_385_597_462 ~= 1.5σ
        0.5 * erfc(1.5 * std::f64::consts::FRAC_1_SQRT_2),
        0.050,
        0.025,
        // 0.954_499_736_104 ~= 2.0σ
        0.5 * erfc(2.0 * std::f64::consts::FRAC_1_SQRT_2),
        0.010,
        // 0.987_580_669_348 ~= 2.5σ
        0.5 * erfc(2.5 * std::f64::consts::FRAC_1_SQRT_2),
        0.005,
        // 0.997_300_203_937 ~= 3.0σ
        0.5 * erfc(3.0 * std::f64::consts::FRAC_1_SQRT_2),
        0.001,
        0.000_500,
        // 0.999_534_741_842 ~= 3.5σ
        0.5 * erfc(3.5 * std::f64::consts::FRAC_1_SQRT_2),
        0.000_100,
        0.000_050,
        // 0.999_936_657_516 ~= 4.0σ
        0.5 * erfc(4.0 * std::f64::consts::FRAC_1_SQRT_2),
        0.000_010,
        0.000_005,
        // 0.999_993_204_654 ~= 4.5σ
        0.5 * erfc(4.5 * std::f64::consts::FRAC_1_SQRT_2),
        0.000_001,
        0.000_000_500,
        // 0.999_999_426_697 ~= 5.0σ
        0.5 * erfc(5.0 * std::f64::consts::FRAC_1_SQRT_2),
        0.000_000_100,
        0.000_000_050,
        // 0.999_999_962_021 ~= 5.5σ
        0.5 * erfc(5.5 * std::f64::consts::FRAC_1_SQRT_2),
        0.000_000_010,
        0.000_000_005,
        0.000_000_001,
        // 0.999_999_998_027 ~= 6.0σ
        0.5 * erfc(6.0 * std::f64::consts::FRAC_1_SQRT_2),
        0.000_000_000_500,
    ];

    write_elorating_table(pvec, win, draw, lose);
}
