# @isdk/jieba

> Use [jieba-rs](https://github.com/messense/jieba-rs)

Note: I only test it on nodejs.

## Usage

```js
import{
  split,
  tokenize,
  addWord,
  removeWord,
  hasWord,
  suggestFreq,
  loadDefaultDict,
  loadDict,
} from "@isdk/nlp-jieba";

loadDefaultDict();

// const dictContent = await readFile("./dict.txt", "utf8");
// loadDict(dictContent); //content can be an UInt8Array too.

split("中华人民共和国武汉市长江大桥");
// [ '中华人民共和国', '武汉市', '长江大桥' ]
split("中华人民共和国武汉市长江大桥", {mode: 'All', hmm: true});
/*
[
  '中',         '中华',
  '中华人民',   '中华人民共和国',
  '华',         '华人',
  '人',         '人民',
  '人民共和国', '民',
  '共',         '共和',
  '共和国',     '和',
  '国',         '武',
  '武汉',       '武汉市',
  '汉',         '市',
  '市长',       '长',
  '长江',       '长江大桥',
  '江',         '大',
  '大桥',       '桥'
]
*/
split("中华人民共和国武汉市长江大桥", {mode: 'Search', hmm: true});
/*
[
  '中华',     '华人',
  '人民',     '共和',
  '共和国',   '中华人民共和国',
  '武汉',     '武汉市',
  '长江',     '大桥',
  '长江大桥'
]
*/
tokenize("中华人民共和国武汉市长江大桥", {hmm: true});
/*
[
  { word: '中华人民共和国', start: 0, end: 7 },
  { word: '武汉市', start: 7, end: 10 },
  { word: '长江大桥', start: 10, end: 14 }
]
*/
tokenize("中华人民共和国武汉市长江大桥", {model: "Search", hmm: true});
/*
[
  { word: '中华', start: 0, end: 2 },
  { word: '华人', start: 1, end: 3 },
  { word: '人民', start: 2, end: 4 },
  { word: '共和', start: 4, end: 6 },
  { word: '共和国', start: 4, end: 7 },
  { word: '中华人民共和国', start: 0, end: 7 },
  { word: '武汉', start: 7, end: 9 },
  { word: '武汉市', start: 7, end: 10 },
  { word: '长江', start: 10, end: 12 },
  { word: '大桥', start: 12, end: 14 },
  { word: '长江大桥', start: 10, end: 14 }
]
*/

## Credit

https://github.com/messense/jieba-rs
