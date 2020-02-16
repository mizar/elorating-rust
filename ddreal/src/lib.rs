extern crate num;
use num::{Float, FromPrimitive, Num, NumCast, One, ToPrimitive, Zero};

// Rust port: double-double arithmetic impl from DD/QD
// DD/QD: https://www.davidhbailey.com/dhbsoftware/

/*
set_display('none)$
fpprec:128$
obase:16$
[
round(bfloat(%e)*2b0^288b0),
round(1b0/bfloat(%pi)*2b0^288b0),
round(1b0/sqrt(2b0)*2b0^288b0),
round(1b0/2b0/bfloat(%pi)*2b0^288b0),
round(180b0/bfloat(%pi)*2b0^288b0),
round(2b0/bfloat(%pi)*2b0^288b0),
round(2b0/sqrt(bfloat(%pi))*2b0^288b0),
round(bfloat(%pi)/180b0*2b0^288b0),
round(bfloat(%pi)/2b0*2b0^288b0),
round(bfloat(%pi)/3b0*2b0^288b0),
round(bfloat(%pi)/4b0*2b0^288b0),
round(bfloat(%pi)/6b0*2b0^288b0),
round(bfloat(%pi)/8b0*2b0^288b0),
round(bfloat(%pi)/16b0*2b0^288b0),
round(3b0*bfloat(%pi)/4b0*2b0^288b0),
round(log(2b0)*2b0^288b0),
round(log(10b0)*2b0^288b0),
round(log(2b0)/log(10b0)*2b0^288b0),
round(1b0/log(10b0)*2b0^288b0),
round(log(10b0)/log(2b0)*2b0^288b0),
round(1b0/log(2b0)*2b0^288b0),
round(sqrt(2b0)*2b0^288b0),
round(bfloat(%pi)*2b0^288b0),
round(2b0*bfloat(%pi)*2b0^288b0),
0]
*/
pub mod hexstrs {
    pub const E: &str =
        "0x02.b7e1_5162_8aed_2a6a_bf71_5880_9cf4_f3c7_62e7_160f_38b4_da56_a784_d904_5190_cfef_324e_7739_p0";
    pub const FRAC_1_PI: &str =
        "0x00.517c_c1b7_2722_0a94_fe13_abe8_fa9a_6ee0_6db1_4acc_9e21_c820_ff28_b1d5_ef5d_e2b0_db92_371d_p0";
    pub const FRAC_1_SQRT_2: &str =
        "0x00.b504_f333_f9de_6484_597d_89b3_754a_be9f_1d6f_60ba_893b_a84c_ed17_ac85_8333_9915_4afc_8304_p0";
    pub const FRAC_1_TAU: &str =
        "0x00.28be_60db_9391_054a_7f09_d5f4_7d4d_3770_36d8_a566_4f10_e410_7f94_58ea_f7ae_f158_6dc9_1b8f_p0";
    pub const FRAC_180_PI: &str =
        "0x39.4bb8_34c7_83ef_70c2_a5d4_dfd0_3495_f5cd_20a8_97df_2fc0_b733_689d_0a6c_4e03_645a_62ce_c07b_p0";
    pub const FRAC_2_PI: &str =
        "0x00.a2f9_836e_4e44_1529_fc27_57d1_f534_ddc0_db62_9599_3c43_9041_fe51_63ab_debb_c561_b724_6e3a_p0";
    pub const FRAC_2_SQRT_PI: &str =
        "0x01.20dd_7504_29b6_d11a_e3a9_14fe_d7fd_8688_2813_41d7_587c_ea2e_7342_b061_99cc_4161_80eb_39f1_p0";
    pub const FRAC_PI_16: &str =
        "0x00.3243_f6a8_885a_308d_3131_98a2_e037_0734_4a40_9382_2299_f31d_0082_efa9_8ec4_e6c8_9452_821e_p0";
    pub const FRAC_PI_180: &str =
        "0x00.0477_d1a8_94a7_4e45_7076_2fb3_74a4_2e26_c805_bd77_a80d_af35_c728_154d_a64a_6428_95b7_b08b_p0";
    pub const FRAC_PI_2: &str =
        "0x01.921f_b544_42d1_8469_898c_c517_01b8_39a2_5204_9c11_14cf_98e8_0417_7d4c_7627_3644_a294_10f3_p0";
    pub const FRAC_PI_3: &str =
        "0x01.0c15_2382_d736_5846_5bb3_2e0f_567a_d116_e158_680b_6335_109a_ad64_fe32_f96f_7983_170d_60a2_p0";
    pub const FRAC_PI_4: &str =
        "0x00.c90f_daa2_2168_c234_c4c6_628b_80dc_1cd1_2902_4e08_8a67_cc74_020b_bea6_3b13_9b22_514a_087a_p0";
    pub const FRAC_PI_6: &str =
        "0x00.860a_91c1_6b9b_2c23_2dd9_9707_ab3d_688b_70ac_3405_b19a_884d_56b2_7f19_7cb7_bcc1_8b86_b051_p0";
    pub const FRAC_PI_8: &str =
        "0x00.6487_ed51_10b4_611a_6263_3145_c06e_0e68_9481_2704_4533_e63a_0105_df53_1d89_cd91_28a5_043d_p0";
    pub const FRAC_3PI_4: &str =
        "0x02.5b2f_8fe6_643a_469e_4e53_27a2_8294_5673_7b06_ea19_9f37_655c_0623_3bf2_b13a_d166_f3de_196d_p0";
    pub const LN_2: &str =
        "0x00.b172_17f7_d1cf_79ab_c9e3_b398_03f2_f6af_40f3_4326_7298_b62d_8a0d_175b_8baa_fa2b_e7b8_7620_p0";
    pub const LN_10: &str =
        "0x02.4d76_3776_aaa2_b05b_a95b_58ae_0b4c_28a3_8a3f_b3e7_6977_e43a_0f18_7a08_07c0_b5ca_58bc_0b5f_p0";
    pub const LOG10_2: &str =
        "0x00.4d10_4d42_7de7_fbcc_47c4_acd6_05be_48bc_1356_9862_a1e8_f9a4_c52f_3793_5be6_31e5_9435_16c1_p0";
    pub const LOG10_E: &str =
        "0x00.6f2d_ec54_9b94_38ca_9aad_d557_d699_ee19_1f71_a301_22e4_d101_1d1f_96a2_7bc7_529e_3aa1_277d_p0";
    pub const LOG2_10: &str =
        "0x03.5269_e12f_346e_2bf9_24af_dbfd_36bf_6d33_65b1_57f8_dece_b53a_46da_b202_0b9e_1674_1994_3f7a_p0";
    pub const LOG2_E: &str =
        "0x01.7154_7652_b82f_e177_7d0f_fda0_d23a_7d11_d6ae_f551_bad2_b4b1_164a_2cd9_a342_648f_bc38_87ef_p0";
    pub const PI: &str =
        "0x03.243f_6a88_85a3_08d3_1319_8a2e_0370_7344_a409_3822_299f_31d0_082e_fa98_ec4e_6c89_4528_21e6_p0";
    pub const SQRT_2: &str =
        "0x01.6a09_e667_f3bc_c908_b2fb_1366_ea95_7d3e_3ade_c175_1277_5099_da2f_590b_0667_322a_95f9_0608_p0";
    pub const TAU: &str =
        "0x06.487e_d511_0b46_11a6_2633_145c_06e0_e689_4812_7044_533e_63a0_105d_f531_d89c_d912_8a50_43cc_p0";
}
pub mod ddconsts {
    use crate::DDReal;
    pub const E: DDReal = DDReal {
        x: [2.718_281_828_459_045_091e0, 1.445_646_891_729_250_158e-16],
    };
    pub const FRAC_1_PI: DDReal = DDReal {
        x: [3.183_098_861_837_906_912e-1, -1.967_867_667_518_248_588e-17],
    };
    pub const FRAC_1_SQRT_2: DDReal = DDReal {
        x: [7.071_067_811_865_475_727e-1, -4.833_646_656_726_456_726e-17],
    };
    pub const FRAC_1_TAU: DDReal = DDReal {
        x: [1.591_549_430_918_953_456e-1, -9.839_338_337_591_242_941e-18],
    };
    pub const FRAC_180_PI: DDReal = DDReal {
        x: [5.729_577_951_308_232_286e1, -1.987_849_567_057_628_721e-15],
    };
    pub const FRAC_2_PI: DDReal = DDReal {
        x: [6.366_197_723_675_813_824e-1, -3.935_735_335_036_497_176e-17],
    };
    pub const FRAC_2_SQRT_PI: DDReal = DDReal {
        x: [1.128_379_167_095_512_559e0, 1.533_545_961_316_588_122e-17],
    };
    pub const FRAC_PI_16: DDReal = DDReal {
        x: [1.963_495_408_493_620_697e-1, 7.654_042_494_670_957_545e-18],
    };
    pub const FRAC_PI_180: DDReal = DDReal {
        x: [1.745_329_251_994_329_547e-2, 2.948_652_270_870_168_205e-19],
    };
    pub const FRAC_PI_2: DDReal = DDReal {
        x: [1.570_796_326_794_896_558e0, 6.123_233_995_736_766_036e-17],
    };
    pub const FRAC_PI_3: DDReal = DDReal {
        x: [1.047_197_551_196_597_853e0, -1.072_081_766_451_090_985e-16],
    };
    pub const FRAC_PI_4: DDReal = DDReal {
        x: [7.853_981_633_974_482_790e-1, 3.061_616_997_868_382_402e-17],
    };
    pub const FRAC_PI_6: DDReal = DDReal {
        x: [5.235_987_755_982_989_267e-1, -5.360_408_832_255_454_924e-17],
    };
    pub const FRAC_PI_8: DDReal = DDReal {
        x: [3.926_990_816_987_241_395e-1, 1.530_808_498_934_191_509e-17],
    };
    pub const FRAC_3PI_4: DDReal = DDReal {
        x: [2.356_194_490_192_344_837e0, 9.184_850_993_605_148_438e-17],
    };
    pub const LN_2: DDReal = DDReal {
        x: [6.931_471_805_599_452_862e-1, 2.319_046_813_846_299_558e-17],
    };
    pub const LN_10: DDReal = DDReal {
        x: [2.302_585_092_994_045_901e0, -2.170_756_223_382_249_597e-16],
    };
    pub const LOG10_2: DDReal = DDReal {
        x: [3.010_299_956_639_811_980e-1, -2.803_728_127_785_170_394e-18],
    };
    pub const LOG10_E: DDReal = DDReal {
        x: [4.342_944_819_032_518_167e-1, 1.098_319_650_216_765_073e-17],
    };
    pub const LOG2_10: DDReal = DDReal {
        x: [3.321_928_094_887_362_182e0, 1.661_617_516_973_592_006e-16],
    };
    pub const LOG2_E: DDReal = DDReal {
        x: [1.442_695_040_888_963_387e0, 2.035_527_374_093_103_311e-17],
    };
    pub const PI: DDReal = DDReal {
        x: [3.141_592_653_589_793_116e0, 1.224_646_799_147_353_207e-16],
    };
    pub const SQRT_2: DDReal = DDReal {
        x: [1.414_213_562_373_095_145e0, -9.667_293_313_452_913_451e-17],
    };
    pub const TAU: DDReal = DDReal {
        x: [6.283_185_307_179_586_232e0, 2.449_293_598_294_706_414e-16],
    };
    pub const MAX: DDReal = DDReal {
        x: [
            1.797_693_134_862_315_708_15e+308,
            9.979_201_547_673_597_950_37e+291,
        ],
    };
    pub const SAFE_MAX: DDReal = DDReal {
        x: [
            1.797_693_108_074_600_728_1e+308,
            9.979_201_547_673_597_950_37e+291,
        ],
    };
    pub const EPS: f64 = 4.930_380_657_631_32e-32; // 2^-104
    pub const MIN_NORMALIZED: f64 = 2.004_168_360_008_972_8e-292; // = 2^(-1022 + 53)
    pub const NDIGITS: usize = 31;
}

