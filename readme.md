# shuba-download
[![Rust](https://github.com/ChengCY-2254/shuba/actions/workflows/rust.yml/badge.svg)](https://github.com/ChengCY-2254/shuba/actions/workflows/rust.yml)

注意，中国大陆无法连接到69shuba请自行使用魔法。

从69书吧网下载书籍，使用[edge WebDriver](https://developer.microsoft.com/en-us/microsoft-edge/tools/webdriver/)进行内容抓取。

如需此项目进行抓取，需要启动[WebDriver](https://developer.microsoft.com/en-us/microsoft-edge/tools/webdriver/)并默认监听在9515端口

## 使用指南

从目录页下载
```shell
./shuba -l https://69shuba.cx/book/46869/
```

下载单章
```shell
./shuba -l https://69shuba.cx/txt/46869/31308058
```

使用`-a`可链接远程WebDriver主机进行数据抓取<br/>

这里在ip为`10.0.0.1`的主机上使用edge进行数据抓取。
```shell
./shuba -a http://10.0.0.1:9515 -l https://69shuba.cx/book/46869/
```

添加代理选项，参数为`--proxy`仅支持socks5代理。<br/>
在一个远程主机上通过代理的方式抓取内容
```shell
./shuba -a http://10.0.0.1:9515 -l https://69shuba.cx/book/46869/ -proxy socks5://10.0.0.251:1082
```

`-p`参数用于下载到指定目录，默认下载到当前目录下的`downloads`文件夹中

`--speed`参数用于每章节下载间隔，默认不限制，单位是秒。
```shell
./shuba -l 'https://69shuba.cx/book/43314.htm' --speed 1
```

使用 `--support` 参数可查看当前版本可以对哪些网站进行抓取操作。
例如：
```shell
./shuba --support
# Shuba
# Keryo
```

## Future
- [x] 支持浏览器代理
- [x] 下载章节
- [x] 下载全本
- [x] 远程连接WebDriver进行抓取
- [ ] 登录数据保存

## 支持的网站
- [x] [69书吧](https://69shuba.cx/)
- [x] [第二书包网](https://www.keryo.net/)
- [x] [顶点小说网](https://www.ddxs.com)

## 条件编译
如果只需要该下载器的某一功能，那么在编译的过程中指定你所需要的功能就好。
有以下features key
- shuba
- keryo
- ddxs
- full
- debug

### 详细说明

目前有以下版本可编译：

- shuba 仅可抓取[69书吧](https://69shuba.cx/)的内容。
- keryo 仅可抓取[第二书包网](https://www.keryo.net/)的内容。
- ddxs 仅可抓取[顶点小说网](https://www.ddxs.com)的内容

预定义版：

- full 全网站支持版（包含实验特性）。
- debug 添加了日志full版本，用于调试使用。
- release 正式特性。

### 编译示例

条件编译的示例代码如下：

注意：若要控制生成包大小，请务必加入`--release`标签以在生产环境中达到最佳效果。

编译仅支持shuba版本
```shell
cargo b --features shuba
```

编译同时支持shuba和keryo的版本
```shell
cargo b --features keryo
```
