package handler

import (
	"context"
	"fmt"
	"html/template"
	"net/http"
	"strings"

	"google.golang.org/appengine"
	"google.golang.org/appengine/log"
)

type HTML string

func registerHandler(
	pattern string,
	handler func(context.Context, *http.Request) (HTML, error)) {
	http.HandleFunc(pattern, func(w http.ResponseWriter, r *http.Request) {
		ctx := appengine.NewContext(r)
		output, err := handler(ctx, r)
		if err != nil {
			log.Errorf(appengine.NewContext(r), "%+v", err)
			http.Error(w, fmt.Sprintf("%+v", err), 500)
			return
		}
		w.Write([]byte(`
<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<title></title>
<link href="https://fonts.googleapis.com/icon?family=Material+Icons" rel="stylesheet">
<link rel="shortcut icon" href="/s/favicon.png">
<style>

@import url('https://fonts.googleapis.com/css?family=Open+Sans:400,600');
@import url('https://fonts.googleapis.com/css?family=Source+Sans+Pro:200,300,400,600,700');
@import url('https://fonts.googleapis.com/css?family=Roboto:100,300,400');

html, body {
	margin: 0;
	background-color: #f7f7f7;
	font-family: "Source Sans Pro", sans-serif;
	font-weight: 300;
	height: 100%;
	-moz-osx-font-smoothing: grayscale;
	-webkit-font-smoothing: antialiased;
}

body {
	display: flex;
	flex-direction: column;
	justify-content: space-between;
}

tt, code, kbd, samp {
	font-family: "Andale Mono", "Courier New", monospace;
}

a:not(:hover) {
	text-decoration: none;
}

#body {
	width: 100%;
	height: 100%;
	border-spacing: 0;
	border-collapse: collapse;
}

#header {
	height: 60px;
}

#article {
	vertical-align: top;
}

#footer {
	height: 60px;
}

#body > tbody > tr > td {
	padding: 0;
}

header {
	display: flex;
	background-color: #509ee3;
	line-height: 60px;
	height: 60px;
	padding: 0 10px;
	color: #fff;
}

a#home {
	display: inline-block;
	margin: auto;
	height: 40px;
	border-radius: 10px;
	padding: 5px;
	font-weight: 700;
	line-height: 40px;
	font-size: 30px;
	color: #fff;
}

a#home:hover {
	background-color: rgba(255, 255, 255, 0.5);
}

#logo {
	height: 40px;
}

#link {
	font-size: 20px;
	margin: 0 1em;
}

header a.material-icons {
	color: #fff;
	border-radius: 30px;
	height: 30px;
	width: 30px;
	display: inline-block;
	text-align: center;
	line-height: 30px;
	margin: auto;
}

header a.material-icons:hover {
	background-color: rgba(255, 255, 255, 0.5);
}

header > span {
	display: inline-flex;
	height: 60px;
	margin: 0;
	vertical-align: bottom;
	flex-grow: 0;
}

header a {
	text-decoration: none;
}

header input[type="text"] {
	border: 0;
	background-color: transparent;
	color: #fff;
	font-size: 15px;
}

header input[type="text"]::placeholder {
	color: #fff;
}

#search_area {
	flex-grow: 10;
	padding: 0 2em;
}

#search_area > a {
	display: inline-block;
	font-size: 20px;
	text-decoration: none;
	font-weight: 500;
	margin: 0 0.7em;
	color: #fff;
}

#search_box {
	display: flex;
	height: 40px;
	width: 100%;
	border-radius: 5px;
	background-color: #62a8e6;
	margin: auto 20px;
	overflow: hidden;
	color: #fff;
}

#search_box input {
	flex-grow: 10;
	border-radius: 5px;
	padding: 10px;
}

#search_box button {
	background-color: transparent;
	border: none;
	line-height: 40px;
	color: #fff;
}

article {
	flex-grow: 10;
}

article section {
	border: 1px solid #ddd;
	background-color: #fff;
	padding: 10px;
	border-radius: 5px;
	margin: 5px 0;
}

#container {
	box-sizing: border-box;
	width: 90%;
	margin: 30px auto;
	padding: 0 10px;
}

h1, h2, h3, h4, h5 {
	margin: 5px 0;
	padding: 0;
}

h1 {
	font-size: 140%;
	color: #444;
	font-weight: 700;
}

h2 {
	font-size: 130%;
	color: #444;
	font-weight: 600;
}

ul.breadcrumb {
	font-size: 90%;
	list-style: none;
	padding: 0 0 0.3em 0;
	border-bottom: 1px solid #ddd;
}

ul.breadcrumb li {
	display: inline;
}

ul.breadcrumb li+li:before {
	padding: 8px;
	color: black;
	content: "/\00a0";
}

ul.breadcrumb li a {
	text-decoration: none;
}

ul.breadcrumb li a:hover {
	text-decoration: underline;
}

table.table {
	width: 100%;
	background-color: #fff;
	border-color: #ddd;
	border-width: 1px;
	border-style: solid;
	border-radius: 5px;
	border-spacing: 0;
	font-weight: 300;
	margin: 5px 0;
	overflow: hidden;
		
}

table.table > thead {
	font-weight: 600;
}

table.table > thead + tbody > tr:nth-child(odd),
table.table > tbody > tr:nth-child(even) {
	background-color: #f8f8f8;
}

table.table > thead + tbody > tr:nth-child(even),
table.table > tbody > tr:nth-child(odd) {
	background-color: #ffffff;
}

table.table tbody th {
	font-weight: 600;
	text-align: left;
}

table.table td, table.table th {
	padding: 0.2em 0.3em;
	white-space: nowrap;
	vertical-align: top;
}

table.table > tbody > tr.clickable:hover {
	background-color: #eeeeee;
}

table.table ul, table.table ol {
	margin: 0;
}

table.form {
	width: 100%;
	max-width: 850px;
	margin: auto;
	table-layout: fixed;
}

table.form td.description {
	padding: 0 0 1em;
}

table.form td, table.form th {
	vertical-align: top;
}

table.form th {
	font-weight: 500;
	text-align: left;
}

table.form td input, table.form td textarea, table.form td select {
	width: 100%;
	border-radius: 5px;
	border: 1px solid #ccc;
	box-sizing: border-box;
	padding: 0 5px;
}

table.form td input, table.form td select {
	height: 22px;
	background: #fff;
}

table.form td textarea {
	height: 20em;
}

table.form button, .buttons .button {
	background: #07e;
	border: 0;
	border-radius: .25rem;
	font-size: 110%;
	color: #fff;
	padding: .5rem 4rem;
	font-family: "Roboto", sans-serif;
	font-weight: 400;
	cursor: pointer;
	text-decoration: none;
	display: inline-block;
	margin: 0 0.3rem 0.3rem;
}

table.form button:hover, .buttons .button:hover {
	background: #06d;
}

table.form .buttons, .buttons {
	text-align: center;
	padding: 0.7rem;
}

table.form td.description {
	font-size: 80%;
}

table.form input.error, table.form textarea.error {
	border-color: red;
}

table.form td.error {
	font-size: 90%;
	color: red;
}

pre, textarea {
	border: 1px solid #ddd;
	background-color: #fff;
	padding: 10px;
	border-radius: 5px;
	font-family: "Andale Mono", monospace;
	word-break: break-all;
	white-space: pre-wrap;
	font-size: 90%;
	margin: 5px 0;
	width: 100%;
	box-sizing: border-box;
}

article a.material-icons {
	text-decoration: none;
	font-size: 100%;
	color: #07e;
	vertical-align: middle;
	cursor: pointer;
}

article a.material-icons:hover {
	color: #5af;
}

article .material-icons {
	font-size: 100%;
	margin: 0 0.15em;
}

.badge {
	text-decoration: none;
	display: inline-flex;
	align-items: center;
}

.success-color {
	color: #080;
}

.failed-color {
	color: #a00;
}

.warning-color {
	color: #c80;
}

.broken-color {
	color: #000;
}

.waiting-color {
	color: #888;
}

.running-color {
	color: #00a;
}

.internal-color {
	color: #000;
}

.canceled-color {
	color: #888;
}

.unknown-color {
	color: #888;
}

.min-column {
	white-space: nowrap;
	width: 0;
}

.max-column {
	overflow: hidden;
	text-overflow: ellipsis;
	width: 100%;
	max-width: 0;
}

.wrap-column {
	width: 100%;
	max-width: 0;
}

.wrap-column code {
	white-space: pre-wrap;
}

table.table td.wrap-column {
	white-space: inherit;
}

#chart {
	border: 1px solid #ddd;
	border-radius: 5px;
	overflow: hidden;
}

.pagination a.button {
	background-color: #fff;
	border: 1px solid #eee;
	display: inline-block;
	width: 30px;
	height: 30px;
	border-radius: 30px;
	text-align: center;
	line-height: 30px;
	margin: 5px;
	font-size: 20px;
	color: #509ee3;
	font-weight: 600;
	text-decoration: none;
}

.pagination a.button:hover {
	background-color: #cdf;
}

footer {
	background: #eee;
	color: #444;
	font-weight: 300;
}

#footer-container {
	width: 90%;
	margin: auto;
	padding: 10px 0 50px;
}

.char-space {
	position: relative;
}

.char-space::before {
	position: absolute;
	top: 0;
	left: 0;
	content: "·";
	color: #ccc;
}

.char-newline::before {
	content: "\\n";
	color: #ccc;
}

table.table-clickable tr[data-href] {
	cursor: pointer;
}

table.table-clickable tr[data-href]:hover td {
	background: #eee;
}

.w400 {
	max-width: 400px;
	height: auto;
}

.pix {
	image-rendering: pixelated;
}

</style>
<script src="https://ajax.googleapis.com/ajax/libs/jquery/3.3.1/jquery.min.js"></script>
<script>
$(function(){
	$('tr[data-href]', 'table.table-clickable').on('click', function(){
	location.href = $(this).data('href');
	});
});
</script>
</head>
<body>
<table id="body" style="table-layout: fixed">
<tr><td id="header">
<header>
<span><a href="/" id="home" title="Home">Unagi Dashboard</a></span>
<span id="search_area">
<a href="/ranking/" title="Ranking">Ranking</a>
<a href="/problems/" title="Problems">Problems</a>
<a href="/programs/" title="Programs">Programs</a>
<a href="/status/" title="Status">Status</a>
</span>
<span>
	
</span>
</header>
</td></tr>
<tr><td id="article" style="overflow-x:scroll">
<article>
<div id="container">
		`))
		w.Write([]byte(output))
		w.Write([]byte(
			`
</div>
</article>
</td></tr>
<tr><td id="footer">

	<footer>
		<div id="footer-container">
			Copyright (C) 2018 Team Unagi<br>
		</div>
	</footer>

</td></tr></table>
<script type="text/javascript" src="//ajax.googleapis.com/ajax/libs/jquery/1.10.2/jquery.min.js" defer></script>
<script type="text/javascript" src="https://www.gstatic.com/charts/loader.js"></script>
</body>
</html>
`))
	})
}

func Escape(s string) HTML {
	return HTML(template.HTMLEscapeString(s))
}

type HTMLBuffer strings.Builder

func (b *HTMLBuffer) WriteHTML(s ...HTML) {
	for _, ss := range s {
		(*strings.Builder)(b).WriteString(string(ss))
	}
}

func (b *HTMLBuffer) WriteString(s string) {
	(*strings.Builder)(b).WriteString(template.HTMLEscapeString(s))
}

func (b *HTMLBuffer) HTML() HTML {
	return HTML((*strings.Builder)(b).String())
}
