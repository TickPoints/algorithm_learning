# 文档语法
本书基于 [mdBook](https://rust-lang.github.io/mdBook)，一脉相传地继承了 [CommonMark](https://commonmark.org) 的语法，标准如下:
## 文本和段落
```md
文本1，文本2

一个段落

段落间要求空行
简单换行会变为空格
```
### 渲染
文本1，文本2

一个段落

段落间要求空行
简单换行会变为空格

## 标题
标题使用`#`标记，并且应该单独占一行。更多的`#`表示更小的标题:
```md
文本和段落可在标题前
#### A
文本和段落可在两标题间
#### B

空行不影响渲染

#### C
文本和段落可在两标题后

##### 更小标题
```
### 渲染
文本和段落可在标题前
#### A
文本和段落可在两标题间
#### B

空行不影响渲染

#### C
文本和段落可在两标题后

##### 更小标题

### 注意
1. 建议`#`数量在 1 至 5 间。
2. 标题应明显分割内容以增强阅读体验。
3. 是否空行取决于语意关系。

## 列表
列表可以是无序的或有序的，有序列表将自动排序:
```md
#### 无序写法1(方形)
* 换行不影响
* 换行不影响

* 换行
不影响
#### 无序写法2(圆形)
- 换行不影响
- 换行不影响

- 换行
不影响

#### 有序写法1
1. 换行不影响

1. 换行不影响
1. 换行
不影响
#### 有序写法2
1. 换行不影响
2. 换行不影响

3. 换行
不影响

列表支持嵌套:
1. 有序写法
    1. A
    2. B
    3. C
2. 无序写法
    * 1
        - 1
        - 2
        - 3
    * 4
        1. A
        2. B
```
### 渲染
#### 无序写法1(方形)
* 换行不影响
* 换行不影响

* 换行
不影响
#### 无序写法2(圆形)
- 换行不影响
- 换行不影响

- 换行
不影响

#### 有序写法1
1. 换行不影响

1. 换行不影响
1. 换行
不影响
#### 有序写法2
1. 换行不影响
2. 换行不影响

3. 换行
不影响

列表支持嵌套:
1. 有序写法
    1. A
    2. B
    3. C
2. 无序写法
    * 1
        - 1
        - 2
        - 3
    * 4
        1. A
        2. B

### 注意
1. 推荐用**有序写法2**和**无序写法2(圆形)**

## 链接
链接到 URL 或本地文件很简单:
```md
本书用 [mdBook](https://github.com/rust-lang/mdBook)。

你可阅读 [README](./README.md) 来了解PR方式。

(直接用不影响)

[直接用不影响]

(本地路径采用相对，不应超出src的范围)

裸链接: <https://rust-lang.github.io/mdBook/format/markdown.html>
```
### 渲染
本书用 [mdBook](https://github.com/rust-lang/mdBook)。

你可阅读 [README](./README.md) 来了解PR方式。

(直接用不影响)

[直接用不影响]

(本地路径采用相对，不应超出src的范围)

裸链接: <https://rust-lang.github.io/mdBook/format/markdown.html>

### 注意
1. 以`.md`结尾的相对链接将被转换为`.html`扩展名。建议尽可能使用`.md`链接。这在 Markdown 文件在 mdBook 外部查看时很有用，例如在 GitHub 或 GitLab 上，这些平台会自动渲染 Markdown。
2. 链接到`README.md`将会被转换为`index.html`。这是因为在某些服务(如 GitHub)中，它们会自动渲染 README 文件，而网页服务器通常期望根文件被称为`index.html`。
3. 用`#`链接到各个标题，如[文本和段落](#文本和段落) (`[文本和段落](#文本和段落)`) 和 [PR贡献指南](./README.md#PR贡献指南) (`[PR贡献指南](./README.md#PR贡献指南)`)

## 图片
包含图片只需包含指向它们的链接(URL或本地文件)，就像上面链接部分一样: 
```md
![License](https://img.shields.io/github/license/TickPoints/algorithm_learning)
```
### 渲染
![License](https://img.shields.io/github/license/TickPoints/algorithm_learning)

## 加粗
文本可以通过在每个侧面包裹两个星号来渲染:
```md
**加粗**
```
**加粗**

## 代码
通过在文本两侧添加反引号从而在单行中获取代码段，推荐为数字(如`1`)和小型代码片段(如`while`)添加。

通过三个连续的反引号(\`\`\`)来创造代码框，在首三个连续的反引号后添加如`rs(Rust)`，`md(Markdown)`，`text`来控制语言和渲染逻辑。
~~~md
```rs
pub fn main() {
    println!("Hello, World!");
}
```

上面在主函数(`main`)中，输出了`Hello, World!`
~~~
### 渲染
```rs
pub fn main() {
    println!("Hello, World!");
}
```

上面在主函数(`main`)中，输出了`Hello, World!`
### 注意
**"\`\`\`"只在一行的开头使用才有效**，另外使用`~~~`也可造成一样的效果。

## 扩展
**mdBook 在标准 CommonMark 规范之外有几个扩展**，同时本书也在这个基础上添加了一些修改:
### 删除线
文本可以通过在每个侧面包裹两个波浪号来渲染:
```md
你好，Rust~~世界~~!
```
#### 渲染
你好，Rust~~世界~~!

### 脚注
脚注会在文本中生成一个小编号的链接，点击后会将读者带到该项底部的脚注文本[^note1]。脚注标签的写法类似于链接引用，前面有一个尖角符号。
```
脚注会在文本中生成一个小编号的链接，点击后会将读者带到该项底部的脚注文本[^note1]。脚注标签的写法类似于链接引用，前面有一个尖角符号。

[^note1]: 必须放在文档底部。
```

#### 注意
1. 推荐以`[^noteX]`的格式书写。

### 表格
阅读Github的 [表格扩展规范](https://github.github.com/gfm/#tables-extension-)

### 数学公式
我们采用[mdBook-KaTeX](https://crates.io/crates/mdbook-katex)渲染[LaTex](https://www.latex-project.org)数学表达式。而不使用[mdBook](https://github.com/rust-lang/mdBook)自带的[MathJax支持](https://rust-lang.github.io/mdBook/format/mathjax.html)以避免一些问题并增强体验。

一般格式如下:
```
行内: $ f(x) = ax^2 + bx + c$

全行:
$$
f(x) = x^2 \\
x \in \R
$$

用`\$`而正常显示\$
```
#### 渲染
行内: $ f(x) = ax^2 + bx + c$

全行:
$$
f(x) = x^2 \\
x \in \R
$$

用`\$`而正常显示\$

### 引用和警告
我们采用[mdbook-alerts](https://crates.io/crates/mdbook-alerts)来添加[Github Markdown扩展的警告功能](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#alerts)支持。同时利用**mdbook**自带的引用功能。

一般格式类似于:
```md
> 引用信息
```
引用文本推荐:
```md
> **“需要引用的信息”**
>
> —— *来源说明*
```
基于引用行还支持警告功能:
```md
请注意，由于mdbook-alerts的BUG，下面内容可能会被转译为html，html的形式在本书中不应该出现，如果需要继续阅读这个代码块可以进入编辑页面查看原始代码。

> [!NOTE]
> 注意信息(常用)。

> [!TIP]
> 提示信息，尽量简短，任何超过20个字的提示信息都应该使用注意信息(一般)。

> [!IMPORTANT]
> 重要信息，非常重要的信息，但又尽量简短。较长的重要信息可以直接在段落中显现(极少用)。

> [!WARNING]
> 警告信息，来自于本书自身的问题，不应该与知识类内容相混淆。任何来自本书自身的问题且需要警告的信息都必须要使用这个(常用)。

> [!CAUTION]
> 谨慎使用信息。在本书中一般不允许使用。
```
#### 渲染
一般格式类似于:

> 引用信息

引用文本推荐:

> **“需要引用的信息”**
>
> —— *来源说明*

基于引用行还支持警告功能:

> [!NOTE]
> 注意信息(常用)。

> [!TIP]
> 提示信息，尽量简短，任何超过20个字的提示信息都应该使用注意信息(一般)。

> [!IMPORTANT]
> 重要信息，非常重要的信息，但又尽量简短。较长的重要信息可以直接在段落中显现(极少用)。

> [!WARNING]
> 警告信息，来自于本书自身的问题，不应该与知识类内容相混淆。任何来自本书自身的问题且需要警告的信息都必须要使用这个(常用)。

> [!CAUTION]
> 谨慎使用信息。在本书中一般不允许使用。

### mermaid
我们采用[mdbook-mermaid](https://crates.io/crates/mdbook-mermaid)来添加添加[mermaid.js](https://mermaid.js.org/#/)支持。使用Mermaid来渲染好的图表，帮助理解。

一般格式类似于[跨行代码](#代码)，在代码框内采用Mermaid的格式:
~~~md
```mermaid
graph TD;
    A-->B;
    A-->C;
    B-->D;
    C-->D;
```
~~~
#### 渲染
```mermaid
graph TD;
    A-->B;
    A-->C;
    B-->D;
    C-->D;
```

## 注意
以上内容为编写文档时可能用到的所有语法，更多语法可参见其他地方(如Github文档)，上面有很多推荐，文档编写者应该尽量满足，同时严格的编写语法是必须满足的。

我们支持 `mermaid` 的特别信息:
- 版本: `11.10.1`(随着需要更新)

[^note1]: 必须放在文档底部。
