if (process.argv.length != 4) {
    console.log("Usage: puzzle_checker puzzle.cond task.desc");
    process.exit(1);
}

var fs = require('fs');
var puzzleData = fs.readFileSync(process.argv[2]).toString();
if (puzzleData.length == 0) {
    console.log("Error! puzzle data is empty.");
    process.exit(1);
}
var taskData = fs.readFileSync(process.argv[3]).toString();
if (taskData.length == 0) {
    console.log("Error! task data is empty.");
    process.exit(1);
}

var jsdom = require('jsdom');
var fs = require('fs');
eval(fs.readFileSync(__dirname + '/lambda.js').toString());

var dom = new jsdom.JSDOM(
    '<html><body><main class="page-content" aria-label="Content">'+
    '<div class="wrapper"><h1 id="check-puzzle">Check Puzzle</h1>'+
    '<center id="main_section"></center>'+
    '</div></main></body></html>');

global.window = dom.window;
global.document = window.document;
global.Blob = window.Blob;
global.FileReader = window.FileReader;
puzzle();

var submit_task = document.getElementById("submit_task");
Object.defineProperty(submit_task, "files", { value: [
    new Blob([puzzleData], { type: "text/plain" }),
] });

var submit_solution = document.getElementById("submit_solution");
Object.defineProperty(submit_solution, "files", { value: [
    new Blob([taskData], { type: "text/plain" }),
] });

async function sleep(t) {
    return await new Promise(r => {
      setTimeout(() => {
        r();
      }, t);
    });
}

async function main() {
    submit_task.onchange();
    submit_solution.onchange();
    await sleep(1000);
    document.getElementById("execute_solution").click();
    await sleep(100);
    while (true) {
        if (document.getElementById("output").innerHTML !=
            "Validating the puzzle solution...") {
            break;
        }
        await sleep(50);
    }
    await sleep(100);
    process.stdout.write(document.getElementById("output").innerHTML + "\n");
}

main();
