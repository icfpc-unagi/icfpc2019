package handler

import (
	"context"
	"net/http"
)

func init() {
	registerHandler("/knapsack_download_form", knapsackDownloadFormHandler)
}

func knapsackDownloadFormHandler(
	ctx context.Context, r *http.Request,
) (HTML, error) {
	return `
<h1>Knapsack download form</h1>
<form action="/knapsack_download" method="POST">
<textarea name="input" style="width: 100%; height: 500px"></textarea>
<input type="submit" value="Download">
</form>	
`, nil
}
