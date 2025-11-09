# 二分查找
考虑以下**查找问题**:

**输入**: $n$ 个数的数组 $[a_1, a_2, \dots, a_n]$ 和一个值 $v$

**输出**: 下标$i$使得$v = A[i]$，或者当$v$不在$A$出现时，返回特殊值$NIL$

上面这个问题是我们在[插入排序](./insert_sort.md)那一节就已经介绍的。

注意到，如果序列`A`已排好序，就可以将该序列的中点与`v`进行比较。根据比较的结果，原序列中有一半就可以不用再做进一步的考虑了，这种在 **有序数组中高效查找特定元素的算法** 被称为 **二分查找(Binary Search)**。
### BINARY-SEARCH
```rust
use std::cmp::Ordering;

pub fn binary_search<T: Ord>(arr: &[T], v: &T) -> Option<usize> {
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
> [!NOTE]
> 在这里[`checked_sub`](https://rustwiki.org/zh-CN/std/primitive.isize.html#method.checked_sub)用于检查整数减法，计算 `self - rhs`，如果发生溢出则返回 `None`。并使用[`Ordering`](https://rustwiki.org/zh-CN/std/cmp/enum.Ordering.html)方便用`match`进行匹配。

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
```rust
pub fn find_left_boundary<T: Ord>(arr: &[T], v: &T) -> Option<usize> {
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
```rust
pub fn find_right_boundary<T: Ord>(arr: &[T], v: &T) -> Option<usize> {
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

> [!NOTE]
> 上面的这些方法和实现都要确保数组已经排序，否则返回的结果无意义。[`binary_search_by`](https://www.rustwiki.org.cn/zh-CN/std/primitive.slice.html#method.binary_search_by)之类的，通常来说与上面的手写性能相差不大，但更具有扩展性。

在较新的版本(`Rust 1.52+`)中，[`partition_point`](https://www.rustwiki.org.cn/zh-CN/std/primitive.slice.html#method.partition_point) 可以用来返回满足条件的第一个元素的位置:
```rust
let v = vec![1, 2, 2, 3, 3, 4, 5];
println!("{}", v.partition_point(|&x| x < 4));  // 第一个不小于4的元素位置
```
> [!NOTE]
> 该函数底层是`self.binary_search_by(|x| if pred(x) { Less } else { Greater }).unwrap_or_else(|i| i)`这种写法使得`binary_search`永远找不到等于的位置，所以就会返回插入之后仍然有序的位置，也就是第一个不满足`pred`函数的位置。(其中`pred`是调用者的输入)

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
```rust
pub fn insert_sort_by_binary_search<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        // 此处直接使用标准库，下面代码完全可以用`partition_point`进行修改，取决于您
        let pos = arr[..i].binary_search(&arr[i]).unwrap_or_else(|pos| pos);
        // 将整个区间右移1个单位(直接使用`swap`理论上也完全可以)
        arr[pos..=i].rotate_right(1);
    }
}
```
上面这个优化仅仅改变了比较次数，不影响总体的时间复杂度。对于大规模数据和操作代价较高的会有一定优化，其他情况下还是使用线性更优。

## 两数之和
考虑下面这道题:

**输入**: $n$ 个整数的数组 $[a_1, a_2, \dots, a_n]$ 和一个整数 $x$

**输出**: 下标 $i_1$和$i_2$ 使得 $A[i_1] + A[i_2] = x$，或者当数组中无两数之和为 $x$ 时，返回特殊值$NIL$

> [!TIP]
> 保证有且仅有一个满足条件的解

我们将用**增量法**，**分治法**和一**特殊方法**解决此问题。

增量法最简单，我们可以用两层循环:
```rust
pub fn incremental(arr: &[i32], x: i32) -> Option<(usize, usize)> {
    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if arr[i] + arr[j] == x {
                return Some((i, j));
            }
        }
    }
    None
}
```
但复杂度将达到$\Theta(n ^ 2)$。

分治法则不难想到二分查找:
```rust
pub fn divide_and_conquer(arr: &[i32], x: i32) -> Option<(usize, usize)> {
    // 先对数组进行排序 (保留原索引)
    let mut sorted: Vec<(usize, &i32)> = arr.iter().enumerate().collect();
    sorted.sort_by(|a, b| a.1.cmp(b.1));
    
    let mut left = 0;
    let mut right = sorted.len() - 1;
    
    while left < right {
        let sum = sorted[left].1 + sorted[right].1;
        if sum == x {
            // 确保返回的索引顺序与原数组一致
            let (i1, i2) = (sorted[left].0, sorted[right].0);
            return Some((i1.min(i2), i1.max(i2)));
        } else if sum < x {
            left += 1;
        } else {
            right -= 1;
        }
    }
    
    None
}
```
该方案虽然使用了排序，但按照归并排序的时间复杂度(标准库的排序实现更为复杂，这里简单的以归并排序为例)，该实现仍然是$\Theta(n \log n)$。

最后一种方法非常特殊，在后面的课程中我们会具体讲到，这里简单介绍一下:

**哈希表(Hash Map 或 Hash Table)** 是一种通过 **哈希函数(Hash Function)** 将 键(key) 映射到存储位置的数据结构。具体来说，哈希函数将任意类型的键转换为一个固定范围内的整数(称为哈希值)，该值作为索引用于在数组中存储对应的值。借助数组的随机访问特性，哈希表在理想情况下的查找、插入和删除操作的时间复杂度均为 $\Theta(1)$。

```rust
// 使用标准库实现的哈希表
use std::collections::HashMap;

