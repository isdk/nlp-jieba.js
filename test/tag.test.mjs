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

  it('should tag a sentence2', () => {
    const result = tag('中国南宁美比尔科技有限公司位于乌鲁木齐的办公室,是一个美丽的地方！欧阳修经理跑得飞快地人们都在那里。"爱因斯坦和居里夫人是科学家!', true)
    expect(result).toMatchInlineSnapshot(`
      [
        {
          "tag": "ns",
          "word": "中国",
        },
        {
          "tag": "ns",
          "word": "南宁",
        },
        {
          "tag": "ns",
          "word": "美",
        },
        {
          "tag": "nrt",
          "word": "比尔",
        },
        {
          "tag": "n",
          "word": "科技",
        },
        {
          "tag": "ntg",
          "word": "有限公司",
        },
        {
          "tag": "v",
          "word": "位于",
        },
        {
          "tag": "ns",
          "word": "乌鲁木齐",
        },
        {
          "tag": "uj",
          "word": "的",
        },
        {
          "tag": "n",
          "word": "办公室",
        },
        {
          "tag": "x",
          "word": ",",
        },
        {
          "tag": "v",
          "word": "是",
        },
        {
          "tag": "m",
          "word": "一个",
        },
        {
          "tag": "ns",
          "word": "美丽",
        },
        {
          "tag": "uj",
          "word": "的",
        },
        {
          "tag": "n",
          "word": "地方",
        },
        {
          "tag": "x",
          "word": "！",
        },
        {
          "tag": "nr",
          "word": "欧阳修",
        },
        {
          "tag": "n",
          "word": "经理",
        },
        {
          "tag": "v",
          "word": "跑",
        },
        {
          "tag": "ud",
          "word": "得",
        },
        {
          "tag": "v",
          "word": "飞快",
        },
        {
          "tag": "uv",
          "word": "地",
        },
        {
          "tag": "n",
          "word": "人们",
        },
        {
          "tag": "d",
          "word": "都",
        },
        {
          "tag": "p",
          "word": "在",
        },
        {
          "tag": "r",
          "word": "那里",
        },
        {
          "tag": "x",
          "word": "。",
        },
        {
          "tag": "x",
          "word": """,
        },
        {
          "tag": "nrt",
          "word": "爱因斯坦",
        },
        {
          "tag": "c",
          "word": "和",
        },
        {
          "tag": "nrt",
          "word": "居里夫人",
        },
        {
          "tag": "v",
          "word": "是",
        },
        {
          "tag": "n",
          "word": "科学家",
        },
        {
          "tag": "x",
          "word": "!",
        },
      ]
    `)
  })
});