const N_INV_FACT: usize = 15;
const INV_FACT: [DDReal; N_INV_FACT] = [
    DDReal {
        x: [1.666_666_666_666_666_57e-01, 9.251_858_538_542_970_66e-18],
    },
    DDReal {
        x: [4.166_666_666_666_666_44e-02, 2.312_964_634_635_742_66e-18],
    },
    DDReal {
        x: [8.333_333_333_333_333_22e-03, 1.156_482_317_317_871_38e-19],
    },
    DDReal {
        x: [1.388_888_888_888_888_94e-03, -5.300_543_954_373_577_06e-20],
    },
    DDReal {
        x: [1.984_126_984_126_984_13e-04, 1.720_955_829_342_070_53e-22],
    },
    DDReal {
        x: [2.480_158_730_158_730_16e-05, 2.151_194_786_677_588_16e-23],
    },
    DDReal {
        x: [2.755_731_922_398_589_25e-06, -1.858_393_274_046_472_08e-22],
    },
    DDReal {
        x: [2.755_731_922_398_588_83e-07, 2.376_771_462_225_029_73e-23],
    },
    DDReal {
        x: [2.505_210_838_544_172_02e-08, -1.448_814_070_935_911_97e-24],
    },
    DDReal {
        x: [2.087_675_698_786_810_02e-09, -1.207_345_059_113_259_97e-25],
    },
    DDReal {
        x: [1.605_904_383_682_161_33e-10, 1.258_529_458_875_209_81e-26],
    },
    DDReal {
        x: [1.147_074_559_772_972_45e-11, 2.065_551_275_283_074_54e-28],
    },
    DDReal {
        x: [7.647_163_731_819_816_41e-13, 7.038_728_777_334_530_01e-30],
    },
    DDReal {
        x: [4.779_477_332_387_385_25e-14, 4.399_205_485_834_081_26e-31],
    },
    DDReal {
        x: [2.811_457_254_345_520_60e-15, 1.650_884_273_086_143_26e-31],
    },
];

/* Table of sin(k * pi/16) and cos(k * pi/16). */
const SIN_TABLE: [DDReal; 5] = [
    DDReal { x: [0.0, 0.0] },
    DDReal {
        x: [
            1.950_903_220_161_282_758e-01,
            -7.991_079_068_461_731_263e-18,
        ],
    },
    DDReal {
        x: [
            3.826_834_323_650_897_818e-01,
            -1.005_077_269_646_158_761e-17,
        ],
    },
    DDReal {
        x: [5.555_702_330_196_021_776e-01, 4.709_410_940_561_676_821e-17],
    },
    DDReal {
        x: [
            7.071_067_811_865_475_727e-01,
            -4.833_646_656_726_456_726e-17,
        ],
    },
];
const COS_TABLE: [DDReal; 5] = [
    DDReal { x: [1.0, 0.0] },
    DDReal {
        x: [9.807_852_804_032_304_306e-01, 1.854_693_999_782_500_573e-17],
    },
    DDReal {
        x: [9.238_795_325_112_867_385e-01, 1.764_504_708_433_667_706e-17],
    },
    DDReal {
        x: [8.314_696_123_025_452_357e-01, 1.407_385_698_472_802_389e-18],
    },
    DDReal {
        x: [
            7.071_067_811_865_475_727e-01,
            -4.833_646_656_726_456_726e-17,
        ],
    },
];

#[derive(Debug, Clone, Copy)]
pub struct DDReal {
    pub x: [f64; 2],
}

impl DDReal {
    #[allow(dead_code)]
    pub fn new(hi: f64, lo: f64) -> Self {
        Self { x: [hi, lo] }
    }
    pub fn hi(&self) -> f64 {
        self.x[0]
    }
    pub fn lo(&self) -> f64 {
        self.x[1]
    }
}

#[inline]
#[allow(dead_code)]
fn quick_two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let err = b - (s - a);
    (s, err)
}

#[inline]
#[allow(dead_code)]
fn quick_two_diff(a: f64, b: f64) -> (f64, f64) {
    let s = a - b;
    let err = (a - s) - a;
    (s, err)
}

#[inline]
#[allow(dead_code)]
fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let bb = s - a;
    let err = (a - (s - bb)) + (b - bb);
    (s, err)
}

#[inline]
#[allow(dead_code)]
fn two_diff(a: f64, b: f64) -> (f64, f64) {
    let s = a - b;
    let bb = s - a;
    let err = (a - (s - bb)) - (b + bb);
    (s, err)
}

#[inline]
#[allow(dead_code)]
fn split(a: f64) -> (f64, f64) {
    let _qd_split_thresh = 6.696_928_794_914_17e+299; // 2^996
    let _qd_splitter = 134_217_729.0; // 2^27+1
    if a > _qd_split_thresh || a < -_qd_split_thresh {
        let _a = a * 3.725_290_298_461_914_062_5e-09; // 2^-28
        let temp = _qd_splitter * _a;
        let hi = temp - (temp - _a);
        let lo = _a - hi;
        let hi = hi * 268_435_456.0; // 2^28
        let lo = lo * 268_435_456.0; // 2^28
        return (hi, lo);
    } else {
        let temp = _qd_splitter * a;
        let hi = temp - (temp - a);
        let lo = a - hi;
        return (hi, lo);
    }
}

