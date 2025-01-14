// import test from "node:test";
// import assert from "node:assert";
import { describe, it, expect } from "vitest";
import { split } from "..";


describe("split", async () => {
  it("dynamic import", async () => {
    const { split } = await import("../dist/nodejs/jieba");
    expect(typeof split).toBe("function");
    expect(split("中国南宁美比尔科技有限公司")).toEqual([ '中国', '南宁', '美', '比尔', '科技', '有限公司' ]);
  })

  it("should split without options", () => {
    expect(typeof split).toBe("function");
    expect(split("中国南宁美比尔科技有限公司")).toEqual([ '中国', '南宁', '美', '比尔', '科技', '有限公司' ]);
  });

  it("should split with options", () => {
    const options = {}
    options.mode = 'Default'
    options.hmm = false
    expect(split("中国南宁美比尔科技有限公司", options)).toEqual([ '中国', '南宁', '美', '比尔', '科技', '有限公司' ]);
  });

  it("should split with all options", () => {
    const options = {}
    options.mode = 'All'
    options.hmm = true
    const result = split("中国南宁美比尔科技有限公司", options);
    expect(result).toEqual([
      '中',       '中国', '国',
      '南',       '南宁', '宁',
      '美',       '比',   '比尔',
      '尔',       '科',   '科技',
      '技',       '有',   '有限',
      '有限公司', '限',   '公',
      '公司',     '司'
    ]);
  });
});
