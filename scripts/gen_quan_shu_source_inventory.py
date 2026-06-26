#!/usr/bin/env python3
"""Regenerate the Volume 1 太微赋 QuanShu source inventory (atomic source items).

This is the canonical generator for
`crates/iztro/rule-corpus/quan-shu/source/volume-01.toml`: edit the `items`
table below (one tuple per atomic cited source unit, in source order) and re-run
to emit a deterministic TOML file. After running, regenerate the coverage report
by running `cargo test -p iztro` (the test prints the expected report on
mismatch) and update `docs/zh-CN/rules/quan-shu-coverage.md`.

It also carries the one-time migration that pointed each QuanShu rule directly at
its atomic `source_id` and dropped `source_clause_id`. That step is idempotent:
on an already-migrated `rules.toml` it is a no-op.

Run from anywhere: `python3 scripts/gen_quan_shu_source_inventory.py`.
"""
import re, pathlib

ROOT = pathlib.Path(__file__).resolve().parent.parent
SRC = ROOT / "crates/iztro/rule-corpus/quan-shu/source/volume-01.toml"
RULES = ROOT / "crates/iztro/rule-corpus/quan-shu/rules.toml"

PREFIX = "quan_shu.v01.tai_wei_fu."

# (clause_id, text, [rule_ids], note_or_None) in markdown source order.
items = [
    ("lu_feng_chong_po", "禄逢冲破，吉处藏凶", ["wealth.lu_feng_chong_po.fortune_undermined"], None),
    ("ma_yu_kong_wang", "马遇空亡，终身奔走", ["migration.tian_ma_void.restless_movement"], None),
    ("sheng_feng_bai_di", "生逢败地，发也虚花", ["fortune.sheng_feng_bai_di.illusory_bloom"], None),
    ("jue_chu_feng_sheng", "绝处逢生，生花不败", ["fortune.jue_chu_feng_sheng.revival"], None),
    ("xing_lin_miao_wang", "星临庙旺，再观生克之机", ["method.xing_lin_miao_wang.brightness_then_relation"], None),
    ("ming_zuo_qiang_gong", "命坐强宫，细察制化之理", ["method.ming_zuo_qiang_gong.restraint_transformation"], None),
    ("ri_yue_fan_bei", "日月最嫌反背", ["life.ri_yue_fan_bei.hardship_pressure"], None),
    ("lu_ma_jiao_chi", "禄马最喜交驰", ["fortune.lu_ma_jiao_chi.favorable_convergence"], None),
    ("tang_ju_kong_wang", "倘居空亡，得失最为要紧", ["fortune.tang_ju_kong_wang.gain_loss_critical"], None),
    ("ruo_feng_bai_di", "若逢败地，扶持大有奇功", ["fortune.ruo_feng_bai_di.support_turnaround"], None),
    ("zi_fu_yi_fu_bi", "紫微天府全依辅弼之功", ["status.zi_fu_yi_fu_bi.needs_assistant_stars"], None),
    ("sha_po_yi_yang_ling", "七杀破军专依羊铃之虐", ["risk.sha_po_yi_yang_ling.malefic_aggravation"], None),
    ("zhu_xing_ji", "诸星吉，逢凶也吉", ["fortune.zhu_xing_ji.benefic_dominance"], None),
    ("zhu_xing_xiong", "诸星凶，逢凶也凶", ["fortune.zhu_xing_xiong.malefic_dominance"], None),
    ("fu_bi_jia_di", "辅弼夹帝为上品", ["status.fu_bi_jia_di.top_grade"], None),
    ("tao_hua_fan_zhu", "桃花犯主为至淫", ["relationship.tao_hua_fan_zhu.excess_romance"], None),
    ("jun_chen_qing_hui", "君臣庆会，材善经邦", ["status.jun_chen_qing_hui.statesmanship"], None),
    ("kui_yue_tong_xing", "魁钺同行，位居台辅", ["status.kui_yue_tong_xing.office_support"], None),
    ("lu_wen_gong_ming", "禄文拱命，贵而且贤", ["status.lu_wen_gong_ming.noble_and_virtuous"], None),
    ("ri_yue_jia_cai", "日月夹财，不权则富", ["wealth.ri_yue_jia_cai.authority_or_wealth"], None),
    ("ma_tou_dai_jian", "马头带剑，镇卫边疆", ["risk.ma_tou_dai_jian.border_command"], None),
    ("xing_qiu_jia_yin", "刑囚夹印，刑杖惟司", ["risk.xing_qiu_jia_yin.judicial_punishment"], None),
    ("shan_yin_chao_gang", "善荫朝纲，仁慈之长", ["temperament.shan_yin_chao_gang.benevolent_leader"], None),
    ("gui_ru_gui_xiang", "贵入贵乡，逢之富贵", ["status.gui_ru_gui_xiang.nobility"], None),
    ("cai_ju_cai_wei", "财居财位，遇者富奢", ["wealth.cai_ju_cai_wei.affluence"], None),
    ("tai_yang_ju_wu", "太阳居午，谓之日丽中天，有专权之贵，敌国之富", ["career.tai_yang_ju_wu.authority_status"], None),
    ("tai_yin_ju_zi", "太阴居子，号曰水澄桂萼，得清要之职，忠谏之材", ["career.tai_yin_ju_zi.clean_office_admonition"], None),
    ("zi_wei_fu_bi_tong_gong", "紫微辅弼同宫，一呼百诺居上品", ["status.zi_wei_fu_bi_tong_gong.commanding_leader"], None),
    ("wen_hao_ju_yin_mao", "文耗居寅卯，谓之众水朝东", ["career.wen_hao_ju_yin_mao.water_flows_east"], None),
    ("ri_yue_shou_zhao_he", "日月守不如照合", ["method.ri_yue_shou_zhao_he.illumination_over_sitting"], None),
    ("yin_fu_ju", "荫福聚不怕凶危", ["fortune.yin_fu_ju.protection_from_harm"], None),
    ("tan_ju_hai_zi", "贪居亥子，名为犯水桃花", ["relationship.tan_ju_hai_zi.water_romance"], None),
    ("xing_yu_tan_lang", "刑遇贪狼，号曰风流彩杖", ["relationship.xing_yu_tan_lang.romance_with_penalty"], None),
    ("sha_lian_tong_wei", "七杀廉贞同位，路上埋尸", ["risk.sha_lian_tong_wei.violent_death"], None),
    ("po_an_tong_xiang", "破军暗曜同乡，水中作冢", ["risk.po_an_tong_xiang.drowning_risk"], None),
    ("lu_ju_nu_pu", "禄居奴仆纵有官也奔驰", ["wealth.lu_ju_nu_pu.toil_for_others"], None),
    ("di_yu_xiong_tu", "帝遇凶徒虽获吉而无道", ["status.di_yu_xiong_tu.power_without_virtue"], None),
    ("di_zuo_jin_che", "帝坐金车则曰金轝捧栉", ["status.di_zuo_jin_che.imperial_carriage"], None),
    ("fu_an_wen_yao", "福安文曜谓之玉袖天香", ["fortune.fu_an_wen_yao.refined_blessing"], None),
    ("tai_yang_hui_wen_chang", "太阳会文昌于官禄，皇殿朝班，富贵全美", ["career.tai_yang_hui_wen_chang.court_honor"], None),
    ("tai_yin_hui_wen_qu", "太阴会文曲于妻宫，蟾宫折桂，文章全盛", ["career.tai_yin_hui_wen_qu.literary_eminence"], None),
    ("lu_cun_tian_cai", "禄存守于田财，堆金积玉", ["wealth.lu_cun_tian_cai.asset_accumulation"], None),
    ("cai_yin_qian_yi", "财荫坐于迁移，巨商高贾", ["wealth.cai_yin_qian_yi.merchant_success"], None),
    ("hao_ju_lu_wei", "耗居禄位，沿途乞食", ["wealth.hao_ju_lu_wei.poverty"], None),
    ("tan_hui_wang_gong", "贪会旺宫，终身鼠窃", ["risk.tan_hui_wang_gong.thievery"], None),
    ("sha_ju_jue_di", "杀居绝地，天年夭似颜回", ["risk.sha_ju_jue_di.early_death"], None),
    ("tan_zuo_sheng_xiang", "贪坐生乡，寿考永如彭祖", ["fortune.tan_zuo_sheng_xiang.longevity"], None),
    ("ji_an_tong_ju", "忌暗同居身命疾厄，沉困尪赢", ["health.ji_an_tong_ju.chronic_illness"], None),
    ("xiong_xing_fu_mu_qian_yi", "凶星会于父母迁移，刑伤破祖", ["risk.xiong_xing_fu_mu_qian_yi.family_harm"], None),
    ("xing_sha_lian_zhen", "刑杀同廉贞于官禄，枷扭难逃", ["risk.xing_sha_lian_zhen.imprisonment"], None),
    ("guan_fu_xing_sha", "官符加刑杀于迁移，离乡遭配", ["risk.guan_fu_xing_sha.exile"], None),
    ("shan_fu_ju_kong", "善福居空位，天竺生涯", ["fortune.shan_fu_ju_kong.monastic_life"], None),
    ("fu_bi_dan_shou", "辅弼单守命宫，离宗庶出", ["life.fu_bi_dan_shou.illegitimate_or_adopted"], None),
    ("sha_lin_shen_ming", "七杀临于身命加恶杀，必定死亡", ["risk.sha_lin_shen_ming.fatal_affliction"], None),
    ("ling_yang_he_ming", "铃羊合于命宫遇白虎，须当刑戮", ["risk.ling_yang_he_ming.execution_risk"], None),
    ("guan_fu_fa_ji_yao", "官府发于吉曜，流杀怕逢破军", ["risk.guan_fu_fa_ji_yao.annual_malefic"], None),
    ("yang_tuo_ping_tai_sui", "羊陀凭太岁以引行，病符官符皆作祸", ["risk.yang_tuo_ping_tai_sui.annual_misfortune"], None),
    ("zou_shu_bo_shi", "奏书博士与流禄，尽作吉祥", ["fortune.zou_shu_bo_shi.auspicious_minor_stars"], None),
    ("li_shi_jiang_jun", "力士将军同青龙，显其权势", ["status.li_shi_jiang_jun.authority_minor_stars"], None),
    ("tong_zi_xian", "童子限如水上泡沤，老人限似风中燃烛", ["timing.tong_zi_xian.fragile_life_stages"], None),
    ("yu_sha_wu_zhi", "遇杀无制乃流年最忌", ["timing.yu_sha_wu_zhi.unchecked_malefic_year"],
     "「遇杀无制乃流年最忌」在原书与下句「人生荣辱限元必有休咎」连读，此处依语义切分为独立 source item。"),
    ("ren_sheng_rong_ru", "人生荣辱限元必有休咎", ["timing.ren_sheng_rong_ru.fortune_in_decades"], None),
    ("chu_shi_gu_pin", "处世孤贫数中逢乎驳杂", ["life.chu_shi_gu_pin.solitary_poverty"], None),
    ("xue_zhi_ci_xuan_wei", "学至此诚玄微矣", ["meta.xue_zhi_ci_xuan_wei.closing_remark"],
     "太微赋例曰之收束语（赞叹治学），非断语；以独立 source item 记录并链接到 rejected 规则。"),
]

