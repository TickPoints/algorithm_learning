# 幂与对数
## 幂
幂是指将一个数(底数)自乘若干次(由指数决定)的运算。其一般形式为 $a ^ n$，表示 $a$ 乘以自身 $n$ 次。

如$10 ^ 3 = 10 \times 10 \times 10 = 1000$中，$10$是底数，$3$是指数。

不同指数的幂有特性:
### 正整数指数
- 域: $n \in \mathbb{N}^+$  [^note1]
- 定义: $a ^ n = a \times a \times \dots \times a$
- 性质:
    - $a ^ m \times a ^ n = a ^ {m + n}$
    - $(a ^ m) ^ n = a ^ {mn}$
    - $a ^ n \times b ^ n = (a + b) ^ n$
    - $a ^ m / a ^ n = a ^ {m - n}$

### 零指数
- 域: $n = 0$
- 定义: $a ^ 0 = 1 (a \neq 0)$
- 注:
    - 这是为了保持幂运算的连续性，例: $a ^ m / a ^ m = 1 = a ^ 0$，确保像这类运算可以正常执行。
    - $0 ^ 0$未定义的。(某些情况下等于一$1$但不推荐使用)

### 负整数指数
- 域: $n \in \mathbb{Z}^-$
- 定义: $a ^ n = \frac{1}{a ^ {-n}}$
- 注:
    - 负指数表示倒数。
    - 这是为了扩展幂运算的定义域，使其在整数范围内封闭。

### 分数指数
- 域: $n = \frac{p}{q} \text{其中} p, q \in \mathbb{Z}, q \neq 0$
- 定义: $a ^ {\frac{p}{q}} = (\sqrt[q]{a}) ^ p$

**注**: 分数指数的本质是将幂运算延拓到有理数集。这里的推广基于正整数指数的性质: $(a ^ m) ^ n = a ^ {mn}$:

如果分数指数要满足该性质，我们可以用一个简单的式子:
$$
(a ^ {\frac{1}{q}}) ^ q = a ^ {(\frac{1}{q} \times q)} = a
$$
看前面的一项
$$
\text{设 } x = (a ^ {\frac{1}{q}}) , \text{则 } x ^ q = a \\
\because (\sqrt[q]{a}) ^ q = a \\
\therefore x = \sqrt[q]{a} \\
\text{即 } a ^ {\frac{1}{q}} = x = \sqrt[q]{a}
$$
再利用$(a ^ m) ^ n = a ^ {mn}$，不难得到$a ^ {\frac{p}{q}} = a ^ {\frac{1}{q} \times p} = (\sqrt[q]{a}) ^ p$。

### 无理指数
- 域: $n \in \mathbb{R/Q}$
- 注: 无理指数将幂运算延拓到实数甚至复数集。

**定义**:
设 $a > 0$ 是一个正实数，$\alpha$ 是一个无理数。选取一个有理数序列 $\{r_n\}$ 使得 $\lim_{n \to \infty} r_n = \alpha$，则无理指数 $a^\alpha$ 定义为:
$$
a^\alpha = \lim_{n \to \infty} a^{r_n}
$$
这个方法的本质是通过有理数无限逼近一个无理数来进行。我们也可以通过指数函数来进行延拓，此处不再赘述。

### 编程中使用
一般来讲，我们将设计的幂算法只支持自然数。更广的作用域，在编程中通常不使用。接下来我们开始设计:

从定义出发，我们不难想到只需要对一个基数进行循环`n`次，每次乘它自身，便可得到:
```rs
pub fn power_iter(base: i64, exponent: u32) -> i64 {
    // 下面省略溢出检测
    let mut result = 1;
    for _ in 0..exponent {
        result *= base;
    }
    result
}
```
它的时间复杂度是$\Theta(n)$，对于较大的输入规模，这个算法可能并不特别好。我们还有一种名为**快速幂**的方法:
```rs
fn power_fast(mut base: i64, mut exponent: u32) -> i64 {
    let mut result = 1;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result *= base;
        }
        base *= base;
        exponent /= 2;
    }
    result
}
```
它采用分治法的策略，复杂度是$\Theta(\log n)$。其数学基础是对于输入规模$n$，如果它是偶数，那么$a ^ n = (a ^ 2)^{n/2}$，如果它是奇数，那么$a ^ n = a(a ^ 2)^{(n - 1)/2}$。快速幂算法实际上是在利用指数的二进制表示:
$$
13 = 1101_2 = 8 + 4 + 1 = 2^3 + 2^2 + 2^0 \\
3^{13} = 3^{(8+4+1)} = 3^8 × 3^4 × 3^1
$$
[^note2]

所以完全可以用位运算:
```rs
fn power_fast(mut base: i64, mut exponent: u32) -> i64 {
    let mut result = 1;
    while exponent > 0 {
        if exponent & 1 == 1 {  // 等价于 exponent % 2 == 1
            result *= base;
        }
        base *= base;
        exponent >>= 1;  // 等价于 exponent /= 2
    }
    result
}
```

## 对数
> 该章节仍在编写，在 [Github仓库](https://github.com/TickPoints/algorithm_learning) 上提交PR以为本书 [贡献内容](/pr_guide/pr_standard.md)。

[^note1]: 见 [集合](/appendices/discrete/set.md)。$\mathbb{N}^+$ 有时也用 $\mathbb{Z}^+$。

[^note2]: $1101_2$中的$_2$表示二进制形式，同理$_{16}$是十六进制。
