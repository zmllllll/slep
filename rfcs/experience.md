# Experience



- 开始有意使用trait了，中间有出现这种情况，就是无脑用trait，结果出现几个trait的功能重复了。比如Generator用来生成Updater的，Filter用来过滤再生成Receiver的，显而易见这两玩意做的事情是一个意思，都是生成，只不过Generator不需要过滤这步，那我就干脆合并吧。合并很好，我只要加个泛型，告诉它要生成的目标类型是什么就可以了。唯一的问题就是参数不统一，那我就用关联类型自己指定。写完了我看了下其他代码，把意思差不多的也用这个trait。

- 后来又遇到新的需求，我希望self能被消耗掉，但是Generator用的是&self，那我加一个self的，emm我原来已经实现这个trait的现在报错了，它们本来就不需要self。好，那我给它写个空的默认实现 

  ```rust
  pub(crate) trait Generator<'a, T> {
      type Ext = i64;
      fn generate(&self, ext: Self::Ext) -> T;
      // 报错
      fn consume(self, ext: Self::Ext) ->T{
          T
      }
  }
  ```

  这是T，没法写默认实现。那只能接着抽象了，再写个Consumer trait，它只管消耗自己生成某某东西，很合理。

- 敢写生命周期了

- 我刚刚发现这个宏用了个寂寞，完全没有省下代码

  ```rust
  #[macro_export]
  macro_rules! gen_receiver {
      ($r: expr, $conn: expr, $uid: expr) => {
          match $r {
              Resources::User(cmd) => cmd.generate($conn),
              Resources::Message(cmd) => cmd.generate($conn),
              Resources::Group(cmd) => cmd.generate($conn),
              Resources::Member(cmd) => cmd.generate($conn),
              Resources::Reviewer(cmd) => cmd.generate($conn),
              Resources::StreamSettings(cmd) => cmd.generate($conn),
              Resources::TopicSettings(cmd) => cmd.generate(($conn, $uid)),
              Resources::Task(cmd) => cmd.generate($conn),
              Resources::TaskId(cmd) => cmd.generate(($conn, $uid)),
              Resources::TaskReceipt(cmd) => cmd.generate($conn),
          }
      };
  }
  ```

- 我知道枚举为什么不好了，和switch case一个样，也不是丑，而是看的时候恶心，没人喜欢一直划鼠标滚轮看代码，还是ctrl+左键跳转方便