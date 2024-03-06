const arr = [1, 2, 1000, 4]
console.log('Before sorting', arr);
arr.sort();
console.log('After sorting', arr);
arr.sort((a, b) => a - b)
console.log('After sorting correctly =)', arr);
