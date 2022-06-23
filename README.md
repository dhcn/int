# INT
> intelligent tradebot

## 架构设计
### 系统设计
- 由简单到复杂逐步发展，由少到多逐步增量，由具体到泛化按需抽象
- 设计思路:把握住业务数据流这个设计思想
  - 数据流本身的实现要根据具体交易所数据情况设计
  - 数据io交互的位置才要要有异步和channel等数据IO缓冲机制
- Kiss原则
  - 在数据结构及相关算法的选择上以最直观简单的同形映射为宜
  - 根据交易业务的需求逐步优化
  - 大道至简:本来面目
- 无规则无以至方圆
  - Rust的内存安全编码规则对编写高可靠性代码有好处。

### 多线程机制
- 不要轻易用Tokio的协程，因为Rust在运行线程是1:1core分配 
- 没有对event_loop异步调度机制做优化的计算拥塞型协程容易把核数占满 
- 所以默认优先使用线程，自己来调度
### 数据订阅机制
- Okex心跳必须通过时间周期来解决
- Binan的心跳Ping ws-rs会自动响应
- Huobi的需要用正常json msg响应。

## 部署说明
### on Ubuntu
- install rustup
```bash
# this is for rust install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- install third party lib
```bash
# this is for openssl-sys Compiling
apt install libssl-dev pkg-config
```
- install & config ossutils
> https://help.aliyun.com/document_detail/120075.html
```bash
# download ossutils
wget http://gosspublic.alicdn.com/ossutil/1.7.0/ossutil64  
chmod 755 ossutil64
./ossutil64 config

```
- 上传前压缩：
```bash
tar -cvzf OkexSubscriber2021-01-01.tar.gz OkexSubscriber2021-01-01
```
- cp file
```bash
./ossutil64 cp OkexSubscriber2021-01-02.tar.gz oss://int-data/Okex/
```


