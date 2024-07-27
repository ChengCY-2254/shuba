# 69shuba-download

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

选择浏览器进行抓取（需要安装并运行**WebDriver**）

```shell
./shuba -l https://69shuba.cx/book/46869/ -b safari
```

使用`-a`可链接远程WebDriver主机进行数据抓取<br/>

这里在ip为`10.0.0.1`的主机上使用edge进行数据抓取。
```shell
./shuba -a http://10.0.0.1:9515 -l https://69shuba.cx/book/46869/ -b edge
```