assert len(items) == 64, len(items)

header = '''# 《紫微斗数全书》 source inventory — Volume 1 pilot slice.
#
# This file tracks atomic cited QuanShu source units (rule-candidate aphorisms)
# from volume-01.md, preserving source order. It does not duplicate
# explanatory/commentary prose; the raw full text remains in
# docs/zh-CN/sources/quan_shu/volume-01.md.
#
# Model (see docs/zh-CN/sources/quan_shu/README.md):
#
#   source item = one atomic cited QuanShu source unit / rule-candidate aphorism
#                 (identified by a stable mnemonic `source_id`)
#   rule        = an executable / normalized / ambiguous / rejected interpretation
#                 linked from a source item via `linked_rule_ids`
#
# A physical Markdown line may contain several source items. Source-item
# boundaries are semantic, not typographical: `。` is the default top-level
# breaker, and a single `。` sentence holding parallel independent aphorisms is
# split further; condition/result commas of one aphorism are not split.
#
# `source_id` identifies the cited source unit, not a physical line/passage, and
# is a stable mnemonic derived from the aphorism (e.g. `ma_yu_kong_wang`).
# `source_order` preserves source order separately from stable identity, so
# inserting an earlier aphorism only requires reviewing `source_order`, never
# rewriting stable `source_id` references.
#
# `source_text_zh_hans` quotes the cited source unit verbatim (no sentence-final
# `。`). Interpretation belongs in the linked rule's `normalized_note_zh_hans`,
# `ClaimSpec`, or i18n claim text — never here.
#
# This is a source inventory, not executable rule metadata. Nothing in `src/`
# parses it; it is validated by tests only (see
# crates/iztro/tests/classical_source_inventory.rs). It records only genuine
# QuanShu source units. The `section = "待校"` / `anchor = "TODO"` placeholders
# are reserved for units believed to be from QuanShu but not yet located in the
# Markdown volumes; none are present in this slice. Rules derived from modeled
# chart structures rather than a cited QuanShu source unit (e.g. 羊陀夹命,
# 昌曲夹命) live in rule-corpus/patterns/, not here.

# --- 太微赋「例曰」段（source inventory，原子化补全） ---
#
# 以下每个 source item 为「例曰」一句断语/规则候选，依 volume-01.md 语义顺序排列，
# 物理换行不决定 source item 边界。全部太微赋 source item 现已链接到运行时规则元数据
# （status = rule_linked）：
#   - 可执行 pilot（马遇空亡、日月反背）保持 executable；
#   - 禄马交驰保持 normalized；
#   - 其余 source item 已链接到 normalized / ambiguous 规则，或对收束语链接到
#     rejected 规则（学至此诚玄微矣）。
# 多数规则为 normalized/ambiguous 而非 executable：可执行覆盖刻意保守，规则元数据
# 的 status 字段如实标注成熟度，规则链接与状态分布见
# docs/zh-CN/rules/quan-shu-coverage.md。
'''

