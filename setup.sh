#!/bin/bash

# 安装必要工具
install_tools() {
    # 安装 cargo-binstall(如果未安装)
    if ! command -v cargo-binstall &>/dev/null; then
        echo "安装 cargo-binstall..."
        cargo install cargo-binstall
    fi

    # 安装 mdBook(优先使用binstall，失败则用git方式)
    echo "安装 mdBook..."
    cargo binstall mdbook || cargo install --git https://github.com/rust-lang/mdBook.git mdbook

    # 安装 mdbook-mermaid
    echo "安装 mdbook-mermaid..."
    cargo binstall mdbook-mermaid || cargo install mdbook-mermaid

    # 安装 mdbook-katex
    echo "安装 mdbook-katex..."
    cargo install mdbook-katex

    # 安装 mdbook-alerts
    echo "安装 mdbook-alerts..."
    cargo binstall mdbook-alerts || cargo install mdbook-alerts

    # 安装 mdbook-alerts
    echo "安装 mdbook-betterlink..."
    cargo binstall mdbook-betterlink || cargo install mdbook-betterlink
}

# 克隆或更新仓库
setup_repo() {
    local repo_dir="algorithm_learning"
    local repo_url="https://github.com/TickPoints/algorithm_learning.git"

    if [ -d "$repo_dir" ]; then
        echo "更新仓库..."
        cd "$repo_dir" && git pull && cd ..
    else
        echo "克隆仓库..."
        git clone "$repo_url"
    fi
}

main() {
    set -e  # 遇到错误立即退出

    install_tools
    setup_repo

    cd algorithm_learning
    echo "启动文档服务，请访问地址：http://localhost:3000"
    mdbook serve
}

main
