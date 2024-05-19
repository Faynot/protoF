const { execSync } = require('child_process');
const path = require('path');

// Function to check if a package is installed
function isInstalled(packageName) {
    try {
        execSync(`${packageName} --version`, { stdio: 'ignore' });
        return true;
    } catch (error) {
        return false;
    }
}

// Function to install a package using npm
function installPackage(packageName) {
    console.log(`Installing ${packageName}...`);
    execSync(`npm install -g ${packageName}`, { stdio: 'inherit' });
    console.log(`${packageName} installed.`);
}

// Function to add a directory to the PATH environment variable
function addToPath(directory) {
    console.log(`Adding ${directory} to the PATH environment variable...`);
    const currentPath = process.env.PATH || '';
    const newPath = `${directory}${path.delimiter}${currentPath}`;
    process.env.PATH = newPath;
}

// Check and install Black for Python
if (!isInstalled('black')) {
    installPackage('black');
    const blackPath = execSync('npm root -g').toString().trim() + path.sep + 'black';
    addToPath(blackPath);
}

// Check and install Prettier
if (!isInstalled('prettier')) {
    installPackage('prettier');
    const prettierPath = execSync('npm root -g').toString().trim() + path.sep + 'prettier';
    addToPath(prettierPath);
}

// Check and install Rustfmt for Rust
if (!isInstalled('rustfmt')) {
    installPackage('rustfmt');
    const rustfmtPath = execSync('npm root -g').toString().trim() + path.sep + 'rustfmt';
    addToPath(rustfmtPath);
}

// Function to format a file
function formatFile(filename) {
    if (filename.endsWith('.rs')) {
        execSync(`rustfmt ${filename}`, { stdio: 'inherit' });
    } else if (filename.endsWith('.py')) {
        execSync(`black ${filename}`, { stdio: 'inherit' });
    } else {
        execSync(`prettier --write ${filename}`, { stdio: 'inherit' });
    }
}

// Get the file name from command line arguments
const scriptName = process.argv[2];
const scriptExtension = process.argv[3];
const cleaningFile = path.join(scriptName, scriptExtension);

try {
    formatFile(cleaningFile);
    console.log("Your code is now clean!");
} catch (error) {
    console.error("Error during formatting:", error.message);
}