#[inline]
#[allow(dead_code)]
fn two_prod(a: f64, b: f64) -> (f64, f64) {
    let p = a * b;
    let err = a.mul_add(b, -p);
    (p, err)
}

#[inline]
#[allow(dead_code)]
fn two_sqr(a: f64) -> (f64, f64) {
    let p = a * a;
    let err = a.mul_add(a, -p);
    (p, err)
}

#[inline]
#[allow(dead_code)]
fn nint_f64(d: f64) -> f64 {
    if d == d.floor() {
        d
    } else {
        (d + 0.5).floor()
    }
}

impl DDReal {
    #[inline]
    fn nint(self) -> Self {
        let hi = nint_f64(self.x[0]);

        if hi == self.x[0] {
            let lo = nint_f64(self.x[1]);

            let (hi, lo) = quick_two_sum(hi, lo);
            return Self { x: [hi, lo] };
        } else {
            if (hi - self.x[0]).abs() == 0.5 && self.x[1] < 0.0 {
                return Self { x: [hi - 1.0, 0.0] };
            }
            return Self { x: [hi, 0.0] };
        }
    }
}

#[inline]
#[allow(dead_code)]
fn aint(d: f64) -> f64 {
    if d >= 0.0 {
        d.floor()
    } else {
        d.ceil()
    }
}

impl DDReal {
    /*
    Computes sin(a) using Taylor series.
    Assumes |a| <= pi/32.
    */
    fn sin_taylor(&self) -> Self {
        if self.is_zero() {
            return Self::zero();
        }

        let thresh: f64 = 0.5 * self.x[0].abs() * crate::ddconsts::EPS;
        let x = -self.sqr();
        let mut i = 0;
        let mut s = *self;
        let mut r = *self;
        loop {
            r = r * x;
            let t = r * INV_FACT[i];
            s = s + t;
            i += 2;
            if !(i < N_INV_FACT && t.x[0].abs() > thresh) {
                break;
            }
        }

        s
    }
    fn cos_taylor(&self) -> Self {
        if self.is_zero() {
            return Self::one();
        }

        let thresh: f64 = 0.5 * crate::ddconsts::EPS;
        let x = -self.sqr();
        let mut i = 1;
        let mut s = self.mul_pwr2(0.5) + 1.0;
        let mut r = self.clone();
        loop {
            r = r * x;
            let t = r * INV_FACT[i];
            s = s + t;
            i += 2;
            if !(i < N_INV_FACT && t.x[0].abs() > thresh) {
                break;
            }
        }

        s
    }
    fn sincos_taylor(&self) -> (Self, Self) {
        if self.is_zero() {
            (Self::zero(), Self::one())
        } else {
            let sin_a = Self::sin_taylor(self);
            (sin_a, (1.0 - sin_a.sqr()).sqrt())
        }
    }
}

impl Zero for DDReal {
    fn zero() -> Self {
        Self { x: [0.0, 0.0] }
    }
    fn is_zero(&self) -> bool {
        self.x[0] == 0.0
    }
    fn set_zero(&mut self) {
        self.x[0] = 0.0;
        self.x[1] = 0.0;
    }
}

impl One for DDReal {
    fn one() -> Self {
        Self { x: [1.0, 0.0] }
    }
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        self.x[0] == 1.0 && self.x[1] == 0.0
    }
    fn set_one(&mut self) {
        self.x[0] = 1.0;
        self.x[1] = 0.0;
    }
}

impl std::ops::Add<f64> for DDReal {
    type Output = Self;
    fn add(self, other: f64) -> Self::Output {
        let (s1, s2) = two_sum(self.x[0], other);
        let s2 = s2 + self.x[1];
        let (s1, s2) = quick_two_sum(s1, s2);
        Self { x: [s1, s2] }
    }
}

impl std::ops::Sub<f64> for DDReal {
    type Output = Self;
    fn sub(self, other: f64) -> Self::Output {
        let (s1, s2) = two_diff(self.x[0], other);
        let s2 = s2 + self.x[1];
        let (s1, s2) = quick_two_sum(s1, s2);
        Self { x: [s1, s2] }
    }
}

impl DDReal {
    #[allow(dead_code)]
    fn add_f64(a: f64, b: f64) -> Self {
        let (s1, s2) = two_sum(a, b);
        Self { x: [s1, s2] }
    }
    #[allow(dead_code)]
    fn sub_f64(a: f64, b: f64) -> Self {
        let (s1, s2) = two_diff(a, b);
        Self { x: [s1, s2] }
    }
    fn ieee_add(a: Self, b: Self) -> Self {
        let (s1, s2) = two_sum(a.x[0], b.x[0]);
        let (t1, t2) = two_sum(a.x[1], b.x[1]);
        let s2 = s2 + t1;
        let (s1, s2) = quick_two_sum(s1, s2);
        let s2 = s2 + t2;
        let (s1, s2) = quick_two_sum(s1, s2);
        Self { x: [s1, s2] }
    }
    fn ieee_sub(a: Self, b: Self) -> Self {
        let (s1, s2) = two_diff(a.x[0], b.x[0]);
        let (t1, t2) = two_diff(a.x[1], b.x[1]);
        let s2 = s2 + t1;
        let (s1, s2) = quick_two_sum(s1, s2);
        let s2 = s2 + t2;
        let (s1, s2) = quick_two_sum(s1, s2);
        Self { x: [s1, s2] }
    }
    #[allow(dead_code)]
    fn sloppy_add(a: Self, b: Self) -> Self {
        let (s, e) = two_sum(a.x[0], b.x[0]);
        let e = e + (a.x[1] + b.x[1]);
        let (s, e) = quick_two_sum(s, e);
        Self { x: [s, e] }
    }
    #[allow(dead_code)]
    fn sloppy_sub(a: Self, b: Self) -> Self {
        let (s, e) = two_diff(a.x[0], b.x[0]);
        let e = (e + a.x[1]) - b.x[1];
        let (s, e) = quick_two_sum(s, e);
        Self { x: [s, e] }
    }
}

impl std::ops::Add for DDReal {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::ieee_add(self, other)
    }
}

impl std::ops::Add<DDReal> for f64 {
    type Output = DDReal;
    fn add(self, other: DDReal) -> DDReal {
        other + self
    }
}

impl std::ops::Sub for DDReal {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::ieee_sub(self, other)
    }
}

impl std::ops::Sub<DDReal> for f64 {
    type Output = DDReal;
    fn sub(self, other: DDReal) -> Self::Output {
        self + (-other)
    }
}

impl DDReal {
    #[allow(dead_code)]
    fn mul_f64(a: f64, b: f64) -> Self {
        let (p, e) = two_prod(a, b);
        Self { x: [p, e] }
    }
    #[allow(dead_code)]
    fn ldexp(self, exp: i32) -> Self {
        Self {
            x: [
                self.x[0] * (exp as f64).exp2(),
                self.x[1] * (exp as f64).exp2(),
            ],
        }
    }
    // must b is a power of 2.
    #[allow(dead_code)]
    fn mul_pwr2(self, b: f64) -> Self {
        Self {
            x: [self.x[0] * b, self.x[1] * b],
        }
    }
}

impl std::ops::Mul<f64> for DDReal {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        let (p1, p2) = two_prod(self.x[0], other);
        let p2 = p2 + (self.x[1] * other);
        let (p1, p2) = quick_two_sum(p1, p2);
        Self { x: [p1, p2] }
    }
}

impl std::ops::Mul for DDReal {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let (p1, p2) = two_prod(self.x[0], other.x[0]);
        let p2 = p2 + (self.x[0] * other.x[1] + self.x[1] * other.x[0]);
        let (p1, p2) = quick_two_sum(p1, p2);
        Self { x: [p1, p2] }
    }
}

impl std::ops::Mul<DDReal> for f64 {
    type Output = DDReal;
    fn mul(self, other: DDReal) -> Self::Output {
        other * self
    }
}

