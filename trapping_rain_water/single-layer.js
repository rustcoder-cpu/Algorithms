function trap(height) {
    let waterFound = 0;
    let maxHeight = Math.max(0, ...height);

    for (let layer = 1; layer <= maxHeight; layer++) {
        let possibleWater = 0, wallEncountered = false;
        for (let i = 0; i < height.length; i++) {
            if (height[i] >= layer) {
                wallEncountered = true;
            }
            if (wallEncountered === true && height[i] < layer) {
                possibleWater++;
            }
            if (wallEncountered === true && height[i] >= layer) {
                waterFound += possibleWater;
                possibleWater = 0;
            }
        }
    }

    return waterFound;
}
console.log(trap([0, 3, 0, 3, 1, 0, 1, 3, 2, 1, 3, 1]));