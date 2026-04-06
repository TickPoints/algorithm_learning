# 集合
**集合**是由不同对象聚集而成的一个整体, 称其中的**对象**为**成员或元素**. 如果一个对象 $x$ 是集合 $S$ 的一个成员, 则写作 $x \in S$ (读作"$x$是$S$的成员"或"$x$在$S$内"). 相反的, 如果对象 $x$ 不是集合 $S$ 的一个成员, 则写作 $x \notin S$. 一个集合不包含相同元素. [^note1]

## 集合的表示
一个集合有多种表示方法, 常见的有枚举出所有的元素, 如: $S = \{1, 2, 3, 4\}$. 在枚举过程中, 如果集合的元素排列有规律则可以通过$\dots$来省略, 甚至构造无限集合: $S = \{2, 4, 6, 8, 10, \dots\}$.

我们还有描述法, 现在比较公认的描述法是$\{x | P(x)\}$[^note2], 其中$P(x)$是元素应该满足的条件, 如: $\{x | x \text{是偶数 且 } 0 < x < 10\}$(这个集合用枚举法表示出来是$\{2, 4, 6, 8\}$).

有时也可以用区间表示: $S = (1, +\infty)$ 表示大于$1$的所有实数.

我们还采用特殊的符号来表示下面几个常见的集合:
- $\varnothing$ 或 $\emptyset$ 或 $\{\}$: **空集**. 是不包含任何元素的唯一集合.
- $\mathbb{Z}^+$ 或 $\mathbb{N}^+$: **正整数集**.
- $\mathbb{Z}_{\geq 0}$ 或 $\mathbb{N}_0$: **非负整数集**
- $\mathbb{N}$: **自然数集**.
- $\mathbb{Z}$: **整数集**.
- $\mathbb{Q}$: **有理数集**.
- $\mathbb{R}^+$: **正实数集**.
- $\mathbb{R}$: **实数集**.
- $\mathbb{C}$: **复数集**.

## 集合间关系
集合间存在一些基本关系, 如下:
1. **包含关系**

若集合 $A$ 中的每一个元素都是集合 $B$ 的元素, 则称 $A$ 是 $B$ 的子集, 记作: $A \subseteq B$

如果 $A$ 是 $B$ 的子集且 $A \neq B$, 则称 $A$ 是 $B$ 的真子集, 记作: $A \subset B $.

2. **相等关系**

若 $A \subseteq B$ 且 $B \subseteq A$, 则 $A = B$.

3. **不相交关系**

若集合 $A$ 和 $B$ 没有公共元素, 即 $A \cap B = \emptyset$, 则称 $A$ 与 $B$ 不相交.

4. 交集、并集、补集、差集 等属于集合运算, 但也可反映集合之间的关系.