impl DDReal {
    #[allow(dead_code)]
    fn div_f64(a: f64, b: f64) -> DDReal {
        let q1 = a / b;

        // Compute a - q1 * b
        let (p1, p2) = two_prod(q1, b);
        let (s, e) = two_diff(a, p1);
        let e = e - p2;

        // get next approximation
        let q2 = (s + e) / b;

        let (s, e) = quick_two_sum(q1, q2);

        Self { x: [s, e] }
    }
    #[allow(dead_code)]
    fn sloppy_div(a: Self, b: Self) -> Self {
        // approximate quotient
        let q1 = a.x[0] / b.x[0];

        // compute  this - q1 * dd
        let r = b * q1;
        let (s1, s2) = two_diff(a.x[0], r.x[0]);
        let s2 = (s2 - r.x[1]) + a.x[1];

        // get next approximation
        let q2 = (s1 + s2) / b.x[0];

        // renormalize
        let (r1, r2) = quick_two_sum(q1, q2);

        Self { x: [r1, r2] }
    }
    #[allow(dead_code)]
    fn accurate_div(a: Self, b: Self) -> Self {
        //approximate quotient
        let q1 = a.x[0] / b.x[0];

        let r = a - q1 * b;

        let q2 = r.x[0] / b.x[0];
        let r = r - (q2 * b);

        let q3 = r.x[0] / b.x[0];

        let (q1, q2) = quick_two_sum(q1, q2);
        DDReal::new(q1, q2) + q3
    }
    #[allow(dead_code)]
    fn fast_div(a: Self, b: Self) -> Self {
        let yr = 1.0 / b.x[0];
        let zh = a.x[0] * yr;
        let (th, tl) = two_prod(zh, b.x[0]);
        let zl = ((a.x[0] - th) - tl) * yr + zh * (a.x[1] / a.x[0] - b.x[1] * yr);
        let (zh, zl) = quick_two_sum(zh, zl);
        Self { x: [zh, zl] }
    }
    #[allow(dead_code)]
    fn inv_f64(a: Self) -> Self {
        1.0 / a
    }
}

impl std::ops::Div<f64> for DDReal {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        // approximate quotient.
        let q1 = self.x[0] / other;

        // Compute  this - q1 * d
        let (p1, p2) = two_prod(q1, other);
        let (s, e) = two_diff(self.x[0], p1);
        let e = e + self.x[1] - p2;

        // get next approximation.
        let q2 = (s + e) / other;

        // renormalize
        let (r1, r2) = quick_two_sum(q1, q2);

        Self { x: [r1, r2] }
    }
}

impl std::ops::Div for DDReal {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self::accurate_div(self, other)
    }
}

impl std::ops::Div<DDReal> for f64 {
    type Output = DDReal;
    fn div(self, other: DDReal) -> Self::Output {
        DDReal::new(self, 0.0) / other
    }
}

impl std::ops::Rem<f64> for DDReal {
    type Output = Self;
    fn rem(self, other: f64) -> Self::Output {
        self / DDReal::new(other, 0.0)
    }
}

impl std::ops::Rem for DDReal {
    type Output = Self;
    fn rem(self, other: Self) -> Self::Output {
        let n = Self::nint(self / other);
        self - n * other
    }
}

impl DDReal {
    #[allow(dead_code)]
    fn divrem(a: Self, b: Self) -> (Self, Self) {
        let n = Self::nint(a / b);
        (n, a - n * b)
    }
}

impl DDReal {
    #[allow(dead_code)]
    fn sqr(&self) -> Self {
        let (p1, p2) = two_sqr(self.x[0]);
        let p2 = p2 + 2.0 * self.x[0] * self.x[1];
        let p2 = p2 + self.x[1] * self.x[1];
        let (s1, s2) = quick_two_sum(p1, p2);
        Self { x: [s1, s2] }
    }
    #[allow(dead_code)]
    fn sqr_f64(a: f64) -> Self {
        let (p1, p2) = two_sqr(a);
        Self { x: [p1, p2] }
    }
}

impl std::ops::Neg for DDReal {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: [-self.x[0], -self.x[1]],
        }
    }
}

impl From<f64> for DDReal {
    fn from(n: f64) -> Self {
        Self { x: [n, 0.0] }
    }
}

#[cfg(has_i128)]
impl From<i128> for DDReal {
    fn from(n: i128) -> Self {
        Self::add_f64(
            (n & -0x1_0000_0000_0000_0000_0000_0000_i128) as f64,
            (n & 0xffff_ffff_ffff_0000_0000_0000_i128) as f64,
        ) + ((n & 0xffff_ffff_ffff_i128) as f64)
    }
}
impl From<i64> for DDReal {
    fn from(n: i64) -> Self {
        Self::add_f64(
            (n & -0x1_0000_0000_i64) as f64,
            (n & 0xffff_ffff_i64) as f64,
        )
    }
}
#[cfg(has_i128)]
impl From<u128> for DDReal {
    fn from(n: u128) -> Self {
        Self::add_f64(
            (n & 0xffff_ffff_0000_0000_0000_0000_0000_0000_u128) as f64,
            (n & 0xffff_ffff_ffff_0000_0000_0000_u128) as f64,
        ) + ((n & 0xffff_ffff_ffff_u128) as f64)
    }
}
impl From<u64> for DDReal {
    fn from(n: u64) -> Self {
        Self::add_f64(
            (n & 0xffff_ffff_0000_0000_u64) as f64,
            (n & 0xffff_ffff_u64) as f64,
        )
    }
}

impl From<f32> for DDReal {
    fn from(n: f32) -> Self {
        (n as f64).into()
    }
}
impl From<isize> for DDReal {
    fn from(n: isize) -> Self {
        (n as i64).into()
    }
}
impl From<i8> for DDReal {
    fn from(n: i8) -> Self {
        (n as i64).into()
    }
}
impl From<i16> for DDReal {
    fn from(n: i16) -> Self {
        (n as i64).into()
    }
}
impl From<i32> for DDReal {
    fn from(n: i32) -> Self {
        (n as i64).into()
    }
}
impl From<usize> for DDReal {
    fn from(n: usize) -> Self {
        (n as u64).into()
    }
}
impl From<u8> for DDReal {
    fn from(n: u8) -> Self {
        (n as u64).into()
    }
}
impl From<u16> for DDReal {
    fn from(n: u16) -> Self {
        (n as u64).into()
    }
}
impl From<u32> for DDReal {
    fn from(n: u32) -> Self {
        (n as u64).into()
    }
}

impl ToPrimitive for DDReal {
    fn to_f32(&self) -> Option<f32> {
        self.x[0].to_f32()
    }
    fn to_f64(&self) -> Option<f64> {
        self.x[0].to_f64()
    }
    fn to_i128(&self) -> Option<i128> {
        Some(self.x[0].to_i128()? + self.x[1].to_i128()?)
    }
    fn to_i64(&self) -> Option<i64> {
        Some(self.x[0].to_i64()? + self.x[1].to_i64()?)
    }
    fn to_u128(&self) -> Option<u128> {
        Some(self.x[0].to_u128()? + self.x[1].to_u128()?)
    }
    fn to_u64(&self) -> Option<u64> {
        Some(self.x[0].to_u64()? + self.x[1].to_u64()?)
    }
}

impl FromPrimitive for DDReal {
    fn from_f64(n: f64) -> Option<Self> {
        Some(n.into())
    }
    fn from_f32(n: f32) -> Option<Self> {
        Some(n.into())
    }
    fn from_usize(n: usize) -> Option<Self> {
        Some(n.into())
    }
    #[cfg(has_i128)]
    fn from_u128(n: u128) -> Option<Self> {
        Some(n.into())
    }
    fn from_u64(n: u64) -> Option<Self> {
        Some(n.into())
    }
    #[cfg(has_i128)]
    fn from_i128(n: i128) -> Option<Self> {
        Some(n.into())
    }
    fn from_i64(n: i64) -> Option<Self> {
        Some(n.into())
    }
}

impl NumCast for DDReal {
    fn from<T: ToPrimitive>(n: T) -> std::option::Option<Self> {
        Self::from_f64(n.to_f64().unwrap())
    }
}

impl PartialEq for DDReal {
    fn eq(&self, other: &Self) -> bool {
        self.x[0] == other.x[0] && self.x[1] == other.x[1]
    }
}

impl PartialOrd for DDReal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.x[0] == other.x[0] {
            self.x[1].partial_cmp(&other.x[1])
        } else {
            self.x[0].partial_cmp(&other.x[0])
        }
    }
}

