const fs = require('fs');
const path = require('path');

const input = fs.readFileSync(path.join(__dirname, 'prod.txt'), 'utf8');

// Part 1
let result = 0;

input.split('\n').forEach((line) => {
  let [left, right] = line.split(',');
  [left, right] = [left.split('-'), right.split('-')].map((arr) => arr.map((num) => Number(num)));
  if      (left[0] <= right[0] && left[1] >= right[1]) result += 1;
  else if (right[0] <= left[0] && right[1] >= left[1]) result += 1;
});

console.log(result);

// Part 2
let result2 = 0;

input.split('\n').forEach((line) => {
  let [left, right] = line.split(',');
  [left, right] = [left.split('-'), right.split('-')].map((arr) => arr.map((num) => Number(num)));
  if      (left[0] <= right[0] && left[1] >= right[0]) result2 += 1;
  else if (right[0] <= left[0] && right[1] >= left[0]) result2 += 1;
});

console.log(result2);
