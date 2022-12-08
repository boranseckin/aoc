const fs = require('fs');
const path = require('path');

const input = fs.readFileSync(path.join(__dirname, 'prod.txt'), 'utf8');

// Part 1
let [stacks, moves] = input.split('\n\n');
stacks = stacks.split('\n').map((line) => line.match(/\s{4}|\w/g));
stacks = stacks
  .pop()
  .map((_, i) => stacks.map((line) => line[i].replace(/\s{4}/g, ''))
    .filter(Boolean)
    .reverse());

moves.split('\n').forEach((line) => {
  const [move, from, to] = line.match(/\d+/g);
  for (let i = 0; i < move; i++) {
    stacks[to - 1].push(stacks[from - 1].pop());
  }
});

console.log(stacks.map((stack) => stack[stack.length - 1]).join(''));

// Part 2
let [stacks2, moves2] = input.split('\n\n');
stacks2 = stacks2.split('\n').map((line) => line.match(/\s{4}|\w/g));
stacks2 = stacks2
  .pop()
  .map((_, i) => stacks2.map((line) => line[i].replace(/\s{4}/g, ''))
    .filter(Boolean)
    .reverse());

moves2.split('\n').forEach((line) => {
  const [move, from, to] = line.match(/\d+/g);
  stacks2[to - 1].push(
    ...stacks2[from - 1]
    .splice(stacks2[from - 1].length - move)
  );
});

console.log(stacks2.map((stack) => stack[stack.length - 1]).join(''));
