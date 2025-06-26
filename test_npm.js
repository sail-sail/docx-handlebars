// Test script to verify npm package works
const { DocxHandlebars } = require('docx-handlebars');

function testNpmPackage() {
  try {
    console.log('Testing docx-handlebars npm package...');
    
    // Create instance
    const processor = new DocxHandlebars();
    console.log('✓ DocxHandlebars instance created');
    
    // Test if we can call methods (they should exist)
    console.log('✓ Available methods:', Object.getOwnPropertyNames(Object.getPrototypeOf(processor)));
    
    console.log('✓ npm package test completed successfully');
  } catch (error) {
    console.error('✗ npm package test failed:', error);
  }
}

testNpmPackage();
