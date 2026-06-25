# 《紫微斗数全书》语料覆盖报告

本报告统计 `crates/iztro/rule-corpus/quan-shu/source/` 中已结构化的 source inventory，仅覆盖《紫微斗数全书》出处条目，不包含项目 pattern/格局规则目录（`crates/iztro/rule-corpus/patterns/`）。

本报告由 `crates/iztro/tests/classical_source_coverage.rs` 生成并校验：修改 source inventory 或 rule corpus 后须重新生成本文件，否则测试 `quan_shu_coverage_report_is_current` 失败。

## Summary

| Metric | Count |
| --- | ---: |
| Source items | 38 |
| Located source items | 38 |
| Pending source items | 0 |
| Clauses | 64 |
| Linked clauses | 3 |
| Unlinked clauses | 61 |
| Linked rules | 3 |
| Executable linked rules | 2 |
| Normalized linked rules | 1 |
| Ambiguous linked rules | 0 |
| Rejected linked rules | 0 |

## Volume 1 — 太微赋

| Metric | Count |
| --- | ---: |
| Source items | 38 |
| Clauses | 64 |
| Linked clauses | 3 |
| Unlinked clauses | 61 |
| Pending source items | 0 |

## Unlinked clauses

| Source ID | Clause ID | Text |
| --- | --- | --- |
| quan_shu.v01.tai_wei_fu.001 | lu_feng_chong_po | 禄逢冲破，吉处藏凶 |
| quan_shu.v01.tai_wei_fu.002 | sheng_feng_bai_di | 生逢败地，发也虚花 |
| quan_shu.v01.tai_wei_fu.002 | jue_chu_feng_sheng | 绝处逢生，生花不败 |
| quan_shu.v01.tai_wei_fu.003 | xing_lin_miao_wang | 星临庙旺，再观生克之机 |
| quan_shu.v01.tai_wei_fu.003 | ming_zuo_qiang_gong | 命坐强宫，细察制化之理 |
| quan_shu.v01.tai_wei_fu.005 | tang_ju_kong_wang | 倘居空亡，得失最为要紧 |
| quan_shu.v01.tai_wei_fu.005 | ruo_feng_bai_di | 若逢败地，扶持大有奇功 |
| quan_shu.v01.tai_wei_fu.006 | zi_fu_yi_fu_bi | 紫微天府全依辅弼之功 |
| quan_shu.v01.tai_wei_fu.006 | sha_po_yi_yang_ling | 七杀破军专依羊铃之虐 |
| quan_shu.v01.tai_wei_fu.007 | zhu_xing_ji | 诸星吉，逢凶也吉 |
| quan_shu.v01.tai_wei_fu.007 | zhu_xing_xiong | 诸星凶，逢凶也凶 |
| quan_shu.v01.tai_wei_fu.008 | fu_bi_jia_di | 辅弼夹帝为上品 |
| quan_shu.v01.tai_wei_fu.008 | tao_hua_fan_zhu | 桃花犯主为至淫 |
| quan_shu.v01.tai_wei_fu.009 | jun_chen_qing_hui | 君臣庆会，材善经邦 |
| quan_shu.v01.tai_wei_fu.010 | kui_yue_tong_xing | 魁钺同行，位居台辅 |
| quan_shu.v01.tai_wei_fu.011 | lu_wen_gong_ming | 禄文拱命，贵而且贤 |
| quan_shu.v01.tai_wei_fu.012 | ri_yue_jia_cai | 日月夹财，不权则富 |
| quan_shu.v01.tai_wei_fu.013 | ma_tou_dai_jian | 马头带剑，镇卫边疆 |
| quan_shu.v01.tai_wei_fu.014 | xing_qiu_jia_yin | 刑囚夹印，刑杖惟司 |
| quan_shu.v01.tai_wei_fu.015 | shan_yin_chao_gang | 善荫朝纲，仁慈之长 |
| quan_shu.v01.tai_wei_fu.016 | gui_ru_gui_xiang | 贵入贵乡，逢之富贵 |
| quan_shu.v01.tai_wei_fu.017 | cai_ju_cai_wei | 财居财位，遇者富奢 |
| quan_shu.v01.tai_wei_fu.018 | tai_yang_ju_wu | 太阳居午，谓之日丽中天，有专权之贵，敌国之富 |
| quan_shu.v01.tai_wei_fu.019 | tai_yin_ju_zi | 太阴居子，号曰水澄桂萼，得清要之职，忠谏之材 |
| quan_shu.v01.tai_wei_fu.020 | zi_wei_fu_bi_tong_gong | 紫微辅弼同宫，一呼百诺居上品 |
| quan_shu.v01.tai_wei_fu.020 | wen_hao_ju_yin_mao | 文耗居寅卯，谓之众水朝东 |
| quan_shu.v01.tai_wei_fu.021 | ri_yue_shou_zhao_he | 日月守不如照合 |
| quan_shu.v01.tai_wei_fu.021 | yin_fu_ju | 荫福聚不怕凶危 |
| quan_shu.v01.tai_wei_fu.022 | tan_ju_hai_zi | 贪居亥子，名为犯水桃花 |
| quan_shu.v01.tai_wei_fu.022 | xing_yu_tan_lang | 刑遇贪狼，号曰风流彩杖 |
| quan_shu.v01.tai_wei_fu.023 | sha_lian_tong_wei | 七杀廉贞同位，路上埋尸 |
| quan_shu.v01.tai_wei_fu.023 | po_an_tong_xiang | 破军暗曜同乡，水中作冢 |
| quan_shu.v01.tai_wei_fu.024 | lu_ju_nu_pu | 禄居奴仆纵有官也奔驰 |
| quan_shu.v01.tai_wei_fu.024 | di_yu_xiong_tu | 帝遇凶徒虽获吉而无道 |
| quan_shu.v01.tai_wei_fu.025 | di_zuo_jin_che | 帝坐金车则曰金轝捧栉 |
| quan_shu.v01.tai_wei_fu.025 | fu_an_wen_yao | 福安文曜谓之玉袖天香 |
| quan_shu.v01.tai_wei_fu.026 | tai_yang_hui_wen_chang | 太阳会文昌于官禄，皇殿朝班，富贵全美 |
| quan_shu.v01.tai_wei_fu.027 | tai_yin_hui_wen_qu | 太阴会文曲于妻宫，蟾宫折桂，文章全盛 |
| quan_shu.v01.tai_wei_fu.028 | lu_cun_tian_cai | 禄存守于田财，堆金积玉 |
| quan_shu.v01.tai_wei_fu.028 | cai_yin_qian_yi | 财荫坐于迁移，巨商高贾 |
| quan_shu.v01.tai_wei_fu.029 | hao_ju_lu_wei | 耗居禄位，沿途乞食 |
| quan_shu.v01.tai_wei_fu.029 | tan_hui_wang_gong | 贪会旺宫，终身鼠窃 |
| quan_shu.v01.tai_wei_fu.030 | sha_ju_jue_di | 杀居绝地，天年夭似颜回 |
| quan_shu.v01.tai_wei_fu.030 | tan_zuo_sheng_xiang | 贪坐生乡，寿考永如彭祖 |
| quan_shu.v01.tai_wei_fu.031 | ji_an_tong_ju | 忌暗同居身命疾厄，沉困尪赢 |
| quan_shu.v01.tai_wei_fu.031 | xiong_xing_fu_mu_qian_yi | 凶星会于父母迁移，刑伤破祖 |
| quan_shu.v01.tai_wei_fu.032 | xing_sha_lian_zhen | 刑杀同廉贞于官禄，枷扭难逃 |
| quan_shu.v01.tai_wei_fu.032 | guan_fu_xing_sha | 官符加刑杀于迁移，离乡遭配 |
| quan_shu.v01.tai_wei_fu.033 | shan_fu_ju_kong | 善福居空位，天竺生涯 |
| quan_shu.v01.tai_wei_fu.033 | fu_bi_dan_shou | 辅弼单守命宫，离宗庶出 |
| quan_shu.v01.tai_wei_fu.034 | sha_lin_shen_ming | 七杀临于身命加恶杀，必定死亡 |
| quan_shu.v01.tai_wei_fu.034 | ling_yang_he_ming | 铃羊合于命宫遇白虎，须当刑戮 |
| quan_shu.v01.tai_wei_fu.035 | guan_fu_fa_ji_yao | 官府发于吉曜，流杀怕逢破军 |
| quan_shu.v01.tai_wei_fu.035 | yang_tuo_ping_tai_sui | 羊陀凭太岁以引行，病符官符皆作祸 |
| quan_shu.v01.tai_wei_fu.036 | zou_shu_bo_shi | 奏书博士与流禄，尽作吉祥 |
| quan_shu.v01.tai_wei_fu.036 | li_shi_jiang_jun | 力士将军同青龙，显其权势 |
| quan_shu.v01.tai_wei_fu.037 | tong_zi_xian | 童子限如水上泡沤，老人限似风中燃烛 |
| quan_shu.v01.tai_wei_fu.037 | yu_sha_wu_zhi | 遇杀无制乃流年最忌 |
| quan_shu.v01.tai_wei_fu.038 | ren_sheng_rong_ru | 人生荣辱限元必有休咎 |
| quan_shu.v01.tai_wei_fu.038 | chu_shi_gu_pin | 处世孤贫数中逢乎驳杂 |
| quan_shu.v01.tai_wei_fu.038 | xue_zhi_ci_xuan_wei | 学至此诚玄微矣 |
