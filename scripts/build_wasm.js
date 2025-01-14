const { execSync } = require('child_process');
// 获取环境变量 TARGET
const TARGET = process.env.TARGET;

if (!TARGET) {
  console.error('TARGET environment variable is not set.');
  process.exit(1);
}

const DistName = process.env.OUT_DIR || "dist";

function main() {
  // 执行 wasm-bindgen 命令
  try {
    const cmd = `wasm-bindgen target/wasm32-unknown-unknown/release/jieba.wasm --out-dir ./${DistName}/${TARGET} --target ${TARGET}`
    execSync(cmd, { stdio: 'inherit' });
    console.log(`Build for ${TARGET} completed successfully.`);
  } catch (error) {
    console.error(error, `\nBuild for ${TARGET} failed.`);
    process.exit(1);
  }

}

main()