const fs = require('fs');
const path = require('path');

const input = fs.readFileSync(path.join(__dirname, 'prod.txt'), 'utf8');

// Part 1
const grid = input.split('\n').map((line) => line.split(''));
const height = grid.length;
const width = grid[0].length;

let visible = height * 2 + width * 2 - 4;

for (let i = 1; i < height - 1; i += 1) {
  for (let j = 1; j < width - 1; j += 1) {
    let isHidden = 0;

    for (let left = j - 1; left >= 0; left -= 1) {
      if (grid[i][left] >= grid[i][j]) {
        isHidden += 1;
        break;
      }
    }

    for (let right = j + 1; right < width; right += 1) {
      if (grid[i][right] >= grid[i][j]) {
        isHidden += 1;
        break;
      }
    }

    for (let top = i - 1; top >= 0; top -= 1) {
      if (grid[top][j] >= grid[i][j]) {
        isHidden += 1;
        break;
      }
    }

    for (let bottom = i + 1; bottom < height; bottom += 1) {
      if (grid[bottom][j] >= grid[i][j]) {
        isHidden += 1;
        break;
      }
    }

    if (isHidden < 4) visible += 1;
  }
}

console.log(visible);

// Part 2
let highest = [0, 0, 0];

for (let i = 1; i < height - 1; i += 1) {
  for (let j = 1; j < width - 1; j += 1) {
    let left = j - 1, right = j + 1, top = i - 1, bottom = i + 1;

    while (left > 0            && grid[i][left]   < grid[i][j]) left   -= 1;
    while (right < width - 1   && grid[i][right]  < grid[i][j]) right  += 1;
    while (top > 0             && grid[top][j]    < grid[i][j]) top    -= 1;
    while (bottom < height - 1 && grid[bottom][j] < grid[i][j]) bottom += 1;

    let scenicScore = (j - left) * (right - j) * (i - top) * (bottom - i);

    if (scenicScore > highest[0]) highest = [scenicScore, i, j];
  }
}

console.log(highest);
