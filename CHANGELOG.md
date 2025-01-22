# Changelog

All notable changes to this project will be documented in this file. See [commit-and-tag-version](https://github.com/absolute-version/commit-and-tag-version) for commit guidelines.

## [2.0.1](https://github.com/isdk/nlp-jieba.js/compare/v2.0.0...v2.0.1) (2025-01-22)


### Bug Fixes

* add_word should ignore empty word to avoid panic in jiebaa-rs ([11edae0](https://github.com/isdk/nlp-jieba.js/commit/11edae0a19ceb9b959b91836a2561201181a2aca))

## [2.0.0](https://github.com/isdk/nlp-jieba.js/compare/v1.1.0...v2.0.0) (2025-01-18)


### ⚠ BREAKING CHANGES

* rename loadDict, loadDefaultDict to addDict, addDefaultDict

* rename loadDict, loadDefaultDict to addDict, addDefaultDict ([6805035](https://github.com/isdk/nlp-jieba.js/commit/6805035233df1daf3b1e358825eb11ba6230aaaa))


### Features

* **jieba-rs:** add_word Reuse deleted word_id to optimize memory usage ([df21eff](https://github.com/isdk/nlp-jieba.js/commit/df21eff10ddba9f220ee1c093cd08644192f0ff6))

## 1.1.0 (2025-01-15)


### Features

* add loadDefaultDict, loadDict ([ad701e1](https://github.com/isdk/nlp-jieba.js/commit/ad701e1eaef51b304e4e079c60a2aec9481861db))
* add suggestFreq ([c527c3f](https://github.com/isdk/nlp-jieba.js/commit/c527c3f6cd8ebeb7246fd3295c1b71938305a449))


### Bug Fixes

* remove-word can not work ([9291973](https://github.com/isdk/nlp-jieba.js/commit/92919731e8f0933f595e958ca1a7f0cfeab0c728))
