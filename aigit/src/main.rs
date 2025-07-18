use git2::{Repository, DiffOptions};

fn test_git2() -> Result<(), git2::Error> {
    // 打开当前目录下的 Git 仓库
    let repo = Repository::open("../")?;

    // 创建 DiffOptions（可选配置，如过滤文件路径）
    let mut diff_options = DiffOptions::new();
    // 示例：仅比较特定文件（如取消注释下一行）
    // diff_options.pathspec("src/main.rs");

    // 获取工作目录与暂存区的差异（即未提交的修改）
    let diff = repo.diff_index_to_workdir(None, Some(&mut diff_options))?;

    // 打印 diff 内容（包括文件路径和行级差异）
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
        if let Ok(text) = std::str::from_utf8(line.content()) {
            print!("{}", text);
        }
        true
    })?;

    Ok(())
}

fn main() {
    println!("获取未提交的 Git 差异：");
    if let Err(e) = test_git2() {
        eprintln!("错误: {}", e);
    }
}