impl Float for DDReal {
    fn infinity() -> Self {
        Self {
            x: [std::f64::INFINITY, 0.0],
        }
    }
    fn neg_infinity() -> Self {
        Self {
            x: [std::f64::NEG_INFINITY, 0.0],
        }
    }
    fn nan() -> Self {
        Self {
            x: [std::f64::NAN, std::f64::NAN],
        }
    }
    fn neg_zero() -> Self {
        Self { x: [-0.0, -0.0] }
    }
    fn min_value() -> Self {
        -crate::ddconsts::MAX
    }
    fn min_positive_value() -> Self {
        Self {
            x: [std::f64::MIN_POSITIVE, 0.0],
        }
    }
    fn epsilon() -> Self {
        Self {
            x: [crate::ddconsts::EPS, 0.0],
        }
    }
    fn max_value() -> Self {
        crate::ddconsts::MAX
    }
    #[inline]
    fn is_nan(self) -> bool {
        self.x[0].is_nan()
    }
    #[inline]
    fn is_infinite(self) -> bool {
        self.x[0].is_infinite()
    }
    #[inline]
    fn is_finite(self) -> bool {
        self.x[0].is_finite()
    }
    #[inline]
    fn is_normal(self) -> bool {
        self.x[0].is_normal()
    }
    fn classify(self) -> std::num::FpCategory {
        self.x[0].classify()
    }
    #[inline]
    fn floor(self) -> Self {
        let x0f = self.x[0].floor();
        Self {
            x: if self.x[0] != x0f {
                [x0f, 0.0]
            } else {
                [self.x[0], self.x[1].floor()]
            },
        }
    }
    #[inline]
    fn ceil(self) -> Self {
        let x0c = self.x[0].ceil();
        Self {
            x: if self.x[0] != x0c {
                [x0c, 0.0]
            } else {
                [self.x[0], self.x[1].ceil()]
            },
        }
    }
    #[inline]
    fn round(self) -> Self {
        let x0r = self.x[0].round();
        Self {
            x: if self.x[0] != x0r {
                [x0r, 0.0]
            } else {
                [self.x[0], self.x[1].round()]
            },
        }
    }
    #[inline]
    fn trunc(self) -> Self {
        let x0t = self.x[0].trunc();
        Self {
            x: if self.x[0] != x0t {
                [x0t, 0.0]
            } else {
                [self.x[0], self.x[1].trunc()]
            },
        }
    }
    fn fract(self) -> Self {
        self - self.trunc()
    }
    fn abs(self) -> Self {
        Self {
            x: if self.x[0] < 0.0 {
                [-self.x[0], -self.x[1]]
            } else {
                [self.x[0], self.x[1]]
            },
        }
    }
    fn signum(self) -> Self {
        Self {
            x: [self.x[0].signum(), 0.0],
        }
    }
    fn is_sign_positive(self) -> bool {
        self.x[0].is_sign_positive()
    }
    fn is_sign_negative(self) -> bool {
        self.x[0].is_sign_negative()
    }
    fn mul_add(self, a: Self, b: Self) -> Self {
        (self * a) + b
    }
    fn recip(self) -> Self {
        1.0 / self
    }
    fn powi(self, n: i32) -> Self {
        if n == 0 {
            if self.is_zero() {
                return Self::nan();
            }
            return Self::one();
        }

        let mut r = self;
        let mut s = Self::one();
        let mut _n = n.abs();

        if _n > 1 {
            // use binary exponentiation
            while _n > 0 {
                if _n % 2 == 1 {
                    s = s * r;
                }
                _n /= 2;
                if _n > 0 {
                    r = r.sqr();
                }
            }
        } else {
            s = r;
        }

        // Compute the reciprocal if n is negative.
        if n < 0 {
            1.0 / s
        } else {
            s
        }
    }
    fn powf(self, n: Self) -> Self {
        (n * self.ln()).exp()
    }
    fn sqrt(self) -> Self {
        self.ln().mul_pwr2(0.5).exp()
    }
    /* Exponential.  Computes exp(x) in double-double precision. */
    fn exp(self) -> Self {
        /*
        Strategy:  We first reduce the size of x by noting that

            exp(kr + m * log(2)) = 2^m * exp(r)^k

        where m and k are integers.  By choosing m appropriately
        we can make |kr| <= log(2) / 2 = 0.347.  Then exp(r) is
        evaluated using the familiar Taylor series.  Reducing the
        argument substantially speeds up the convergence.
        */

        let k = 512.0;
        let inv_k = 1.0 / k;

        if self.x[0] <= -709.0 {
            return Self::zero();
        }
        if self.x[0] >= 709.0 {
            return Self::infinity();
        }
        if self.is_zero() {
            return Self::one();
        }
        if self.is_one() {
            return crate::ddconsts::E;
        }

        let m = (self.x[0] / crate::ddconsts::LN_2.x[0]).round();
        let r = Self::mul_pwr2(self - crate::ddconsts::LN_2 * m, inv_k);
        let mut p = r.sqr();
        let mut s = r + Self::mul_pwr2(p, 0.5);
        p = p * r;
        let mut t = p * INV_FACT[0];

        for i in 1..5 {
            s = s + t;
            p = p * r;
            t = p * INV_FACT[i];
            if t.x[0].abs() <= inv_k * crate::ddconsts::EPS {
                break;
            }
        }

        s = s + t;

        s = Self::mul_pwr2(s, 2.0) + s.sqr();
        s = Self::mul_pwr2(s, 2.0) + s.sqr();
        s = Self::mul_pwr2(s, 2.0) + s.sqr();
        s = Self::mul_pwr2(s, 2.0) + s.sqr();
        s = Self::mul_pwr2(s, 2.0) + s.sqr();
        s = Self::mul_pwr2(s, 2.0) + s.sqr();
        s = Self::mul_pwr2(s, 2.0) + s.sqr();
        s = Self::mul_pwr2(s, 2.0) + s.sqr();
        s = Self::mul_pwr2(s, 2.0) + s.sqr();
        s = s + 1.0;

        Self::ldexp(s, m as i32)
    }
    fn exp2(self) -> Self {
        (self * crate::ddconsts::LN_2).exp()
    }
    /*
    Logarithm.  Computes log(x) in double-double precision.
    This is a natural logarithm (i.e., base e).
    */
    fn ln(self) -> Self {
        /*
        Strategy.  The Taylor series for log converges much more
        slowly than that of exp, due to the lack of the factorial
        term in the denominator.  Hence this routine instead tries
        to determine the root of the function

            f(x) = exp(x) - a

        using Newton iteration.  The iteration is given by

            x' = x - f(x)/f'(x)
            = x - (1 - a * exp(-x))
            = x + a * exp(-x) - 1.

        Only one iteration is needed, since Newton's iteration
        approximately doubles the number of digits per iteration.
        */
        if self.is_one() {
            return Self::zero();
        }

        if self.x[0] <= 0.0 {
            return Self::nan();
        }

        let x = Self::new(self.x[0].ln(), 0.0); /* Initial approximation */

        x - Self::one() + self * (-x).exp()
    }
    fn log(self, base: Self) -> Self {
        self.ln() / base.ln()
    }
    fn log2(self) -> Self {
        self.ln() * crate::ddconsts::LOG2_E
    }
    fn log10(self) -> Self {
        self.ln() * crate::ddconsts::LOG10_E
    }
    fn max(self, other: Self) -> Self {
        if self > other {
            self
        } else {
            other
        }
    }
    fn min(self, other: Self) -> Self {
        if self < other {
            self
        } else {
            other
        }
    }
    fn abs_sub(self, other: Self) -> Self {
        (self - other).max(Self::one())
    }
    fn cbrt(self) -> Self {
        (self.ln() / 3.0).exp()
    }
    fn hypot(self, other: Self) -> Self {
        self.sqr() + other.sqr()
    }
    fn sin(self) -> Self {
        /*
        Strategy.  To compute sin(x), we choose integers a, b so that

        x = s + a * (pi/2) + b * (pi/16)

        and |s| <= pi/32.  Using the fact that

        sin(pi/16) = 0.5 * sqrt(2 - sqrt(2 + sqrt(2)))

        we can compute sin(x) from sin(s), cos(s).  This greatly
        increases the convergence of the sine Taylor series.
        */

        if self.is_zero() {
            return Self::zero();
        }

        // approximately reduce modulo 2*pi
        let z = (self * crate::ddconsts::FRAC_1_TAU).nint();
        let r = self - crate::ddconsts::TAU * z;

        // approximately reduce modulo pi/2 and then modulo pi/16.
        let q = (r.x[0] * crate::ddconsts::FRAC_2_PI.x[0]).round();
        let t = r - crate::ddconsts::FRAC_PI_2 * q;
        let j = q as i32;
        let q = (t.x[0] / crate::ddconsts::FRAC_PI_16.x[0]).round();
        let t = t - crate::ddconsts::FRAC_PI_16 * q;
        let k = q as i32;
        let abs_k = k.abs() as usize;

        if j < -2 || j > 2 {
            return Self::nan();
        }
        if abs_k > 4 {
            return Self::nan();
        }
        if k == 0 {
            return match j {
                0 => t.sin_taylor(),
                1 => t.cos_taylor(),
                -1 => -t.cos_taylor(),
                _ => -t.sin_taylor(),
            };
        }

        let u = COS_TABLE[abs_k];
        let v = SIN_TABLE[abs_k];
        let (sin_t, cos_t) = t.sincos_taylor();
        if j == 0 {
            if k > 0 {
                u * sin_t + v * cos_t
            } else {
                u * sin_t - v * cos_t
            }
        } else if j == 1 {
            if k > 0 {
                u * cos_t - v * sin_t
            } else {
                u * cos_t + v * sin_t
            }
        } else if j == -1 {
            if k > 0 {
                v * sin_t - u * cos_t
            } else {
                -u * cos_t - v * sin_t
            }
        } else {
            if k > 0 {
                -u * sin_t - v * cos_t
            } else {
                v * cos_t - u * sin_t
            }
        }
    }
    fn cos(self) -> Self {
        if self.is_zero() {
            return Self::one();
        }

        // approximately reduce modulo 2*pi
        let z = (self * crate::ddconsts::FRAC_1_TAU).nint();
        let r = self - z * crate::ddconsts::TAU;

        // approximately reduce modulo pi/2 and then modulo pi/16
        let q = (r.x[0] * crate::ddconsts::FRAC_2_PI.x[0]).round();
        let t = r - crate::ddconsts::FRAC_PI_2 * q;
        let j = q as i32;
        let q = (t.x[0] / crate::ddconsts::FRAC_PI_16.x[0]).round();
        let t = t - crate::ddconsts::FRAC_PI_16 * q;
        let k = q as i32;
        let abs_k = k.abs() as usize;

        if j < -2 || j > 2 {
            return Self::nan();
        }
        if abs_k > 4 {
            return Self::nan();
        }

        let (sin_t, cos_t) = t.sincos_taylor();
        let u = COS_TABLE[abs_k];
        let v = SIN_TABLE[abs_k];

        if j == 0 {
            if k > 0 {
                u * cos_t - v * sin_t
            } else {
                u * cos_t + v * sin_t
            }
        } else if j == 1 {
            if k > 0 {
                -u * sin_t - v * cos_t
            } else {
                v * cos_t - u * sin_t
            }
        } else if j == -1 {
            if k > 0 {
                u * sin_t + v * cos_t
            } else {
                u * sin_t - v * cos_t
            }
        } else {
            if k > 0 {
                v * sin_t - u * cos_t
            } else {
                -u * cos_t - v * sin_t
            }
        }
    }
    fn sin_cos(self) -> (Self, Self) {
        if self.is_zero() {
            return (Self::zero(), Self::one());
        }

        // approximately reduce modulo 2*pi
        let z = (self * crate::ddconsts::FRAC_1_TAU).nint();
        let r = self - crate::ddconsts::TAU * z;

        // approximately reduce module pi/2 and pi/16
        let q = (r.x[0] * crate::ddconsts::FRAC_2_PI.x[0]).round();
        let t = r - crate::ddconsts::FRAC_PI_2 * q;
        let j = q as i32;
        let abs_j = j.abs() as usize;
        let q = (t.x[0] / crate::ddconsts::FRAC_PI_16.x[0]).round();
        let t = t - crate::ddconsts::FRAC_PI_16 * q;
        let k = q as i32;
        let abs_k = k.abs() as usize;

        if abs_j > 2 {
            // error("(dd_real::sincos): Cannot reduce modulo pi/2.");
            return (Self::nan(), Self::nan());
        }

        if abs_k > 4 {
            // error("(dd_real::sincos): Cannot reduce modulo pi/16.");
            return (Self::nan(), Self::nan());
        }

        let (sin_t, cos_t) = t.sincos_taylor();

        if abs_k == 0 {
            return (sin_t, cos_t);
        }

        let u = COS_TABLE[abs_k];
        let v = SIN_TABLE[abs_k];

        let (s, c) = if k > 0 {
            (u * sin_t + v * cos_t, u * cos_t - v * sin_t)
        } else {
            (u * sin_t - v * cos_t, u * cos_t + v * sin_t)
        };

        if abs_j == 0 {
            (s, c)
        } else if j == 1 {
            (c, -s)
        } else if j == -1 {
            (-c, s)
        } else {
            (-s, -c)
        }
    }
    fn tan(self) -> Self {
        let (s, c) = self.sin_cos();
        s / c
    }
    fn asin(self) -> Self {
        let abs_a = self.abs();

        if abs_a > Self::one() {
            // error("(dd_real::acos): Argument out of domain.");
            return Self::nan();
        }

        if abs_a.is_one() {
            if self.is_sign_positive() {
                crate::ddconsts::FRAC_PI_2
            } else {
                -crate::ddconsts::FRAC_PI_2
            }
        } else {
            self.atan2((Self::one() - self.sqr()).sqrt())
        }
    }
    fn acos(self) -> Self {
        let abs_a = self.abs();

        if abs_a > Self::one() {
            // error("(dd_real::acos): Argument out of domain.");
            return Self::nan();
        }

        if abs_a.is_one() {
            if self.is_sign_positive() {
                Self::zero()
            } else {
                crate::ddconsts::PI
            }
        } else {
            (Self::one() - self.sqr()).atan2(self)
        }
    }
    fn atan(self) -> Self {
        self.atan2(Self::one())
    }
    fn atan2(self, other: Self) -> Self {
        /*
        Strategy: Instead of using Taylor series to compute
        arctan, we instead use Newton's iteration to solve
        the equation

                sin(z) = y/r    or    cos(z) = x/r

        where r = sqrt(x^2 + y^2).
        The iteration is given by

                z' = z + (y - sin(z)) / cos(z)          (for equation 1)
                z' = z - (x - cos(z)) / sin(z)          (for equation 2)

        Here, x and y are normalized so that x^2 + y^2 = 1.
        If |x| > |y|, then first iteration is used since the
        denominator is larger.  Otherwise, the second is used.
        */

        let (y, x) = (self, other);

        if x.is_zero() {
            if y.is_zero() {
                /* Both x and y is zero. */
                // error("(dd_real::atan2): Both arguments zero.");
                return Self::nan();
            }

            if y.is_sign_positive() {
                return crate::ddconsts::FRAC_PI_2;
            } else {
                return -crate::ddconsts::FRAC_PI_2;
            }
        } else if y.is_zero() {
            if x.is_sign_positive() {
                return Self::zero();
            } else {
                return crate::ddconsts::PI;
            }
        }

        if x == y {
            if y.is_sign_positive() {
                return crate::ddconsts::FRAC_PI_4;
            } else {
                return -crate::ddconsts::FRAC_3PI_4;
            }
        }

        if x == -y {
            if y.is_sign_positive() {
                return crate::ddconsts::FRAC_3PI_4;
            } else {
                return -crate::ddconsts::FRAC_PI_4;
            }
        }

        let r = (x.sqr() + y.sqr()).sqrt();
        let xx = x / r;
        let yy = y / r;

        /* Compute double precision approximation to atan. */
        let z = Self::from_f64(y.x[0].atan2(x.x[0])).unwrap();

        if xx.x[0].abs() > yy.x[0].abs() {
            /* Use Newton iteration 1.  z' = z + (y - sin(z)) / cos(z)  */
            let (sin_z, cos_z) = z.sin_cos();
            return z + (yy - sin_z) / cos_z;
        } else {
            /* Use Newton iteration 2.  z' = z - (x - cos(z)) / sin(z)  */
            let (sin_z, cos_z) = z.sin_cos();
            return z - (xx - cos_z) / sin_z;
        }
    }
    fn exp_m1(self) -> Self {
        if !(self.x[0] * crate::ddconsts::LOG2_E).round().is_zero() {
            return self.exp() - 1.0;
        }
        /*
        Strategy:  We first reduce the size of x by noting that

            exp(kr) = exp(r)^k

        where m and k are integers.  By choosing m appropriately
        we can make |kr| <= log(2) / 2 = 0.347.  Then exp(r) is
        evaluated using the familiar Taylor series.  Reducing the
        argument substantially speeds up the convergence.
        */

        let k = 512.0;
        let inv_k = 1.0 / k;

        if self.x[0] <= -709.0 {
            return Self::zero();
        }
        if self.x[0] >= 709.0 {
            return Self::infinity();
        }
        if self.is_zero() {
            return Self::one();
        }
        if self.is_one() {
            return crate::ddconsts::E;
        }

        let r = Self::mul_pwr2(self, inv_k);
        let mut p = r.sqr();
        let mut s = r + Self::mul_pwr2(p, 0.5);
        p = p * r;
        let mut t = Self::zero();
        for i in 0..5 {
            t = p * INV_FACT[i];
            s = s + t;
            p = p * r;
            if t.abs().x[0] <= inv_k * crate::ddconsts::EPS {
                break;
            }
        }

        s = s + t;

        s = Self::mul_pwr2(s, 2.0) + s.sqr();
        s = Self::mul_pwr2(s, 2.0) + s.sqr();
        s = Self::mul_pwr2(s, 2.0) + s.sqr();
        s = Self::mul_pwr2(s, 2.0) + s.sqr();
        s = Self::mul_pwr2(s, 2.0) + s.sqr();
        s = Self::mul_pwr2(s, 2.0) + s.sqr();
        s = Self::mul_pwr2(s, 2.0) + s.sqr();
        s = Self::mul_pwr2(s, 2.0) + s.sqr();
        s = Self::mul_pwr2(s, 2.0) + s.sqr();
        s
    }
    fn ln_1p(self) -> Self {
        (self + 1.0).ln()
    }
    fn sinh(self) -> Self {
        if self.is_zero() {
            return Self::one();
        }

        if self.abs().x[0] > 0.05 {
            let ea = self.exp();
            return (ea - ea.recip()).mul_pwr2(0.5);
        }

        /*
        since a is small, using the above formula gives
        a lot of cancellation.  So use Taylor series.
        */
        let r = self.sqr();
        let thresh = self.x[0].abs() * crate::ddconsts::EPS;
        let mut m = 1.0;
        let mut s = self;
        let mut t = self;

        loop {
            m += 2.0;
            t = t * r / ((m - 1.0) * m);

            s = s + t;
            if t.x[0].abs() <= thresh {
                return s;
            }
        }
    }
    fn cosh(self) -> Self {
        if self.is_zero() {
            return Self::one();
        }

        let ea = self.exp();
        return (ea + ea.recip()).mul_pwr2(0.5);
    }
    fn tanh(self) -> Self {
        if self.is_zero() {
            return Self::zero();
        }

        if self.x[0].abs() > 0.05 {
            let ea = self.exp();
            let inv_ea = self.recip();
            return (ea - inv_ea) / (ea + inv_ea);
        } else {
            let s = self.sinh();
            let c = (s.sqr() + 1.0).sqrt();
            return s / c;
        }
    }
    fn asinh(self) -> Self {
        (self + (self.sqr() + 1.0).sqrt()).ln()
    }
    fn acosh(self) -> Self {
        if self < Self::one() {
            // error("(dd_real::acosh): Argument out of domain.");
            return Self::nan();
        }

        (self + (self.sqr() - 1.0).sqrt()).ln()
    }
    fn atanh(self) -> Self {
        if self.abs() >= Self::one() {
            // error("(dd_real::atanh): Argument out of domain.");
            return Self::nan();
        }

        ((1.0 + self) / (1.0 - self)).ln().mul_pwr2(0.5)
    }
    fn integer_decode(self) -> (u64, i16, i8) {
        self.x[0].integer_decode()
    }
    fn to_degrees(self) -> Self {
        self * crate::ddconsts::FRAC_180_PI
    }
    fn to_radians(self) -> Self {
        self * crate::ddconsts::FRAC_PI_180
    }
}

