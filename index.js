import init, { find_path } from './pkg/wasm_project.js';

export function runPathfinding() {
    // Get input values
    const startX = parseInt(document.getElementById("startX").value, 10);
    const startY = parseInt(document.getElementById("startY").value, 10);
    const endX = parseInt(document.getElementById("endX").value, 10);
    const endY = parseInt(document.getElementById("endY").value, 10);

    // Ensure inputs are valid numbers
    if (isNaN(startX) || isNaN(startY) || isNaN(endX) || isNaN(endY)) {
        document.getElementById("output").textContent = "Error: All inputs are required and must be numbers.";
        return;
    }
    init().then(() => {
        let path = find_path(startX, startY, endX, endY);
        document.getElementById("output").textContent = `Path: ${JSON.stringify(path)}`;
    });
}

window.runPathfinding = runPathfinding;
