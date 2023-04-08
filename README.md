*介绍
这是一个frame aware scheduling调度的框架

*使用
下载最新release，用magisk安装

*其它
1. 你可以clone/fork这个仓库
```
git clone https://github.com/shadow3aaa/fas-framework
```
2. 然后在这个项目创建新的rust模块，你可以写一个新的类型并且为它实现WatcherNeed trait或者Controller trait
3. 然后，参考src/main.rs里面已经有的方式把它们按需要传给Watcher::new()，来添加新的监视器/控制器
