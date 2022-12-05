const fs = require('fs');
const path = require('path');

const input = fs.readFileSync(path.join(__dirname, 'prod.txt'), 'utf8');

const elves = input.split('\n\n').map((line) => eval(line.replace(/\n/g, '+')));

// Part 1
console.log(Math.max(...elves));

// Part 2
const top3 = [0, 0, 0];
top3[0] = Math.max(...elves);
top3[1] = Math.max(...elves.filter((i) => !(i in top3)));
top3[2] = Math.max(...elves.filter((i) => !(i in top3)));
console.log(eval(top3.join('+')));
