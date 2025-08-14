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
它采用分治法的策略，时间复杂度是$\Theta(\log n)$。其数学基础是对于输入规模$n$，如果它是偶数，那么$a ^ n = (a ^ 2)^{n/2}$，如果它是奇数，那么$a ^ n = a(a ^ 2)^{(n - 1)/2}$。快速幂算法实际上是在利用指数的二进制表示:
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
利用[`std`](https://rustwiki.org/zh-CN/std)中[`pow`](https://rustwiki.org/zh-CN/std/primitive.i64.html#method.pow)可以直接进行幂运算:
```rs
let base = 2;
let exponent = 3;
let result = base.pow(exponent); // 2^3 = 8
```

## 对数
**对数(Logarithm)** 是数学中用于简化乘除运算的一种函数，定义为**幂运算的逆运算**。具体来说，如果满足以下等式:
$$
a ^ b = N (a > 0, a \neq 1)
$$
那$b$就是以$a$为底$N$的对数，记作:
$$
b = \log_{a}{N}
$$
其中，$a$为底数($a > 1$)，$N$为真数($N > 0$)。在上面这两个式子中，我们可以得到对数与指数的关系:
$$
\log_{a}{N} = b \iff a ^ b = N
$$
我们常用几种特殊的对数:
- $10$: **常用对数**。$\log_10$(可以简写为$\lg$)，用于科学计算和工程。
- $\mathrm{e}$: **自然对数**。$\log_{\mathrm{e}}$(可以简写为$\ln$)，广泛用于微积分和自然科学。
- $2$: **二进制对数**。$\log_2$(仅在计算机科学中，我们可以简写为$\log$)，常见于计算机科学和信息论。

### 性质
对数的运算有很多特殊的性质:
#### 乘法性质
**性质**：  
$$ \log_a (MN) = \log_a M + \log_a N \quad (M, N > 0) $$

**证明**：  
设 $ x = \log_a M $，$ y = \log_a N $，则：  
$$ a^x = M, \quad a^y = N $$  
两式相乘：  
$$ a^x \cdot a^y = MN \implies a^{x+y} = MN $$  
取对数得：  
$$ \log_a (MN) = x + y = \log_a M + \log_a N $$  
**证毕**。

#### 除法性质
**性质**：  
$$ \log_a \left( \frac{M}{N} \right) = \log_a M - \log_a N \quad (M, N > 0) $$

**证明**：  
设 $ x = \log_a M $，$ y = \log_a N $，则：  
$$ a^x = M, \quad a^y = N $$  
两式相除：  
$$ \frac{a^x}{a^y} = \frac{M}{N} \implies a^{x-y} = \frac{M}{N} $$  
取对数得：  
$$ \log_a \left( \frac{M}{N} \right) = x - y = \log_a M - \log_a N $$  
**证毕**。

#### 幂运算性质
**性质**：  
$$ \log_a (M^k) = k \log_a M \quad (M > 0, \, k \in \mathbb{R}) $$

**证明**：  
设 $ x = \log_a M $，则：  
$$ a^x = M \implies (a^x)^k = M^k \implies a^{kx} = M^k $$  
取对数得：  
$$ \log_a (M^k) = kx = k \log_a M $$  
**证毕**。

#### 换底公式
**性质**：  
$$ \log_a N = \frac{\log_b N}{\log_b a} \quad (a, b, N > 0, \, a, b \neq 1) $$

**证明**：  
设 $ x = \log_a N $，则：  
$$ a^x = N \implies \log_b (a^x) = \log_b N \implies x \log_b a = \log_b N $$  
解得：  
$$ x = \frac{\log_b N}{\log_b a} $$  
即：  
$$ \log_a N = \frac{\log_b N}{\log_b a} $$  
**证毕**。

#### 特殊值性质
**性质1**：  
$$ \log_a 1 = 0 $$  
**证明**：  
由 $ a^0 = 1 $，根据定义：  
$$ \log_a 1 = 0 $$  
**证毕**。

**性质2**：  
$$ \log_a a = 1 $$  
**证明**：  
由 $ a^1 = a $，根据定义：  
$$ \log_a a = 1 $$  
**证毕**。

对数将复杂的乘除、幂运算转化为加减乘除，是数学和科学中不可或缺的工具。
### 在编程中使用
其实在幂那里我们就已经知道了——由于计算机的二进制特性，利用位运算我们可以计算整数以$2$为底的对数:
```rs
pub fn log2_bitwise(n: u32) -> u32 {
    // 下面省略数据校验
    31 - n.leading_zeros()
}
```
**位操作法** 的核心思想是**一个整数的二进制表示中，最高有效位(MSB)的位置即为其以2为底的对数**。`n.leading_zeros()` 返回`n`的二进制表示中前导零[^note3]的个数。`u32`的二进制长度是`32`，由于位置编号从`0`开始(范围是`0`~`31`)，所以最高有效位是`31 - leading_zeros`。对于其他底数，我们也可以使用**牛顿迭代法**:
```rs
pub fn log_newton(x: f64, base: f64, epsilon: f64) -> f64 {
    // 下面省略数据校验
    let mut y = (x.ln() / base.ln()).max(0.0); // 初始猜测
    loop {
        let next_y = y - (base.powf(y) - x) / (base.powf(y) * base.ln());
        if (next_y - y).abs() < epsilon {
            return next_y;
        }
        y = next_y;
    }
}
```
**牛顿迭代法** 是一种用于近似求解方程 $f(x)=0$ 的根的高效数值方法。其主要**通过切线逼近逐步修正解的近似值**。从一个初始猜测值$x_0$开始。用当前点$(x_n, f(x_n))$的切线（导数）更新$x_{n+1}$:
$$x_{n+1} = x_n - \frac{f(x_n)}{f'(x_n)}$$
重复迭代，直到$|x_{n+1} - x_n|$小于预设精度$\epsilon$[^note4]。

利用[`std`](https://rustwiki.org/zh-CN/std)中[相关方法](https://rustwiki.org/zh-CN/std/primitive.f64.html)可以直接进行对数运算:
```rs
// log
let five = 5.0f32;
// log5(5) - 1 == 0
let abs_difference = (five.log(5.0) - 1.0).abs();

// log2
let two = 2.0f32;
// log2(2) - 1 == 0
let abs_difference = (two.log2() - 1.0).abs();

// log10
let ten = 10.0f64;
// log10(10) - 1 == 0
let abs_difference = (ten.log10() - 1.0).abs();

// ln
let one = 1.0_f64;
// e^1
let e = one.exp();
// ln(e) - 1 == 0
let abs_difference = (e.ln() - 1.0).abs();
```

## 练习与回答
1. 试证明**一个整数的二进制表示中，最高有效位(MSB)的位置即为其以2为底的对数**。
### 证明
**命题**：对于正整数$n$，其二进制最高有效位的位置$k$满足:
$$k = \lfloor \log_2 n \rfloor$$

**证明**：

设$n$的二进制表示为:
$$n = b_k \cdot 2^k + b_{k-1} \cdot 2^{k-1} + \cdots + b_0 \cdot 2^0 \quad (b_k = 1)$$

因为$b_k = 1$，所以:
$$2^k \leq n \leq 2^{k+1} - 1 < 2^{k+1}$$

对不等式取$\log_2$:
$$k \leq \log_2 n < k + 1$$

因此:
$$k = \lfloor \log_2 n \rfloor$$

[^note1]: 见 [集合](/appendices/discrete/set.md)。$\mathbb{N}^+$ 有时也用 $\mathbb{Z}^+$。

[^note2]: $1101_2$中的$_2$表示二进制形式，同理$_{16}$是十六进制。

[^note3]: **前导零(Leading Zeros)** 是指一个数的二进制表示中，从最高位开始连续出现的零，直到遇到第一个`1`为止。

[^note4]: **$\epsilon$(epsilon)** 是希腊字母表的第5个字母。表示任意小的正数，用于量化“无限接近”的概念。计算机科学中向如上迭代算法中也可表示停止阈值，一般用常量(如`const EPS = 0.000001`)。