示意图:
```plot
{
  "map": {
    "xA": {"type": "g-number-list", "begin": 0, "end": 240, "expr": "3.0 + 1.4*cos(2*3.14159*i/239)"},
    "yA": {"type": "g-number-list", "begin": 0, "end": 240, "expr": "2.5 + 1.4*sin(2*3.14159*i/239)"},
    "xB": {"type": "g-number-list", "begin": 0, "end": 240, "expr": "3.4 + 1.4*cos(2*3.14159*i/239)"},
    "yB": {"type": "g-number-list", "begin": 0, "end": 240, "expr": "2.5 + 1.4*sin(2*3.14159*i/239)"}
 },
  "layout": {
    "title": "子集关系",
    "show_legend": false,
    "width": 560,
    "height": 360,
    "plot_background_color": {rgb_color: "rgb(255, 255, 255)"},
    "margin": {"pad": 20}
 },
  "data": [
    {"type": "scatter", "x": "map.xA", "y": "map.yA", "mode": "none", "fill": "toself", "fill_color": {rgba_color: "rgba(52, 152, 219, 0.7)"}},
    {"type": "scatter", "x": "map.xB", "y": "map.yB", "mode": "none", "fill": "toself", "fill_color": {rgba_color: "rgba(230, 126, 34, 0.7)"}},
    {"type": "scatter", "x": "map.xA", "y": "map.yA", "mode": "lines"},
    {"type": "scatter", "x": "map.xB", "y": "map.yB", "mode": "lines"},
    {"type": "scatter", "x": [2.8], "y": [2.5], "mode": "text", "text": "A"},
    {"type": "scatter", "x": [4.2], "y": [2.5], "mode": "text", "text": "B"},
    {"type": "scatter", "x": [3.7], "y": [4.2], "mode": "text", "text": "A ⊂ B"}
  ],
  "config": {"static_plot": true, "display_logo": false}
}
```
```plot
{
  "map": {
    "xAB": {"type": "g-number-list", "begin": 0, "end": 240, "expr": "3.2 + 1.4*cos(2*3.14159*i/239)"},
    "yAB": {"type": "g-number-list", "begin": 0, "end": 240, "expr": "2.5 + 1.4*sin(2*3.14159*i/239)"}
 },
  "layout": {
    "title": "完全相等",
    "show_legend": false,
    "width": 560,
    "height": 360,
    "plot_background_color": {rgb_color: "rgb(255, 255, 255)"},
    "margin": {"pad": 20}
 },
  "data": [
    {"type": "scatter", "x": "map.xAB", "y": "map.yAB", "mode": "none", "fill": "toself", "fill_color": {rgba_color: "rgba(155, 89, 182, 0.75)"}},
    {"type": "scatter", "x": "map.xAB", "y": "map.yAB", "mode": "lines"},
    {"type": "scatter", "x": [3.2], "y": [2.5], "mode": "text", "text": "A = B"}
  ],
  "config": {"static_plot": true, "display_logo": false}
}
```
```plot
{
  "map": {
    "xA": {"type": "g-number-list", "begin": 0, "end": 240, "expr": "2.9 + 1.4*cos(2*3.14159*i/239)"},
    "yA": {"type": "g-number-list", "begin": 0, "end": 240, "expr": "2.5 + 1.4*sin(2*3.14159*i/239)"},
    "xB": {"type": "g-number-list", "begin": 0, "end": 240, "expr": "4.0 + 1.4*cos(2*3.14159*i/239)"},
    "yB": {"type": "g-number-list", "begin": 0, "end": 240, "expr": "2.5 + 1.4*sin(2*3.14159*i/239)"},
    "xI1": {"type": "g-number-list", "begin": 0, "end": 160, "expr": "2.9 + 1.4*cos(1.18 - 2.36*i/159)"},
    "yI1": {"type": "g-number-list", "begin": 0, "end": 160, "expr": "2.5 + 1.4*sin(1.18 - 2.36*i/159)"},
    "xI2": {"type": "g-number-list", "begin": 0, "end": 160, "expr": "4.0 + 1.4*cos(1.96 + 2.36*i/159)"},
    "yI2": {"type": "g-number-list", "begin": 0, "end": 160, "expr": "2.5 + 1.4*sin(1.96 + 2.36*i/159)"}
 },
  "layout": {
    "title": "交集",
    "show_legend": false,
    "width": 560,
    "height": 360,
    "plot_background_color": {rgb_color: "rgb(255, 255, 255)"},
    "margin": {"pad": 20}
 },
  "data": [
    {"type": "scatter", "x": "map.xA", "y": "map.yA", "mode": "lines"},
    {"type": "scatter", "x": "map.xB", "y": "map.yB", "mode": "lines"},
    {"type": "scatter", "x": "map.xI1", "y": "map.yI1", "mode": "lines"},
    {"type": "scatter", "x": "map.xI2", "y": "map.yI2", "mode": "lines", "fill": "tonext",},
    {"type": "scatter", "x": "map.xI1", "y": "map.yI1", "mode": "lines"},
    {"type": "scatter", "x": "map.xI2", "y": "map.yI2", "mode": "lines", "fill": "tonext", "fill_color": {rgba_color: "rgba(52, 152, 219, 0.65)"}},
    {"type": "scatter", "x": [2.3], "y": [2.5], "mode": "text", "text": "A"},
    {"type": "scatter", "x": [4.7], "y": [2.5], "mode": "text", "text": "B"},
    {"type": "scatter", "x": [3.45], "y": [2.5], "mode": "text", "text": "A ∩ B"}
  ],
  "config": {"static_plot": true, "display_logo": false}
}
```
```plot
{
  "map": {
    "xA": {"type": "g-number-list", "begin": 0, "end": 240, "expr": "2.9 + 1.4*cos(2*3.14159*i/239)"},
    "yA": {"type": "g-number-list", "begin": 0, "end": 240, "expr": "2.5 + 1.4*sin(2*3.14159*i/239)"},
    "xB": {"type": "g-number-list", "begin": 0, "end": 240, "expr": "4.0 + 1.4*cos(2*3.14159*i/239)"},
    "yB": {"type": "g-number-list", "begin": 0, "end": 240, "expr": "2.5 + 1.4*sin(2*3.14159*i/239)"}
 },
  "layout": {
    "title": "并集",
    "show_legend": false,
    "width": 560,
    "height": 360,
    "plot_background_color": {rgb_color: "rgb(255, 255, 255)"},
    "margin": {"pad": 20}
 },
  "data": [
    {"type": "scatter", "x": "map.xA", "y": "map.yA", "mode": "none", "fill": "toself", "fill_color": {rgba_color: "rgba(46, 204, 113, 0.75)"}},
    {"type": "scatter", "x": "map.xB", "y": "map.yB", "mode": "none", "fill": "toself", "fill_color": {rgba_color: "rgba(46, 204, 113, 0.75)"}},
    {"type": "scatter", "x": "map.xA", "y": "map.yA", "mode": "lines"},
    {"type": "scatter", "x": "map.xB", "y": "map.yB", "mode": "lines"},
    {"type": "scatter", "x": [2.3], "y": [2.5], "mode": "text", "text": "A"},
    {"type": "scatter", "x": [4.7], "y": [2.5], "mode": "text", "text": "B"},
    {"type": "scatter", "x": [3.45], "y": [4.35], "mode": "text", "text": "A ∪ B"}
  ],
  "config": {"static_plot": true, "display_logo": false}
}
```
```plot
{
  "map": {
    "uX": {"type": "raw", "data": [0.6, 6.4, 6.4, 0.6, 0.6]},
    "uY": {"type": "raw", "data": [0.6, 0.6, 4.8, 4.8, 0.6]},
    "xA": {"type": "g-number-list", "begin": 0, "end": 240, "expr": "3.2 + 1.4*cos(2*3.14159*i/239)"},
    "yA": {"type": "g-number-list", "begin": 0, "end": 240, "expr": "2.7 + 1.4*sin(2*3.14159*i/239)"}
 },
  "layout": {
    "title": "补集: A (相对全集 U)",
    "show_legend": false,
    "width": 560,
    "height": 360,
    "plot_background_color": {rgb_color: "rgb(255, 255, 255)"},
    "margin": {"pad": 20}
 },
  "data": [
    {"type": "scatter", "x": "map.uX", "y": "map.uY", "mode": "none", "fill": "toself", "fill_color": {rgba_color: "rgba(149, 165, 166, 0.5)"}},
    {"type": "scatter", "x": "map.xA", "y": "map.yA", "mode": "none", "fill": "toself", "fill_color": {rgba_color: "rgba(255, 255, 255, 1.0)"}},
    {"type": "scatter", "x": "map.uX", "y": "map.uY", "mode": "lines"},
    {"type": "scatter", "x": "map.xA", "y": "map.yA", "mode": "lines"},
    {"type": "scatter", "x": [6.1], "y": [4.5], "mode": "text", "text": "U"},
    {"type": "scatter", "x": [3.2], "y": [2.7], "mode": "text", "text": "A"},
    {"type": "scatter", "x": [5.2], "y": [2.7], "mode": "text", "text": "A¯"}
  ],
  "config": {"static_plot": true, "display_logo": false}
}
```
```plot
{
  "map": {
    xA: {type: "g-number-list", begin: 0, end: 240, expr: "2.9 + 1.4*cos(2*3.14159*i/239)"},
    yA: {type: "g-number-list", begin: 0, end: 240, expr: "2.5 + 1.4*sin(2*3.14159*i/239)"},

    xB: {type: "g-number-list", begin: 0, end: 240, expr: "4.0 + 1.4*cos(2*3.14159*i/239)"},
    yB: {type: "g-number-list", begin: 0, end: 240, expr: "2.5 + 1.4*sin(2*3.14159*i/239)"},

    xI_A: {type: "g-number-list", begin: 0, end: 200, expr: "2.9 + 1.4*cos(1.166 - (2*1.166)*i/199)"},
    yI_A: {type: "g-number-list", begin: 0, end: 200, expr: "2.5 + 1.4*sin(1.166 - (2*1.166)*i/199)"},

    xI_B: {type: "g-number-list", begin: 0, end: 200, expr: "4.0 + 1.4*cos((3.14159 - 1.166) + (2*1.166)*i/199)"},
    yI_B: {type: "g-number-list", begin: 0, end: 200, expr: "2.5 + 1.4*sin((3.14159 - 1.166) + (2*1.166)*i/199)"}
 },
  "layout": {
    "title": "差集及对称差",
    "show_legend": false,
    "width": 560,
    "height": 360,
    "plot_background_color": {rgb_color: "rgb(255, 255, 255)"},
    "margin": {"pad": 20}
 },
  "data": [
    {type: "scatter", x: "map.xA", y: "map.yA", mode: "none", fill: "toself", fill_color: {rgba_color: "rgba(52, 152, 219, 0.75)"}},
    {type: "scatter", x: "map.xB", y: "map.yB", mode: "none", fill: "toself", fill_color: {rgba_color: "rgba(230, 126, 34, 0.75)"}},

    {type: "scatter", x: "map.xI_A", y: "map.yI_A", mode: "lines", fill: "none"},
    {type: "scatter", x: "map.xI_B", y: "map.yI_B", mode: "lines", fill: "tonext", fill_color: {rgba_color: "rgba(255,255,255,1.0)"}},

    {type: "scatter", x: "map.xA", y: "map.yA", mode: "lines"},
    {type: "scatter", x: "map.xB", y: "map.yB", mode: "lines"},

    {type: "scatter", x: [2.3], y: [2.5], mode: "text", text: "A−B"},
    {type: "scatter", x: [4.8], y: [2.5], mode: "text", text: "B−A"},
    {type: "scatter", x: [3.55], y: [4.25], mode: "text", text: "A △ B"}
  ],
  "config": {"static_plot": true, "display_logo": false}
}
```

