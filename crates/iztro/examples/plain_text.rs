use iztro::render::render_chart_stack_text;
use iztro::{
    ChartAlgorithmKind, EarthlyBranch, Gender, MethodProfile, SolarChartRequest, SolarDay,
    SolarMonth, by_solar,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = SolarChartRequest::builder()
        .solar_year(1990)
        .solar_month(SolarMonth::new(5)?)
        .solar_day(SolarDay::new(17)?)
        .birth_time(EarthlyBranch::Chen)
        .gender(Gender::Female)
        .method_profile(MethodProfile::new(
            "readme_demo",
            ChartAlgorithmKind::QuanShu,
            "README plain text demo",
        ))
        .build()?;

    let chart = by_solar(request)?;
    let snapshot = chart.stack_snapshot();

    println!("{}", render_chart_stack_text(&snapshot));

    Ok(())
}
