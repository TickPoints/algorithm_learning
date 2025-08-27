# 二分查找
考虑以下**查找问题**:

**输入**: $n$ 个数的数组 $<a_1, a_2, \dots, a_n>$ 和一个值 $v$

**输出**: 下标$i$使得$v = A[i]$，或者当$v$不在$A$出现时，返回特殊值$NIL$

上面这个问题是我们在[插入排序](./insert_sort.md)那一节就已经介绍的。

注意到，如果序列`A`已排好序，就可以将该序列的中点与`v`进行比较。根据比较的结果，原序列中有一半就可以不用再做进一步的考虑了，这种在 **有序数组中高效查找特定元素的算法** 被称为 **二分查找(Binary Search)**。
### 实现1
```rust
use std::cmp::Ordering;

pub fn realize1<T: Ord>(arr: &[T], v: &T) -> Option<usize> {
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
\text{mid} &= \frac{\text{low}}{2} + \frac{\text{high}}{2} \\
&= \frac{\text{low}}{2} + \frac{\text{high} - \text{low} + \text{low}}{2} \\
&= \frac{\text{low}}{2} + \frac{\text{high} - \text{low}}{2} + \frac{\text{low}}{2} \\
&= \text{low} + \frac{\text{high} - \text{low}}{2}
\end{align*}
$$
该式可以避免溢出。(事实上，[标准库的实现](https://www.rustwiki.org.cn/zh-CN/src/core/slice/mod.rs.html#2779-2785)也是这么做的)

利用条件`low <= high`可以确保区间`[low, high]`成立。

接着，判定`arr[mid]`与`v`的关系。当二者相等时(`Ordering::Equal`)，直接返回当前索引；如果较大说明右半部分不存在，通过收紧(`high = mid - 1`)，将右边部分排除，同理较小时用`low = mid + 1`将左半部分排除。

二分查找显然使用了分治法[^note1]，不过比较特殊的是它没有合并的步骤(并且采用迭代法)。我们利用[循环不变式](./loop_invariant.md)来证明二分查找的正确性:

> 在每次循环迭代开始时，如果目标值 `v` 存在于数组中，则 `v` 一定位于子数组 `arr[low..high-1]` 中，其中 `arr` 是已排序的数组，`low` 和 `high` 是当前搜索区间的边界索引。

这个循环不变式比较简单，留给读者自证。从上面的过程，我们不难证得$T(n) = \Theta(\log n)$。

我们增高难度，在现有的基础上，我们不再保证有且仅有一个`i`，即`A`数组中可能存在多个`v`，要求给出最小的`i`。这是一个常见的求左边界的二分算法，我们用Rust实现:
### 实现2
```rust
pub fn realize2<T: Ord>(arr: &[T], v: &T) -> Option<usize> {
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
### 实现3
```rust
pub fn realize3<T: Ord>(arr: &[T], v: &T) -> Option<usize> {
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

在向我们刚刚的数组切片或有序数组中，[`std`](https://www.rustwiki.org.cn/zh-CN/std/primitive.slice.html)标准库提供了一系列方法:
```rust
let v = vec![1, 3, 5, 7, 9];
assert_eq!(v.binary_search(&5), Ok(2));
assert_eq!(v.binary_search(&4), Err(2));    // 插入后为 [1, 3, 4, 5, 7, 9]
```
上面这个例子中[`binary_search`](https://www.rustwiki.org.cn/zh-CN/std/primitive.slice.html#method.binary_search)返回`Result<usize, usize>`，`Ok(index)` 中 `index` 为元素所在位置，`Err(index)` 中则为未找到元素时，如果将元素插入到数组，保持有序的位置。[`binary_search_by`](https://www.rustwiki.org.cn/zh-CN/std/primitive.slice.html#method.binary_search_by)允许通过函数来设置查找规则，[`binary_search_by_key`](https://www.rustwiki.org.cn/zh-CN/std/primitive.slice.html#method.binary_search_by_key)允许通过键(如结构体字段)查找。

> **注**: 上面的这些方法和实现都要确保数组已经排序，否则返回的结果无意义。[`binary_search_by`](https://www.rustwiki.org.cn/zh-CN/std/primitive.slice.html#method.binary_search_by)之类的，通常来说与上面的手写性能相差不大，但更具有扩展性。

在较新的版本(`Rust 1.52+`)中，[`partition_point`](https://www.rustwiki.org.cn/zh-CN/std/primitive.slice.html#method.partition_point) 可以用来返回满足条件的第一个元素的位置:
```rust
let v = vec![1, 2, 2, 3, 3, 4, 5];
println!("{}", v.partition_point(|&x| x < 4));  // 第一个不小于4的元素位置
```
> **注**: 该函数底层是`self.binary_search_by(|x| if pred(x) { Less } else { Greater }).unwrap_or_else(|i| i)`这种写法使得`binary_search`永远找不到等于的位置，所以就会返回插入之后仍然有序的位置，也就是第一个不满足`pred`函数的位置。(其中`pred`是调用者的输入)

我们重新来看[插入排序](./insert_sort.md):
```rs
pub fn insert_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}
```
其中
```rs
while j > 0 && arr[j] < arr[j - 1] {
    arr.swap(j, j - 1);
    j -= 1;
}
```
这个部分是将需要排序的元素在有序数组中移动找到适合的位置，在有序数组中查找，完全可以利用二分:
### 实现4
```rust
pub fn realize4<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        // 此处直接使用标准库，下面代码完全可以用`partition_point`进行修改，取决于您
        let pos = arr[..i].binary_search(&arr[i]).unwrap_or_else(|pos| pos);
        // 将整个区间右移1个单位(直接使用`swap`理论上也完全可以)
        arr[pos..=i].rotate_right(1);
    }
}
```
上面这个优化仅仅改变了比较次数，不影响总体的时间复杂度。对于大规模数据和操作代价较高的会有一定优化，其他情况下还是使用线性更优。

> 该章节仍在编写，在 [Github仓库](https://github.com/TickPoints/algorithm_learning) 上提交PR以为本书 [贡献内容](/pr_guide/pr_standard.md)。

[^note1]: 二分查找是通过缩小搜索区间来工作，而不是逐步构建解，因此不属于增量方法。它属于分治法的一种特例，我们称作 **减治法(decrease and conquer)**。