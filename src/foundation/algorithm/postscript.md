# 后记
## 迭代器与循环
> [!NOTE]
> 通过对Rust标准的迭代器进行介绍，帮助部分学者补充相关知识，并为后续大量使用迭代器代替循环做铺垫。

迭代器从名字来看就跟迭代有关，我们来回顾一下迭代法: 通过循环结构逐步逼近目标。可见迭代就是通过循环结构进行操作。然而很多循环结构，例如遍历等的代码具有非常普遍的相似性，迭代器就可以用来帮助我们减少相似的代码。所以: **迭代器(Iterator)** 是负责遍历序列中的每一项和决定序列何时结束的逻辑结构。

在Rust中，迭代器是**惰性的**，它是实现了[Trait `std::iter::Iterator`](https://www.rustwiki.org.cn/zh-CN/std/iter/trait.Iterator.html)的数据结构。基本的迭代器分为三类: `IterMut`，`Iter`，`IterMut`，分别代表着不同所有权机制(拥有，不可变引用，可变引用)。对应的可用`into_iter`等函数来取得。迭代的基础是数据，我们必须在数据上进行迭代，标准库大部分数据都实现了[`Iterator`](https://www.rustwiki.org.cn/zh-CN/std/iter/trait.Iterator.html):
```rs
let nums = vec![1i32, 2, 3];
let squared: Vec<i32> = nums.iter();
```
由于惰性性质，在不使用任何方法时，迭代器不产生任何效果(仅在使用时求值评估)。

接下来介绍相关方法:
```rs
// 下面相关方法均省略有关`self`和范型的定义
map(FnMut(Self::Item) -> B) -> Map<Self, F>
filter(FnMut(&Self::Item) -> bool) -> Filter<Self, F>
```
上面两个方法用来过滤元素或进行中间操作:
```rust
// 将数据平方
let nums = vec![1i32, 2, 3];
let squared: Vec<i32> = nums.iter()
    .map(|&x: &i32| x.pow(2)) // 明确参数类型
    .collect();
// 筛选数据
let numbers = 1..10;
let evens: Vec<_> = numbers
    .filter(|x: &i32| x % 2 == 0) // 明确类型注解
    .collect();
```
它们都需要使用`collect`，方便在最后转换为数据集合，上面使用的中间结构(`Map`等)同样是迭代器，因此你是可以进行连续操作的。
```rs
fold(init: B, FnMut(B, Self::Item) -> B) -> B
```
同样是最后操作，但其结果是一个具体值，利用它，我们可以进行求和操作:
```rust
let sum = [1, 2, 3].iter()
    .fold(0i32, |acc: i32, &x| acc + x);
```
我们可以通过另一个函数`reduce(FnMut(B, Self::Item) -> B) -> Option<B>`，来寻找最大值:
```rust
let max = [5, 3, 8].iter()
    .reduce(|a: &i32, b: &i32| if a > b { a } else { b });
```
这两个函数在一定程度上可以互相转换，`fold`函数是要求初始值，`reduce`则不用，其期待用途也不同。

另外常用的是几个控制流操作:
```rs
// 限制最大元素数量
take(n: usize) -> Take<Self>
// 转为带索引元组
enumerate() -> Enumerate<Self>
// 跳过前几个元素
skip(n: usize) -> Skip<Self>
// 压缩(合并)迭代器
zip(other: U) -> Zip<Self, <U as IntoIterator>::IntoIter>
// 展开嵌套结构
flatten() -> Flatten<Self>
// 在迭代器每个元素上调用闭包(与`map`不同，它不返回内容)
for_each<F>(f: FnMut(Self::Item))
// 在中间观测
inspect(f: FnMut(&Self::Item)) -> Inspect<Self, FnMut(&Self::Item)>
```
使用示例如下:
```rust
// 取前5个元素
let taken = (1..).take(5);    // 1~5 (由于惰性性质，可安全切割)

// 带索引遍历
for (i, c) in "Rust".chars().enumerate() {
    println!("{}: {}", i, c);
}
// 0: 'R' 1: 'u' 2: 's' 3: 't'

// 跳过前3个元素
let skipped = (1..10).skip(3); // 4~9

// 合并两个序列
let zipped: Vec<_> = (1..4)
    .zip(["a", "b", "c"])
    .collect();
// [(1, "a"), (2, "b"), (3, "c")]

// 展开嵌套结构
let matrix = [[1, 2], [3, 4]];
let flat: Vec<_> = matrix.iter().flatten().collect();
// [1, 2, 3, 4]

// 调试观察
(1..4).inspect(|x| println!("Processing: {}", x))
    .for_each(|x| { /* 逻辑 */ });
```
### 更多
Rust的`for`循环语法实际上是迭代器的语法糖:
```rust
let values = vec![1, 2, 3, 4, 5];

for x in values {
    println!("{x}");
}
```
Rust将其反糖化为:
```rust
let values = vec![1, 2, 3, 4, 5];
{
    let result = match IntoIterator::into_iter(values) {
        mut iter => loop {
            let next;
            match iter.next() {
                Some(val) => next = val,
                None => break,
            };
            let x = next;
            let () = { println!("{x}"); };
        },
    };
    result
}
```
另外我们常使用的区间也是基于迭代器的:
```rust
let numbers = 0..;                      // 生成一个无限迭代器
let five_numbers = numbers.take(5);     // 切割为有限

for number in five_numbers {
    println!("{number}");
}
```
由于Rust迭代器的零成本特性，它一般不会拖累性能。甚至在某些优化条件下，拥有比默认情况(手写的一般逻辑)更快的效果。

## 本章历史背景
### 循环不变式
循环不变式概念源于程序形式化验证的需求。**罗伯特·弗洛伊德(Robert W. Floyd)** 是1967年的图灵奖得主，同年他在论文 **《为程序分配意义》(Assigning Meanings to Programs)** 中首次系统化提出循环不变式，作为证明程序正确性的数学工具。**东尼·霍尔(Tony Hoare)** 是1980年图灵奖得主，在1969年发表 **《计算机程序设计的公理基础》(An Axiomatic Basis for Computer Programming)**，提出**霍尔逻辑(Hoare Logic)**，将循环不变式纳入公理化验证框架。

### 插入排序
这个算法思想早见于计算机科学之前(1938年的时候就有通过机械来对卡片进行排序的系统)，1950年代在计算机上系统实现。

1970年代后，弗洛伊德和霍尔的循环不变式正式在教科书中用来证明。

### 归并排序
**约翰·冯·诺依曼(John von Neumann)** 在1945年提出归并排序，最早采用分治思想的排序算法。他自己并没有使用循环不变式，但分治递归的思想天然适用于循环不变式的证明框架。

**托马斯·科尔曼(Thomas H. Cormen)** 等人在1990版的《算法导论》中首次将循环不变式作为算法分析的必备工具。

### 时间复杂度
**艾伦·图灵(Alan Turing)** 1936年在论文 **《关于可计算数》(On Computable Numbers)** 中隐含“计算步数”概念，为时间复杂度奠基。

**尤里斯·哈特马尼斯和理查德·斯特恩斯(Juris Hartmanis & Richard E. Stearns)** 在1965年论文 **《关于算法的计算复杂度》(On the Computational Complexity of Algorithms)** 首次明确定义“时间复杂度”(Time Complexity)，获1993年图灵奖。

### 渐近符号
**保罗·巴赫曼(Paul Bachmann)** 在数论著作(1894)中首创大$O$符号。后来由**唐纳德·克努特(Donald Knuth)** 在1976年 **《计算机程序设计艺术》(卷1)** 中推广其用于算法分析，在其同一著作另一卷中使用了$\Theta$定义严格紧确界，同年也定义了下界(它们的严格定义将在下一章介绍)。

> “**Floyd的循环不变式和Knuth的$\Theta$记号，将算法分析从经验描述提升为精确科学。”**
>
> —— *《算法设计的艺术》(The Art of Algorithm Design, 2015)*