impl DDReal {
    fn decf_parse(s: &[u8]) -> Option<Self> {
        // ^[+-]?
        let (s, negative) = match s.split_first() {
            Some((&b'+', s)) => (s, false),
            Some((&b'-', s)) => (s, true),
            Some(_) => (s, false),
            None => return None,
        };

        let mut s = s;

        // ([0-9][0-9_]*)?
        let mut frac = Self::zero();
        let mut digit_seen = false;
        loop {
            let (s_, digit) = match s.split_first() {
                Some((&c @ b'0'..=b'9', s)) => (s, c - b'0'),
                Some((&b'_', s_)) if digit_seen => {
                    s = s_;
                    continue;
                }
                _ => break,
            };

            s = s_;
            digit_seen = true;
            frac = frac * 16.0 + digit as f64;
        }

        // \.
        let mut period_seen = false;
        let mut s = match s.split_first() {
            Some((&b'.', s)) => {
                period_seen = true;
                s
            }
            _ => s,
        };

        // \.([0-9][0-9_]*)?
        let mut frac_digit_seen = false;
        if period_seen {
            let mut fbase = Self::one();
            loop {
                if !period_seen {
                    break;
                }
                let (s_, digit) = match s.split_first() {
                    Some((&c @ b'0'..=b'9', s)) => (s, c - b'0'),
                    Some((&b'_', s_)) if frac_digit_seen => {
                        s = s_;
                        continue;
                    }
                    _ => break,
                };

                s = s_;
                frac_digit_seen = true;
                fbase = fbase / 10.0;

                if digit != 0 {
                    frac = frac + fbase * (digit as f64);
                }
            }
        }

        if !(digit_seen || frac_digit_seen) {
            return None;
        }

        if negative {
            frac = -frac;
        }

        let mut exponent: i32 = 0;

        // [Ee]
        let mut pexp_seen = false;
        let s = match s.split_first() {
            Some((&b'E', s)) | Some((&b'e', s)) => {
                pexp_seen = true;
                s
            }
            _ => s,
        };

        if pexp_seen {
            // [+-]
            let (mut s, negative_exponent) = match s.split_first() {
                Some((&b'+', s)) => (s, false),
                Some((&b'-', s)) => (s, true),
                Some(_) => (s, false),
                None => return None,
            };

            let mut pexp_digit_seen = false;
            // [0-9_]*[0-9][0-9_]*
            loop {
                let (s_, digit) = match s.split_first() {
                    Some((&c @ b'0'..=b'9', s)) => (s, c - b'0'),
                    Some((&b'_', s_)) => {
                        s = s_;
                        continue;
                    }
                    None if pexp_digit_seen => break,
                    _ => return None,
                };

                s = s_;
                pexp_digit_seen = true;

                exponent = exponent * 10 + digit as i32;
            }
            if negative_exponent {
                exponent = -exponent;
            }
            frac = frac * Self::from_i64(10).unwrap().powi(exponent);
        } else if s.split_first().is_some() {
            return None;
        }

        Some(frac)
    }
}