pub fn search_by_hash_map(arr: &[i32], x: i32) -> Option<(usize, usize)> {
    let mut map = HashMap::new();
    
    for (i, &num) in arr.iter().enumerate() {
        // 计算与当前项相加等于x的值
        let complement = x - num;
        // 在哈希表中寻找是否有complement
        if let Some(&j) = map.get(&complement) {
            // 如果有，直接返回当前索引和complement所在索引
            return Some((j, i));
        }
        // 否则，保存当前项对应的索引
        map.insert(num, i);
    }
    
    None
}
```
该算法是单循环的，又因为哈希表的读取运算是常数时间，所以这个实现时间复杂度为$\Theta(n)$。

---
## 练习与回答
1. 我们作以下考虑: 虽然归并排序的最坏情况运行时间为$\Theta(n \log n)$，而插入排序的最坏情况运行时间为$\Theta(n ^ 2)$，但是插入排序中的常量因子可能使得它在$n$较小时，在许多机器上实际运行得更快。因此，在归并排序中当子问题变得足够小时，采用插入排序来使递归的叶**变粗是有意义的**。考虑对归并排序的一种修改，使用插入排序来排序长度为$k$的$n/k$个子表，然后使用标准的合并机制来合并这些子表，这里$k$是一个待定的值。
```rust
pub fn merge_sort_by_insert(arr: &mut [i32], k: usize) {
    let n = arr.len();
    if n <= k {
        insertion_sort(arr);
        return;
    }
    let mid = n / 2;
    merge_sort_by_insert(&mut arr[..mid], k);
    merge_sort_by_insert(&mut arr[mid..], k);
    merge(arr, mid);
}

fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn merge(arr: &mut [i32], mid: usize) {
    let mut temp = arr.to_vec();
    let (mut i, mut j, mut k) = (0, mid, 0);
    while i < mid && j < arr.len() {
        if temp[i] <= temp[j] {
            arr[k] = temp[i];
            i += 1;
        } else {
            arr[k] = temp[j];
            j += 1;
        }
        k += 1;
    }
    while i < mid {
        arr[k] = temp[i];
        i += 1;
        k += 1;
    }
    while j < arr.len() {
        arr[k] = temp[j];
        j += 1;
        k += 1;
    }
}
```
这种实现与 **Timsort**[^note2] 的核心思想一致，结合了归并与插入的优点，接下来我们深入讨论:

在上述算法中，插入排序可以在$\Theta(n k)$时间内排序每个长度为$k$的$n / k$个子表(每个子表排序的时间复杂度为$\Theta(k ^ 2)$，故$T(n) = \Theta(k ^ 2) \cdot (n / k) = \Theta(n k)$)。同样不难看出，合并子表的时间复杂度是$\Theta(n \log (n / k))$，综上就有该算法最坏情况下的时间复杂度为$\Theta(n k + n \log (n / k))$。为了确保任何$k$的取值不能使修改后算法时间复杂度高于原算法，$k = O(\log n)$[^note3]。

在实际使用中，$k$不能随着的$n$增长增长过快，所以通常使用常数或对数级，一般选用20至100的常数，或者利用$k = \log n$的对数情况动态调整。更现实化的时候，可以通过cpu缓存规则或者混合策略(分段使用常数和对数)来解决。

[^note1]: 二分查找是通过缩小搜索区间来工作，而不是逐步构建解，因此不属于增量方法。它属于分治法的一种特例，我们称作 **减治法(decrease and conquer)**。

[^note2]: **Timsort** 是一种高效的混合排序算法，由 **提姆·彼得斯(Tim Peters)** 在 2002 年为 Python 语言设计(Python 2.3 及后续版本的默认排序算法)。其针对现实世界中的部分有序数据进行了优化，具有稳定性和适应性。

[^note3]: 即$k$的增长速度不超过$\log n$，这里读者可以先行自证，此类标准的证明方法在后续讲到。
