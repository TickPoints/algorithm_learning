# 非紧渐近符号
## 算法的时间复杂度
在了解接下来的非紧渐近符号之前, 我们先来思考如何用一般(紧)渐近符号描述一个算法的时间复杂度.
### $\Theta$ 的充要条件定理
我们以一个定理来引入, 即渐近符号的夹逼定理, 或者更正式的称为:

> **Theta 的充要条件定理** (Theorem on the equivalence of $\Theta$ and the combination of $O$ and $\Omega$)

如下:
对于两个正函数 $f(n)$ 和 $g(n)$, 有:
$$f(n) = \Theta(g(n)) \quad \text{当且仅当} \quad f(n) = O(g(n)) \quad \text{且} \quad f(n) = \Omega(g(n))$$

这个定理说明了 Big-Theta 符号是 Big-O 和 Big-Omega 的“交集”. 其证明也非常的简单, 可以通过分别证明充分性和必要性解决. 通过前面的学习, 完成它应该不是什么大问题, 故本书不再赘述.

通过这个定义, 我们可以更好的了解算法的时间复杂度是怎么表示的. 在之前我们已经聊过这个问题了, 现在再深入地解决一下:

- 所有的符号都并非只是描述算法的一种情况, 我们一般给出三种情况: 最坏, 最优(好)和一般[^note1], 在无特定规定下, 符号描述可以参照脚注2[^note2].
- $O(g(n))$ 表示在对应情况下, 算法的时间复杂度不超过 $g(n)$ 的常数倍.
- $\Omega(g(n))$ 表示在对应情况下, 算法的时间复杂度至少为 $g(n)$ 的常数倍.
- $\Theta(g(n))$ 表示在对应情况下, 算法的时间复杂度总在 $g(n)$ 的两个常数倍之间.
- 以上三个表达都只是简单描述, 任何需要专业化的表达, 最好参照前面的定义进行.

## $o$ 符号
接下来, 我们来看今天的主题——非紧渐近符号: 非紧渐近符号们提供了 **非紧界(Non-tight bounds)**. 它们有时也被称为 **严格渐近符号(Strict asymptotic notation)** 或 **小记号(Little notations)**[^note3].

我们来看 $o$ 符号的定义:
$$
o(g(n)) = \{ f(n) | \forall c > 0, n_0 \in \mathbb{N} : \forall n \geq n_0, |f(n)| < c \cdot |g(n)| \}
$$

与 $O$ 主要区别在于: 在 $f(n) = O(g(n))$ 中, 界 $0 \leq f(n) \leq cg(n)$ 对某个常量 $c > 0$ 成立, 在 $f(n) = o(g(n))$ 中, 界 $0 \leq f(n) < cg(n)$ 对所有常量 $c > 0$ 成立. 这也就是说，在 $o$ 记号中, 当 $n$ 趋向于无穷大时,  $f(n)$ 对于 $g(n)$ 就微不足道了, 即:
$$
\lim_{n \to \infty} \frac{f(n)}{g(n)} = 0
$$

关于非紧界这个描述, 是因为 $O$ 包含同阶函数而 $o$ 反之. 如, $f(n) = O(n)$ 但 $f(n) \neq o(n)$. 同理, 可以推出 $f(n) = o(g(n)) Rightarrow f(n) = O(g(n))$. 在本章节的[练习与回答](#练习与回答)中, 也可以通过这种方法判断正误.

> [!WARNING]
> 该章节仍在编写, 欢迎在 [GitHub仓库](https://github.com/TickPoints/algorithm_learning) 提交PR贡献内容.

---
## 练习与回答
1. 判断陈述是否正确.
    - $3n^2 + 5n = O(n^2)$
    - $3n^2 + 5n = o(n^2)$
    - $\sqrt{n} = o(n)$
    - $\log n = O(n)$
    - $\log n = o(n)$

$3n^2 + 5n = o(n^2)$ 错误, 正确的应该是 $3n^2 + 5n = O(n^2)$ 或 $3n^2 + 5n = o(n^3)$. 其余四个均正确.

> [!WARNING]
> 该章节仍在编写, 欢迎在 [GitHub仓库](https://github.com/TickPoints/algorithm_learning) 提交PR贡献内容.


[^note1]: 事实上为: 最坏情况(Worst-case)、最好情况(Best-case)、平均情况(Average-case)、摊还情况(Amortized)

[^note2]: 对于算法的最坏情况, 一般使用$O$；对于算法的平均情况, 一般使用$\Theta$；对于算法的最好情况, 一般使用$\Omega$.

[^note3]: $o$(little-o) 是 $O$(Big-O) 的小记号. $\omega$(little-omega) 是 $\Omega$(Big-Omega) 的小记号.
