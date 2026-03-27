package projects

// ─── Worktree 규칙 ─────────────────────────────────────

#BranchType: "feat" | "fix" | "refactor" | "docs" | "test" | "release" | "hotfix"

#WorktreeRule: {
	// 폴더 네이밍: {프로젝트}@{type}-{name}
	// 예: veilkey@feat-auth, mac-host-commands@fix-mount
	folder_pattern: =~"^[a-zA-Z0-9_-]+@(feat|fix|refactor|docs|test|release|hotfix)-[a-zA-Z0-9_-]+$"

	// main 브랜치는 접미사 없음: {프로젝트}/
	main_folder_pattern: =~"^[a-zA-Z0-9_-]+$"

	// 프로젝트당 최대 worktree 수
	max_worktrees: int & <=3 | *3

	// 방치 경고 기준 (일)
	stale_days: int | *7

	// 머지 후 자동 삭제
	auto_cleanup_on_merge: bool | *true
}

#Worktree: {
	project!:    string
	branch!:     string
	type!:       #BranchType
	name!:       string
	folder!:     string
	created_at!: string
	status!:     "active" | "stale" | "merged"
}

worktree_rules: #WorktreeRule & {
	max_worktrees:         3
	stale_days:            7
	auto_cleanup_on_merge: true
}
