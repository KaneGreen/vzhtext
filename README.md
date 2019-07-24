# vzhtext: Vertical Zh-cn Text
# vzhtext：竖排中文文本
## Rearrange Chinese text to a vertical text read from top to bottom, right-to-left
## 将中文文本排版为从上往下、从右往左阅读的竖排文本
## Helping you avoid non-strict censorship and keyword search
## 帮助您规避轻度的审查和关键词检索

## Compile 编译
### Need the [Rust toolchain] 需要[Rust工具链]
### `$ cargo build --release`

## Usage 使用
```Bash
$ vzhtext 将中文文本排版为从上往下、从右往左阅读的竖排文本
竖左、从本将
排阅从上排中
文读右往版文
本的往下为文
$ vzhtext 3 仅支持纯中文文本和中文标点，数字、英文字母、英文标点等无法正常显示
常无标、文字点中文纯仅
显法点英字、，文本中支
示正等文母英数标和文持
$ vzhtext 指定每列字数的数字参数可前可后 4
前字字指
可参数定
后数的每
　可数列
$ vzhtext 两段中文将会被 5 拼在一起
 会两
　被段
　拼中
　在文
　一将
```
----
## Contributing
**Pull requests are always welcome.** Thank all of you.

[Rust toolchain]: https://www.rust-lang.org/
[Rust工具链]: https://www.rust-lang.org/