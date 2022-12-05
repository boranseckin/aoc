const fs = require('fs');
const path = require('path');

const WINS  = ['AY', 'BZ', 'CX'];
const DRAWS = ['AX', 'BY', 'CZ'];

const input = fs.readFileSync(path.join(__dirname, 'prod.txt'), 'utf8');

// Part 1
let result = 0;

input.split('\n').forEach((line) => {
  const [a, b] = line.split(' ');

  if (WINS.includes(`${a}${b}`)) result += 6;
  else if (DRAWS.includes(`${a}${b}`)) result += 3;

  switch (b) {
    case 'X': result += 1; break;
    case 'Y': result += 2; break;
    case 'Z': result += 3; break;
    default: break;
  }
});

console.log(result);

// Part 2
const WINS2  = { 'A': 'Y', 'B': 'Z', 'C': 'X' };
const LOSES2 = { 'A': 'Z', 'B': 'X', 'C': 'Y' };
const DRWAS2 = { 'A': 'X', 'B': 'Y', 'C': 'Z' };

let result2 = 0;

input.split('\n').forEach((line) => {
  let [a, b] = line.split(' ');

  switch (b) {
    case 'X': b = LOSES2[a]; break;
    case 'Y': b = DRWAS2[a]; break;
    case 'Z': b = WINS2[a]; break;
    default: break;
  }

  if (WINS.includes(`${a}${b}`)) result2 += 6;
  else if (DRAWS.includes(`${a}${b}`)) result2 += 3;

  switch (b) {
    case 'X': result2 += 1; break;
    case 'Y': result2 += 2; break;
    case 'Z': result2 += 3; break;
    default: break;
  }
});

console.log(result2);
