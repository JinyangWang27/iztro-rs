# 《紫微斗数全书》语料覆盖报告

本报告统计 `crates/iztro/rule-corpus/quan-shu/source/` 中已结构化的 source inventory，仅覆盖《紫微斗数全书》出处条目，不包含项目 pattern/格局规则目录（`crates/iztro/rule-corpus/patterns/`）。

source inventory 以原子 source item 记录每条受引出处单元；`source_id` 为稳定助记符，`source_order` 单独保存出处顺序。规则经 `linked_rule_ids` 链接。

本报告由 `crates/iztro/tests/classical_source_coverage.rs` 生成并校验：修改 source inventory 或 rule corpus 后须重新生成本文件，否则测试 `quan_shu_coverage_report_is_current` 失败。

## Summary

| Metric | Count |
| --- | ---: |
| Source items | 117 |
| Located source items | 117 |
| Pending source items | 0 |
| Linked source items | 64 |
| Unlinked source items | 53 |
| Linked rules | 64 |
| Executable linked rules | 5 |
| Normalized linked rules | 41 |
| Ambiguous linked rules | 17 |
| Rejected linked rules | 1 |

## Volume 1 — 太微赋

| Metric | Count |
| --- | ---: |
| Source items | 64 |
| Linked source items | 64 |
| Unlinked source items | 0 |
| Pending source items | 0 |

## Volume 1 — 定富局

Category: `pattern_rule`

| Metric | Count |
| --- | ---: |
| Source items | 6 |
| Linked classical rule source items | 0 |
| Segmented pattern-only source items | 6 |
| Pending source items | 0 |

## Volume 1 — 定贵局

Category: `pattern_rule`

| Metric | Count |
| --- | ---: |
| Source items | 27 |
| Linked classical rule source items | 0 |
| Segmented pattern-only source items | 27 |
| Pending source items | 0 |

## Volume 1 — 定贫贱局

Category: `pattern_rule`

| Metric | Count |
| --- | ---: |
| Source items | 8 |
| Linked classical rule source items | 0 |
| Segmented pattern-only source items | 8 |
| Pending source items | 0 |

## Volume 1 — 定杂局

Category: `pattern_rule`

| Metric | Count |
| --- | ---: |
| Source items | 8 |
| Linked classical rule source items | 0 |
| Segmented pattern-only source items | 8 |
| Pending source items | 0 |

## Unlinked source items

