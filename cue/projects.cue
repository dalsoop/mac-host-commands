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


github: [Name=string]: #Project & {
	name: Name
	host: "github"
}

github: {
	"dal-forge": {
		path:   "~/프로젝트/dal-forge"
		org:    "dalsoop"
		repo:   "dalforge"
		url:    "https://github.com/dalsoop/dalforge"
		branch: "main"
		status: "active"
		lang:   ["go"]
	}

	"veilkey-go-package": {
		path:   "~/프로젝트/veilkey-go-package"
		org:    "veilkey"
		repo:   "veilkey-go-package"
		url:    "https://github.com/veilkey/veilkey-go-package"
		branch: "main"
		status: "active"
		lang:   ["go"]
	}

	"SoulFlow-Orchestrator": {
		path:   "~/프로젝트/SoulFlow-Orchestrator"
		org:    "berrzebb"
		repo:   "SoulFlow-Orchestrator"
		url:    "https://github.com/berrzebb/SoulFlow-Orchestrator"
		branch: "main"
		status: "active"
		lang:   ["typescript"]
	}

	"dalcenter": {
		path:   "~/프로젝트/dalcenter"
		org:    "dalsoop"
		repo:   "dalcenter"
		url:    "https://github.com/dalsoop/dalcenter"
		branch: "main"
		status: "active"
		lang:   ["go"]
	}

	"obsidian-plugin-cue": {
		path:   "~/프로젝트/obsidian-plugin-cue"
		org:    "dalsoop"
		repo:   "obsidian-plugin-cue"
		url:    "https://github.com/dalsoop/obsidian-plugin-cue"
		branch: "main"
		status: "active"
		lang:   ["typescript"]
	}

	"dalsoop-tmux-tools": {
		path:   "~/프로젝트/dalsoop-tmux-tools"
		org:    "dalsoop"
		repo:   "dalsoop-tmux-tools"
		url:    "https://github.com/dalsoop/dalsoop-tmux-tools"
		branch: "main"
		status: "active"
		lang:   ["rust"]
	}

	"veilkey-chain": {
		path:   "~/프로젝트/veilkey-chain"
		org:    "veilkey"
		repo:   "veilkey-chain"
		url:    "https://github.com/veilkey/veilkey-chain"
		branch: "main"
		status: "active"
		lang:   ["go"]
	}

	"veilkey": {
		path:   "~/프로젝트/veilkey"
		org:    "veilkey"
		repo:   "veilkey-selfhosted"
		url:    "https://github.com/veilkey/veilkey-selfhosted"
		branch: "main"
		status: "active"
		lang:   ["rust"]
	}

	"mac-host-commands": {
		path:   "~/프로젝트/mac-host-commands"
		org:    "dalsoop"
		repo:   "mac-host-commands"
		url:    "https://github.com/dalsoop/mac-host-commands"
		branch: "main"
		status: "active"
		lang:   []
	}

}

gitea: [Name=string]: #Project & {
	name: Name
	host: "gitea"
}

gitea: {
}

gitlab: [Name=string]: #Project & {
	name: Name
	host: "gitlab"
}

gitlab: {
}

local: [Name=string]: #Project & {
	name: Name
	host: "none"
}

local: {
}

_summary: {
	github_count: len(github)
	gitea_count:  len(gitea)
	gitlab_count: len(gitlab)
	local_count:  len(local)
	total:        github_count + gitea_count + gitlab_count + local_count
}
