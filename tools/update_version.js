#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

/**
 * 更新项目版本号的脚本
 * 使用方法: node tools/update-version.js <new-version>
 * 或者通过 npm: npm run version -- <new-version>
 */

function updateVersion(newVersion) {
    if (!newVersion) {
        console.error('错误: 请提供新的版本号');
        console.log('使用方法: npm run version -- <new-version>');
        console.log('示例: npm run version -- 0.1.5');
        process.exit(1);
    }

    // 验证版本号格式 (简单的语义化版本验证)
    const versionRegex = /^\d+\.\d+\.\d+(-[a-zA-Z0-9-]+(\.[a-zA-Z0-9-]+)*)?$/;
    if (!versionRegex.test(newVersion)) {
        console.error('错误: 版本号格式无效，请使用语义化版本格式 (例如: 1.0.0, 1.0.0-beta.1)');
        process.exit(1);
    }

    console.log(`开始更新版本号到 ${newVersion}...`);

    try {
        // 1. 更新 package.json
        updatePackageJson(newVersion);
        
        // 2. 更新 Cargo.toml
        updateCargoToml(newVersion);
        
        // 3. 更新 pkg-npm/package.json
        updatePkgNpmPackageJson(newVersion);
        
        // 4. 更新 pkg-jsr/jsr.json
        updatePkgJsrJson(newVersion);
        
        // 5. 更新 pkg-jsr/deno.json
        updatePkgJsrDenoJson(newVersion);
        
        // 6. 更新测试文件中的版本号
        updateTestFiles(newVersion);
        
        // 7. 自动更新 Cargo.lock (通过运行 cargo check)
        updateCargoLock();

        console.log('✅ 版本号更新完成!');
        console.log('\n更新的文件:');
        console.log('- package.json');
        console.log('- Cargo.toml');
        console.log('- pkg-npm/package.json');
        console.log('- pkg-jsr/jsr.json');
        console.log('- pkg-jsr/deno.json');
        console.log('- tests/npm_test/package.json');
        console.log('- tests/jsr_test/test.ts');
        console.log('\n请运行 npm run build 来重新构建项目');
        
    } catch (error) {
        console.error('❌ 更新版本号时发生错误:', error.message);
        process.exit(1);
    }
}

function updatePackageJson(newVersion) {
    const packageJsonPath = path.join(__dirname, '..', 'package.json');
    console.log('更新 package.json...');
    
    const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
    const oldVersion = packageJson.version;
    
    packageJson.version = newVersion;
    
    fs.writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2) + '\n');
    console.log(`  ✓ package.json: ${oldVersion} → ${newVersion}`);
}

function updateCargoToml(newVersion) {
    const cargoTomlPath = path.join(__dirname, '..', 'Cargo.toml');
    console.log('更新 Cargo.toml...');
    
    let cargoToml = fs.readFileSync(cargoTomlPath, 'utf8');
    const oldVersionMatch = cargoToml.match(/version\s*=\s*"([^"]+)"/);
    const oldVersion = oldVersionMatch ? oldVersionMatch[1] : '未知';
    
    // 替换 [package] 部分的版本号
    cargoToml = cargoToml.replace(
        /^(\[package\][\s\S]*?version\s*=\s*)"[^"]+"/, 
        `$1"${newVersion}"`
    );
    
    fs.writeFileSync(cargoTomlPath, cargoToml);
    console.log(`  ✓ Cargo.toml: ${oldVersion} → ${newVersion}`);
}

function updateCargoLock() {
    console.log('更新 Cargo.lock...');
    
    const { execSync } = require('child_process');
    
    try {
        // 运行 cargo check 来自动更新 Cargo.lock
        execSync('cargo check', {
            cwd: path.join(__dirname, '..'),
            stdio: 'pipe'
        });
        
        console.log('  ✓ Cargo.lock 已通过 cargo check 自动更新');
    } catch (error) {
        throw new Error(`cargo check 失败: ${error.message}`);
    }
}

