# Embedded Unit Testing Template Project
这是一个用于嵌入式单元测试的模板项目.我在搭建这个项目的过程中需要的不少坑.主要是一下几个方面:

1. rustc的编译标志需要额外添加一些,虽然不是每个都是必须的,但是貌似必须含有一下两项:

```toml
rustflags = [
  "-C", "link-arg=-Tlink.x",
  "-C", "link-arg=-Tdefmt.x",
]
```

2. 文件结构需要如下所示:
   .─┬─►.cargo/.config.toml
   │
   ├─►src
   │ ├►lib.rs
   │ ├►main.rs
   │ └►A.rs(Specific implement)
   ├─►tests
   │ │
   │ └►tested_A.rs(针对A的测试平台)
   │
   └─►Cargo.toml

3. 在Cargo.toml中需要引入

```toml
[dependencies]
defmt-rtt = "0.4"
defmt = "1.0.1"

[dev-dependencies]
defmt-test = "0.4.0"

[lib]
harness = false #这里需要设置成false不然 lib.rs会一直提示找不到test

[[test]] #将需要测试的模块平台放入tests目录下,并且作如下引用
name = "macro_custom" # tests/macro_custom.rs
harness = false


```

将harness设置为false,即禁用rust标准库的测试框架.

4. 添加测试所需命令
   在运行测试命令前,需要用到probe-rs,因此需要在.cargo/config.toml中做如下设置:

```toml
[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip STM32G431RBTx"


```

5. lib.rs设置
如果被测模块中含有vec等需要分配堆空间的情况,需要想main.rs一样在lib.rs中提前分配好堆空间.
