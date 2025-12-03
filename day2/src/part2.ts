const finalSum = process.argv[2].split(',').reduce((sum, range) => {
    const [min, max] = range.split('-').map(Number);
    for (let i = min; i <= max; i++) {
        if (/^(\d+)\1+$/.test(String(i))) {
            sum += i;
        }
    }
    return sum;
}, 0);
console.log(finalSum);