function updatePkgNpmPackageJson(newVersion) {
    const pkgNpmPackageJsonPath = path.join(__dirname, '..', 'pkg-npm', 'package.json');
    console.log('更新 pkg-npm/package.json...');
    
    const packageJson = JSON.parse(fs.readFileSync(pkgNpmPackageJsonPath, 'utf8'));
    const oldVersion = packageJson.version;
    
    packageJson.version = newVersion;
    
    fs.writeFileSync(pkgNpmPackageJsonPath, JSON.stringify(packageJson, null, 2) + '\n');
    console.log(`  ✓ pkg-npm/package.json: ${oldVersion} → ${newVersion}`);
}

function updatePkgJsrJson(newVersion) {
    const pkgJsrJsonPath = path.join(__dirname, '..', 'pkg-jsr', 'jsr.json');
    console.log('更新 pkg-jsr/jsr.json...');
    
    const jsrJson = JSON.parse(fs.readFileSync(pkgJsrJsonPath, 'utf8'));
    const oldVersion = jsrJson.version;
    
    jsrJson.version = newVersion;
    
    fs.writeFileSync(pkgJsrJsonPath, JSON.stringify(jsrJson, null, 2) + '\n');
    console.log(`  ✓ pkg-jsr/jsr.json: ${oldVersion} → ${newVersion}`);
}

function updatePkgJsrDenoJson(newVersion) {
    const pkgJsrDenoJsonPath = path.join(__dirname, '..', 'pkg-jsr', 'deno.json');
    console.log('更新 pkg-jsr/deno.json...');
    
    const denoJson = JSON.parse(fs.readFileSync(pkgJsrDenoJsonPath, 'utf8'));
    const oldVersion = denoJson.version;
    
    denoJson.version = newVersion;
    
    fs.writeFileSync(pkgJsrDenoJsonPath, JSON.stringify(denoJson, null, 2) + '\n');
    console.log(`  ✓ pkg-jsr/deno.json: ${oldVersion} → ${newVersion}`);
}

function updateTestFiles(newVersion) {
    console.log('更新测试文件中的版本号...');
    
    // 更新 tests/npm_test/package.json 中的依赖版本
    const npmTestPackageJsonPath = path.join(__dirname, '..', 'tests', 'npm_test', 'package.json');
    if (fs.existsSync(npmTestPackageJsonPath)) {
        const packageJson = JSON.parse(fs.readFileSync(npmTestPackageJsonPath, 'utf8'));
        if (packageJson.dependencies && packageJson.dependencies['docx-handlebars']) {
            const oldVersion = packageJson.dependencies['docx-handlebars'];
            packageJson.dependencies['docx-handlebars'] = newVersion;
            fs.writeFileSync(npmTestPackageJsonPath, JSON.stringify(packageJson, null, 2) + '\n');
            console.log(`  ✓ tests/npm_test/package.json: ${oldVersion} → ${newVersion}`);
        }
    }
    
    // 更新 tests/jsr_test/test.ts 中的 JSR 包版本
    const jsrTestFilePath = path.join(__dirname, '..', 'tests', 'jsr_test', 'test.ts');
    if (fs.existsSync(jsrTestFilePath)) {
        let testFileContent = fs.readFileSync(jsrTestFilePath, 'utf8');
        const jsrImportRegex = /from "jsr:@sail\/docx-handlebars@(\d+\.\d+\.\d+(?:-[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*)?)"/;
        const match = testFileContent.match(jsrImportRegex);
        
        if (match) {
            const oldVersion = match[1];
            testFileContent = testFileContent.replace(jsrImportRegex, `from "jsr:@sail/docx-handlebars@${newVersion}"`);
            fs.writeFileSync(jsrTestFilePath, testFileContent);
            console.log(`  ✓ tests/jsr_test/test.ts: @${oldVersion} → @${newVersion}`);
        }
    }
}

// 获取命令行参数
const args = process.argv.slice(2);
const newVersion = args[0];

// 执行版本更新
updateVersion(newVersion);
