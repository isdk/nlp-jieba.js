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