| Source ID | Order | Text |
| --- | ---: | --- |
| quan_shu.v01.ding_fu_ju.cai_yin_jia_yin | 1 | 财荫夹印 相守命武梁来夹是也，田宅宫亦然 |
| quan_shu.v01.ding_fu_ju.ri_yue_jia_cai | 2 | 日月夹财 武守命日月来夹是也，财帛宫亦然 |
| quan_shu.v01.ding_fu_ju.cai_lu_jia_ma | 3 | 财禄夹马 马守命武禄来夹是也，逢生旺尤妙 |
| quan_shu.v01.ding_fu_ju.yin_yin_gong_shen | 4 | 荫印拱身 身临田宅梁相拱冲是也，勿坐空亡 |
| quan_shu.v01.ding_fu_ju.ri_yue_zhao_bi | 5 | 日月照璧 日月临田宅宫是也，喜居墓库 |
| quan_shu.v01.ding_fu_ju.jin_can_guang_hui | 6 | 金灿光辉 太阳单守，命在午宫是也 |
| quan_shu.v01.ding_gui_ju.ri_yue_jia_ming | 1 | 日月夹命 不坐空亡遇逢本宫有吉星是也 |
| quan_shu.v01.ding_gui_ju.ri_chu_fu_sang | 2 | 日出扶桑 日在卯守命是也，守官禄宫亦然 |
| quan_shu.v01.ding_gui_ju.yue_luo_hai_gong | 3 | 月落亥宫 月在亥守命是也，又名月朗天门 |
| quan_shu.v01.ding_gui_ju.yue_sheng_cang_hai | 4 | 月生沧海 月在子宫守田宅是也 |
| quan_shu.v01.ding_gui_ju.fu_bi_gong_zhu | 5 | 辅弼拱主 紫微守命二星来拱是也，夹之亦然 |
| quan_shu.v01.ding_gui_ju.jun_chen_qing_hui | 6 | 君臣庆会 紫微左右同守命是也，更会相武阴妙上 |
| quan_shu.v01.ding_gui_ju.cai_yin_jia_lu | 7 | 财印夹禄 禄守命梁相来夹是也，入财亦然 |
| quan_shu.v01.ding_gui_ju.lu_ma_pei_yin | 8 | 禄马佩印 马前有禄印星同宫是也 |
| quan_shu.v01.ding_gui_ju.zuo_gui_xiang_gui | 9 | 坐贵向贵 谓魁钺在命迭相坐拱是也 |
| quan_shu.v01.ding_gui_ju.ma_tou_dai_jian | 10 | 马头带剑 谓马有刃是也不是居午格 |
| quan_shu.v01.ding_gui_ju.qi_sha_chao_dou | 11 | 七杀朝斗 见前批注 |
| quan_shu.v01.ding_gui_ju.ri_yue_bing_ming | 12 | 日月并明 见前批注 |
| quan_shu.v01.ding_gui_ju.ming_zhu_chu_hai | 13 | 明珠出海 见前批注 |
| quan_shu.v01.ding_gui_ju.ri_yue_tong_lin | 14 | 日月同临 见前批注 |
| quan_shu.v01.ding_gui_ju.xing_qiu_jia_yin | 15 | 刑囚夹印 天刑廉贞同临身命主武勇之人 |
| quan_shu.v01.ding_gui_ju.ke_quan_lu_gong | 16 | 科权禄拱 见前批注 |
| quan_shu.v01.ding_gui_ju.tan_huo_xiang_feng | 17 | 贪火相逢 谓二星守命同居庙旺是也 |
| quan_shu.v01.ding_gui_ju.wu_qu_shou_yuan | 18 | 武曲守垣 武守命卯宫是也，余不是 |
| quan_shu.v01.ding_gui_ju.fu_xiang_chao_yuan | 19 | 府相朝垣 见前批注 |
| quan_shu.v01.ding_gui_ju.zi_fu_chao_yuan | 20 | 紫府朝垣 见前批注 |
| quan_shu.v01.ding_gui_ju.wen_xing_an_gong | 21 | 文星暗拱 见前批注 |
| quan_shu.v01.ding_gui_ju.quan_lu_sheng_feng | 22 | 权禄生逢 二星守命庙旺是也，陷不是 |
| quan_shu.v01.ding_gui_ju.yang_ren_ru_miao | 23 | 羊刃入庙 辰戍丑未守命遇吉是也 |
| quan_shu.v01.ding_gui_ju.ju_ji_ju_mao | 24 | 巨机居卯 见前批注 |
| quan_shu.v01.ding_gui_ju.ming_lu_an_lu | 25 | 明禄暗禄 见前批注 |
| quan_shu.v01.ding_gui_ju.ke_ming_an_lu | 26 | 科明暗禄 见前批注 |
| quan_shu.v01.ding_gui_ju.jin_yu_fu_jia | 27 | 金舆扶驾 紫微守命前后有日月来夹是也 |
| quan_shu.v01.ding_pin_jian_ju.sheng_bu_feng_shi | 1 | 生不逢时 命坐空亡逢廉贞是也 |
| quan_shu.v01.ding_pin_jian_ju.lu_feng_liang_sha | 2 | 禄逢两杀 禄坐空亡又逢空劫杀星是也 |
| quan_shu.v01.ding_pin_jian_ju.ma_luo_kong_wang | 3 | 马落空亡 马既落亡虽禄冲会无用主奔波 |
| quan_shu.v01.ding_pin_jian_ju.ri_yue_cang_hui | 4 | 日月藏辉 日月反背又逢巨暗是也 |
| quan_shu.v01.ding_pin_jian_ju.cai_yu_qiu_chou | 5 | 财与囚仇 武贞同守身命是也 |
| quan_shu.v01.ding_pin_jian_ju.yi_sheng_gu_pin | 6 | 一生孤贫 谓破守命星陷地是也 |
| quan_shu.v01.ding_pin_jian_ju.jun_zi_zai_ye | 7 | 君子在野 谓四杀守身命而言临陷地是也 |
| quan_shu.v01.ding_pin_jian_ju.liang_chong_hua_gai | 8 | 两重华盖 谓禄存化禄坐命遇空劫是也 |
| quan_shu.v01.ding_za_ju.feng_yun_ji_hui | 1 | 风云际会 身命虽弱二限逢禄马是也 |
| quan_shu.v01.ding_za_ju.jin_shang_tian_hua | 2 | 锦上添花 谓限破恶星而行吉地是也 |
| quan_shu.v01.ding_za_ju.lu_shuai_ma_kun | 3 | 禄衰马困 限逢七杀禄马空亡是也 |
| quan_shu.v01.ding_za_ju.yi_jin_huan_xiang | 4 | 衣锦还乡 少年不遂四十后行墓运是也 |
| quan_shu.v01.ding_za_ju.bu_shu_wu_yi | 5 | 步数无依 前限接后限连绵不分是也 |
| quan_shu.v01.ding_za_ju.shui_shang_jia_xing | 6 | 水上驾星 一年好一年不好是也 |
| quan_shu.v01.ding_za_ju.ji_xiong_xiang_ban | 7 | 吉凶相伴 命有主星限前则发限衰不发是也 |
| quan_shu.v01.ding_za_ju.ku_mu_feng_chun | 8 | 枯木逢春 谓命衰限好是也 |
| quan_shu.v01.dou_shu_gu_sui_fu.shi_zhong_yin_yu | 1 | 子午巨门石中隐玉，明禄暗禄锦上添花 |
| quan_shu.v03.zhu_xing_tong_yuan.zi_fu_jia_ming | 1 | 紫府夹命为贵格 |
| quan_shu.v03.zhu_xing_tong_yuan.lian_zhen_qi_sha_miao_wang | 2 | 廉贞七杀居庙旺反为积富之人 杀居午奇格，若陷地化忌，贫贱残疾 |
| quan_shu.v03.zhu_xing_tong_yuan.qing_yang_ru_miao | 3 | 擎羊入庙富贵声扬 加吉万论 |
