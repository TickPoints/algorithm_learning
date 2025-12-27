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
- $\mathbb{R}^+$: **正数集**.
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
