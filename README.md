# @isdk/nlp-jieba

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![NPM Version](https://img.shields.io/npm/v/@isdk/nlp-jieba)](https://www.npmjs.com/package/@isdk/nlp-jieba)

## Introduction

`@isdk/nlp-jieba` uses the Chinese segmentation tool `jieba_rs` in a WebAssembly (WASM) environment. It provides seamless integration with JavaScript, allowing you to perform Chinese text segmentation, part-of-speech tagging, and other NLP tasks efficiently in both Node.js and browser environments.

## Features

- **Segmentation Modes**: Supports default (precise), full, and search modes.
- **Dictionary Management**: Can load default or custom dictionaries and supports dynamic addition and removal of words.
- **Frequency Adjustment**: Provides suggestions for word frequency and allows manual setting of word frequencies.
- **Part-of-Speech Tagging**: Supports part-of-speech tagging based on the Hidden Markov Model (HMM).

## Installation

### Using npm

```bash
npm install @isdk/nlp-jieba
```

## Examples

### Basic Usage in the Browser

```js
import init, {addDict, split} from '@isdk/nlp-jieba';

async function main() {
  await init();  // Initialize the WASM module, only required in the browser

  / Load custom dictionary (optional)
  const dictContent = `word1 100\nword2 200`;
  await addDict(dictContent);

  // Segmentation example
  const result = split("我爱北京天安门");
  console.log(result); // Output segmentation result
}

main();
```

### Basic Usage in Node.js

```js
import {addDict, split} from '@isdk/nlp-jieba';


// Load custom dictionary (optional)
const dictContent = `word1 100\nword2 200`;
await addDict(dictContent);

// Segmentation example
const result = split("我爱北京天安门");
console.log(result); // Output segmentation result
```

## Advanced Usage

### Custom Segmentation Options

```js
const options = {
    mode: "Search", // Options: "Default", "All", "Search"
    hmm: true       // Enable HMM model, default is false
};

const words = split("我喜欢编程", options);
console.log(words);
```

### Adding New Words

```js
addWord("新词汇", 100, "n"); // Add a noun
console.log(hasWord("新词汇")); // Check if the word exists
```

### Part-of-Speech Tagging

```js
const tags = tag("我喜欢吃苹果", true); // true: enable HMM
tags.forEach(tag => {
    console.log(`${tag.word}: ${tag.tag}`);
});
```

### Tokenize

The `tokenize` function returns segmentation results along with position information.

```js
const options = {
    mode: "Search", // Options: "Default", "Search"
    hmm: true       // Enable HMM model, default is false
};

const tokens = tokenize("我喜欢吃苹果", options);
tokens.forEach(token => {
    console.log(`Word: ${token.word}, Start: ${token.start}, End: ${token.end}`);
});
```

## API Documentation

* `split(text: string, options?: JiebaSplitOptions)`: Segment the text.
* `tokenize(text: string, options?: JiebaSplitOptions)`: Segment the text and return position information.
* `addDict(dict_content: string | Uint8Array, clear?: boolean)`: Load a custom dictionary. The optional parameter `clear` indicates whether to clear the existing dictionary.
* `addDefaultDict(clear?: boolean)`: Load the default dictionary. The optional parameter `clear` indicates whether to clear the existing dictionary.
* `clear()`: Clear all loaded words.
* `suggestFreq(segment: string)`: Get suggested word frequency.
* `addWord(word: string, freq?: number, tag?: string)`: Add a new word.
* `removeWord(word: string)`: Remove a word.
* `hasWord(word: string)`: Check if a word exists.
* `tag(sentence: string, hmm?: boolean)`: Perform part-of-speech tagging on a sentence.

## Contribution

Welcome contributions in any form, including bug reports, code improvements, and documentation enhancements.

## License

`@isdk/nlp-jieba` is licensed under the MIT License.

## Credit

* https://github.com/messense/jieba-rs
* https://github.com/fengkx/jieba-wasm
