# @isdk/nlp-jieba

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![NPM Version](https://img.shields.io/npm/v/@isdk/nlp-jieba)](https://www.npmjs.com/package/@isdk/nlp-jieba)

## 简介

`@isdk/nlp-jieba` 是一个用于在WebAssembly (WASM) 环境中使用中文分词工具 `jieba_rs` 的Rust库。它提供了与JavaScript的无缝集成，允许你在nodejs, 浏览器端高效地进行中文分词、词性标注等操作。

## 特性

- **分词模式**：支持默认(精确)、全模式和搜索模式。
- **词典管理**：可以加载默认词典或自定义词典，并支持动态添加和删除词汇。
- **词频调整**：提供建议频率和手动设置词汇频率的功能。
- **词性标注**：支持基于HMM的词性标注功能。

## 安装

### 使用npm安装

```bash
npm install @isdk/nlp-jieba
```

## 使用示例

### 浏览器环境下的基本用法

```js
import init, {addDict, split} from '@isdk/nlp-jieba';

async function main() {
  await init(); // 初始化WASM模块，仅在浏览器环境下存在

  // 加载自定义词典（可选）
  const dictContent = `word1 100\nword2 200`;
  await addDict(dictContent);

  // 分词示例
  const result = split("我爱北京天安门");
  console.log(result); // 输出分词结果
}

main();
```

### Node.js 环境下的基本用法

```js
const { split, addDict } = require('@isdk/nlp-jieba');


// 加载自定义词典（可选）
const dictContent = `word1 100\nword2 200`;
await addDict(dictContent);

// 分词示例
const result = split("我爱北京天安门");
console.log(result); // 输出分词结果
```

## 高级用法

### 自定义分词选项

```js
const options = {
    mode: "Search", // 可选值："Default", "All", "Search"
    hmm: true       // 是否启用HMM模型，默认为false
};

const words = split("我喜欢编程", options);
console.log(words);
```

### 添加新词

```js
addWord("新词汇", 100, "n"); // 添加一个名词
console.log(hasWord("新词汇")); // 检查词汇是否存在
```

### 词性标注

```js
const tags = tag("我喜欢吃苹果", true); // true: enable HMM
tags.forEach(tag => {
    console.log(`${tag.word}: ${tag.tag}`);
});
```

结巴词性对照表

- a 形容词
  - ad 副形词，直接作状语的形容词。
  - ag 形容词性语素
  - an 名形词， 具有名词功能的形容词。
- b 区别词，b取自汉字“别”的声母
- c 连词， c取自英语连词 conjunction 的第1个字母。
- d 副词, 取 adverb 的第2个字母，因其第1个字母已用于形容词。
  - df
  - dg 副语素
- e 叹词
- f 方位词, 取汉字“方”的声母
- g 语素， 绝大多数语素都能作为合成词的“词根”，取汉字“根”的声母。
- h 前接成分，取英语 head的第1个字母。
- i 成语，取英语成语 idiom的第1个字母。
- j 简称略称，取汉字“简”的声母。
- k 后接成分
- l 习用语，习用语尚未成为成语，有点“临时性”，取“临”的声母。
- m 数词，取英语 numeral的第3个字母，n，u已有他用。
  - mg
  - mq 数量词
- n 名词
  - ng 名词性语素
  - nr 人名
    - nr1 汉语姓氏
    - nr2 汉语名字
    - nrj 日语人名
    - nrt 音译人名
    - nrfg 名人人名
  - ns 地名
    - nst 音译地名
  - nt 机构团体名
    - ntg 机构团体名语素
  - nz 其他专名，“专”的声母的第 1个字母为z，名词代码n和z并在一起。
    - nzo 职业名， o是“occupation”的第一个字母。
  - nl 名词性惯用语
  - ng 名词性语素
- o 拟声词，取英语拟声词 onomatopoeia的第1个字母。
- p 介词，取英语介词 prepositional的第1个字母。
- q 量词，取英语 quantity的第1个字母。
- r 代词，取英语代词 pronoun的第2个字母,因p已用于介词。
  - rg 代词性语素
  - rr 人称代词
  - rz 指示代词
- s 处所词，取英语 space的第1个字母。
- t 时间词，取英语 time的第1个字母。
  - tg 时语素
- u 助词，取英语助词 auxiliary
  - ud 结构助词 得
  - ug 时态助词
  - uj 结构助词 的
  - ul 时态助词 了
  - uv 结构助词 地
  - uz 时态助词 着
- v 动词，取英语动词 verb的第一个字母。
  - vd 副动词，直接作状语的动词。动词和副词的代码并在一起。
  - vg 动词性语素
  - vi 不及物动词
  - vn 名动词
  - vf 趋向动词
- x 非语素词（包含标点符号）
- y 语气词，取汉字“语”的声母。
- z 状态词，取汉字“状”的声母的第一个字母。

### Tokenize

`tokenize` 函数不仅会进行分词，还会返回每个词的起始和结束位置。

```js
const options = {
    mode: "Search", // 可选值："Default", "Search"
    hmm: true       // 是否启用HMM模型，默认为false
};

const tokens = tokenize("我喜欢吃苹果", options);
tokens.forEach(token => {
    console.log(`词: ${token.word}, 起始位置: ${token.start}, 结束位置: ${token.end}`);
});
```

## API文档

* `split(text: string, options?: JiebaSplitOptions)`：对文本进行分词。
* `tokenize(text: string, options?: JiebaSplitOptions)`：对文本进行分词并返回带有位置信息的结果。
* `addDict(dict_content: string | Uint8Array, clear?: boolean)`：加载自定义词典，可选参数 clear 表示是否清除现有词典。
* `addDefaultDict(clear?: boolean)`: 加载默认词典，可选参数 clear 表示是否清除现有词典。
* `clear()`：清除所有已加载的词汇。
* `suggestFreq(segment: string)`：获取建议词频。
* `addWord(word: string, freq?: number, tag?: string)`：添加新词。
* `removeWord(word: string)`：移除词汇。
* `hasWord(word: string)`：检查词汇是否存在。
* `tag(sentence: string, hmm?: boolean)`：对句子进行词性标注。

## 贡献

欢迎任何形式的贡献，包括但不限于报告问题、改进代码、改进文档等。

## 许可证

`@isdk/nlp-jieba` 使用 MIT 许可证。

## Credit

* https://github.com/messense/jieba-rs
* https://github.com/fengkx/jieba-wasm
