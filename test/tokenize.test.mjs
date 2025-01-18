import { describe, it, expect } from "vitest";
import { addDefaultDict, hasWord, tokenize } from "../dist/nodejs/jieba";

if (!hasWord('我们')) addDefaultDict()

describe('tokenize', () => {
  it('should tag a sentence', () => {
    const result = tokenize('我喜欢吃苹果')
    console.log('🚀 ~ it ~ result:', result)
    expect(result).toMatchInlineSnapshot(`
      [
        {
          "end": 1,
          "start": 0,
          "word": "我",
        },
        {
          "end": 3,
          "start": 1,
          "word": "喜欢",
        },
        {
          "end": 4,
          "start": 3,
          "word": "吃",
        },
        {
          "end": 6,
          "start": 4,
          "word": "苹果",
        },
      ]
    `)
  })
});