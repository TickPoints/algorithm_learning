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
o(g(n)) = \{ f(n) | \forall c > 0, \exists n_0 \in \mathbb{N}, \forall n \geq n_0 : |f(n)| < c \cdot |g(n)| \}
$$

与 $O$ 主要区别在于: 在 $f(n) = O(g(n))$ 中, 界 $0 \leq f(n) \leq cg(n)$ 对某个常量 $c > 0$ 成立, 在 $f(n) = o(g(n))$ 中, 界 $0 \leq f(n) < cg(n)$ 对所有常量 $c > 0$ 成立. 这也就是说，在 $o$ 记号中, 当 $n$ 趋向于无穷大时,  $f(n)$ 对于 $g(n)$ 就微不足道了, 即:
$$
\lim_{n \to \infty} \frac{f(n)}{g(n)} = 0
$$

关于非紧界这个描述, 是因为 $O$ 包含同阶函数而 $o$ 反之. 如, $f(n) = O(n)$ 但 $f(n) \neq o(n)$. 同理, 可以推出 $f(n) = o(g(n)) \Rightarrow f(n) = O(g(n))$. 在本章节的[练习与回答](#练习与回答)中, 也可以通过这种方法判断正误.

## $\omega$ 符号
不严谨地说，它与 $o$ 符号相反. 我们可以利用 $o$ 来定义它, 下面这个定义同样是它的一个性质:
$$
f(n) \in w(g(n)) \quad \text{当且仅当} \quad g(n) \in o(f(n))
$$
形式化的定义是:
$$
\omega(g(n)) = \{ f(n) | \forall c > 0, \exists n_0 \in \mathbb{N}, \forall n \geq n_0 : |f(n)| > c \cdot |g(n)| \}
$$

同样地, 我们有 $f(n) = \omega(g(n)) \Rightarrow f(n) = \Omega(g(n))$. 例如, $n^2 = \omega(n)$ 但 $n^2 \neq \omega(n^2)$ 不过 $n^2 = \Omega(n^2)$.

同时还要注意, 我们使用的是 $\Rightarrow$, 这意味着关于 Big-O 和 little-o, Big-Omega 和 little-omega 上面这两个结论都不可逆.

另外我们同样有个极限表达:
$$
\lim_{n \to \infty} \frac{f(n)}{g(n)} = \infty
$$

> [!WARNING]
> 该章节仍在编写, 欢迎在 [GitHub仓库](https://github.com/TickPoints/algorithm_learning) 提交PR贡献内容.

---
## 练习与回答
1. 判断下列关于 Big-O 和 little-o 的陈述是否正确.
    - $3n^2 + 5n = O(n^2)$
    - $3n^2 + 5n = o(n^2)$
    - $\sqrt{n} = o(n)$
    - $\log n = O(n)$
    - $\log n = o(n)$

$3n^2 + 5n = o(n^2)$ 错误, 正确的应该是 $3n^2 + 5n = O(n^2)$ 或 $3n^2 + 5n = o(n^3)$. 其余四个均正确.

2. 根据已知内容, 判断下列关于 Big-Omega 和 little-omega 陈述是否正确.

已知：
- 若 $\lim_{n\to\infty} \frac{f(n)}{g(n)} = \infty$, 则 $f(n) = \omega(g(n))$
- 若 $\lim_{n\to\infty} \frac{f(n)}{g(n)} = c > 0$, 则 $f(n) = \Theta(g(n)) \subseteq \Omega(g(n))$, 但不是 $\omega$

则判断:
- $f(n) = n^3$, $g(n) = n^2$ 是否满足 $f(n) = \omega(g(n))$
- $f(n) = 5n \log n$, $g(n) = 4n \log n$ 是否满足 $f(n) = \omega(g(n))$
- $f(n) = n \log n$, $g(n) = n$ 是否满足 $f(n) = \omega(g(n))$ 又满足 $f(n) = \Omega(g(n))$

对于上面已知内容, 可以在下一节具体学习. 以上除第2题以外都正确, 通过极限法证明即可.

> [!WARNING]
> 该章节仍在编写, 欢迎在 [GitHub仓库](https://github.com/TickPoints/algorithm_learning) 提交PR贡献内容.


[^note1]: 事实上为: 最坏情况(Worst-case)、最好情况(Best-case)、平均情况(Average-case)、摊还情况(Amortized)

[^note2]: 对于算法的最坏情况, 一般使用$O$；对于算法的平均情况, 一般使用$\Theta$；对于算法的最好情况, 一般使用$\Omega$.

[^note3]: $o$(little-o) 是 $O$(Big-O) 的小记号. $\omega$(little-omega) 是 $\Omega$(Big-Omega) 的小记号.
当然可以！下面我们先简要回顾一下这四个渐近符号的定义，然后设计一系列由浅入深的练习题，帮助你清晰地区分 Big-Omega (Ω)、little-omega (ω)、Big-O (O) 和 little-o (o)。
