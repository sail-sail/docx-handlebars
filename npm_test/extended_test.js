// Extended test script to verify npm package functionality
const { DocxHandlebars } = require('docx-handlebars');
const fs = require('fs');
const path = require('path');

async function testDocumentProcessing() {
  try {
    console.log('🧪 Testing docx-handlebars npm package (version 0.1.4)...\n');
    
    // Create instance
    const processor = new DocxHandlebars();
    console.log('✓ DocxHandlebars instance created');
    
    // Test template file path
    const templatePath = path.join(__dirname, '..', 'examples', 'template.docx');
    console.log('📄 Template path:', templatePath);
    
    // Check if template file exists
    if (!fs.existsSync(templatePath)) {
      console.log('⚠️  Template file not found, skipping document processing test');
      return;
    }
    
    // Load template
    const templateData = fs.readFileSync(templatePath);
    console.log('✓ Template file loaded, size:', templateData.length, 'bytes');
    
    // Load template into processor
    processor.load_template(templateData);
    console.log('✓ Template loaded into processor');
    
    // Get template variables
    const variables = processor.get_template_variables();
    console.log('✓ Template variables:', variables);
    
    // Test data for rendering
    const testData = {
      company: "测试公司",
      address: "测试地址123号",
      phone: "123-456-7890",
      email: "test@example.com",
      items: [
        { name: "数据分析工具", description: "专业的数据分析软件", price: 1000 },
        { name: "项目管理系统", description: "高效的项目管理平台", price: 2000 },
        { name: "客户关系管理", description: "完整的CRM解决方案", price: 1500 }
      ]
    };
    
    // Render document
    console.log('🔄 Rendering document with test data...');
    const renderedData = processor.render(JSON.stringify(testData));
    console.log('✓ Document rendered successfully, size:', renderedData.length, 'bytes');
    
    // Save output
    const outputPath = path.join(__dirname, 'test_output_npm.docx');
    fs.writeFileSync(outputPath, renderedData);
    console.log('✓ Output saved to:', outputPath);
    
    console.log('\n🎉 npm package test completed successfully!');
    console.log('📋 Test Summary:');
    console.log('   - Package version: 0.1.4');
    console.log('   - Instance creation: ✓');
    console.log('   - Template loading: ✓');
    console.log('   - Variable extraction: ✓');
    console.log('   - Document rendering: ✓');
    console.log('   - Output generation: ✓');
    
  } catch (error) {
    console.error('❌ npm package test failed:', error.message);
    console.error('Stack trace:', error.stack);
  }
}

testDocumentProcessing();
