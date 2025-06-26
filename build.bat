@echo off
echo Building docx-handlebars for all platforms...

echo.
echo Building Rust library...
cargo build --release
if %ERRORLEVEL% neq 0 (
    echo Rust build failed!
    exit /b 1
)

echo.
echo Building WASM for web...
wasm-pack build --target web --out-dir pkg
if %ERRORLEVEL% neq 0 (
    echo WASM web build failed!
    exit /b 1
)

echo.
echo Building WASM for Node.js...
wasm-pack build --target nodejs --out-dir pkg-node
if %ERRORLEVEL% neq 0 (
    echo WASM Node.js build failed!
    exit /b 1
)

echo.
echo Building WASM for bundler...
wasm-pack build --target bundler --out-dir pkg-bundler
if %ERRORLEVEL% neq 0 (
    echo WASM bundler build failed!
    exit /b 1
)

echo.
echo All builds completed successfully!
echo.
echo Output directories:
echo   - Rust: target/release/
echo   - Web: pkg/
echo   - Node.js: pkg-node/
echo   - Bundler: pkg-bundler/
echo.
echo Ready for publishing!
