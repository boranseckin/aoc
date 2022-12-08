const fs = require('fs');
const path = require('path');

const input = fs.readFileSync(path.join(__dirname, 'prod.txt'), 'utf8');

// Part 1
let result = 0;

input.split('\n').forEach((line) => {
  const [left, right] = [line.slice(0, line.length / 2), line.slice(line.length / 2)];
  const common = left.split('').filter((char) => right.includes(char));
  if (common[0].charCodeAt() > 96) result += common[0].charCodeAt() - 96;
  else result += common[0].charCodeAt() - 64 + 26;
});

console.log(result);

// Part 2
let result2 = 0;

const input2 = input.split('\n');
for (let i = 0; i < input2.length; i += 3) {
  const group = input2.slice(i, i + 3).map((line) => line.split(''));
  const common = group[0]
    .filter((char) => group[1].includes(char))
    .filter((char) => group[2].includes(char));
  if (common[0].charCodeAt() > 96) result2 += common[0].charCodeAt() - 96;
  else result2 += common[0].charCodeAt() - 64 + 26;
}

console.log(result2);
