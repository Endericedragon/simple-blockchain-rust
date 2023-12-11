# simple-blockchain-rust

[本系列](https://github.com/Endericedragon/simple-blockchain-rust)是用Rust实现简单的区块链，包括区块和区块链，工作量证明，交易和UTXO集合，持久化，钱包及用rust-libp2p实现的点对点分布式网络。本仓库为fork获得，原仓库、代码详解等**请关注原作者微信公众号：coding到灯火阑珊**。

该系列教程为渐进式的6个部分：

- [用Rust实现区块链 - 1 区块和区块链](https://mp.weixin.qq.com/s?__biz=Mzg5MjA1ODYzNg==&mid=2247484460&idx=1&sn=b79b1051f40db383a2d2feb568cb3fe8&chksm=cfc2a94ff8b52059b2402785330133ce6a6734a3abcd3343c08154716acca5eb8369a4f4cd12&token=1912223334&lang=zh_CN#rd)

- [用Rust实现区块链 - 2 工作量证明(PoW)](https://mp.weixin.qq.com/s?__biz=Mzg5MjA1ODYzNg==&mid=2247484469&idx=1&sn=c722722580f9838b9136cf3ac6611c8e&chksm=cfc2a956f8b520405e0aa11fc1484d3b676f6f9b19cb536165e7fb0602d4db03f63167dcf59b&token=1151139300&lang=zh_CN#rd)

- [用Rust实现区块链 - 3 持久化](https://mp.weixin.qq.com/s?__biz=Mzg5MjA1ODYzNg==&mid=2247484477&idx=1&sn=cf1789dcbc1a7ca9381539e36314a2e9&chksm=cfc2a95ef8b52048027a6466c097f5954815a50e29ba0a22687fc6f218a552378e963ff6a9a2&token=1609755589&lang=zh_CN#rd)

- [用Rust实现区块链 - 4 交易与UTXO集合](https://mp.weixin.qq.com/s?__biz=Mzg5MjA1ODYzNg==&mid=2247484487&idx=1&sn=01802f8dc60ac7dd1924888937b65d50&chksm=cfc2a924f8b520326a25718a2b97e24aac25355578ab727ae5fd6e907030ec39cc434cb90752&token=1933715555&lang=zh_CN#rd)

- [用Rust实现区块链 - 5 钱包](https://mp.weixin.qq.com/s?__biz=Mzg5MjA1ODYzNg==&mid=2247484495&idx=1&sn=4eea98f046a92bb9e87163bab44aff68&chksm=cfc2a92cf8b5203af27a527d01f1cf699d77089a3ad2e7082fc7e707ae2885e96a611a8069f8&token=391513474&lang=zh_CN#rd)

- [用Rust实现区块链 - 6 点对点网络(P2P)](https://mp.weixin.qq.com/s?__biz=Mzg5MjA1ODYzNg==&mid=2247484503&idx=1&sn=82427d27153bce04488b95878e7584f0&chksm=cfc2a934f8b5202274556e6ea3294b48dc8ee5075f559fce65f7ce91d9a2619e70a915252d20&token=2032429111&lang=zh_CN#rd)

## 功能测试

对阶段6（即最终完全体阶段）进行测试。

### 单节点测试

运行一个节点：
```bash
RUST_LOG=info cargo run --quiet server data1
```

其中的`data1`代表将该节点的持久化存储放在`data1`目录下。节点开始工作并等待指令输入。可输入的指令均为单行的JSON格式，详见如下示例：

- `{"Blocks":""}`：查看当前区块链的所有区块信息。
- `{"CreateWallet":"<<username>>"}`：为用户\<\<username\>\>创建一个钱包。执行完成后，该用户的钱包中将有10单位代币的余额，终端将输出钱包地址。
- `{"Genesis":"<<wallet_address>>"}`：基于创建的钱包地址，构造一个区块链的创世区块。代入上一步的钱包地址即可。此时若再执行第一条指令，就能看见区块链中出现了第一个区块。
- `{"Sync":""}`：同步指令。连接到其他节点时有效，可以同步区块信息。
- `{"Trans": {"from":"<<source_wallet_address>>","to":"<<dest_wallet_address>>","amount":"4"}}`

本例中年使用的指令如下：
```json
{"Blocks":""}
{"CreateWallet":"Endericedragon"}
{"Genesis":"19AoeaeJpYqZteyfwwA9CNozo5bZBWFPJy"}
```

### 双节点测试

再运行一个节点：
```bash
RUST_LOG=info cargo run --quiet server data2
```

将该节点的持久化存储放在`data2`目录下。这个节点启动之后将会自动连接到上一个节点。在新的节点上查询区块信息，会发现啥也没有。因此，需要进行同步。在新节点中执行同步指令，然后查看区块信息，会发现信息已经同步，旧节点中的区块出现在了新节点的区块列表中。

接下来进行交易测试。在新节点中为用户Hyakuri2906创建钱包，然后在旧节点以Endericedragon的身份向Hyakuri2906转账4单位代币。操作结束后查看两个节点的区块信息，会发现多了一个区块，其中记载了Endericedragon所有的10单位代币的去向：共有两个去向，4单位代币的和6单位代币的。前者是Endericedragon向Hyakuri2906的打款，后者是Endericedragon自己给自己的打款。这是因为UTXO要求所有代币必须一次性使用掉，即使一次转账并未用光全部代币。

以上过程在新节点的终端中用代码如下描述：
```json
{"Sync":""}
{"Blocks":""}
{"CreateWallet":"Hyakuri2906"}
{"Trans": {"from":"19AoeaeJpYqZteyfwwA9CNozo5bZBWFPJy","to":"1LhQm7zb3TtXYR1q9u9ypMGNfoxBvHupkh","amount":"4"}}
{"Blocks":""}
```

至此，我们已经完成了区块链的基本功能测试，并确定这个简单的示例区块链拥有如下功能：
- 为给定用户名创建钱包的能力
- 自动发现（至少是局域网内）其他peers的能力
- 多节点之间的信息同步能力（新加入网络的节点需要手动同步一次）
- 在不同用户之间进行代币转账的能力

同时根据原作者公众号上的描述，我们还能知道这个区块链具有一些特性：
- 使用SHA3-256作为哈希算法
- 支持基于工作量的证明（PoW）
- 使用UTXO模型进行交易
- 使用SledDB进行持久化存储

## 笔者收集到的有用信息

[crates.io](https://crates.io)上有个tag叫[no_std](https://crates.io/keywords/no_std)，带上这个tag的`crate`可以在`no_std`环境下直接运行。寻找替代品的时候应该有用。


