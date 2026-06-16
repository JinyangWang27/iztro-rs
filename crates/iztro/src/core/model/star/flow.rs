use crate::core::model::star::name::StarName;
use serde::{Deserialize, Serialize};

/// Horoscope flow-star scope for normalized runtime identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlowStarScope {
    /// Decadal flow-star runtime identity.
    Decadal,
    /// Yearly flow-star runtime identity.
    Yearly,
    /// Monthly flow-star runtime identity.
    Monthly,
    /// Daily flow-star runtime identity.
    Daily,
    /// Hourly flow-star runtime identity.
    Hourly,
}

/// Base identity shared by upstream horoscope flow-star runtime names.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlowStarBase {
    /// Kui flow star (魁).
    Kui,
    /// Yue flow star (钺).
    Yue,
    /// Chang flow star (昌).
    Chang,
    /// Qu flow star (曲).
    Qu,
    /// Lu flow star (禄).
    Lu,
    /// Yang flow star (羊).
    Yang,
    /// Tuo flow star (陀).
    Tuo,
    /// Ma flow star (马).
    Ma,
    /// Luan flow star (鸾).
    Luan,
    /// Xi flow star (喜).
    Xi,
}

/// Returns the upstream runtime star name for a normalized flow-star identity.
///
/// Runtime names stay scope-specific for serde and metadata fidelity. This
/// normalized identity supports future shared placement logic, but this module
/// does not place flow stars.
pub const fn flow_star_name(scope: FlowStarScope, base: FlowStarBase) -> StarName {
    match scope {
        FlowStarScope::Decadal => match base {
            FlowStarBase::Kui => StarName::YunKui,
            FlowStarBase::Yue => StarName::YunYue,
            FlowStarBase::Chang => StarName::YunChang,
            FlowStarBase::Qu => StarName::YunQu,
            FlowStarBase::Lu => StarName::YunLu,
            FlowStarBase::Yang => StarName::YunYang,
            FlowStarBase::Tuo => StarName::YunTuo,
            FlowStarBase::Ma => StarName::YunMa,
            FlowStarBase::Luan => StarName::YunLuan,
            FlowStarBase::Xi => StarName::YunXi,
        },
        FlowStarScope::Yearly => match base {
            FlowStarBase::Kui => StarName::LiuKui,
            FlowStarBase::Yue => StarName::LiuYue,
            FlowStarBase::Chang => StarName::LiuChang,
            FlowStarBase::Qu => StarName::LiuQu,
            FlowStarBase::Lu => StarName::LiuLu,
            FlowStarBase::Yang => StarName::LiuYang,
            FlowStarBase::Tuo => StarName::LiuTuo,
            FlowStarBase::Ma => StarName::LiuMa,
            FlowStarBase::Luan => StarName::LiuLuan,
            FlowStarBase::Xi => StarName::LiuXi,
        },
        FlowStarScope::Monthly => match base {
            FlowStarBase::Kui => StarName::YueKui,
            FlowStarBase::Yue => StarName::YueYue,
            FlowStarBase::Chang => StarName::YueChang,
            FlowStarBase::Qu => StarName::YueQu,
            FlowStarBase::Lu => StarName::YueLu,
            FlowStarBase::Yang => StarName::YueYang,
            FlowStarBase::Tuo => StarName::YueTuo,
            FlowStarBase::Ma => StarName::YueMa,
            FlowStarBase::Luan => StarName::YueLuan,
            FlowStarBase::Xi => StarName::YueXi,
        },
        FlowStarScope::Daily => match base {
            FlowStarBase::Kui => StarName::RiKui,
            FlowStarBase::Yue => StarName::RiYue,
            FlowStarBase::Chang => StarName::RiChang,
            FlowStarBase::Qu => StarName::RiQu,
            FlowStarBase::Lu => StarName::RiLu,
            FlowStarBase::Yang => StarName::RiYang,
            FlowStarBase::Tuo => StarName::RiTuo,
            FlowStarBase::Ma => StarName::RiMa,
            FlowStarBase::Luan => StarName::RiLuan,
            FlowStarBase::Xi => StarName::RiXi,
        },
        FlowStarScope::Hourly => match base {
            FlowStarBase::Kui => StarName::ShiKui,
            FlowStarBase::Yue => StarName::ShiYue,
            FlowStarBase::Chang => StarName::ShiChang,
            FlowStarBase::Qu => StarName::ShiQu,
            FlowStarBase::Lu => StarName::ShiLu,
            FlowStarBase::Yang => StarName::ShiYang,
            FlowStarBase::Tuo => StarName::ShiTuo,
            FlowStarBase::Ma => StarName::ShiMa,
            FlowStarBase::Luan => StarName::ShiLuan,
            FlowStarBase::Xi => StarName::ShiXi,
        },
    }
}

