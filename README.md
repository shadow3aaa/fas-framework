*介绍
这是一个frame aware scheduling调度的框架
这类调度会感知帧变化来间接的，隐晦的反映性能需求

就好像一个传统的猜数字游戏，但是神秘数字是不断变化的,如果说这个数字就是性能需求，而当前性能是我们猜的数字,那么可以说，fas就是在告诉我们"比猜的小"或者"比猜的大"

*使用
下载最新release，用magisk安装

*其它
1. 你可以clone/fork这个仓库
git clone https://github.com/shadow3aaa/fas-framework
2. 然后在这个项目创建新的rust模块，你可以写一个新的类型并且为它实现WatcherNeed trait或者Controller trait
3. 然后，参考src/main.rs里面已经有的方式把它们按需要传给Watcher::new()，来添加新的监视器/控制器
