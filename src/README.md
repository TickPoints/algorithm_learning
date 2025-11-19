# 算法学习
![License](https://img.shields.io/github/license/TickPoints/algorithm_learning)
![Stars](https://img.shields.io/badge/dynamic/json?colorA=0D1117&colorB=1F6FEB&label=Stars&query=$.stargazers_count&url=https://api.github.com/repos/TickPoints/algorithm_learning)
![Deploy Status](https://img.shields.io/github/deployments/tickpoints/algorithm_learning/github-pages?label=Deploy%20Status&colorA=1e1e3f&colorB=a162e8)
![Last Commit](https://img.shields.io/github/last-commit/TickPoints/algorithm_learning/main?label=Last%20Commit&colorA=1e1e3f&colorB=20c997)
## 目的
用**Rust**实现[《算法导论》第3版](https://github.com/0voice/expert_readed_books/blob/master/%E8%AE%A1%E7%AE%97%E6%9C%BA%E7%A7%91%E5%AD%A6/%E7%AE%97%E6%B3%95%E5%AF%BC%E8%AE%BA_%E5%8E%9F%E4%B9%A6%E7%AC%AC3%E7%89%88_CHS.pdf)中的所有伪代码，同时对一些例题进行解答。本项目专为中文开发者设计，旨在辅助算法学习与实践。
## 阅读
本书用[**mdBook**](https://rust-lang.github.io/mdBook)，阅读方法可参考 [这里](https://rust-lang.github.io/mdBook/guide/reading.html)。

> [!NOTE]
> 阅读前请确保至少掌握Rust基础语法，可通过[官方书籍](https://doc.rust-lang.org/reference/)来补充了解，部分语法知识会单独介绍，中文版也可参照[中文版官方书籍](https://www.rustwiki.org.cn/zh-CN/reference/)。

> [!NOTE]
> 通常来说，由于《算法导论》对于大部分语言的支持性，基础语法一般可以完成算法导论中的所有内容。但实际使用时我们更推荐用Rust的零成本抽象，所以会补充介绍，必要时还会额外用算法知识解释。

> [!NOTE]
> 另外一部分地方(如标准库函数介绍)也给出[中文版官方文档](https://www.rustwiki.org.cn/zh-CN/std/index.html)的链接。

内容从《算法导论》第一部分第2章(这是正式开始介绍算法的地方)开始提供。
## 其它

### 开源协议
本书以[**MIT**](https://choosealicense.com/licenses/mit/#)协议开源。

### 主题
采用**Catppuccin**提供的[主题](https://github.com/catppuccin/mdBook)，许可证同样是**MIT**。

### 插件
#### `mdbook-mermaid`
采用**Jan-Erik Rediger**提供的[**mdbook-mermaid**](https://github.com/badboy/mdbook-mermaid)插件，许可证是**MPL**。本书以**保留原 MPL 文件的版权声明和许可证文本**进行兼容。

#### `mdbook-katex`
采用**Lucas Zanini**提供的[**mdbook-katex**](https://github.com/lzanini/mdbook-katex)插件，许可证同样是**MIT**。

#### `mdbook-alerts`
采用**Λlisue**提供的[**mdbook-alerts**](https://github.com/lambdalisue/rs-mdbook-alerts)，许可证同样是**MIT**。

#### `mdbook-betterlink`
采用**TickPoints**提供的[**mdbook-betterlink**](https://github.com/TickPoints/mdbook-betterlink)，许可证同样是**MIT**。

### 字体
采用[**monaspace**](https://github.com/githubnext/monaspace)，许可证是[**SIL OFL**](https://github.com/githubnext/monaspace/blob/main/LICENSE)。本书以**保留原 OFL 文件的版权声明和许可证文本**进行兼容。

### 参考书
本书参考《算法导论》编写。

## 特殊
### 关于近期更改
由于**`mdbook`**升级到`v0.5`导致的破坏性更新，许多插件已不再可以正常工作。我们通过降级处理，等待它们再次恢复工作。所以现在请使用`0.4`版本的**MDBOOK**，并推荐使用[`setup.sh`](https://github.com/TickPoints/algorithm_learning/blob/main/setup.sh)给出的插件版本(部分兼容新版本的插件也无法与原`v0.4`版本一起工作)。更多相关内容可以查看[#2](https://github.com/TickPoints/algorithm_learning/issues/2)
