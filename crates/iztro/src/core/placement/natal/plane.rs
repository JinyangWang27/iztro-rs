//! Natal chart-plane anchor resolution.
//!
//! This module maps an algorithm/plane pair to the Life-palace anchor used by
//! minimal natal chart construction. Request adaptation and construction of the
//! Heaven chart remain facade responsibilities.

use crate::core::error::ChartError;
use crate::core::model::chart::{Chart, PalaceName};
use crate::core::model::profile::{ChartAlgorithmKind, ChartPlane};
use crate::core::placement::natal::minimal::NatalChartAnchor;

/// Resolves the Life-palace anchor for an algorithm and chart plane.
///
/// Heaven charts use the calculated Life Palace without invoking
/// `build_heaven_chart`. Zhongzhou Earth and Human charts derive their explicit
/// anchors from the Heaven chart's Body and Spirit palaces respectively.
pub(crate) fn resolve_natal_chart_anchor<F>(
    algorithm: ChartAlgorithmKind,
    plane: ChartPlane,
    build_heaven_chart: F,
) -> Result<NatalChartAnchor, ChartError>
where
    F: FnOnce() -> Result<Chart, ChartError>,
{
    match (algorithm, plane) {
        (_, ChartPlane::Heaven) => Ok(NatalChartAnchor::CalculatedLifePalace),
        (ChartAlgorithmKind::Zhongzhou, ChartPlane::Earth) => {
            let heaven_chart = build_heaven_chart()?;
            let body_branch = heaven_chart
                .body_palace_branch()
                .ok_or(ChartError::RequiredLifeBodyPalaceMissing)?;
            Ok(NatalChartAnchor::ExplicitLifePalace(body_branch))
        }
        (ChartAlgorithmKind::Zhongzhou, ChartPlane::Human) => {
            let heaven_chart = build_heaven_chart()?;
            let spirit_branch = heaven_chart
                .required_palace_by_name(PalaceName::Spirit)?
                .branch();
            Ok(NatalChartAnchor::ExplicitLifePalace(spirit_branch))
        }
        _ => Err(ChartError::UnsupportedChartPlane { algorithm, plane }),
    }
}

#[cfg(test)]
mod tests {
    use super::resolve_natal_chart_anchor;
    use crate::core::error::ChartError;
    use crate::core::model::calendar::{BirthContext, CalendarDate, Gender};
    use crate::core::model::chart::{Chart, PalaceName};
    use lunar_lite::{EarthlyBranch, HeavenlyStem};
    use crate::core::model::profile::{ChartAlgorithmKind, ChartPlane, MethodProfile};
    use crate::core::placement::natal::input::NatalChartInput;
    use crate::core::placement::natal::life_body::LunarMonth;
    use crate::core::placement::natal::minimal::{NatalChartAnchor, build_minimal_natal_chart};

    fn heaven_chart() -> Chart {
        build_minimal_natal_chart(NatalChartInput::new(
            BirthContext::new(
                CalendarDate::solar(1990, 5, 17),
                EarthlyBranch::Chen,
                Gender::Female,
            ),
            MethodProfile::new(
                "natal_plane_resolver_test",
                ChartAlgorithmKind::Zhongzhou,
                "Natal plane resolver test",
            ),
            LunarMonth::new(4).expect("valid lunar month"),
            HeavenlyStem::Geng,
            EarthlyBranch::Wu,
        ))
        .expect("Heaven chart should build")
    }

    #[test]
    fn quanshu_heaven_uses_calculated_life_palace() {
        let anchor =
            resolve_natal_chart_anchor(ChartAlgorithmKind::QuanShu, ChartPlane::Heaven, || {
                panic!("Heaven chart should not be built for QuanShu Heaven")
            })
            .expect("QuanShu Heaven should resolve");

        assert_eq!(anchor, NatalChartAnchor::CalculatedLifePalace);
    }

    #[test]
    fn zhongzhou_heaven_uses_calculated_life_palace() {
        let anchor =
            resolve_natal_chart_anchor(ChartAlgorithmKind::Zhongzhou, ChartPlane::Heaven, || {
                panic!("Heaven chart should not be built for Zhongzhou Heaven")
            })
            .expect("Zhongzhou Heaven should resolve");

        assert_eq!(anchor, NatalChartAnchor::CalculatedLifePalace);
    }

    #[test]
    fn zhongzhou_earth_uses_heaven_body_palace_branch() {
        let chart = heaven_chart();
        let expected_branch = chart
            .body_palace_branch()
            .expect("Heaven chart should have a Body Palace branch");

        let anchor =
            resolve_natal_chart_anchor(ChartAlgorithmKind::Zhongzhou, ChartPlane::Earth, || {
                Ok(chart)
            })
            .expect("Zhongzhou Earth should resolve");

        assert_eq!(
            anchor,
            NatalChartAnchor::ExplicitLifePalace(expected_branch),
        );
    }

    #[test]
    fn zhongzhou_human_uses_heaven_spirit_palace_branch() {
        let chart = heaven_chart();
        let expected_branch = chart
            .required_palace_by_name(PalaceName::Spirit)
            .expect("Heaven chart should have a Spirit Palace")
            .branch();

        let anchor =
            resolve_natal_chart_anchor(ChartAlgorithmKind::Zhongzhou, ChartPlane::Human, || {
                Ok(chart)
            })
            .expect("Zhongzhou Human should resolve");

        assert_eq!(
            anchor,
            NatalChartAnchor::ExplicitLifePalace(expected_branch),
        );
    }

    #[test]
    fn unsupported_plane_combination_returns_error() {
        for (algorithm, plane) in [
            (ChartAlgorithmKind::QuanShu, ChartPlane::Earth),
            (ChartAlgorithmKind::Placeholder, ChartPlane::Human),
        ] {
            assert_eq!(
                resolve_natal_chart_anchor(algorithm, plane, || {
                    panic!("Heaven chart should not be built for unsupported combinations")
                }),
                Err(ChartError::UnsupportedChartPlane { algorithm, plane }),
            );
        }
    }
}
