# 插入排序
## 主问题重现
我们在下面用形式化语言[^note1] 给出主问题的一个重现:
### 输入
$n$ 个数的数组 $[a_1, a_2, \dots, a_n]$
### 输出
输入数组的升序排列 $[a_1', a_2', \dots, a_n']$，满足 $a_1' \leq a_2' \leq \dots \leq a_n'$
## 主问题思考
这个主问题是我们本章将要讨论的基点。是一个非常经典的排序问题。在这里我们引入一个相对简单的排序: **插入排序(Insert Sort)**。

来看下面实现:
### INSERT-SORT
```rust
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
可以发现: **插入排序的核心逻辑是通过逐步比较和移动元素来维护一个有序的子序列**。具体地:
- 有序部分逐步扩张：将数组分为已排序部分(初始仅含第一个元素)和未排序部分，每次从未排序部分取出第一个元素，将其插入到已排序部分的正确位置，直到所有元素有序。
- 原地插入：插入操作通过依次向后移动元素实现，无需额外空间。
> [!NOTE]
> 这里我们采用了泛型，使得任何实现了[`Ord trait`](https://rustwiki.org/zh-CN/std/cmp/trait.Ord.html)的数组(切片)均可进行插入排序。

另一种方法不通过[`swap`](https://rustwiki.org/zh-CN/std/primitive.slice.html#method.swap)交换，而是将操作值保存，直到最后再进行赋值，像接下来的实现:
```rust
pub fn insert_sort_of_copy<T: Ord + Copy>(arr: &mut [T]) {
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
> [!NOTE]
> 需要[`Copy`](https://rustwiki.org/zh-CN/std/marker/trait.Copy.html)

同理，也可以使用[`Clone`](https://rustwiki.org/zh-CN/std/clone/trait.Clone.html):
```rust
pub fn insert_sort_of_clone<T: Ord + Clone>(arr: &mut [T]) {
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
1. 尝试将[INSERT-SORT](#INSERT-SORT)改为降序排列(满足 $a_1' \geq a_2' \geq \dots \geq a_n'$ )。

```rust
pub fn insert_sort_in_reverse<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() - 1 {
        let mut j = i;
        while j > 0 && arr[j] > arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}
```
> [!NOTE]
> 可以发现: 在插入排序中，只需修改比较条件即可改变排序方向

所以我们也有下面这种设置排序规则的:
```rust
// 实现以`compare`来控制排序规则的插入排序
pub fn insert_sort_by<T, F>(arr: &mut [T], compare: F)
where
    F: Fn(&T, &T) -> bool,
{
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && compare(&arr[j], &arr[j - 1]) {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}
```
上面这种比较，对于基础较好的读者来说，应该不成问题。这个比较很显然的允许我们操作一些更特殊的类型，比如说结构体:
```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

pub fn insert_sort_by<T, F>(arr: &mut [T], compare: F)
where
    F: Fn(&T, &T) -> bool,
{
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && compare(&arr[j], &arr[j - 1]) {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn main() {
    // 创建可变的结构体数组
    let mut people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];
    
    // 按年龄升序排序
    realize4(&mut people, |a, b| a.age < b.age);
    println!("按年龄排序: {:?}", people);
    
    // 按名字字典序排序
    realize4(&mut people, |a, b| a.name < b.name);
    println!("按名字排序: {:?}", people);
}
```
我们再来看别的例题，继续深化学习:

2. 考虑以下**查找问题**:

**输入**: $n$ 个数的数组 $[a_1, a_2, \dots, a_n]$ 和一个值 $v$

**输出**: 下标$i$使得$v = A[i]$，或者当$v$不在$A$出现时，返回特殊值$NIL$[^note2]

尝试给出一个**线性查找**[^note3]的例子。

> 观察来看，这道题目适合[`Option`](https://rustwiki.org/zh-CN/std/option/enum.Option.html)可以用`Option::None`来解决`NIL`。

### LINEAR-SEARCH
```rust
pub fn linear_search<T: PartialEq>(arr: &[T], v: T) -> Option<usize> {
    for i in 0..arr.len() {
        if v == arr[i] {
            return Some(i);
        }
    }
    None
}
```
> 这里用 [PartialEq](https://rustwiki.org/zh-CN/std/cmp/trait.PartialEq.html) 而非 [Eq](https://rustwiki.org/zh-CN/std/cmp/trait.Eq.html) 以支持部分(宽松)相等，而无需反射(自反)

[^note1]: 本处"形式化语言"并非指是一种由严格定义的符号组成的字符串集合，而是描述一类算法对于 输入/输出 关系的控制，以自然语言化，较精准化，弱数学化提供算法行为。

[^note2]: `NIL`是算法描述中表示"空"或"终止"的通用符号。并且与`NULL`等空指针中区分。在一些结构(如链表)，`NIL`是一类特殊哨兵值(后见)，在标准库实现中，NIL就有所存在。

[^note3]: **线性查找(Linear Search)**，也称为顺序查找，是一种简单直观的搜索算法。它的核心思想是逐个遍历数据集中的元素，直到找到目标值或遍历完所有元素。
