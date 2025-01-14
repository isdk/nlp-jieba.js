const path = require("node:path");
const fs = require("node:fs");
const { execSync } = require('child_process');

const platforms = ["bundler", "deno", "nodejs", "web"];
const rootDir = path.join(__dirname, "..");
const DistName = process.env.OUT_DIR || "dist";

const distDir = path.resolve(rootDir, DistName);
async function main() {
  await Promise.all(
    platforms.map(async (platform) => {
      const inputFile = path.join(distDir, platform, "jieba_bg.wasm");
      const outputFile = path.join(
        distDir,
        platform,
        "jieba_bg_o.wasm"
      );
      const cmd = `wasm-opt -Oz -o ${outputFile} ${inputFile}`;

      execSync(cmd, { stdio: 'inherit' });
      fs.renameSync(outputFile, inputFile);
    })
  );
}

main();
