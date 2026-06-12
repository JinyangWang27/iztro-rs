//! Palace heavenly-stem assignment from the birth year stem.
//!
//! This implements the classical 起五行寅例 (the rule that fixes the stem of the
//! Yin palace from the birth year stem) followed by forward stem assignment
//! through the twelve earthly branches.
//!
//! ```text
//! 甲己之岁起丙寅   Jia / Ji  year -> Bing at Yin
//! 乙庚之岁起戊寅   Yi  / Geng year -> Wu   at Yin
//! 丙辛之岁起庚寅   Bing/ Xin  year -> Geng at Yin
//! 丁壬之岁起壬寅   Ding/ Ren  year -> Ren  at Yin
//! 戊癸之岁起甲寅   Wu  / Gui  year -> Jia  at Yin
//! ```
//!
//! From the Yin palace, stems proceed forward one Heavenly Stem per Earthly
//! Branch. Because ten stems cover twelve branches cyclically, two branches in
//! every chart repeat the stems of Yin and Mao.

use lunar_lite::{EARTHLY_BRANCHES, EarthlyBranch, HeavenlyStem};

/// Returns the Heavenly Stem assigned to the Yin palace for a birth year stem.
///
/// This is the 起五行寅例 anchor; all other palace stems are derived by counting
/// forward from this stem.
const fn yin_palace_stem(year_stem: HeavenlyStem) -> HeavenlyStem {
    use HeavenlyStem::{Bing, Geng, Jia, Ren, Wu};

    match year_stem {
        HeavenlyStem::Jia | HeavenlyStem::Ji => Bing,
        HeavenlyStem::Yi | HeavenlyStem::Geng => Wu,
        HeavenlyStem::Bing | HeavenlyStem::Xin => Geng,
        HeavenlyStem::Ding | HeavenlyStem::Ren => Ren,
        HeavenlyStem::Wu | HeavenlyStem::Gui => Jia,
    }
}

/// Returns the palace Heavenly Stem for a single Earthly Branch.
///
/// The Yin palace takes the [`yin_palace_stem`] anchor; every other branch takes
/// the stem reached by counting forward from Yin along the month cycle
/// (`Yin -> Mao -> ... -> Hai -> Zi -> Chou`). The forward distance is measured
/// over twelve branch steps, so the stem cycle of ten wraps as expected.
pub fn palace_stem_for_branch(year_stem: HeavenlyStem, branch: EarthlyBranch) -> HeavenlyStem {
    let forward_steps = (branch.index() + EARTHLY_BRANCHES.len() - EarthlyBranch::Yin.index())
        % EARTHLY_BRANCHES.len();

    yin_palace_stem(year_stem).offset(forward_steps as isize)
}

/// Returns the twelve palace Heavenly Stems for a birth year stem.
///
/// The returned array is ordered by the canonical Earthly Branch sequence
/// (`Zi, Chou, Yin, Mao, Chen, Si, Wu, Wei, Shen, You, Xu, Hai`), matching
/// [`EARTHLY_BRANCHES`]. Index `i` therefore holds the stem of the palace whose
/// branch is `EARTHLY_BRANCHES[i]`. The array does **not** start at Yin.
pub fn palace_stems_from_year_stem(year_stem: HeavenlyStem) -> [HeavenlyStem; 12] {
    let mut stems = [HeavenlyStem::Jia; 12];

    let mut index = 0;
    while index < EARTHLY_BRANCHES.len() {
        stems[index] = palace_stem_for_branch(year_stem, EARTHLY_BRANCHES[index]);
        index += 1;
    }

    stems
}
