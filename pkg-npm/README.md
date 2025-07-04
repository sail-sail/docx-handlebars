# docx-handlebars

[![Crates.io](https://img.shields.io/crates/v/docx-handlebars.svg)](https://crates.io/crates/docx-handlebars)
[![Documentation](https://docs.rs/docx-handlebars/badge.svg)](https://docs.rs/docx-handlebars)
[![License](https://img.shields.io/crates/l/docx-handlebars.svg)](https://github.com/sail-sail/docx-handlebars#license)

[中文文档](README_zh.md) | English

A Rust library for processing DOCX files with Handlebars templates, supporting multiple platforms:
- 🦀 Rust native
- 🌐 WebAssembly (WASM)
- 📦 npm package
- 🟢 Node.js
- 🦕 Deno
- 🌍 Browser
- 📋 JSR (JavaScript Registry)

## Features

- ✅ **Smart Merging**: Automatically handles Handlebars syntax split by XML tags
- ✅ **DOCX Validation**: Built-in file format validation to ensure valid input files
- ✅ **Handlebars Support**: Full template engine with variables, conditionals, loops, and helper functions
- ✅ **Cross-platform**: Rust native + WASM support for multiple runtimes
- ✅ **TypeScript**: Complete type definitions and intelligent code completion
- ✅ **Zero Dependencies**: WASM binary with no external dependencies
- ✅ **Error Handling**: Detailed error messages and type-safe error handling

## Installation

### Rust

```bash
cargo add docx-handlebars
```

### npm

```bash
npm install docx-handlebars
```

### Deno

```typescript
import { render, init } from "jsr:@sail/docx-handlebars";
```

## Usage Examples

### Rust

```rust
use docx_handlebars::render_handlebars;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read DOCX template file
    let template_bytes = std::fs::read("template.docx")?;
    
    // Prepare data
    let data = json!({
        "name": "John Doe",
        "company": "ABC Technology Ltd.",
        "position": "Software Engineer",
        "projects": [
            {"name": "Project A", "status": "Completed"},
            {"name": "Project B", "status": "In Progress"}
        ],
        "has_bonus": true,
        "bonus_amount": 5000
    });
    
    // Render template
    let result = render_handlebars(template_bytes, &data)?;
    
    // Save result
    std::fs::write("output.docx", result)?;
    
    Ok(())
}
```

### JavaScript/TypeScript (Node.js)

```javascript
import { render, init } from 'docx-handlebars';
import fs from 'fs';

async function processTemplate() {
    // Initialize WASM module
    await init();
    
    // Read template file
    const templateBytes = fs.readFileSync('template.docx');
    
    // Prepare data
    const data = {
        name: "Jane Smith",
        company: "XYZ Tech Ltd.",
        position: "Senior Developer",
        projects: [
            { name: "E-commerce Platform", status: "Completed" },
            { name: "Mobile App", status: "In Development" }
        ],
        has_bonus: true,
        bonus_amount: 8000
    };
    
    // Render template
    const result = render(templateBytes, JSON.stringify(data));
    
    // Save result
    fs.writeFileSync('output.docx', new Uint8Array(result));
}

processTemplate().catch(console.error);
```

### Deno

```typescript
import { render, init } from "https://deno.land/x/docx_handlebars/mod.ts";

async function processTemplate() {
    // Initialize WASM module
    await init();
    
    // Read template file
    const templateBytes = await Deno.readFile("template.docx");
    
    // Prepare data
    const data = {
        name: "Alice Johnson",
        department: "R&D Department",
        projects: [
            { name: "AI Customer Service", status: "Live" },
            { name: "Data Visualization Platform", status: "In Development" }
        ]
    };
    
    // Render template
    const result = render(templateBytes, JSON.stringify(data));
    
    // Save result
    await Deno.writeFile("output.docx", new Uint8Array(result));
}

if (import.meta.main) {
    await processTemplate();
}
```

### Browser

```html
<!DOCTYPE html>
<html>
<head>
    <title>DOCX Handlebars Example</title>
</head>
<body>
    <input type="file" id="fileInput" accept=".docx">
    <button onclick="processFile()">Process Template</button>
    
    <script type="module">
        import { render, init } from './pkg/docx_handlebars.js';
        
        // Initialize WASM
        await init();
        
        window.processFile = async function() {
            const fileInput = document.getElementById('fileInput');
            const file = fileInput.files[0];
            
            if (!file) return;
            
            const arrayBuffer = await file.arrayBuffer();
            const templateBytes = new Uint8Array(arrayBuffer);
            
            const data = {
                name: "John Doe",
                company: "Example Company"
            };
            
            try {
                const result = render(templateBytes, JSON.stringify(data));
                
                // Download result
                const blob = new Blob([new Uint8Array(result)], {
                    type: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document'
                });
                const url = URL.createObjectURL(blob);
                const a = document.createElement('a');
                a.href = url;
                a.download = 'processed.docx';
                a.click();
            } catch (error) {
                console.error('Processing failed:', error);
            }
        };
    </script>
</body>
</html>
```

## Template Syntax

### Basic Variable Substitution

```handlebars
Employee Name: {{name}}
Company: {{company}}
Position: {{position}}
```

### Conditional Rendering

```handlebars
{{#if has_bonus}}
Bonus: ${{bonus_amount}}
{{else}}
No bonus
{{/if}}

{{#unless is_intern}}
Full-time employee
{{/unless}}
```

### Loop Rendering

```handlebars
Project Experience:
{{#each projects}}
- {{name}}: {{description}} ({{status}})
{{/each}}

Skills:
{{#each skills}}
{{@index}}. {{this}}
{{/each}}
```

### Helper Functions

Built-in helper functions:

```handlebars
{{upper name}}           <!-- Convert to uppercase -->
{{lower company}}        <!-- Convert to lowercase -->
{{len projects}}         <!-- Array length -->
{{#if (eq status "completed")}}Completed{{/if}}    <!-- Equality comparison -->
{{#if (gt score 90)}}Excellent{{/if}}              <!-- Greater than comparison -->
{{#if (lt age 30)}}Young{{/if}}                    <!-- Less than comparison -->
```

### Complex Example

```handlebars
=== Employee Report ===

Basic Information:
Name: {{employee.name}}
Department: {{employee.department}}
Position: {{employee.position}}
Hire Date: {{employee.hire_date}}

{{#if employee.has_bonus}}
💰 Bonus: ${{employee.bonus_amount}}
{{/if}}

Project Experience ({{len projects}} total):
{{#each projects}}
{{@index}}. {{name}}
   Description: {{description}}
   Status: {{status}}
   Team Size: {{team_size}} people
   
{{/each}}

Skills Assessment:
{{#each skills}}
- {{name}}: {{level}}/10 ({{years}} years experience)
{{/each}}

{{#if (gt performance.score 90)}}
🎉 Performance Rating: Excellent
{{else if (gt performance.score 80)}}
👍 Performance Rating: Good
{{else}}
📈 Performance Rating: Needs Improvement
{{/if}}
```

## Error Handling

The library provides detailed error types and messages:

### Rust

```rust
use docx_handlebars::{render_handlebars, DocxError};

match render_handlebars(template_bytes, &data) {
    Ok(result) => {
        println!("Processing successful!");
        std::fs::write("output.docx", result)?;
    }
    Err(e) => match e.downcast_ref::<DocxError>() {
        Some(DocxError::FileTooSmall) => {
            eprintln!("Error: File too small, not a valid DOCX file");
        }
        Some(DocxError::InvalidZipSignature) => {
            eprintln!("Error: File is not a valid ZIP/DOCX format");
        }
        Some(DocxError::MissingRequiredFile(filename)) => {
            eprintln!("Error: Missing required DOCX file: {}", filename);
        }
        _ => {
            eprintln!("Other error: {}", e);
        }
    }
}
```

### JavaScript/TypeScript

```javascript
try {
    const result = render(templateBytes, JSON.stringify(data));
    console.log('Processing successful!');
} catch (error) {
    if (error.message.includes('File size insufficient')) {
        console.error('File too small, not a valid DOCX file');
    } else if (error.message.includes('Invalid ZIP signature')) {
        console.error('File is not a valid ZIP/DOCX format');
    } else if (error.message.includes('Missing required DOCX file')) {
        console.error('File does not contain required DOCX components');
    } else if (error.message.includes('Template rendering failed')) {
        console.error('Handlebars template syntax error or data mismatch');
    } else {
        console.error('Processing failed:', error.message);
    }
}
```

## Build and Development

### Build WASM Packages

```bash
# Build all targets
npm run build

# Or build separately
npm run build:web    # Browser version
npm run build:npm    # Node.js version 
npm run build:jsr    # Deno version
```

### Run Examples

```bash
# Rust example
cargo run --example rust_example

# Node.js example
node examples/node_example.js

# Deno example  
deno run --allow-read --allow-write examples/deno_example.ts

# Browser example
cd tests/npm_test
node serve.js
# Then open http://localhost:8080 in your browser
# Select examples/template.docx file to test
```

## Technical Features

### Smart Merging Algorithm

The core innovation of this library is the intelligent merging of Handlebars syntax that has been split by XML tags. In DOCX files, when users input template syntax, Word may split it into multiple XML tags.

### File Validation

Built-in DOCX file validation ensures input file integrity:

1. **ZIP Format Validation**: Check file signature and structure
2. **DOCX Structure Validation**: Ensure required files are present
   - `[Content_Types].xml`
   - `_rels/.rels` 
   - `word/document.xml`
3. **MIME Type Validation**: Verify correct content types

## Performance and Compatibility

- **Zero Copy**: Efficient memory management between Rust and WASM
- **Streaming Processing**: Suitable for handling large DOCX files
- **Cross-platform**: Support for Windows, macOS, Linux, Web
- **Modern Browsers**: Support for all modern browsers that support WASM

## License

This project is licensed under the MIT License - see the [LICENSE-MIT](LICENSE-MIT) file for details.

## Support

- 📚 [Documentation](https://docs.rs/docx-handlebars)
- 🐛 [Issue Tracker](https://github.com/sail-sail/docx-handlebars/issues)
- 💬 [Discussions](https://github.com/sail-sail/docx-handlebars/discussions)

---

<div align="center">
  <p>
    <strong>docx-handlebars</strong> - Making DOCX template processing simple and efficient
  </p>
  <p>
    <a href="https://github.com/sail-sail/docx-handlebars">⭐ Star the project</a>
    ·
    <a href="https://github.com/sail-sail/docx-handlebars/issues">🐛 Report issues</a>
    ·
    <a href="https://github.com/sail-sail/docx-handlebars/discussions">💬 Join discussions</a>
  </p>
</div>
