const fs = require('fs');
const path = require('path');

const input = fs.readFileSync(path.join(__dirname, 'prod.txt'), 'utf8');

// Part 1
let marker = '';
for (const char of input.split('')) {
  const index = marker.indexOf(char);
  if (index === -1) marker += char;
  else marker = marker.slice(index + 1) + char;

  if (marker.length === 4) break;
}

console.log(input.indexOf(marker) + 4);

// Part 2
let marker2 = '';
for (const char of input.split('')) {
  const index = marker2.indexOf(char);
  if (index === -1) marker2 += char;
  else marker2 = marker2.slice(index + 1) + char;

  if (marker2.length === 14) break;
}

console.log(input.indexOf(marker2) + 14);
