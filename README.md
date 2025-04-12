# CPPGO

一个类似于Cargo的轻量级C++包管理工具

基于Rust编写

核心特点：
1. 不需要额外的依赖库，开箱即用（有可以直接调用的可执行文件）
2. 支持配置文件设置依赖库和版本
3. 支持类似cargo的命令
```shell
cppgo new <name>         # 初始化项目
cppgo add <package>      # 添加依赖（写入 cppgo.toml）
cppgo install            # 下载/构建依赖
cppgo build              # 编译主程序
cppgo run                # 编译 + 运行 main.cpp
cppgo clean              # 清理依赖和编译内容 
cppgo search <package>   # 搜索包
cppgo remove <package>   # 移除依赖
cppgo 
```

## TODO
cppgo search \<package\> 暂时只能搜本地的