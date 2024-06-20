use qlab_instrument::bond::Bond;
use qlab_math::interpolation::spline::natural_cubic::NaturalCubic;
use qlab_termstructure::yield_curve::YieldCurve;
use qlab_time::date::Date;
use qlab_time::day_count::act_365::Act365;
use qlab_time::frequency::Frequency;

#[test]
fn main() {
    let bond_id = "20 yr bond";
    let issue_date = Date::from_ymd(2023, 5, 8).unwrap();
    let first_coupon_date = Date::from_ymd(2023, 11, 7).unwrap();
    let penultimate_coupon_date = Date::from_ymd(2042, 5, 7).unwrap();
    let maturity_date = Date::from_ymd(2042, 11, 7).unwrap();
    let coupon_frequency = Frequency::SA;
    let coupon_rate = 0.062;
    let face_value = 1000.00;
    let bond_20_yr = Bond::new(
        bond_id,
        issue_date,
        first_coupon_date,
        penultimate_coupon_date,
        maturity_date,
        coupon_frequency,
        coupon_rate,
        face_value,
    )
    .unwrap();
    let spot_settle_date = Date::from_ymd(2023, 10, 10).unwrap();
    let maturities = [
        Date::from_ymd(2023, 10, 11).unwrap(),
        Date::from_ymd(2024, 1, 10).unwrap(),
        Date::from_ymd(2024, 4, 10).unwrap(),
        Date::from_ymd(2024, 10, 10).unwrap(),
        Date::from_ymd(2025, 10, 10).unwrap(),
        Date::from_ymd(2026, 10, 12).unwrap(),
        Date::from_ymd(2028, 10, 10).unwrap(),
        Date::from_ymd(2030, 10, 10).unwrap(),
        Date::from_ymd(2033, 10, 10).unwrap(),
        Date::from_ymd(2038, 10, 11).unwrap(),
        Date::from_ymd(2043, 10, 12).unwrap(),
        Date::from_ymd(2053, 10, 10).unwrap(),
    ];
    let spot_yields: Vec<f64> = vec![
        0.02, 0.0219, 0.0237, 0.0267, 0.0312, 0.0343, 0.0378, 0.0393, 0.04, 0.0401, 0.0401, 0.04,
    ];
    let yield_curve: YieldCurve<Act365, NaturalCubic<f64>> =
        YieldCurve::new(spot_settle_date, &maturities, &spot_yields).unwrap();
    let val = bond_20_yr
        .discounted_value(spot_settle_date, &yield_curve)
        .unwrap();
    println!("{}", bond_20_yr.bond_id());
    println!("{val}"); // 1314.5577192000126
}
