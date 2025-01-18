import { describe, it, expect } from "vitest";
import { addDefaultDict, tag, hasWord } from "..";

if (!hasWord('我们')) addDefaultDict()

describe('tag', () => {
  it('should tag a sentence', () => {
    const result = tag('这是一个伸手不见五指的黑夜。我叫孙悟空，我爱北京，我爱Python和C++了。')
    expect(result).toMatchInlineSnapshot(`
      [
        {
          "tag": "x",
          "word": "这是",
        },
        {
          "tag": "m",
          "word": "一个",
        },
        {
          "tag": "i",
          "word": "伸手不见五指",
        },
        {
          "tag": "uj",
          "word": "的",
        },
        {
          "tag": "n",
          "word": "黑夜",
        },
        {
          "tag": "x",
          "word": "。",
        },
        {
          "tag": "r",
          "word": "我",
        },
        {
          "tag": "v",
          "word": "叫",
        },
        {
          "tag": "nr",
          "word": "孙悟空",
        },
        {
          "tag": "x",
          "word": "，",
        },
        {
          "tag": "r",
          "word": "我",
        },
        {
          "tag": "v",
          "word": "爱",
        },
        {
          "tag": "ns",
          "word": "北京",
        },
        {
          "tag": "x",
          "word": "，",
        },
        {
          "tag": "r",
          "word": "我",
        },
        {
          "tag": "v",
          "word": "爱",
        },
        {
          "tag": "eng",
          "word": "Python",
        },
        {
          "tag": "c",
          "word": "和",
        },
        {
          "tag": "nz",
          "word": "C++",
        },
        {
          "tag": "ul",
          "word": "了",
        },
        {
          "tag": "x",
          "word": "。",
        },
      ]
    `)
  })
});