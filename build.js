#!/usr/bin/env node

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

console.log('🚀 Building docx-handlebars for all platforms...');

function runCommand(command, description) {
  console.log(`\n📦 ${description}...`);
  try {
    execSync(command, { stdio: 'inherit' });
    console.log(`✅ ${description} completed successfully`);
  } catch (error) {
    console.error(`❌ ${description} failed!`);
    process.exit(1);
  }
}

function copyFile(src, dest, description) {
  try {
    if (fs.existsSync(src)) {
      fs.copyFileSync(src, dest);
      console.log(`✓ Copied ${description}`);
    } else {
      console.warn(`⚠ ${src} not found, skipping`);
    }
  } catch (error) {
    console.error(`❌ Failed to copy ${description}:`, error.message);
  }
}

function ensureDir(dir) {
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }
}

// 构建 Rust 库
runCommand('cargo build --release', 'Building Rust library');

// 构建 npm 包
runCommand('wasm-pack build --target web --out-dir pkg-npm', 'Building WASM for npm');

// 构建 JSR 包
runCommand('wasm-pack build --target web --out-dir pkg-jsr', 'Building JSR package WASM files');

// 确保 pkg-jsr 目录存在
ensureDir('pkg-jsr');

// 复制必要文件到 pkg-jsr 目录
console.log('\n📋 Copying additional files to JSR package...');
copyFile('LICENSE-MIT', 'pkg-jsr/LICENSE-MIT', 'LICENSE-MIT');
copyFile('README.md', 'pkg-jsr/README.md', 'README.md');

console.log('✅ JSR package files are ready');

console.log('\n🎉 All builds completed successfully!');
console.log('\n📁 Output directories:');
console.log('  - Rust: target/release/');
console.log('  - npm: pkg-npm/ (supports both Node.js and browsers)');
console.log('  - JSR: pkg-jsr/ (supports Deno and Node.js)');
console.log('\n🚀 Ready for publishing!');
console.log('\n📦 Publishing commands:');
console.log('  npm: cd pkg-npm && npm publish');
console.log('  JSR: cd pkg-jsr && deno publish');
