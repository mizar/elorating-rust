// EloRating Calculator

use ddreal::DDReal;
use num::Float;
use special::{elorating, erfc, erfcinv, ibetainvlogit, logitbetaln, SpecialFloat};
use std::env;

fn write_elorating_table<T: SpecialFloat>(pvec: Vec<f64>, win: f64, draw: f64, lose: f64) {
    let games = win + draw + lose;
    let win_hdraw = win + 0.5 * draw;
    let lose_hdraw = lose + 0.5 * draw;

    println!("---");

    println!();
    println!("Float Type: {}", std::any::type_name::<T>());
    println!("Win-Draw-Lose: {}-{}-{}", win, draw, lose);
    println!("Games: {}", games);
    println!("EloRating median: {:+.2}", elorating(win_hdraw, lose_hdraw));
    println!("WinRate: {:.2}%", win_hdraw / games * 100.0);
    println!("DrawRate: {:.2}%", draw / games * 100.0);

    println!();
    println!("   σ         x            p      |           EloRating estimated range           |  Δrange  | error_l error_u");
    println!("---------------------------------+-----------------------------------------------+----------+----------------");

    for p in pvec {
        let t_p = T::from_f64(p).unwrap();
        let t_win = T::from_f64(win_hdraw).unwrap();
        let t_win1 = T::from_f64(win_hdraw + 1.0).unwrap();
        let t_lose = T::from_f64(lose_hdraw).unwrap();
        let t_lose1 = T::from_f64(lose_hdraw + 1.0).unwrap();
        let lower = ibetainvlogit::<T>(t_p, t_win, t_lose1);
        let upper = ibetainvlogit::<T>(t_p, t_lose, t_win1).negate();
        println!(
            "{:5.3}σ{:12.8}%{:12.8}% |{:+12.4} ({:6.2}%) ~{:+12.4} ({:6.2}%) |{:9.2} |{:+8.0e}{:+8.0e}",
            erfcinv(p * 2.0) * std::f64::consts::SQRT_2,
            100.0 * (1.0 - p * 2.0),
            100.0 * p,
            lower.elo(),
            lower.px().to_f64().unwrap() * 100.0,
            upper.elo(),
            upper.px().to_f64().unwrap() * 100.0,
            upper.elo() - lower.elo(),
            (logitbetaln(lower, t_win, t_lose1).exp() - t_p).to_f64().unwrap(),
            (logitbetaln(upper.negate(), t_lose, t_win1).exp() - t_p).to_f64().unwrap(),
        );
    }

    println!();
}

fn main() {
    let pvec: Vec<f64> = vec![
        0.500,
        0.495,
        0.490,
        0.485,
        0.480,
        0.475,
        0.470,
        0.465,
        0.460,
        0.455,
        0.450,
        0.425,
        0.400,
        0.375,
        0.350,
        0.325,
        // 0.382_924_922_548 ~= 0.5σ
        0.5 * erfc(0.5 * std::f64::consts::FRAC_1_SQRT_2),
        0.300,
        0.275,
        0.250,
        0.225,
        0.200,
        0.175,
        // 0.682_689_492_137 ~= 1.0σ
        0.5 * erfc(1.0 * std::f64::consts::FRAC_1_SQRT_2),
        0.150,
        0.125,
        0.100,
        0.075,
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

    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        usage();
        return;
    }

    let win = args[1].parse::<f64>().unwrap();
    let draw = args[2].parse::<f64>().unwrap();
    let lose = args[3].parse::<f64>().unwrap();
    let opt = if args.len() < 5 { "" } else { args[4].as_str() };
    match opt {
        "f64" => write_elorating_table::<f64>(pvec.clone(), win, draw, lose),
        "DDReal" => write_elorating_table::<DDReal>(pvec.clone(), win, draw, lose),
        "both" => {
            write_elorating_table::<f64>(pvec.clone(), win, draw, lose);
            write_elorating_table::<DDReal>(pvec.clone(), win, draw, lose);
        },
        _ => write_elorating_table::<f64>(pvec.clone(), win, draw, lose)
    }

    /*
    for (win, draw, lose) in ([
        (0.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (0.0, 1.0, 0.0),
        (1.0, 0.0, 1.0),
        (120.0, 0.0, 80.0),
        (987654321098765.0, 1.0, 0.0),
        (987654321098765.0, 0.0, 123456789012345.0),
    ]).iter() {
        write_elorating_table::<f64>(pvec.clone(), *win, *draw, *lose);
        write_elorating_table::<DDReal>(pvec.clone(), *win, *draw, *lose);
    }
    */

}

