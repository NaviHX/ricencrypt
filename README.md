# RicEncrypt

基于Logistic映射与排序变换的图片混淆.

# 运行

运行示例见`sample`文件夹, 示例采用了行混淆.

```
USAGE:
    ricencrypt <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    dencrypt
    encrypt
    help        Print this message or the help of the given subcommand(s)
```

# 原理

将Logistic映射获得的数列进行排序, 通过排序变换混淆同一行 / 列的像素.

