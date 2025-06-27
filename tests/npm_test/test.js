// Test script to verify npm package works
const { DocxHandlebars } = require('docx-handlebars');

function testNpmPackage() {
  try {
    console.log('Testing docx-handlebars npm package...');
    
    // Create instance
    const processor = new DocxHandlebars();
    console.log('✓ DocxHandlebars instance created');
    
    // Test if we can call methods (they should exist)
    const methods = Object.getOwnPropertyNames(Object.getPrototypeOf(processor));
    console.log('✓ Available methods:', methods);
    
    // Test method existence
    if (typeof processor.load_template === 'function') {
      console.log('✓ load_template method exists');
    }
    if (typeof processor.render === 'function') {
      console.log('✓ render method exists');
    }
    if (typeof processor.get_template_variables === 'function') {
      console.log('✓ get_template_variables method exists');
    }
    
    console.log('✓ npm package test completed successfully');
  } catch (error) {
    console.error('✗ npm package test failed:', error.message);
  }
}

testNpmPackage();
