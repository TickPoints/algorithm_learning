# 算法学习
![License](https://img.shields.io/github/license/TickPoints/algorithm_learning)
![Stars](https://img.shields.io/badge/dynamic/json?colorA=0D1117&colorB=1F6FEB&label=Stars&query=$.stargazers_count&url=https://api.github.com/repos/TickPoints/algorithm_learning)
![Deploy Status](https://img.shields.io/github/deployments/tickpoints/algorithm_learning/github-pages?label=Deploy%20Status&colorA=1e1e3f&colorB=a162e8)
![Last Commit](https://img.shields.io/github/last-commit/TickPoints/algorithm_learning/main?label=Last%20Commit&colorA=1e1e3f&colorB=20c997)

## 目的
用**Rust**实现[《算法导论》第3版](https://github.com/0voice/expert_readed_books/blob/master/%E8%AE%A1%E7%AE%97%E6%9C%BA%E7%A7%91%E5%AD%A6/%E7%AE%97%E6%B3%95%E5%AF%BC%E8%AE%BA_%E5%8E%9F%E4%B9%A6%E7%AC%AC3%E7%89%88_CHS.pdf)中的所有伪代码，同时对一些例题进行解答。本项目专为中文开发者设计，旨在辅助算法学习与实践。

## 阅读
从[这里](https://tickpoints.github.io/algorithm_learning)开始阅读本书。

## 本地构建
我们准备了一个`setup.sh`来帮助您快速构建:
```bash
curl -sSL https://raw.githubusercontent.com/TickPoints/algorithm_learning/main/setup.sh | bash
```
或者通过
```bash
# 国内加速镜像
curl -sSL https://ghproxy.com/https://raw.githubusercontent.com/TickPoints/algorithm_learning/main/setup.sh | bash
```

如果脚本无法正常运行，也可以按照下面的方式安装:
### 安装必要工具:
1. 安装 [mdBook](https://rust-lang.github.io/mdBook/):
```bash
# 默认方式
cargo install mdbook
# binstall
cargo binstall mdbook
# 安装托管的最新版本
cargo install --git https://github.com/rust-lang/mdBook.git mdbook
```
2. 安装 [mdbook-mermaid插件](https://github.com/badboy/mdbook-mermaid):
```bash
# 默认方式
cargo install mdbook-mermaid
# binstall
cargo binstall mdbook-mermaid
```
3. 安装 [mdbook-katex插件](https://github.com/lzanini/mdbook-katex):
```bash
cargo install mdbook-katex
```
4. 安装[mdbook-alerts插件](https://github.com/lambdalisue/rs-mdbook-alerts):
```bash
# 默认方式
cargo install mdbook-alerts
# binstall
cargo binstall mdbook-alerts
```
5. 安装[mdbook-betterlink插件](https://github.com/TickPoints/mdbook-betterlink):
```bash
# 默认方式
cargo install mdbook-betterlink
# binstall
cargo binstall mdbook-betterlink
```
### 克隆仓库
```bash
git clone https://github.com/TickPoints/algorithm_learning.git
cd algorithm_learning
```

### 构建或启用服务
```bash
mdbook build # serve 也会默认调用 build
mdbook serve # 在 localhost:3000 查看实时预览
```

## 贡献
**欢迎贡献代码、修复错误或优化文档！**

可参考 [PR贡献指南](src/pr_guide/README.md) 了解详情(可在托管的网站上查阅)。

## 许可证
本项目使用 [**MIT**](LICENSE.txt) 协议开源，其它依赖项目有关内容可从阅读页面了解。