## 集合运算及其基本定律
设 $U$ 为全集, $A$, $B$, $C$ 为 $U$ 的子集.

### 常见运算
- 并集: $A \cup B = {x | x \in A \text{ 或 } x \in B}$
- 交集: $A \cap B = {x | x \in A \text{ 且 } x \in B}$
- 差集: $A \setminus B = {x | x \in A \text{ 且 } x \notin B}$
- 补集: $A^c = {x \in U | x \notin A}$, 也记作 $U \setminus A$
- 对称差: $A \triangle B = (A \setminus B) \cup (B \setminus A)$

### 相关运算定律
| 定律名称 | 公式表达 |
|--------|---------|
| 幂等律 | $A \cup A = A$, $A \cap A = A$ |
| 交换律 | $A \cup B = B \cup A$, $A \cap B = B \cap A$ |
| 结合律 | $(A \cup B) \cup C = A \cup (B \cup C)$, $(A \cap B) \cap C = A \cap (B \cap C)$ |
| 分配律 | $A \cup (B \cap C) = (A \cup B) \cap (A \cup C)$, $A \cap (B \cup C) = (A \cap B) \cup (A \cap C)$ |

上述定律与实数的四则运算相似, 主要集中在并集和交集两种运算, 其它运算(如差集和补集)则不一定存在.

