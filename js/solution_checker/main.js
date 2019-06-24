if (process.argv.length != 4 && process.argv.length != 5) {
    console.log("Usage: validate task.desc solution.sol [booster.buy]");
    process.exit(1);
}

var hasBooster = process.argv.length == 5;

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
    new Blob([function() {
        let data = fs.readFileSync(process.argv[2]).toString();
        if (data.length == 0) {
            console.log("Error! task data is empty.");
            process.exit(1);
        }
        return data;
    }()], { type: "text/plain" }),
] });

var submit_solution = document.getElementById("submit_solution");
Object.defineProperty(submit_solution, "files", { value: [
    new Blob([function() {
        let data = fs.readFileSync(process.argv[3]).toString();
        if (data.length == 0) {
            console.log("Error! solution data is empty.");
            process.exit(1);
        }
        return data
    }()], { type: "text/plain" }),
] });

var boosterData = "";
if (hasBooster) {
    var boosterData = fs.readFileSync(process.argv[4]).toString();
    if (boosterData.length != 0) {
        var submit_boosters = document.getElementById("submit_boosters");
        Object.defineProperty(submit_boosters, "files", { value: [
            new Blob([boosterData], { type: "text/plain" }),
        ]});
    }
}

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
    submit_solution.onchange();
    if (boosterData != "") {
        submit_boosters.onchange();
    }
    await sleep(1000);
    document.getElementById("execute_solution").click();
    await sleep(500);
    while (true) {
        var output = document.getElementById("output").innerHTML;
        if (output == "Done uploading solution") {
            document.getElementById("execute_solution").click();
            await sleep(1000);
        }
        if (output != "Pre-processing and validating the task...") {
            break;
        }
        await sleep(50);
    }
    await sleep(100);
    process.stdout.write(document.getElementById("output").innerHTML + "\n");
}

main();
