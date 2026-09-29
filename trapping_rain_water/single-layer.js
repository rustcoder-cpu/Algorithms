function trap(height) {
    let possibleWater = 0, waterFound = 0, wallEncountered = false;
    for (let i = 0; i < height.length; i++) {
        if (height[i] === 1) {
            wallEncountered = true;
        }
        if (wallEncountered === true && height[i] === 0) {
            possibleWater++;
        }
        if (wallEncountered === true && height[i] === 1) {
            waterFound += possibleWater;
            possibleWater = 0;
        }
    }
    return waterFound;
}
console.log(trap([0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 2, 1]));