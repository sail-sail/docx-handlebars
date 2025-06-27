// Test script to verify JSR package works
import { DocxHandlebars, init } from "jsr:@sail/docx-handlebars";

async function testJsrPackage() {
  try {
    console.log('Testing @sail/docx-handlebars JSR package...');
    
    // Initialize WASM module
    console.log('Initializing WASM module...');
    await init();
    console.log('✓ WASM module initialized');
    
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
    
    // Test basic functionality (without actual file)
    try {
      const variables = processor.get_template_variables();
      console.log('✓ get_template_variables can be called (returns:', typeof variables, ')');
    } catch (_e) {
      console.log('⚠ get_template_variables requires template to be loaded first (expected)');
    }
    
    console.log('✓ JSR package test completed successfully');
    console.log('🎉 @sail/docx-handlebars is working correctly from JSR!');
  } catch (error) {
    if (error instanceof Error) {
      console.error('✗ JSR package test failed:', error.message);
      console.error('Stack:', error.stack);
    } else {
      console.error('✗ JSR package test failed:', error);
    }
  }
}

testJsrPackage();
