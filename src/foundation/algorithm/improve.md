# 思考与提高
## 冒泡排序
介绍[插入排序](./insert_sort.md)的时候我们提到了另一种简单却低效的排序算法——**冒泡排序(Bubble Sort)**。
### BUBBLE-SORT
```rust
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    // 测试整数排序
    let mut numbers = [64, 34, 25, 12, 22, 11, 90];
    println!("排序前: {:?}", numbers);
    bubble_sort(&mut numbers);
    println!("排序后: {:?}", numbers);

    // 测试字符串排序
    let mut words = ["banana", "apple", "pear", "orange", "grape"];
    println!("排序前: {:?}", words);
    bubble_sort(&mut words);
    println!("排序后: {:?}", words);
}
```
关于这个排序，我们的循环不变式是这样的: 每次外层循环(`i`循环)结束时，数组的最后`i`个元素是已排序的，并且是整个数组的最大`i`个元素。

上式读者自证不难，从上面的循环不变式不难看出: 冒泡排序通过两两互相比较，每次都将这次最大的数据放到后面。很显然，它用的是跟插入排序一样的增量法，时间复杂度也为$T(n) = n ^2$。

当然我们也不难看出，如果一次循环没有进行任何交换说明已经排好序了，可以提前终止:
```rust
pub fn bubble_sort_optimized<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false; // 标记本轮是否发生交换
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        // 如果本轮没有交换，说明数组已经有序，提前终止
        if !swapped {
            break;
        }
    }
}
```
现在冒泡排序主要用于教学，由于效率低几乎不会实际使用。插入排序由于内部有更高效的循环机制，对于较小的数据反而更快，还可以 **与归并相结合得到TimSort**。

## 霍纳规则
**霍纳(William George Horner，1786–1837)** 是英国数学家，以提出 **霍纳规则(Horner's Rule)** 而闻名。该规则用于高效计算多项式的值。

我们来看这样一个多项式:
$$
P(x) = a_n x^n + a_{n-1} x^{n-1} + a_{n-2} x^{n-2} + \dots + a_1 x + a_0
$$
上面这样的多项式便是一般的标准多项式。霍纳规则的核心思想非常简单，即不断的提取公因式$x$，从而完成降次:
$$
P(x) = a_0 + x (a_1 + x(a_2 + x(a_3 + \dots + x (a_{n - 1} + x a_n) + \dots)))
$$
但真正重要的是，我们可以在这个过程中融入**迭代法**，从而得到:
```rust
/// 使用霍纳规则计算多项式 P(x) = a_n x^n + a_{n-1} x^{n-1} + a_{n-2} x^{n-2} + \dots + a_1 x + a_0
/// 输入：
/// - `coeffs`: 多项式系数切片，按 `a_0` 到 `a_n` 顺序排列
/// - `x`: 计算多项式值的点
/// 返回：
/// - 计算结果 `T`
/// 
/// 要求：
/// - `T` 必须支持加法 (`+`) 和乘法 (`*`) 运算
/// - `T` 必须实现 `Copy` trait(或改用 `clone()`)
pub fn horner<T>(coeffs: &[T], x: T) -> T
where
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy,
{
    coeffs.iter().rev().fold(
        // 初始值：0(如果 coeffs 为空，返回 0)
        // 注意：这里假设 T 有 `Default` 或 `Zero` trait，但 Rust 标准库没有 `Zero` trait，
        // 所以更通用的做法是 panic 或返回 Option<T> (此处简化处理)
        // 实际使用时，建议确保 coeffs 非空，或调整逻辑
        coeffs.first().copied().unwrap_or_else(|| panic!("empty coefficients")),
        |acc, &coeff| acc * x + coeff,
    )
}
```
> 注: 上面的内容使用文档注释进行了详细介绍，因为霍纳规则的通常不直接使用函数来调用，上面的写法也仅作示例。

这种迭代的思想在泰勒展开、密码学，之前提到的牛顿迭代都有广泛的应用。霍纳并非最早提出此方法，类似思想可追溯至中国古代数学家**秦九韶(《数书九章》，高次方程数值解)**，因此在中国也称为 **秦九韶算法**。

## 迭代器与循环
> 注: 下文并非本书的主体内容，通过对Rust标准的零成本迭代器进行介绍，帮助部分学者补充相关知识，并为后续大量使用迭代器代替循环做铺垫。

> 该章节仍在编写，在 [Github仓库](https://github.com/TickPoints/algorithm_learning) 上提交PR以为本书 [贡献内容](/pr_guide/pr_standard.md)。