| 定律名称 | 公式表达 |
|--------|---------|
| 德摩根律 | $(A \cup B)^c = A^c \cap B^c$, $(A \cap B)^c = A^c \cup B^c$ |
| 吸收律 | $A \cup (A \cap B) = A$, $A \cap (A \cup B) = A$ |
| 零元与单位元 | $A \cup \emptyset = A$, $A \cap U = A$, $A \cup U = U$, $A \cap \emptyset = \emptyset$ |
| 补集律 | $A \cup A^c = U$, $A \cap A^c = \emptyset$ |

这些定律在集合恒等式证明和逻辑推理中非常有用.

### 笛卡尔积
给定两个集合 A 和 B, 它们的 **笛卡尔积(Cartesian Product)** 记作 $A \times B$, 定义为所有有序对 $a, b$ 的集合, 其中 $a \in A$, $b \in B$.

即:
$$
A \times B = {(a, b) | a \in A, b \in B}
$$
> [!WARNING]
> 该章节仍在编写, 在 [Github仓库](https://github.com/TickPoints/algorithm_learning) 上提交PR以为本书 [贡献内容](/pr_guide/pr_standard.md).

[^note1]: **多重集(Multiset)** 也称为 **袋(bag)** 是数学中一种广义的集合概念. 与普通集合不同, 多重集中的元素可以重复出现.

[^note2]: 《算法导论》中也使用一种描述法, 即$\{x : P(x)\}$. 用 $:$ 代替 $|$. 在数学中, $:$ 和 $|$ 在集合构造中通常可以互换使用, 读作"使得"(such that). 考虑我国习惯, 本书尽量使用 $|$.
