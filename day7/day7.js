const fs = require('fs');
const path = require('path');

const input = fs.readFileSync(path.join(__dirname, 'prod.txt'), 'utf8');

// Part 1
let currentDir = '';
let tree = { '': {} };

const get = (dir) => {
  return dir.split('/').reduce((acc, cur) => acc[cur], tree);
}

const set = (dir, value) => {
  const lastIndex = dir.lastIndexOf('/');
  const [root, last] = [dir.slice(0, lastIndex), dir.slice(lastIndex + 1)];
  root.split('/').reduce((acc, cur) => acc[cur], tree)[last] = value;
}

const changeDirectory = (dir) => {
  if (dir === '/') {
    currentDir = '';
  } else if (dir === '..') {
    currentDir = currentDir.slice(0, currentDir.lastIndexOf('/'));
  } else {
    currentDir = `${currentDir}/${dir}`;
  }
}

const list = (input) => {
  let total = 0;

  input.split('\n').slice(1).forEach((item) => {
    if (item.startsWith('dir')) {
      set(`${currentDir}/${item.slice(4)}`, {});
    } else {
      let [size, name] = item.split(' ');
      size = parseInt(size);
      set(`${currentDir}/${name}`, size);
      total += size;
    }
  });

  set(`${currentDir}/_`, total);

  let tempDir = currentDir;
  while (tempDir) {
    const parent = tempDir.slice(0, tempDir.lastIndexOf('/'));
    const parentSize = get(`${parent}/_`);
    set(`${parent}/_`, parentSize + total);
    tempDir = parent;
  }
}

const getSmallSizes = (dir) => {
  let result = 0;

  Object.entries(dir).forEach(([key, value]) => {
    if (value['_'] && value['_'] < 100000) result += value['_'];
    result += getSmallSizes(dir[key]);
  });

  return result;
}

input.split('\n$').forEach((command) => {
  command = command.trim();

  if (command.startsWith('cd')) {
    changeDirectory(command.slice(3));
  } else if (command.startsWith('ls')) {
    list(command);
  }
});

console.log(getSmallSizes(tree['']));

// Part 2
const free = 70000000 - tree['']['_'];
const required = 30000000 - free;

const findOptimalDelete = (path) => {
  let optimal = [70000000, ''], current = [0, ''];

  Object.entries(get(path)).forEach(([key, value]) => {
    if (value['_'] && value['_'] >= required) {
      const nested = findOptimalDelete(`${path}/${key}`);
      current = nested[0] > value['_'] ? [value['_'], `${path}/${key}`] : nested;
    }

    if ((current[0] >= required && current[0] < optimal[0])) optimal = current;
  });

  return optimal;
}

console.log(findOptimalDelete(''));
