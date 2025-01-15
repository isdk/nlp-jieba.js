import { describe, it, expect } from "vitest";
import { loadDefaultDict, loadDict, clear, hasWord, addWord, removeWord, suggestFreq } from "..";

loadDefaultDict()

describe("dict", () => {
  it("should clear dict", () => {
    expect(hasWord("我们")).toBeTruthy();
    clear();
    expect(hasWord("我们")).toBeFalsy();
    loadDefaultDict();
    expect(hasWord("我们")).toBeTruthy();
  });
  it("should add/remove word", () => {
    expect(hasWord("我们开始")).toBeFalsy();
    addWord("我们开始");
    expect(hasWord("我们开始")).toBeTruthy();
    removeWord("我们开始");
    expect(hasWord("我们开始")).toBeFalsy();
  });

  it("should add/remove word with freq", ()=> {
    addWord("我们开始", 300);
    expect(suggestFreq("我们开始")).toBe(300)
    removeWord("我们开始");
    expect(hasWord("我们开始")).toBeFalsy()
    addWord("我们开始", 301);
    expect(suggestFreq("我们开始")).toBe(301)
    console.log('🚀 ~ it ~ addWord!!')
    expect(hasWord("中出")).toBeTruthy();
    expect(suggestFreq("中出")).toBe(348)
    addWord("中出", 300);
    // But it's less than calculated freq 100
    expect(suggestFreq("中出")).toBe(348)
    addWord("中出", 500);
    // Now it's significant enough
    expect(suggestFreq("中出")).toBe(500)
  })
  it("should load dict", () => {
    const dict = `
      我们开始 300
      快乐无限 500
    `;
    clear();
    expect(hasWord("我们开始")).toBeFalsy();
    expect(hasWord("快乐无限")).toBeFalsy();
    loadDict(dict);
    expect(hasWord("我们开始")).toBeTruthy();
    expect(hasWord("快乐无限")).toBeTruthy();
    clear();
    loadDefaultDict()
  });
});
