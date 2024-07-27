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