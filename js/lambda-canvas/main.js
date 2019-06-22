if (process.argv.length != 3) {
    console.log("Usage: validate task.desc");
    process.exit(1);
}

var fs = require('fs');
var taskData = fs.readFileSync(process.argv[2]).toString();
// var solutionData = fs.readFileSync(process.argv[3]).toString();

var jsdom = require('jsdom');
var fs = require('fs');
eval(fs.readFileSync(__dirname + '/lambda.js').toString());

var dom = new jsdom.JSDOM(
    '<html><body>'+
    '<table>'+
    '<tr>'+
    '  <td><div><canvas style="display: block" id="canvas" /></div></td>'+
    '  <td valign="top">'+
    '    <div id="main_section" valign="top"></div>'+
    '    <div>Execution speed: <text id="speed_text"></text></div>'+
    '    <br />'+
    '    <table border="0">'+
    '      <tr><td>r</td><td>Prepare/Reset</td></tr>'+
    '      <tr><td>Space (s)</td><td>Start/Pause execution</td></tr>'+
    '      <tr><td>Right (d)</td><td>Increase speed</td></tr>'+
    '      <tr><td>Left (a)</td><td>Decrease speed</td></tr>'+
    '    </table>        '+
    '  </td>'+
    '</tr>'+
    '</table>'+
    '</body></html>');

global.window = dom.window;
global.document = window.document;
global.Blob = window.Blob;
global.FileReader = window.FileReader;
render();



// var submit_solution = document.getElementById("submit_solution");
// Object.defineProperty(submit_solution, "files", { value: [
//     new Blob([solutionData], { type: "text/plain" }),
// ] });

async function sleep(t) {
    return await new Promise(r => {
      setTimeout(() => {
        r();
      }, t);
    });
}

async function main() {
    while (document.getElementById("submit_task") == null) {
        await sleep(50);
    }

    var canvas = document.getElementById("canvas");
    var ctx = canvas.getContext("2d");
    var status = {};
    ctx.fillText = function (text) {
        status["text"] = text;
    };

    var submit_task = document.getElementById("submit_task");
    Object.defineProperty(submit_task, "files", { value: [
        new Blob([taskData], { type: "text/plain" }),
    ]});
    submit_task.onchange();
    while (status["text"] != "Done uploading task description") {
        await sleep(100);
    }
    // console.log(document.getElementsByTagName("canvas")[0].toDataURL());
}

main();
