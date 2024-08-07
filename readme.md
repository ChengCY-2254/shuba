# 69shuba-download
[![Rust](https://github.com/ChengCY-2254/shuba/actions/workflows/rust.yml/badge.svg)](https://github.com/ChengCY-2254/shuba/actions/workflows/rust.yml)

注意，中国大陆无法连接到69shuba请自行使用魔法。

从69书吧网下载书籍，使用[edge WebDriver](https://developer.microsoft.com/en-us/microsoft-edge/tools/webdriver/)进行内容抓取。

如需此项目进行抓取，需要启动[WebDriver](https://developer.microsoft.com/en-us/microsoft-edge/tools/webdriver/)并默认监听在9515端口

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


## Future
- [x] 支持浏览器代理
- [x] 下载章节
- [x] 下载全本
- [x] 远程连接WebDriver进行抓取
- [x] 将数据模型和trait分离使其可以支持其它网站
- [ ] 登录数据保存
- [ ] 添加epub格式

## 支持的网站
- [x] [69书吧](https://69shuba.cx/)
- [ ] [第二书包网](https://www.keryo.net/)