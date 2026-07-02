# 《紫微斗数全书》语料覆盖报告

本报告统计 `crates/iztro/rule-corpus/quan-shu/source/` 中已结构化的 source inventory，仅覆盖《紫微斗数全书》出处条目，不包含项目 pattern/格局规则目录（`crates/iztro/rule-corpus/patterns/`）。

source inventory 以原子 source item 记录每条受引出处单元；`source_id` 为稳定助记符，`source_order` 单独保存出处顺序。规则经 `linked_rule_ids` 链接。

本报告由 `crates/iztro/tests/classical_source_coverage.rs` 生成并校验：修改 source inventory 或 rule corpus 后须重新生成本文件，否则测试 `quan_shu_coverage_report_is_current` 失败。

## Summary

| Metric | Count |
| --- | ---: |
| Source items | 270 |
| Located source items | 270 |
| Pending source items | 0 |
| Linked source items | 64 |
| Unlinked source items | 206 |
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

## Volume 1 — 斗数骨髓赋

Category: `aphorism_rule`

| Metric | Count |
| --- | ---: |
| Source items | 154 |
| Linked classical rule source items | 0 |
| Segmented aphorism source items | 154 |
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
| quan_shu.v01.dou_shu_gu_sui_fu.tai_ji_xing_chan | 1 | 太极星缠，乃群宿众星之主，天门运限，即扶身助命之源，在天则运用无常，在人则命有格局 |
| quan_shu.v01.dou_shu_gu_sui_fu.xian_ming_ge_ju | 2 | 先明格局，次看众星 |
| quan_shu.v01.dou_shu_gu_sui_fu.tong_nian_tong_shi | 3 | 或有同年同月同日同时而生，则有贫贱富贵寿夭之异 |
| quan_shu.v01.dou_shu_gu_sui_fu.e_xian_ji_bai_wan | 4 | 或在恶限，积百万之金银 |
| quan_shu.v01.dou_shu_gu_sui_fu.wang_xiang_lian_nian_kun | 5 | 或在旺乡，遭连年之困苦 |
| quan_shu.v01.dou_shu_gu_sui_fu.huo_fu_bu_ke_yi_tu | 6 | 祸福不可一途而惟，吉凶不可一例而断 |
| quan_shu.v01.dou_shu_gu_sui_fu.yi_shi_rong_ku | 7 | 要知一世之荣枯，定看五行之宫位 |
| quan_shu.v01.dou_shu_gu_sui_fu.li_ming_gui_jian | 8 | 立命可知贵贱 |
| quan_shu.v01.dou_shu_gu_sui_fu.an_shen_gen_ji | 9 | 安身便晓根基 |
| quan_shu.v01.dou_shu_gu_sui_fu.xian_kan_fu_de | 10 | 第一先看福德，再三细考迁移，分对宫之体用，定三合之源流 |
| quan_shu.v01.dou_shu_gu_sui_fu.ming_wu_zheng_yao | 11 | 命无正曜，夭折孤贫 |
| quan_shu.v01.dou_shu_gu_sui_fu.ji_you_xiong_xing | 12 | 吉有凶星，美玉瑕玷 |
| quan_shu.v01.dou_shu_gu_sui_fu.gen_yuan_jian_gu | 13 | 既得根源坚固，须知合局相生，坚固则富贵延寿，相生则财官昭著 |
| quan_shu.v01.dou_shu_gu_sui_fu.ming_hao_shen_hao | 14 | 命好身好限好到老荣昌 |
| quan_shu.v01.dou_shu_gu_sui_fu.ming_shuai_shen_shuai | 15 | 命衰身衰限衰终身乞丐 |
| quan_shu.v01.dou_shu_gu_sui_fu.jia_gui_jia_lu | 16 | 夹贵夹禄少人知 |
| quan_shu.v01.dou_shu_gu_sui_fu.jia_quan_jia_ke | 17 | 夹权夹科世所宜 |
| quan_shu.v01.dou_shu_gu_sui_fu.jia_ri_jia_yue | 18 | 夹日夹月谁能遇 |
| quan_shu.v01.dou_shu_gu_sui_fu.jia_chang_jia_qu | 19 | 夹昌夹曲主贵兮 |
| quan_shu.v01.dou_shu_gu_sui_fu.jia_kong_jia_jie | 20 | 夹空夹劫主贫贱 |
| quan_shu.v01.dou_shu_gu_sui_fu.jia_yang_jia_tuo | 21 | 夹羊夹陀为乞丐 |
| quan_shu.v01.dou_shu_gu_sui_fu.lian_zhen_qi_sha | 22 | 廉贞七杀反为积富之人 |
| quan_shu.v01.dou_shu_gu_sui_fu.tian_liang_tai_yin | 23 | 天梁太阴却作飘蓬之客 |
| quan_shu.v01.dou_shu_gu_sui_fu.lian_zhen_xia_jian | 24 | 廉贞主下贱孤寒 |
| quan_shu.v01.dou_shu_gu_sui_fu.tai_yin_kuai_le | 25 | 太阴主一生快乐 |
| quan_shu.v01.dou_shu_gu_sui_fu.xian_pin_hou_fu_wu_tan | 26 | 先贫后富武贪同身命之宫 |
| quan_shu.v01.dou_shu_gu_sui_fu.xian_fu_hou_pin_jie_sha | 27 | 先富后贫只为运限逢劫杀 |
| quan_shu.v01.dou_shu_gu_sui_fu.quan_lu_cai_guan | 28 | 出世荣华权禄守财官之位 |
| quan_shu.v01.dou_shu_gu_sui_fu.jie_kong_cai_fu | 29 | 生来贫贱劫空临财福之乡 |
| quan_shu.v01.dou_shu_gu_sui_fu.wen_qu_wu_qu | 30 | 文曲武曲为人多学多能 |
| quan_shu.v01.dou_shu_gu_sui_fu.zuo_fu_you_bi | 31 | 左辅右弼禀性克宽克厚 |
| quan_shu.v01.dou_shu_gu_sui_fu.tian_fu_tian_xiang | 32 | 天府天相乃为衣禄之神，为仕为官定主亨通之兆 |
| quan_shu.v01.dou_shu_gu_sui_fu.ke_ming_xian_xiong | 33 | 苗而不秀科名陷于凶神 |
| quan_shu.v01.dou_shu_gu_sui_fu.cai_lu_ruo_di | 34 | 发不主财禄主躔于弱地 |
| quan_shu.v01.dou_shu_gu_sui_fu.qi_sha_chao_dou | 35 | 七杀朝斗爵禄荣昌 |
| quan_shu.v01.dou_shu_gu_sui_fu.zi_fu_tong_gong | 36 | 紫府同宫终身福厚 |
| quan_shu.v01.dou_shu_gu_sui_fu.zi_wei_ju_wu | 37 | 紫微居午无杀凑位至三公 |
| quan_shu.v01.dou_shu_gu_sui_fu.tian_fu_lin_xu | 38 | 天府临戌有星扶腰金衣紫 |
| quan_shu.v01.dou_shu_gu_sui_fu.ke_quan_lu_gong | 39 | 科权禄拱名誉昭彰 |
| quan_shu.v01.dou_shu_gu_sui_fu.wu_qu_miao_yuan | 40 | 武曲庙垣威名赫奕 |
| quan_shu.v01.dou_shu_gu_sui_fu.ke_ming_lu_an | 41 | 科明禄暗位列三台 |
| quan_shu.v01.dou_shu_gu_sui_fu.ri_yue_tong_lin | 42 | 日月同临官居侯伯 |
| quan_shu.v01.dou_shu_gu_sui_fu.ju_ji_tong_gong | 43 | 巨机同宫公卿之位 |
| quan_shu.v01.dou_shu_gu_sui_fu.tan_ling_bing_shou | 44 | 贪铃并守将相之名 |
| quan_shu.v01.dou_shu_gu_sui_fu.tian_kui_tian_yue | 45 | 天魁天钺盖世文章 |
| quan_shu.v01.dou_shu_gu_sui_fu.tian_lu_tian_ma | 46 | 天禄天马惊人甲第 |
| quan_shu.v01.dou_shu_gu_sui_fu.zuo_fu_wen_chang | 47 | 左辅文昌会吉星尊居八座 |
| quan_shu.v01.dou_shu_gu_sui_fu.tan_lang_huo_xing | 48 | 贪狼火星居庙旺名镇诸邦 |
| quan_shu.v01.dou_shu_gu_sui_fu.ju_ri_tong_gong | 49 | 巨日同宫官封三代 |
| quan_shu.v01.dou_shu_gu_sui_fu.zi_fu_chao_yuan | 50 | 紫府朝垣食禄万钟 |
| quan_shu.v01.dou_shu_gu_sui_fu.ke_quan_dui_gong | 51 | 科权对拱跃三汲于禹门 |
| quan_shu.v01.dou_shu_gu_sui_fu.ri_yue_bing_ming | 52 | 日月并明佐九重于尧殿 |
| quan_shu.v01.dou_shu_gu_sui_fu.fu_xiang_ming_gong | 53 | 府相同来会命宫全家食禄 |
| quan_shu.v01.dou_shu_gu_sui_fu.san_he_ming_zhu | 54 | 三合明珠生旺地稳步蟾宫 |
| quan_shu.v01.dou_shu_gu_sui_fu.qi_sha_po_jun | 55 | 七杀破军宜出外 |
| quan_shu.v01.dou_shu_gu_sui_fu.ji_yue_tong_liang | 56 | 机月同梁作吏人 |
| quan_shu.v01.dou_shu_gu_sui_fu.zi_fu_ri_yue_wang_di | 57 | 紫府日月居旺地断定公侯器 |
| quan_shu.v01.dou_shu_gu_sui_fu.ri_yue_ke_lu_chou | 58 | 日月科禄丑宫中定是方伯公 |
| quan_shu.v01.dou_shu_gu_sui_fu.tian_liang_tian_ma_xian | 59 | 天梁天马陷飘荡无疑 |
| quan_shu.v01.dou_shu_gu_sui_fu.lian_zhen_sha_bu_jia | 60 | 廉贞杀不加声名远播 |
| quan_shu.v01.dou_shu_gu_sui_fu.ri_zhao_lei_men | 61 | 日照雷门富贵荣华 |
| quan_shu.v01.dou_shu_gu_sui_fu.yue_lang_tian_men | 62 | 月朗天门进爵封侯 |
| quan_shu.v01.dou_shu_gu_sui_fu.yin_feng_fu_xiang | 63 | 寅逢府相位登一品之荣 |
| quan_shu.v01.dou_shu_gu_sui_fu.mu_feng_zuo_you | 64 | 墓逢左右尊居八座之贵 |
| quan_shu.v01.dou_shu_gu_sui_fu.liang_ju_wu_wei | 65 | 梁居午位官资清显 |
| quan_shu.v01.dou_shu_gu_sui_fu.qu_yu_liang_xing | 66 | 曲遇梁星位至台纲 |
| quan_shu.v01.dou_shu_gu_sui_fu.ke_lu_xun_feng | 67 | 科禄巡逢周勃欣然入相 |
| quan_shu.v01.dou_shu_gu_sui_fu.wen_xing_an_gong | 68 | 文星暗拱贾谊允矣登科 |
| quan_shu.v01.dou_shu_gu_sui_fu.qing_yang_huo_xing | 69 | 擎羊火星威权出众 |
| quan_shu.v01.dou_shu_gu_sui_fu.tan_wu_tong_xing | 70 | 贪武同行威镇边夷 |
| quan_shu.v01.dou_shu_gu_sui_fu.li_guang_bu_feng | 71 | 李广不封擎羊逢于力士 |
| quan_shu.v01.dou_shu_gu_sui_fu.yan_hui_yao_zhe | 72 | 颜回夭折文昌陷于天伤 |
| quan_shu.v01.dou_shu_gu_sui_fu.zhong_you_wei_meng | 73 | 仲由威猛廉贞入庙遇将军 |
| quan_shu.v01.dou_shu_gu_sui_fu.zi_yu_cai_neng | 74 | 子羽才能巨宿同梁冲且合 |
| quan_shu.v01.dou_shu_gu_sui_fu.yin_shen_tong_liang | 75 | 寅申最喜同梁会 |
| quan_shu.v01.dou_shu_gu_sui_fu.chen_xu_xian_ju_men | 76 | 辰戌应嫌陷巨门 |
| quan_shu.v01.dou_shu_gu_sui_fu.lu_dao_ma_dao | 77 | 禄倒马倒忌太岁之合劫空 |
| quan_shu.v01.dou_shu_gu_sui_fu.yun_shuai_xian_shuai | 78 | 运衰限衰喜紫微之解凶厄 |
| quan_shu.v01.dou_shu_gu_sui_fu.gu_pin_duo_shou | 79 | 孤贫多有寿 |
| quan_shu.v01.dou_shu_gu_sui_fu.fu_gui_yao_wang | 80 | 富贵即夭亡 |
| quan_shu.v01.dou_shu_gu_sui_fu.diao_ke_sang_men | 81 | 吊客丧门绿珠有堕楼之厄 |
| quan_shu.v01.dou_shu_gu_sui_fu.guan_fu_tai_sui | 82 | 官符太岁公冶有缧绁之忧 |
| quan_shu.v01.dou_shu_gu_sui_fu.xian_zhi_tian_luo_di_wang | 83 | 限至天罗地网屈原溺水而身亡 |
| quan_shu.v01.dou_shu_gu_sui_fu.yun_yu_di_jie_tian_kong | 84 | 运遇地劫天空阮籍有贫穷之苦 |
| quan_shu.v01.dou_shu_gu_sui_fu.wen_chang_wen_qu_hui_lian_zhen | 85 | 文昌文曲会廉贞丧命天年 |
| quan_shu.v01.dou_shu_gu_sui_fu.ming_kong_xian_kong | 86 | 命空限空无吉凑功名蹭蹬 |
| quan_shu.v01.dou_shu_gu_sui_fu.sheng_feng_tian_kong | 87 | 生逢天空犹如半天折翅 |
| quan_shu.v01.dou_shu_gu_sui_fu.ming_zhong_yu_jie | 88 | 命中遇劫恰如浪里行船 |
| quan_shu.v01.dou_shu_gu_sui_fu.xiang_yu_ying_xiong | 89 | 项羽英雄限至天空而丧国 |
| quan_shu.v01.dou_shu_gu_sui_fu.shi_chong_hao_fu | 90 | 石崇豪富限行劫地以亡家 |
| quan_shu.v01.dou_shu_gu_sui_fu.lv_hou_zhuan_quan | 91 | 吕后专权两重天禄天马 |
| quan_shu.v01.dou_shu_gu_sui_fu.yang_fei_hao_se | 92 | 杨妃好色三合文曲文昌 |
| quan_shu.v01.dou_shu_gu_sui_fu.tian_liang_yu_ma | 93 | 天梁遇马女命贱而且淫 |
| quan_shu.v01.dou_shu_gu_sui_fu.chang_qu_jia_chi | 94 | 昌曲夹墀男命贵而且显 |
| quan_shu.v01.dou_shu_gu_sui_fu.ji_ju_mao_you | 95 | 极居卯酉多为脱俗僧人 |
| quan_shu.v01.dou_shu_gu_sui_fu.zhen_ju_mao_you | 96 | 贞居卯酉定是公胥吏辈 |
| quan_shu.v01.dou_shu_gu_sui_fu.zuo_fu_tong_gong | 97 | 左府同宫尊居万乘 |
| quan_shu.v01.dou_shu_gu_sui_fu.lian_zhen_qi_sha_liu_dang | 98 | 廉贞七杀流荡天涯 |
| quan_shu.v01.dou_shu_gu_sui_fu.deng_tong_e_si | 99 | 邓通饿死运逢大耗之乡 |
| quan_shu.v01.dou_shu_gu_sui_fu.fu_zi_jue_liang | 100 | 夫子绝粮限到天伤之内 |
| quan_shu.v01.dou_shu_gu_sui_fu.ling_chang_luo_wu | 101 | 铃昌罗武限至投河 |
| quan_shu.v01.dou_shu_gu_sui_fu.ju_huo_qing_yang | 102 | 巨火擎羊终身缢死 |
| quan_shu.v01.dou_shu_gu_sui_fu.ming_li_feng_kong | 103 | 命里逢空不飘流即主疾苦 |
| quan_shu.v01.dou_shu_gu_sui_fu.ma_tou_dai_jian | 104 | 马头带剑非夭折则主刑伤 |
| quan_shu.v01.dou_shu_gu_sui_fu.zi_wu_po_jun | 105 | 子午破军加官进禄 |
| quan_shu.v01.dou_shu_gu_sui_fu.chang_tan_ju_ming | 106 | 昌贪居命粉骨碎尸 |
| quan_shu.v01.dou_shu_gu_sui_fu.chao_dou_yang_dou | 107 | 朝斗仰斗爵禄荣昌 |
| quan_shu.v01.dou_shu_gu_sui_fu.wen_gui_wen_hua | 108 | 文桂文华九重贵显 |
| quan_shu.v01.dou_shu_gu_sui_fu.dan_chi_gui_chi | 109 | 丹墀桂墀早遂青云之志 |
| quan_shu.v01.dou_shu_gu_sui_fu.he_lu_gong_lu | 110 | 合禄拱禄定为巨擘之臣 |
| quan_shu.v01.dou_shu_gu_sui_fu.yin_yang_hui_chang_qu | 111 | 阴阳会昌曲出世荣华 |
| quan_shu.v01.dou_shu_gu_sui_fu.fu_bi_yu_cai_guan | 112 | 辅弼遇财官衣绯着紫 |
| quan_shu.v01.dou_shu_gu_sui_fu.ju_liang_xiang_hui | 113 | 巨梁相会廉贞并 |
| quan_shu.v01.dou_shu_gu_sui_fu.he_lu_yuan_yang | 114 | 合禄鸳鸯一世荣 |
| quan_shu.v01.dou_shu_gu_sui_fu.wu_qu_xian_gong | 115 | 武曲闲宫多手艺 |
| quan_shu.v01.dou_shu_gu_sui_fu.tan_lang_xian_di | 116 | 贪狼陷地作屠人 |
| quan_shu.v01.dou_shu_gu_sui_fu.tian_lu_chao_yuan | 117 | 天禄朝垣身荣贵显 |
| quan_shu.v01.dou_shu_gu_sui_fu.kui_xing_lin_ming | 118 | 魁星临命位列三台 |
| quan_shu.v01.dou_shu_gu_sui_fu.wu_qu_ju_gan_xu_hai | 119 | 武曲居干戌亥上，最怕太阴逢贪狼 |
| quan_shu.v01.dou_shu_gu_sui_fu.hua_lu_mu_zhong | 120 | 化禄还为好，休向墓中藏 |
| quan_shu.v01.dou_shu_gu_sui_fu.shi_zhong_yin_yu | 121 | 子午巨门石中隐玉，明禄暗禄锦上添花 |
| quan_shu.v01.dou_shu_gu_sui_fu.zi_wei_chen_xu_po_jun | 122 | 紫微辰戌遇破军，富而不贵有虚名 |
| quan_shu.v01.dou_shu_gu_sui_fu.chang_qu_po_jun | 123 | 昌曲破军逢刑克多劳碌 |
| quan_shu.v01.dou_shu_gu_sui_fu.tan_wu_mu_zhong | 124 | 贪武墓中居三十才发福 |
| quan_shu.v01.dou_shu_gu_sui_fu.tian_tong_xu_gong | 125 | 天同戌宫为反背，丁人化吉主大贵 |
| quan_shu.v01.dou_shu_gu_sui_fu.ju_men_chen_xu | 126 | 巨门辰戌为陷地，辛人化吉禄峥嵘 |
| quan_shu.v01.dou_shu_gu_sui_fu.ji_liang_you_shang | 127 | 机梁酉上化吉者，纵遇财官也不荣 |
| quan_shu.v01.dou_shu_gu_sui_fu.ri_yue_fan_bei_shi_hui | 128 | 日月最嫌反背乃为失辉 |
| quan_shu.v01.dou_shu_gu_sui_fu.ming_shen_jing_qiu | 129 | 命身定要精求恐差分数 |
| quan_shu.v01.dou_shu_gu_sui_fu.yin_zhi_yan_nian | 130 | 阴骘延年增百福，至于陷地不遭伤 |
| quan_shu.v01.dou_shu_gu_sui_fu.ming_shi_yun_jian | 131 | 命实运坚稿田得雨 |
| quan_shu.v01.dou_shu_gu_sui_fu.ming_shuai_xian_ruo | 132 | 命衰限弱嫩草遭霜 |
| quan_shu.v01.dou_shu_gu_sui_fu.lun_ming_xing_shan_e | 133 | 论命必推星善恶 |
| quan_shu.v01.dou_shu_gu_sui_fu.ju_po_qing_yang | 134 | 巨破擎羊性必刚 |
| quan_shu.v01.dou_shu_gu_sui_fu.fu_xiang_tong_liang | 135 | 府相同梁性必好 |
| quan_shu.v01.dou_shu_gu_sui_fu.huo_jie_kong_tan | 136 | 火劫空贪性不常 |
| quan_shu.v01.dou_shu_gu_sui_fu.chang_qu_lu_ji | 137 | 昌曲禄机清秀巧 |
| quan_shu.v01.dou_shu_gu_sui_fu.yin_yang_zuo_you | 138 | 阴阳左右最慈祥 |
| quan_shu.v01.dou_shu_gu_sui_fu.wu_po_zhen_tan | 139 | 武破贞贪冲合曲全固贵 |
| quan_shu.v01.dou_shu_gu_sui_fu.yang_tuo_qi_sha | 140 | 羊陀七杀相杂互见则伤 |
| quan_shu.v01.dou_shu_gu_sui_fu.tan_lang_lian_zhen_po_jun | 141 | 贪狼廉贞破军恶 |
| quan_shu.v01.dou_shu_gu_sui_fu.qi_sha_qing_yang_tuo_luo | 142 | 七杀擎羊陀罗凶 |
| quan_shu.v01.dou_shu_gu_sui_fu.huo_xing_ling_xing | 143 | 火星铃星专作祸 |
| quan_shu.v01.dou_shu_gu_sui_fu.jie_kong_shang_shi | 144 | 劫空伤使祸重重 |
| quan_shu.v01.dou_shu_gu_sui_fu.ju_men_ji_xing | 145 | 巨门忌星皆不吉 |
| quan_shu.v01.dou_shu_gu_sui_fu.yun_shen_ming_xian_ji | 146 | 运身命限忌相逢 |
| quan_shu.v01.dou_shu_gu_sui_fu.tai_sui_guan_fu | 147 | 更兼太岁官符至，官非口舌决不空 |
| quan_shu.v01.dou_shu_gu_sui_fu.diao_ke_sang_men_you_yu | 148 | 吊客丧门又相遇，管教灾病两相攻 |
| quan_shu.v01.dou_shu_gu_sui_fu.qi_sha_shou_shen | 149 | 七杀守身终是夭 |
| quan_shu.v01.dou_shu_gu_sui_fu.tan_lang_ru_ming | 150 | 贪狼入命必为娼 |
| quan_shu.v01.dou_shu_gu_sui_fu.xin_hao_ming_wei | 151 | 心好命微亦主寿 |
| quan_shu.v01.dou_shu_gu_sui_fu.xin_du_ming_gu | 152 | 心毒命固亦夭亡 |
| quan_shu.v01.dou_shu_gu_sui_fu.qian_jin_gui_yun_qu | 153 | 今人命有千金贵，运去之时岂久长 |
| quan_shu.v01.dou_shu_gu_sui_fu.shu_nei_bao_cang | 154 | 数内包藏多少理，学者须当仔细详 |
| quan_shu.v03.zhu_xing_tong_yuan.zi_fu_jia_ming | 1 | 紫府夹命为贵格 |
| quan_shu.v03.zhu_xing_tong_yuan.lian_zhen_qi_sha_miao_wang | 2 | 廉贞七杀居庙旺反为积富之人 杀居午奇格，若陷地化忌，贫贱残疾 |
| quan_shu.v03.zhu_xing_tong_yuan.qing_yang_ru_miao | 3 | 擎羊入庙富贵声扬 加吉万论 |