fn usage() {
    println!("elorating.exe <win> <draw> <lose> [f64|DDReal]");
}

#[allow(dead_code)]
fn test() {
    use std::str::FromStr;

    for (s, f) in [
        ("0", "0"),
        ("1", "1"),
        ("E",              "0x02.b7e1_5162_8aed_2a6a_bf71_5880_9cf4_f3c7_62e7_160f_38b4_da56_a784_d904_5190_cfef_324e_7739_p0"),
        ("FRAC_1_TAU",     "0x00.28be_60db_9391_054a_7f09_d5f4_7d4d_3770_36d8_a566_4f10_e410_7f94_58ea_f7ae_f158_6dc9_1b8f_p0"),
        ("FRAC_1_PI",      "0x00.517c_c1b7_2722_0a94_fe13_abe8_fa9a_6ee0_6db1_4acc_9e21_c820_ff28_b1d5_ef5d_e2b0_db92_371d_p0"),
        ("FRAC_1_SQRT_2",  "0x00.b504_f333_f9de_6484_597d_89b3_754a_be9f_1d6f_60ba_893b_a84c_ed17_ac85_8333_9915_4afc_8304_p0"),
        ("FRAC_180_PI",    "0x39.4bb8_34c7_83ef_70c2_a5d4_dfd0_3495_f5cd_20a8_97df_2fc0_b733_689d_0a6c_4e03_645a_62ce_c07b_p0"),
        ("FRAC_2_PI",      "0x00.a2f9_836e_4e44_1529_fc27_57d1_f534_ddc0_db62_9599_3c43_9041_fe51_63ab_debb_c561_b724_6e3a_p0"),
        ("FRAC_2_SQRT_PI", "0x01.20dd_7504_29b6_d11a_e3a9_14fe_d7fd_8688_2813_41d7_587c_ea2e_7342_b061_99cc_4161_80eb_39f1_p0"),
        ("FRAC_PI_16",     "0x00.3243_f6a8_885a_308d_3131_98a2_e037_0734_4a40_9382_2299_f31d_0082_efa9_8ec4_e6c8_9452_821e_p0"),
        ("FRAC_PI_180",    "0x00.0477_d1a8_94a7_4e45_7076_2fb3_74a4_2e26_c805_bd77_a80d_af35_c728_154d_a64a_6428_95b7_b08b_p0"),
        ("FRAC_PI_2",      "0x01.921f_b544_42d1_8469_898c_c517_01b8_39a2_5204_9c11_14cf_98e8_0417_7d4c_7627_3644_a294_10f3_p0"),
        ("FRAC_PI_3",      "0x01.0c15_2382_d736_5846_5bb3_2e0f_567a_d116_e158_680b_6335_109a_ad64_fe32_f96f_7983_170d_60a2_p0"),
        ("FRAC_PI_4",      "0x00.c90f_daa2_2168_c234_c4c6_628b_80dc_1cd1_2902_4e08_8a67_cc74_020b_bea6_3b13_9b22_514a_087a_p0"),
        ("FRAC_PI_6",      "0x00.860a_91c1_6b9b_2c23_2dd9_9707_ab3d_688b_70ac_3405_b19a_884d_56b2_7f19_7cb7_bcc1_8b86_b051_p0"),
        ("FRAC_PI_8",      "0x00.6487_ed51_10b4_611a_6263_3145_c06e_0e68_9481_2704_4533_e63a_0105_df53_1d89_cd91_28a5_043d_p0"),
        ("FRAC_3PI_4",     "0x02.5b2f_8fe6_643a_469e_4e53_27a2_8294_5673_7b06_ea19_9f37_655c_0623_3bf2_b13a_d166_f3de_196d_p0"),
        ("LN_2",           "0x00.b172_17f7_d1cf_79ab_c9e3_b398_03f2_f6af_40f3_4326_7298_b62d_8a0d_175b_8baa_fa2b_e7b8_7620_p0"),
        ("LN_10",          "0x02.4d76_3776_aaa2_b05b_a95b_58ae_0b4c_28a3_8a3f_b3e7_6977_e43a_0f18_7a08_07c0_b5ca_58bc_0b5f_p0"),
        ("LOG10_2",        "0x00.4d10_4d42_7de7_fbcc_47c4_acd6_05be_48bc_1356_9862_a1e8_f9a4_c52f_3793_5be6_31e5_9435_16c1_p0"),
        ("LOG10_E",        "0x00.6f2d_ec54_9b94_38ca_9aad_d557_d699_ee19_1f71_a301_22e4_d101_1d1f_96a2_7bc7_529e_3aa1_277d_p0"),
        ("LOG2_10",        "0x03.5269_e12f_346e_2bf9_24af_dbfd_36bf_6d33_65b1_57f8_dece_b53a_46da_b202_0b9e_1674_1994_3f7a_p0"),
        ("LOG2_E",         "0x01.7154_7652_b82f_e177_7d0f_fda0_d23a_7d11_d6ae_f551_bad2_b4b1_164a_2cd9_a342_648f_bc38_87ef_p0"),
        ("PI",             "0x03.243f_6a88_85a3_08d3_1319_8a2e_0370_7344_a409_3822_299f_31d0_082e_fa98_ec4e_6c89_4528_21e6_p0"),
        ("SQRT_2",         "0x01.6a09_e667_f3bc_c908_b2fb_1366_ea95_7d3e_3ade_c175_1277_5099_da2f_590b_0667_322a_95f9_0608_p0"),
        ("TAU",            "0x06.487e_d511_0b46_11a6_2633_145c_06e0_e689_4812_7044_533e_63a0_105d_f531_d89c_d912_8a50_43cc_p0"),
    ].iter() {
        let dd = DDReal::from_str(f).unwrap();
        println!("{:>14} : {} {:#x}", s, dd, dd);
    }
    for s in [
        "1.35781220007039464739769136052735188826566614E+0001",
        "3.17823842997348984212895391439981193809771347E-0006",
        "3.14820702833493003545826236239083394571995671E-0004",
        "1.27937416087229845006934584904736618598070572E-0002",
        "2.78748303060299808744345690552596166059251493E-0001",
        "3.57487639582285701807582585579290271336089099E+0000",
        "2.79272804215633250156669351783752812175650972E+0001",
        "1.33213846503797389894468858322687847549726114E+0002",
        "3.79504051924654223127926344491479357839857722E+0002",
        "6.15621499930282594633468081962352923412741184E+0002",
        "5.24004008691006507011182613589749851171431576E+0002",
        "2.04187662020237118761681790759964964799068736E+0002",
        "2.86456197727291086831913426471935542005422307E+0001",
        "8.95072101413389847373058347512910403979947773E-0001",
        "1.84108633157612656306027334817135207544862157E-0003",
        "9.99999999999999999999999999829177067439186649E-0001",
        "2.00000000000000000000000143725325109773686179E+0000",
        "2.99999999999999999999878069870191969828857942E+0000",
        "4.00000000000000000031822896056305388990947492E+0000",
        "4.99999999999999996032765694796886928790459606E+0000",
        "6.00000000000000300343091566980971296037249121E+0000",
        "6.99999999999983752474626982882253159057375788E+0000",
        "8.00000000000719155188030217651616848093810689E+0000",
        "8.99999999970006818618226539512826008489484540E+0000",
        "1.00000000142050052373091324295304916612836237E+0001",
        "1.09999989539201196803612424783730853335056534E+0001",
        "1.20002381089341943372805397259444226612900582E+0001",
    ]
    .iter()
    {
        let dd = DDReal::from_str(s).unwrap();
        println!("{} {:#x}", dd, dd);
    }
    for dd in [
        ddreal::ddconsts::LN_10 * ddreal::ddconsts::LOG10_E,
        ddreal::ddconsts::LN_2 * ddreal::ddconsts::LOG2_E,
        ddreal::ddconsts::E.ln(),
        ddreal::ddconsts::LN_10.exp(),
        ddreal::ddconsts::LN_2.exp(),
        ddreal::ddconsts::LN_2.gammaln(),
    ]
    .iter()
    {
        println!("{} {:#x}", dd, dd);
    }
}
