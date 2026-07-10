# 纯文本命盘演示

本演示展示当前支持的本命排盘事实面：从类型化的阳历输入流经 `by_solar`，再进入
renderer-neutral 的堆叠 snapshot，最后进入 `render` 模块的纯文本演示。

```text
solar input -> by_solar -> ChartStackSnapshot -> render module plain text output
```

运行方式：

```bash
cargo run -p iztro --example plain_text
```

该示例使用 fixture 兜底的支持字段，对应 1990-05-17 辰时女性本命盘。它只渲染命盘事实；
解读判断与叙事报告与排盘相互分离。

捕获的输出存放于
[`docs/examples/plain_text_1990_05_17_chen_female.txt`](../examples/plain_text_1990_05_17_chen_female.txt)。

关于该演示周边更完整的“已实现 / 暂缓”范围，参见
[`current-status.md`](current-status.md)。

## 本地 Iced GUI 原型

workspace 还包含一个本地桌面原型：

```bash
cargo run -p iztro-gui
```

> 构建前置要求：GUI 内置的 CJK 字体由 Git LFS 跟踪并在编译期通过 `include_bytes!`
> 嵌入。请先 `git lfs install && git lfs pull`，否则字体资源会停留在指针文件状态。
> 详见 [CONTRIBUTING.zh-CN.md](../../CONTRIBUTING.zh-CN.md)。

它打开时显示**启动页**而非默认生成一张命盘：你可以输入出生信息并生成命盘，或重新打开
本地**已保存的命盘**。生成的命盘会持久化到每用户本地数据目录下的一个小 JSON 文件
（`<data_local_dir>/iztro-gui/charts.json`）；只存储归一化后的出生输入，重新打开时每张
保存的命盘都会通过核心 facade 确定性地重建。没有当前目录回退：若没有可用的本地数据目录，
GUI 会在无持久化的情况下启动，并给出一个非致命提示，而不是把保存的命盘散落各处。

生成的命盘从 `StaticChartProjection` 渲染，使用内置的思源宋体（Source Han Serif SC）
显示中文，并用 Iced 的 GPU 渲染器。在 WSL 上，当 `DISPLAY` 可用时优先使用 WSLg 的稳定
XWayland 端点，避免不稳定的 `wgpu 0.19` Wayland surface 路径，同时保留 GPU 加速重绘；
原生 Linux 保持其正常的窗口后端选择。中间面板在 snapshot 提供时显示四柱标签
（年柱/月柱/日柱/时柱）。宫位单元采用类 iztro 的静态布局：星曜在单元内分区显示，而非以
分类徽标呈现。主星在左上方以较大的紫色字体显示；辅星在上方中部；杂曜在右上方。亮度标签
（庙旺得利平陷不）与 **科 / 权 / 禄 / 忌** 四化标记紧随每个星名内联显示。颜色与位置承载
分类信息，因此单元不再显示 主星/辅星/杂曜/神煞 分组标签。“十二神”神煞沿底部排列：
长生/博士在左下，将前/岁前在右下。一个紧凑图例解释色调含义。这纯粹是对核心 projection
的渲染端展示——每个星的色调由其 `kind`/`family` 分类得到，GUI 不计算任何星曜、四柱、大限、
运限 overlay、四化、三方四正、解读、规则、成格、八字解读或叙事。

**悬停**某宫会连同其 **三方四正**（对宫 / 财 / 官）相关宫位一起高亮：被悬停的宫得到更强
强调，相关宫得到柔和的填充背景。指针位于宫位上方时，悬停优先于黏性的点击选择；工具栏开关
控制高亮模式，关闭时只强调当前宫本身。关系完全来自每个宫位视图上预备好的、renderer-neutral
的 `surround` 字段——GUI 不做任何宫位算术。携带四化的本命星会显示紧凑的、按分类着色的
**科 / 权 / 禄 / 忌** 徽标，读取自预备好的 `mutagen` 字段；GUI 不计算任何四化。

底部运限面板是**生效的**，而不仅是一个选择指示器。第一行在正常的**大限**行之前承载
**本命**与**限前**单元。生成或重新打开的命盘默认处于**限前**，展示无 overlay 的本命底盘。
随后导航按层级解锁：**大限 → 流年/小限 → 流月 → 流日 → 流时**。每次启用的点击都向核心
请求一份新预备的 `StaticChartProjection`，选择某个父级会清除更深的选择。

底部面板保留农历标签：流月使用**正月至腊月**，流日使用既有的 3×10 **初一至三十**网格。
29 天的农历月会禁用三十；没有 31 天的阳历单元。闰月选择仍暂缓。核心通过内部的有界日期
解析器解析所选农历路径，只构建到所选 scope 的部分运限堆叠。不暴露公开的 `lunar_to_solar`
API。GUI 只向 `static_temporal_chart_view` 发送层级索引路径，并渲染核心预备的标签、启用
标志、选中标志、overlay 和宫位名。本命事实在各选择间保持不可变，禁用的单元保持惰性。

右侧可折叠的检视面板**展示**结构化分析——全书规则（《紫微斗数全书》经典规则）与格局
（patterns）——这些数据由 GUI 向核心的逐层分析 facade 请求并缓存；GUI 本身不做任何规则
求值或格局检测（检视面板的缓存模型见 [`current-status.md`](current-status.md)）。展开一条
规则命中或有出处的格局时，会显示其典籍引文（典籍、卷/节，格局另含逐字原文），由渲染时经
`rules::source::source_section` 与格局出处元数据解析（ADR 0010）——GUI 状态中不存引文数据。除了呈现
这些结构化命中及其证据之外，本 GUI 仍是一个原型化的命盘事实查看器：它不生成确定性解读、
八字解读或叙事输出。
