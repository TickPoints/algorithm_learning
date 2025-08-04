# 插入排序
## 主问题重现
我们在下面用形式化语言[^note1] (已加之修改)给出主问题的一个重现:
### 输入
\\(n\\) 个数的数组 \\(<a_1, a_2, \dots, a_n>\\)
### 输出
输入数组的升序排列 \\(<a_1', a_2', \dots, a_n'>\\)，满足 \\(a_1' \leq a_2' \leq \dots \leq a_n'\\)
## 主问题思考
在《算法导论》中，我们先引入了插入排序，这并不特别简单(不如冒泡)但可以让我们进行较高级的初步学习。

来看下面实现:
### 实现1
```rs
pub fn realize<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}
```
可以发现: 插入排序的核心逻辑是通过逐步比较和移动元素来维护一个有序的子序列。具体地:
- 有序部分逐步扩张：将数组分为已排序部分(初始仅含第一个元素)和未排序部分，每次从未排序部分取出第一个元素，将其插入到已排序部分的正确位置，直到所有元素有序。
- 原地插入：插入操作通过依次向后移动元素实现，无需额外空间。
> **TIP**: 这里我们采用了泛型，使得任何实现了[`Ord trait`](https://rustwiki.org/zh-CN/std/cmp/trait.Ord.html)的数组切片(理论上[`Vec`](https://rustwiki.org/zh-CN/std/vec/struct.Vec.html)也会自动转换)均可进行插入排序

在《算法导论》中，其实是不是通过[`swap`](https://rustwiki.org/zh-CN/std/primitive.slice.html#method.swap)，而是将操作值保存，直到最后再进行赋值，这样可以减少中间的连续交换导致的空间和时间损耗。代价是需要更高的数学水平，同时需要更多的`trait`(像接下来的实现)。
### 实现2
```rs
pub fn realize2<T: Ord + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        // 保存当前元素的值
        let key = arr[i];
        // 找到合适的插入位置
        let mut j = i;
        while j > 0 && key < arr[j - 1] {
            // 向后移动元素而不是交换
            arr[j] = arr[j - 1];
            j -= 1;
        }
        // 将保存的值插入到正确位置
        arr[j] = key;
    }
}
```
(**TIP**: 需要[`Copy`](https://rustwiki.org/zh-CN/std/marker/trait.Copy.html))

也可以使用[`Clone`](https://rustwiki.org/zh-CN/std/clone/trait.Clone.html):
```rs
pub fn realize2_clone<T: Ord + Clone>(arr: &mut [T]) {
    for i in 1..arr.len() {
        // 保存当前元素的值
        let key = arr[i].clone();
        // 找到合适的插入位置
        let mut j = i;
        while j > 0 && key < arr[j - 1] {
            // 向后移动元素而不是交换
            arr[j] = arr[j - 1].clone();
            j -= 1;
        }
        // 将保存的值插入到正确位置
        arr[j] = key;
    }
}
```
---
## 练习与回答
1. 尝试将[实现1](#实现1)改为降序排列(满足 \\(a_1' \geq a_2' \geq \dots \geq a_n'\\) )。
### 实现3
```
// 对应练习 1
pub fn realize3<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() - 1 {
        let mut j = i;
        while j > 0 && arr[j] > arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}
```

[^note1]: 本处"形式化语言"并非指是一种由严格定义的符号组成的字符串集合，而是描述一类算法对于 输入/输出 关系的控制，以自然语言化，较精准化，弱数学化提供算法行为。