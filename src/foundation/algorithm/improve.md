# 思考与提高
## 插入排序
在上一章，我们深入分析了递归和迭代。现在我们可以把插入排序表示为如下的一个递归过程。为了排序`A[1..n]`，我们递归地排序`A[1..n-1]`，然后把`A[n]`插入已排序的数组`A[1..n-1]`。为插入排序的这个递归版本的最坏情况时间复杂度写一个递归式。

我们先给出一个Rust实现:
### 实现1
```rust
pub fn realize<T: Ord>(arr: &mut [T]) {
    // 基本情况: 数组长度为 0 或 1 时已经有序
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

> **注**: 其实$n \leq 1, n \in \mathbb{N}$更精准(`n = 0`时为空数组)。

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

## 二分查找
考虑以下**查找问题**:

**输入**: $n$ 个数的数组 $<a_1, a_2, \dots, a_n>$ 和一个值 $v$

**输出**: 下标$i$使得$v = A[i]$，或者当$v$不在$A$出现时，返回特殊值$NIL$

上面这个问题是我们在[插入排序](./insert_sort.md)那一节就已经介绍的。

注意到，如果序列`A`已排好序，就可以将该序列的中点与`v`进行比较。根据比较的结果，原序列中有一半就可以不用再做进一步的考虑了，这种在 **有序数组中高效查找特定元素的算法** 被称为 **二分查找(Binary Search)**。
### 实现2
```rust
use std::cmp::Ordering;
pub fn realize2<T: Ord>(arr: &[T], v: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len().checked_sub(1)?;               // 处理空数组

    while low <= high {
        let mid = low + (high - low) / 2;
        match arr[mid].cmp(v) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid - 1,
        }
    }

    None
}
```
> **注**: 在这里[`checked_sub`](https://rustwiki.org/zh-CN/std/primitive.isize.html#method.checked_sub)用于检查整数减法，计算 `self - rhs`，如果发生溢出则返回 `None`。并使用[`Ordering`](https://rustwiki.org/zh-CN/std/cmp/enum.Ordering.html)方便用`match`进行匹配。

首先，我们要确定区间`[low, high]`，`low`为数组首索引(在Rust中为`0`)，`high`为数组尾索引(在Rust中为`arr.len() - 1`)。

然后，在区间内找到中间值`mid = (low + high) / 2`，这可能会溢出，利用简单数学证明可得:
$$
\begin{align*}
mid &= low / 2 + high / 2 \\
&= low / 2 + (high -low + low) / 2 \\
&= low / 2 + (high - low) / 2 + low / 2 \\
&= low + (high - low) / 2
\end{align*}
$$
该式可以避免溢出。

利用条件`low <= high`可以确保区间`[low, high]`成立。

接着，判定`arr[mid]`与`v`的关系。当二者相等时(`Ordering::Equal`)，直接返回当前索引；如果较大说明右半部分不存在，通过收紧(`high = mid - 1`)，将右边部分排除，同理较小时用`low = mid + 1`将左半部分排除。

二分查找显然使用了分治法[^note1]，不过比较特殊的是它没有合并的步骤(并且采用迭代法)。我们利用[循环不变式](./loop_invariant.md)来证明二分查找的正确性:

> 在每次循环迭代开始时，如果目标值 `v` 存在于数组中，则 `v` 一定位于子数组 `arr[low..high-1]` 中，其中 `arr` 是已排序的数组，`low` 和 `high` 是当前搜索区间的边界索引。

这个循环不变式比较简单，留给读者自证。从上面的过程，我们不难证得$T(n) = \Theta(\log n)$。

我们增高难度，在现有的基础上，我们不再保证有且仅有一个`i`，即`A`数组中可能存在多个`v`，要求给出最小的`i`。这是一个常见的求左边界的二分算法，我们用Rust实现:
### 实现3
```rust
pub fn realize3<T: Ord>(arr: &[T], v: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len().checked_sub(1)?;

    while low < high {          // 取消等于
        let mid = low + (high - low) / 2;
        if arr[mid] < *v {      // 注意解引用
            low = mid + 1;      // 与之前相同
        } else {
            high = mid;         // 目标值在左侧或当前位置
        }
    }

    if arr.get(low)? == v {     // 使用get更安全
        Some(low)
    } else {
        None
    }
}
```
这里主要逻辑改写在比较部分: 当`arr[mid] < v`时，目标值必然在右侧，所以移动`low`；当`arr[mid] >= v`目标值可能出现在左侧或当前位置(`arr[mid] == v`)，所以移动`high`到`mid`(不是`mid - 1`)。重点在`low >= high`说明所有的`v`都已出现(这里就是上面`low < high`不使用等号的原因)，那么`low`必然在最小`v`的位置上。当然这个算法有个问题，不存在时会误报，所以要二次判断。

同理，不难写出寻找最大`i`的二分算法:
### 实现4
```rust
pub fn realize4<T: Ord>(arr: &[T], v: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len().checked_sub(1)?;

    while low < high {
        let mid = low + (high - low + 1) / 2;   // 向上取整
        if arr[mid] <= *v {
            low = mid;                          // 保留当前位置
        } else {
            high = mid - 1;
        }
    }

    if arr.get(low)? == v {
        Some(low)
    } else {
        None
    }
}
```
可以发现，比较逻辑的改写比较复杂，在比较微小的地方出错就会导致算法进入死循环或错估，所以循环不变式在判断二分算法的正确性上非常重要。

> 该章节仍在编写，在 [Github仓库](https://github.com/TickPoints/algorithm_learning) 上提交PR以为本书 [贡献内容](/pr_guide/pr_standard.md)。

[^note1]: 二分查找是通过缩小搜索区间来工作，而不是逐步构建解，因此不属于增量方法。它属于分治法的一种特例，我们称作 **减治法(decrease and conquer)**。
