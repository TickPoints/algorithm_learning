# 利用归并分析
## 插入排序
我们可以把插入排序表示为如下的一个递归过程。为了排序`A[1..n]`，我们递归地排序`A[1..n-1]`，然后把`A[n]`插入已排序的数组`A[1..n-1]`。为插入排序的这个递归版本的最坏情况时间复杂度写一个递归式。

我们先给出一个Rust实现:
### 实现1
```rust
pub fn realize<T: Ord>(arr: &mut [T]) {
    // 基本情况：数组长度为 0 或 1 时已经有序
    if arr.len() <= 1 {
        return;
    }

    let n = arr.len();

    // 递归排序前 n-1 个元素
    realize(&mut arr[..n - 1]);

    // 将最后一个元素插入到已排序的前 n-1 个元素中
    let mut i = n - 1;
    while i > 0 && arr[i - 1] > arr[i] {
        arr.swap(i - 1, i);
        i -= 1;
    }
}

fn main() {
    // 测试用例
    let mut arr = [5, 2, 4, 6, 1, 3];
    println!("排序前: {:?}", arr);

    realize(&mut arr);
    println!("排序后: {:?}", arr);

    let mut empty_arr: [i32; 0] = [];
    realize(&mut empty_arr);
    println!("空数组: {:?}", empty_arr);

    let mut single_arr = [7];
    realize(&mut single_arr);
    println!("单元素数组: {:?}", single_arr);
}
```
先观察其基线情况: 当数组长度为1时(即 `n = 1`)，数组已经是有序的，不需要任何操作，故
$$T(n) = \Theta(1) \text{ if } n = 1$$

**注**: 其实$n \leq 1, n \in \mathbb{N}$更精准(`n = 0`时为空数组)。

然后，我们进行递归式分析。因为为了排序`A[1..n]`，我们递归地排序`A[1..n-1]`，所以$T(n) = T(n - 1) + \Theta(n)$ (其中$\Theta(n)$是进行插入的时间复杂度)。综上则有:

$$
T(n) = \begin{cases}
T(n-1) + \Theta(n) & \text{if } n > 1, \\
\Theta(1) & \text{if } n = 1.
\end{cases}
$$

**注**: 我们可以进一步分析，对递归式展开:
$$
\begin{align*}
T(n) &= T(n-1) + c \cdot n \\
&= T(n-2) + c \cdot (n-1) + c \cdot n \\
&= T(n-3) + c \cdot (n-2) + c \cdot (n-1) + c \cdot n \\
&\ \vdots \\
&= T(1) + c \cdot 2 + c \cdot 3 + \dots + c \cdot n \\
&= d + c \cdot (2 + 3 + \dots + n) \\
&= d + c \cdot \left( \frac{n(n+1)}{2} - 1 \right) \\
&= \Theta(n^2)
\end{align*}
$$
可见递归版插入排序与迭代版最坏时间复杂度一致。


> 该章节仍在编写，在 [Github仓库](https://github.com/TickPoints/algorithm_learning) 上提交PR以为本书 [贡献内容](/pr_guide/pr_standard.md)。