def esc(s):
    return s.replace('\\', '\\\\').replace('"', '\\"')

out = [header]
for i, (cid, text, rules_ids, note) in enumerate(items, start=1):
    rid_list = ", ".join('"%s"' % r for r in rules_ids)
    block = []
    block.append("[[source_item]]")
    block.append('source_id = "%s%s"' % (PREFIX, cid))
    block.append("source_order = %d" % i)
    block.append('work = "zi_wei_dou_shu_quan_shu"')
    block.append("volume = 1")
    block.append('section = "太微赋"')
    block.append('category = "aphorism_rule"')
    block.append('status = "rule_linked"')
    block.append('doc_path = "docs/zh-CN/sources/quan_shu/volume-01.md"')
    block.append('anchor = "太微赋"')
    block.append('source_text_zh_hans = "%s"' % esc(text))
    block.append('linked_rule_ids = [%s]' % rid_list)
    if note:
        block.append('notes_zh_hans = "%s"' % esc(note))
    out.append("\n".join(block))

SRC.write_text("\n" + "\n\n".join(out) + "\n", encoding="utf-8")
print("wrote", SRC, "with", len(items), "source items")

# ---- Transform rules.toml ----
txt = RULES.read_text(encoding="utf-8")

# Build clause_id -> new source_id map (= prefix + clause_id).
# For each rule block: replace its source_id line value with prefix+source_clause_id,
# and drop the source_clause_id line.
lines = txt.splitlines()
res = []
i = 0
while i < len(lines):
    line = lines[i]
    m = re.match(r'^source_id = "(.+)"$', line)
    if m and i + 1 < len(lines):
        m2 = re.match(r'^source_clause_id = "(.+)"$', lines[i + 1])
        if m2:
            clause = m2.group(1)
            res.append('source_id = "%s%s"' % (PREFIX, clause))
            i += 2  # skip source_clause_id line
            continue
    res.append(line)
    i += 1
txt = "\n".join(res) + ("\n" if txt.endswith("\n") else "")

# Fix ri_yue_fan_bei source_text fidelity.
txt = txt.replace(
    'source_text_zh_hans = "日月反背，劳碌辛苦"',
    'source_text_zh_hans = "日月最嫌反背"',
)

RULES.write_text(txt, encoding="utf-8")
print("transformed", RULES)