impl DDReal {
    fn hexf_parse(s: &[u8]) -> Option<Self> {
        // ^[+-]?
        let (s, negative) = match s.split_first() {
            Some((&b'+', s)) => (s, false),
            Some((&b'-', s)) => (s, true),
            Some(_) => (s, false),
            None => return None,
        };

        // (0[xX])?
        let mut s = if s.starts_with(b"0x") || s.starts_with(b"0X") {
            &s[2..]
        } else {
            s
        };

        // ([0-9a-fA-F][0-9a-fA-F_]*)?
        let mut frac = Self::zero();
        let mut digit_seen = false;
        loop {
            let (s_, digit) = match s.split_first() {
                Some((&c @ b'0'..=b'9', s)) => (s, c - b'0'),
                Some((&c @ b'a'..=b'f', s)) => (s, c - b'a' + 10),
                Some((&c @ b'A'..=b'F', s)) => (s, c - b'A' + 10),
                Some((&b'_', s_)) if digit_seen => {
                    s = s_;
                    continue;
                }
                _ => break,
            };

            s = s_;
            digit_seen = true;
            frac = frac * 16.0 + digit as f64;
        }

        // \.
        let mut period_seen = false;
        let mut s = match s.split_first() {
            Some((&b'.', s)) => {
                period_seen = true;
                s
            }
            _ => s,
        };

        // \.([0-9a-fA-F][0-9a-fA-F_]*)?
        let mut frac_digit_seen = false;
        if period_seen {
            let mut fbase = 1.0_f64;
            loop {
                if !period_seen {
                    break;
                }
                let (s_, digit) = match s.split_first() {
                    Some((&c @ b'0'..=b'9', s)) => (s, c - b'0'),
                    Some((&c @ b'a'..=b'f', s)) => (s, c - b'a' + 10),
                    Some((&c @ b'A'..=b'F', s)) => (s, c - b'A' + 10),
                    Some((&b'_', s_)) if frac_digit_seen => {
                        s = s_;
                        continue;
                    }
                    _ => break,
                };

                s = s_;
                frac_digit_seen = true;
                fbase *= 0.0625; // 1.0/16.0

                if digit != 0 {
                    frac = frac + fbase * (digit as f64);
                }
            }
        }

        if !(digit_seen || frac_digit_seen) {
            return None;
        }

        if negative {
            frac = -frac;
        }

        let mut exponent: i32 = 0;

        // [Pp]
        let mut pexp_seen = false;
        let s = match s.split_first() {
            Some((&b'P', s)) | Some((&b'p', s)) => {
                pexp_seen = true;
                s
            }
            _ => s,
        };

        if pexp_seen {
            // [+-]
            let (mut s, negative_exponent) = match s.split_first() {
                Some((&b'+', s)) => (s, false),
                Some((&b'-', s)) => (s, true),
                Some(_) => (s, false),
                None => return None,
            };

            let mut pexp_digit_seen = false;
            // [0-9_]*[0-9][0-9_]*
            loop {
                let (s_, digit) = match s.split_first() {
                    Some((&c @ b'0'..=b'9', s)) => (s, c - b'0'),
                    Some((&b'_', s_)) => {
                        s = s_;
                        continue;
                    }
                    None if pexp_digit_seen => break,
                    _ => return None,
                };

                s = s_;
                pexp_digit_seen = true;

                exponent = exponent * 10 + digit as i32;
            }
            if negative_exponent {
                exponent = -exponent;
            }
            frac = frac.ldexp(exponent);
        } else if s.split_first().is_some() {
            return None;
        }
        Some(frac)
    }
}

