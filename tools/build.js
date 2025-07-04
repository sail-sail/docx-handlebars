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

function removeGitignoreFiles(directories) {
  console.log('\n🧹 Cleaning up auto-generated .gitignore files...');
  directories.forEach(dir => {
    const gitignorePath = path.join(dir, '.gitignore');
    try {
      if (fs.existsSync(gitignorePath)) {
        fs.unlinkSync(gitignorePath);
        console.log(`✓ Removed ${gitignorePath}`);
      } else {
        console.log(`✓ No .gitignore found in ${dir}`);
      }
    } catch (error) {
      console.error(`❌ Failed to remove ${gitignorePath}:`, error.message);
    }
  });
}

function ensureDir(dir) {
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }
}

function addMainFieldToPackageJson(packageJsonPath) {
  try {
    if (fs.existsSync(packageJsonPath)) {
      const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
      
      // 检查是否已经有 main 字段
      if (!packageJson.main && packageJson.module) {
        // 在 module 字段前面添加 main 字段
        const { module, ...rest } = packageJson;
        const updatedPackageJson = {
          ...rest,
          main: module, // 使用相同的文件作为 main 入口
          module,
        };
        
        fs.writeFileSync(packageJsonPath, JSON.stringify(updatedPackageJson, null, 2) + '\n');
        console.log(`✓ Added "main" field to ${path.basename(packageJsonPath)}`);
      } else if (packageJson.main) {
        console.log(`✓ "main" field already exists in ${path.basename(packageJsonPath)}`);
      }
    }
  } catch (error) {
    console.error(`❌ Failed to update ${packageJsonPath}:`, error.message);
  }
}

function removeUnnecessaryFiles() {
  console.log('\n🧹 Cleaning up unnecessary files...');
  
  // 删除 pkg-jsr 下的 package.json
  const jsrFilesToRemove = ['pkg-jsr/package.json'];
  jsrFilesToRemove.forEach(filePath => {
    try {
      if (fs.existsSync(filePath)) {
        fs.unlinkSync(filePath);
        console.log(`✓ Removed ${filePath}`);
      } else {
        console.log(`✓ ${filePath} not found, skipping`);
      }
    } catch (error) {
      console.error(`❌ Failed to remove ${filePath}:`, error.message);
    }
  });
}

// 构建 Rust 库
runCommand('cargo build --release', 'Building Rust library');

// 构建 npm 包
runCommand('wasm-pack build --target web --out-dir pkg-npm', 'Building WASM for npm');

// 为 npm 包添加 main 字段以支持 CommonJS
console.log('\n🔧 Updating npm package.json...');
addMainFieldToPackageJson('pkg-npm/package.json');

// 构建 JSR 包
runCommand('wasm-pack build --target web --out-dir pkg-jsr', 'Building JSR package WASM files');

// 确保 pkg-jsr 目录存在
ensureDir('pkg-jsr');

// 复制必要文件到 pkg-jsr 目录
console.log('\n📋 Copying additional files to JSR package...');
copyFile('LICENSE-MIT', 'pkg-jsr/LICENSE-MIT', 'LICENSE-MIT');

console.log('✅ JSR package files are ready');

// 清理自动生成的 .gitignore 文件
removeGitignoreFiles(['pkg-npm', 'pkg-jsr']);

// 删除不必要的文件
removeUnnecessaryFiles();

console.log('\n🎉 All builds completed successfully!');
console.log('\n📁 Output directories:');
console.log('  - Rust: target/release/');
console.log('  - npm: pkg-npm/ (supports both Node.js and browsers)');
console.log('  - JSR: pkg-jsr/ (supports Deno and Node.js)');
console.log('\n🚀 Ready for publishing!');
console.log('\n📦 Publishing commands:');
console.log('  npm run publish');
