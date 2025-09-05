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

> 该章节仍在编写，在 [Github仓库](https://github.com/TickPoints/algorithm_learning) 上提交PR以为本书 [贡献内容](/pr_guide/pr_standard.md)。