impl std::fmt::Display for DDReal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{:.18e}, {:.18e}]", self.x[0], self.x[1])
    }
}

impl std::fmt::LowerHex for DDReal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_nan() {
            return write!(f, "nan");
        }
        let mut mantissa = num::BigInt::zero();
        let mut exponent = 0i32;
        for i in 0..2 {
            let (_mantissa, _exponent, _sign) = self.x[i].integer_decode();
            if _mantissa.is_zero() {
            } else if mantissa.is_zero() {
                mantissa = num::BigInt::from(_mantissa) * _sign;
                exponent = _exponent as i32;
            } else if exponent > _exponent as i32 {
                mantissa = (mantissa << ((exponent - (_exponent as i32)) as usize))
                    + (num::BigInt::from(_mantissa) * _sign);
                exponent = _exponent as i32;
            } else {
                mantissa += (num::BigInt::from(_mantissa) * _sign)
                    << (((_exponent as i32) - exponent) as usize);
            }
        }
        let d_exp = ((exponent % 4) + 4) % 4;
        mantissa <<= d_exp as usize;
        exponent -= d_exp;
        while !mantissa.is_zero() && (mantissa.clone() % num::BigInt::from(16)).is_zero() {
            mantissa >>= 4;
            exponent += 4;
        }
        let mut mantissa_fhexs = mantissa.bits().max(1) - 1 >> 2;
        if !mantissa.is_zero() {
            if let Some(n) = f.precision() {
                if n > mantissa_fhexs {
                    let d_prec = n - mantissa_fhexs;
                    mantissa <<= d_prec << 2;
                    mantissa_fhexs = n;
                    exponent -= (d_prec << 2) as i32;
                } else if mantissa_fhexs > n {
                    let d_prec = mantissa_fhexs - n;
                    mantissa =
                        (mantissa + (num::BigInt::from(8) << (d_prec - 1 << 2))) >> (d_prec << 2);
                    mantissa_fhexs = n;
                    exponent += (d_prec << 2) as i32;
                }
            }
        }
        let mut mantissa_str = mantissa.to_str_radix(16);
        if mantissa_fhexs > 0 {
            let (str_l, str_r) = mantissa_str.split_at(mantissa_str.len() - mantissa_fhexs);
            mantissa_str = str_l.to_string() + "." + str_r;
            exponent += (mantissa_fhexs << 2) as i32;
        }
        write!(
            f,
            "{}{}{}p{}",
            if f.sign_plus() && self.is_sign_positive() {
                "+"
            } else if (f.sign_plus() || f.sign_minus()) && self.is_sign_negative() {
                "-"
            } else {
                ""
            },
            if f.alternate() { "0x" } else { "" },
            mantissa_str,
            exponent
        )
    }
}

impl std::str::FromStr for DDReal {
    type Err = String;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        (if str.starts_with("0x") || str.starts_with("0X") {
            Self::hexf_parse(str.as_bytes())
        } else {
            Self::decf_parse(str.as_bytes())
        })
        .ok_or_else(|| "parse error".to_owned())
    }
}

impl Num for DDReal {
    type FromStrRadixErr = String;
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        match radix {
            10 => Self::decf_parse(str.as_bytes()).ok_or_else(|| "parse error".to_owned()),
            16 => Self::hexf_parse(str.as_bytes()).ok_or_else(|| "parse error".to_owned()),
            _ => Err("radix not implemented".to_owned()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    #[test]
    fn consts_hexstr_e() {
        assert_eq!(
            crate::ddconsts::E,
            crate::DDReal::from_str(crate::hexstrs::E).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_frac_1_pi() {
        assert_eq!(
            crate::ddconsts::FRAC_1_PI,
            crate::DDReal::from_str(crate::hexstrs::FRAC_1_PI).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_frac_1_sqrt_2() {
        assert_eq!(
            crate::ddconsts::FRAC_1_SQRT_2,
            crate::DDReal::from_str(crate::hexstrs::FRAC_1_SQRT_2).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_frac_1_tau() {
        assert_eq!(
            crate::ddconsts::FRAC_1_TAU,
            crate::DDReal::from_str(crate::hexstrs::FRAC_1_TAU).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_frac_180_pi() {
        assert_eq!(
            crate::ddconsts::FRAC_180_PI,
            crate::DDReal::from_str(crate::hexstrs::FRAC_180_PI).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_frac_2_pi() {
        assert_eq!(
            crate::ddconsts::FRAC_2_PI,
            crate::DDReal::from_str(crate::hexstrs::FRAC_2_PI).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_frac_2_sqrt_pi() {
        assert_eq!(
            crate::ddconsts::FRAC_2_SQRT_PI,
            crate::DDReal::from_str(crate::hexstrs::FRAC_2_SQRT_PI).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_frac_pi_16() {
        assert_eq!(
            crate::ddconsts::FRAC_PI_16,
            crate::DDReal::from_str(crate::hexstrs::FRAC_PI_16).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_frac_pi_180() {
        assert_eq!(
            crate::ddconsts::FRAC_PI_180,
            crate::DDReal::from_str(crate::hexstrs::FRAC_PI_180).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_frac_pi_2() {
        assert_eq!(
            crate::ddconsts::FRAC_PI_2,
            crate::DDReal::from_str(crate::hexstrs::FRAC_PI_2).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_frac_pi_3() {
        assert_eq!(
            crate::ddconsts::FRAC_PI_3,
            crate::DDReal::from_str(crate::hexstrs::FRAC_PI_3).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_frac_pi_4() {
        assert_eq!(
            crate::ddconsts::FRAC_PI_4,
            crate::DDReal::from_str(crate::hexstrs::FRAC_PI_4).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_frac_pi_6() {
        assert_eq!(
            crate::ddconsts::FRAC_PI_6,
            crate::DDReal::from_str(crate::hexstrs::FRAC_PI_6).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_frac_pi_8() {
        assert_eq!(
            crate::ddconsts::FRAC_PI_8,
            crate::DDReal::from_str(crate::hexstrs::FRAC_PI_8).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_frac_3pi_4() {
        assert_eq!(
            crate::ddconsts::FRAC_3PI_4,
            crate::DDReal::from_str(crate::hexstrs::FRAC_3PI_4).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_ln_2() {
        assert_eq!(
            crate::ddconsts::LN_2,
            crate::DDReal::from_str(crate::hexstrs::LN_2).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_ln_10() {
        assert_eq!(
            crate::ddconsts::LN_10,
            crate::DDReal::from_str(crate::hexstrs::LN_10).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_log10_2() {
        assert_eq!(
            crate::ddconsts::LOG10_2,
            crate::DDReal::from_str(crate::hexstrs::LOG10_2).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_log10_e() {
        assert_eq!(
            crate::ddconsts::LOG10_E,
            crate::DDReal::from_str(crate::hexstrs::LOG10_E).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_log2_10() {
        assert_eq!(
            crate::ddconsts::LOG2_10,
            crate::DDReal::from_str(crate::hexstrs::LOG2_10).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_log2_e() {
        assert_eq!(
            crate::ddconsts::LOG2_E,
            crate::DDReal::from_str(crate::hexstrs::LOG2_E).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_pi() {
        assert_eq!(
            crate::ddconsts::PI,
            crate::DDReal::from_str(crate::hexstrs::PI).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_sqrt_2() {
        assert_eq!(
            crate::ddconsts::SQRT_2,
            crate::DDReal::from_str(crate::hexstrs::SQRT_2).unwrap()
        )
    }
    #[test]
    fn consts_hexstr_tau() {
        assert_eq!(
            crate::ddconsts::TAU,
            crate::DDReal::from_str(crate::hexstrs::TAU).unwrap()
        )
    }
}
