if (process.argv.length != 4) {
    console.log("Usage: validate task.desc solution.sol");
    process.exit(1);
}

var fs = require('fs');
var taskData = fs.readFileSync(process.argv[2]).toString();
if (taskData.length == 0) {
    console.log("Error! task data is empty.");
    process.exit(1);
}
var solutionData = fs.readFileSync(process.argv[3]).toString();
if (solutionData.length == 0) {
    console.log("Error! solution data is empty.");
    process.exit(1);
}

var jsdom = require('jsdom');
var fs = require('fs');
eval(fs.readFileSync(__dirname + '/lambda.js').toString());

var dom = new jsdom.JSDOM(
    '<html><body><main class="page-content" aria-label="Content" id="main">'+
    '<div class="wrapper">'+
    '<h1 id="solution-checker">Solution Checker</h1>'+
    '<center id="main_section"></center>'+
    '</div>'+
    '</main></body></html>');

global.window = dom.window;
global.document = window.document;
global.Blob = window.Blob;
global.FileReader = window.FileReader;
validate();

var submit_task = document.getElementById("submit_task");
Object.defineProperty(submit_task, "files", { value: [
    new Blob([taskData], { type: "text/plain" }),
] });

var submit_solution = document.getElementById("submit_solution");
Object.defineProperty(submit_solution, "files", { value: [
    new Blob([solutionData], { type: "text/plain" }),
] });

async function sleep(t) {
    return await new Promise(r => {
      setTimeout(() => {
        r();
      }, t);
    });
}

async function main() {
    await sleep(100);
    submit_task.onchange();
    await sleep(100);
    while (true) {
        if (document.getElementById("output").innerHTML ==
            "Done uploading task description") {
            break;
        }
        await sleep(50);
    }
    await sleep(100);
    submit_solution.onchange();
    await sleep(100);
    while (true) {
        output = document.getElementById("output").innerHTML;
        if (output == "Done uploading solution" ||
            output == "Failed: solution text is malformed") {
            break;
        }
        await sleep(50);
    }
    await sleep(100);
    document.getElementById("execute_solution").click();
    await sleep(100);
    while (true) {
        if (document.getElementById("output").innerHTML !=
            "Pre-processing and validating the task...") {
            break;
        }
        await sleep(50);
    }
    await sleep(100);
    process.stdout.write(document.getElementById("output").innerHTML + "\n");
}

main();
