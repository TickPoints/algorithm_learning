#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! walkdir = "2"
//! ```

use std::fs;
use std::io;
use std::path::Path;
use std::process;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("用法: {} <目录路径>", args[0]);
        process::exit(1);
    }

    let dir = &args[1];
    let mut total_files = 0;
    let mut processed_files = 0;
    let mut total_replacements = 0;

    for entry in WalkDir::new(dir).into_iter() {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                eprintln!("✗ 遍历错误: {}", err);
                continue;
            }
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let ext = match entry.path().extension() {
            Some(e) => e,
            None => continue,
        };

        if ext != "md" && ext != "txt" {
            continue;
        }

        total_files += 1;
        match process_file(entry.path()) {
            Ok(count) if count > 0 => {
                processed_files += 1;
                total_replacements += count;
                println!("✓ 已处理: {:?} ({} 处替换)", entry.path(), count);
            }
            Ok(_) => {} // 无替换，无需输出
            Err(e) => eprintln!("✗ 处理失败 {:?}: {}", entry.path(), e),
        }
    }

    println!("\n========== 统计信息 ==========");
    println!("扫描文件总数: {}", total_files);
    println!("修改文件数量: {}", processed_files);
    println!("替换标点总数: {}", total_replacements);
    println!("=============================");
}

fn process_file(path: &Path) -> Result<usize, io::Error> {
    let content = fs::read_to_string(path)?;
    let (new_content, count) = convert_punctuation(&content);

    if count > 0 {
        fs::write(path, &new_content)?;
    }

    Ok(count)
}

fn convert_punctuation(text: &str) -> (String, usize) {
    let mut result = String::with_capacity(text.len());
    let mut count = 0;
    let mut in_code_block = false;

    let mut lines = text.lines().peekable();

    while let Some(line) = lines.next() {
        // 检测代码块边界
        if line.trim_start().starts_with("```") {
            in_code_block = !in_code_block;
            result.push_str(line);
            if lines.peek().is_some() {
                result.push('\n');
            }
            continue;
        }

        // 跳过代码块内的内容
        if in_code_block {
            result.push_str(line);
            if lines.peek().is_some() {
                result.push('\n');
            }
            continue;
        }

        // 处理普通行
        let (processed, line_count) = process_line(line);
        count += line_count;
        result.push_str(&processed);

        if lines.peek().is_some() {
            result.push('\n');
        }
    }

    // 保留原文件末尾的换行符
    if text.ends_with('\n') && !result.ends_with('\n') {
        result.push('\n');
    }

    (result, count)
}

#[derive(Debug, Clone, Copy)]
struct Segment {
    start: usize,
    end: usize,
}

fn process_line(line: &str) -> (String, usize) {
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();

    if chars.is_empty() {
        return (String::new(), 0);
    }

    // 找出所有需要保护的区域
    let protected = find_protected_segments(&chars);

    let mut result = String::with_capacity(len + len / 5); // 预分配，估计 20% 增长空间
    let mut count = 0;

    // 追踪当前所在保护段，避免每次遍历全部段
    let mut prot_idx = 0;

    for i in 0..len {
        while prot_idx < protected.len() && protected[prot_idx].end < i {
            prot_idx += 1;
        }

        let is_protected =
            prot_idx < protected.len() && i >= protected[prot_idx].start && i <= protected[prot_idx].end;

        if is_protected {
            result.push(chars[i]);
            continue;
        }

        let ch = chars[i];
        match ch {
            '，' => {
                result.push(',');
                count += 1;
                if i + 1 < len && chars[i + 1] != ' ' {
                    result.push(' ');
                }
            }
            '。' => {
                result.push('.');
                count += 1;
                if i + 1 < len && chars[i + 1] != ' ' {
                    result.push(' ');
                }
            }
            '：' => {
                result.push(':');
                count += 1;
                if i + 1 < len && chars[i + 1] != ' ' {
                    result.push(' ');
                }
            }
            '！' => {
                result.push('!');
                count += 1;
                if i + 1 < len && chars[i + 1] != ' ' {
                    result.push(' ');
                }
            }
            '？' => {
                result.push('?');
                count += 1;
                if i + 1 < len && chars[i + 1] != ' ' {
                    result.push(' ');
                }
            }
            '（' => {
                result.push('(');
                count += 1;
            }
            '）' => {
                result.push(')');
                count += 1;
            }
            _ => result.push(ch),
        }
    }

    (result, count)
}

fn find_protected_segments(chars: &[char]) -> Vec<Segment> {
    let mut segments = Vec::new();
    let len = chars.len();
    let mut i = 0;

    // 合并扫描：同时检测反引号代码段和链接 URL
    while i < len {
        match chars[i] {
            '`' => {
                let start = i;
                let mut backtick_count = 0;
                while i < len && chars[i] == '`' {
                    backtick_count += 1;
                    i += 1;
                }

                let mut found_end = false;
                while i < len {
                    if chars[i] == '`' {
                        let mut closing_count = 0;
                        while i < len && chars[i] == '`' {
                            closing_count += 1;
                            i += 1;
                        }
                        if closing_count == backtick_count {
                            segments.push(Segment { start, end: i - 1 });
                            found_end = true;
                            break;
                        }
                    } else {
                        i += 1;
                    }
                }

                if !found_end {
                    i = start + 1; // 回退
                }
            }
            '[' => {
                let mut j = i + 1;
                let mut bracket_depth = 1;
                while j < len && bracket_depth > 0 {
                    match chars[j] {
                        '[' => bracket_depth += 1,
                        ']' => bracket_depth -= 1,
                        _ => {}
                    }
                    j += 1;
                }

                if j < len && chars[j] == '(' {
                    let url_start = j;
                    let mut k = j + 1;
                    let mut paren_depth = 1;
                    while k < len && paren_depth > 0 {
                        match chars[k] {
                            '(' => paren_depth += 1,
                            ')' => paren_depth -= 1,
                            _ => {}
                        }
                        k += 1;
                    }
                    if paren_depth == 0 {
                        segments.push(Segment { start: url_start, end: k - 1 });
                        i = k;
                        continue;
                    }
                }
                i = j;
            }
            _ => i += 1,
        }
    }

    segments
}
