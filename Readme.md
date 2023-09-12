# Rust HW5：mini redis



3210104927 刘子鸣



## 运行方法



本次作业实现了`redis`中的`get, set, del, ping, exit`五个命令，以及能够过滤含有非法字符串（暂设为`illegal`）的指令。

运行方式为：

```shell
# Terminal 1
cargo run --bin server

# Terminal 2
cargo build --bin client
./taeget/debug/client
<命令(小写输入)> <参数1(可选)> <参数2(可选)>
```



## 运行结果



分别对五个指令以及`filter`过滤器进行测试：

![image-20230912162523729](/images/image-20230912162523729.png)