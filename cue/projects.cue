package projects

#Project: {
	name:    string
	path:    string
	host:    "github" | "gitlab" | "gitea" | "none"
	org:     string | *""
	repo:    string | *""
	url:     string | *""
	branch:  string | *"main"
	status:  "active" | "archived" | "stale" | "local-only"
	lang:    [...string] | *[]
	tags:    [...string] | *[]
	note:    string | *""
}

