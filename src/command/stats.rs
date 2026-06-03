use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::cli::StatsArgs;
use crate::utils::error::CliResult;
use crate::utils::output::OutputConfig;

pub async fn execute_safe(args: StatsArgs, _output: &OutputConfig) -> CliResult<()> {
    let mut stats = HashMap::new();
    
    // 使用 map_err 将标准 io 错误包装为 CliError::fatal
    let current_dir = std::env::current_dir().map_err(|e| {
        crate::utils::error::CliError::fatal(format!("无法获取当前目录: {}", e))
    })?;

    // 调用递归遍历函数（通过 map_err 转换错误）
    visit_dirs(&current_dir, &mut stats).map_err(|e| {
        crate::utils::error::CliError::fatal(format!("遍历目录失败: {}", e))
    })?;


    if args.json_format {
        // 输出 JSON 格式
        let json_str = serde_json::to_string_pretty(&stats).map_err(|e| {
            crate::utils::error::CliError::fatal(format!("JSON 序列化失败: {}", e))
        })?;
        println!("{}", json_str);
    } else {
        // 默认输出文本结果
        for (ext, count) in &stats {
            println!("{}: {}", ext, count);
        }
    }

    Ok(())
}

// 递归遍历工作区
fn visit_dirs(dir: &Path, stats: &mut HashMap<String, usize>) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            if file_name_str == ".libra" || file_name_str == "target" || file_name_str == ".git" {
                continue;
            }

            if path.is_dir() {
                visit_dirs(&path, stats)?;
            } else if path.is_file() {
                // 获取文件后缀名
                let ext = path.extension()
                    .map(|s| s.to_string_lossy().to_string())
                    // 没有后缀名的归类为 no_extension
                    .unwrap_or_else(|| "no_extension".to_string());

                *stats.entry(ext).or_insert(0) += 1;
            }
        }
    }
    Ok(())
}