/// Returns normalized flow-star identity parts for matrix flow stars.
///
/// Non-matrix runtime names, including the yearly 年解 helper, return `None`.
pub const fn try_flow_star_parts(star: StarName) -> Option<(FlowStarScope, FlowStarBase)> {
    match star {
        StarName::YunKui => Some((FlowStarScope::Decadal, FlowStarBase::Kui)),
        StarName::YunYue => Some((FlowStarScope::Decadal, FlowStarBase::Yue)),
        StarName::YunChang => Some((FlowStarScope::Decadal, FlowStarBase::Chang)),
        StarName::YunQu => Some((FlowStarScope::Decadal, FlowStarBase::Qu)),
        StarName::YunLu => Some((FlowStarScope::Decadal, FlowStarBase::Lu)),
        StarName::YunYang => Some((FlowStarScope::Decadal, FlowStarBase::Yang)),
        StarName::YunTuo => Some((FlowStarScope::Decadal, FlowStarBase::Tuo)),
        StarName::YunMa => Some((FlowStarScope::Decadal, FlowStarBase::Ma)),
        StarName::YunLuan => Some((FlowStarScope::Decadal, FlowStarBase::Luan)),
        StarName::YunXi => Some((FlowStarScope::Decadal, FlowStarBase::Xi)),
        StarName::LiuKui => Some((FlowStarScope::Yearly, FlowStarBase::Kui)),
        StarName::LiuYue => Some((FlowStarScope::Yearly, FlowStarBase::Yue)),
        StarName::LiuChang => Some((FlowStarScope::Yearly, FlowStarBase::Chang)),
        StarName::LiuQu => Some((FlowStarScope::Yearly, FlowStarBase::Qu)),
        StarName::LiuLu => Some((FlowStarScope::Yearly, FlowStarBase::Lu)),
        StarName::LiuYang => Some((FlowStarScope::Yearly, FlowStarBase::Yang)),
        StarName::LiuTuo => Some((FlowStarScope::Yearly, FlowStarBase::Tuo)),
        StarName::LiuMa => Some((FlowStarScope::Yearly, FlowStarBase::Ma)),
        StarName::LiuLuan => Some((FlowStarScope::Yearly, FlowStarBase::Luan)),
        StarName::LiuXi => Some((FlowStarScope::Yearly, FlowStarBase::Xi)),
        StarName::YueKui => Some((FlowStarScope::Monthly, FlowStarBase::Kui)),
        StarName::YueYue => Some((FlowStarScope::Monthly, FlowStarBase::Yue)),
        StarName::YueChang => Some((FlowStarScope::Monthly, FlowStarBase::Chang)),
        StarName::YueQu => Some((FlowStarScope::Monthly, FlowStarBase::Qu)),
        StarName::YueLu => Some((FlowStarScope::Monthly, FlowStarBase::Lu)),
        StarName::YueYang => Some((FlowStarScope::Monthly, FlowStarBase::Yang)),
        StarName::YueTuo => Some((FlowStarScope::Monthly, FlowStarBase::Tuo)),
        StarName::YueMa => Some((FlowStarScope::Monthly, FlowStarBase::Ma)),
        StarName::YueLuan => Some((FlowStarScope::Monthly, FlowStarBase::Luan)),
        StarName::YueXi => Some((FlowStarScope::Monthly, FlowStarBase::Xi)),
        StarName::RiKui => Some((FlowStarScope::Daily, FlowStarBase::Kui)),
        StarName::RiYue => Some((FlowStarScope::Daily, FlowStarBase::Yue)),
        StarName::RiChang => Some((FlowStarScope::Daily, FlowStarBase::Chang)),
        StarName::RiQu => Some((FlowStarScope::Daily, FlowStarBase::Qu)),
        StarName::RiLu => Some((FlowStarScope::Daily, FlowStarBase::Lu)),
        StarName::RiYang => Some((FlowStarScope::Daily, FlowStarBase::Yang)),
        StarName::RiTuo => Some((FlowStarScope::Daily, FlowStarBase::Tuo)),
        StarName::RiMa => Some((FlowStarScope::Daily, FlowStarBase::Ma)),
        StarName::RiLuan => Some((FlowStarScope::Daily, FlowStarBase::Luan)),
        StarName::RiXi => Some((FlowStarScope::Daily, FlowStarBase::Xi)),
        StarName::ShiKui => Some((FlowStarScope::Hourly, FlowStarBase::Kui)),
        StarName::ShiYue => Some((FlowStarScope::Hourly, FlowStarBase::Yue)),
        StarName::ShiChang => Some((FlowStarScope::Hourly, FlowStarBase::Chang)),
        StarName::ShiQu => Some((FlowStarScope::Hourly, FlowStarBase::Qu)),
        StarName::ShiLu => Some((FlowStarScope::Hourly, FlowStarBase::Lu)),
        StarName::ShiYang => Some((FlowStarScope::Hourly, FlowStarBase::Yang)),
        StarName::ShiTuo => Some((FlowStarScope::Hourly, FlowStarBase::Tuo)),
        StarName::ShiMa => Some((FlowStarScope::Hourly, FlowStarBase::Ma)),
        StarName::ShiLuan => Some((FlowStarScope::Hourly, FlowStarBase::Luan)),
        StarName::ShiXi => Some((FlowStarScope::Hourly, FlowStarBase::Xi)),
        _ => None,
    }
}
