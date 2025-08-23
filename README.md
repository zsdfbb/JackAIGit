# AIgit

Reimagining Git Usage with AI

```text
Usage: aigit [OPTIONS] [COMMAND]

Commands:
  diff    Show the diff between the working tree and the index
  commit  Commit the current changes
  list    List all commits
  show    Show commit details
  help    Print this message or the help of the given subcommand(s)

Options:
  -p, --platforms  Show supported platforms
  -h, --help       Print help
  -V, --version    Print version
```

## Install

please install rust and git first. And run:

./install.sh

## Test

please install aigit first. And run:

./test.sh

## Demo

![aigit_commit_usage](images/aigit_commit_usage.gif)

## Question and Answer

### 如何支持局域网中的其他通过IP访问其接口?

在 Linux 环境中，需要修改 ollama 的 service 文件。

1、首先停止ollama服务：systemctl stop ollama 

2、修改ollama的service文件：/etc/systemd/system/ollama.service 在[Service]下边增加两行：

Environment="OLLAMA_HOST=0.0.0.0"
Environment="OLLAMA_ORIGINS=*"

3、重载daemon文件 sudo systemctl daemon-reload
4、启动ollama服务 sudo systemctl restart ollama