use std::collections::HashMap;

#[tokio::test]
async fn test_stats_counts_extensions_in_workdir() {
    let temp_dir = tempfile::tempdir().unwrap();
    let path = temp_dir.path();

    std::fs::write(path.join("file1.txt"), "hello").unwrap();
    std::fs::write(path.join("file2.txt"), "world").unwrap();
    std::fs::write(path.join("main.rs"), "fn main() {}").unwrap();
    std::fs::write(path.join("no_extension_file"), "no ext").unwrap(); 
    
    std::fs::create_dir(path.join("target")).unwrap();
    std::fs::write(path.join("target/should_ignore.txt"), "ignore").unwrap();
    std::fs::create_dir(path.join(".libra")).unwrap();
    std::fs::write(path.join(".libra/config"), "config").unwrap();

    let orig_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(path).unwrap();

    let args = libra::cli::StatsArgs { json_format: false }; 
    let output_config = libra::utils::output::OutputConfig::default();
    
    let res = libra::command::stats::execute_safe(args, &output_config).await;
    
    std::env::set_current_dir(orig_dir).unwrap();

    assert!(res.is_ok(), "stats 命令应该在合法的目录中成功执行");
}
