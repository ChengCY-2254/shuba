# 69shuba-download

从69书吧网下载书籍，使用[edge WebDriver](https://developer.microsoft.com/en-us/microsoft-edge/tools/webdriver/)进行内容抓取。

如需此项目进行抓取，需要启动[WebDriver](https://developer.microsoft.com/en-us/microsoft-edge/tools/webdriver/)并默认监听在9515端口

[id1]: 研究中，目前没什么办法，有办法也测试不了。
**[由于有些代理软件会妨碍对WebDriver的连接，一定不要放弃，请多尝试一下其它的代理软件，或者探索你所使用的代理软件功能，我本人没遇到过这个问题，也没有第二个设备来调试，多折腾折腾代理软件吧（开个全局代理之类的？）][id1]**


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

由于69shuba屏蔽了中国大陆的ip所以添加了代理检查，如果不想检查则可以添加`-c`参数以跳过检查。
```shell
./shuba -a http://10.0.0.1:9515 -l https://69shuba.cx/book/46869/ -b edge -c
```

## Future
- [ ] 浏览器代理（~~研究中，文档稀烂，有无作用都不知道~~）