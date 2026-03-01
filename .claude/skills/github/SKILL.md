---
name: github
description: Create and manage GitHub issues, pull requests, and interact with repositories. Use when creating issues, PRs, viewing GitHub resources, managing labels, or checking CI status.
argument-hint: [ message ]
disable-model-invocation: true
allowed-tools: Bash(gh issue view *), Bash(gh issue create *), Bash(gh issue list *), Bash(gh pr view *), Bash(gh pr create *), Bash(gh pr comment *)
---

# GitHub Assistant

- Interact with GitHub repositories using the GitHub CLI (`gh`).
- In all interactions and commit messages, be extremely concise and sacrifice
  grammar for the sake of concision.

## Gather context

- Infer the type of resource being requested from $ARGUMENTS: issue, pull
  request, labels, etc.
- Evaluate $ARGUMENTS to see if all mandatory parameters from "gh" are defined;
  if not, ask more questions to define them

## Take action

- Execute the required action

### PR Comments

<pr-comment-rule>
When I say to add a comment to a PR with a TODO on it, use 'checkbox' markdown format to add the TODO. For instance:

<example>
- [ ] A description of the todo goes here
</example>
</pr-comment-rule>

### Git

- Use `conventional commits`. E.g. "feat: something", "fix: something", "chore:
  something", etc.
- Be careful making commits to the `main`. Ask for confirmation before
  continuing
- Create branches with prefixes: "feat", "hotfix", "tests", "chore", "docs",
  etc.

## Verify results

- Use either the "git" or "gh" command to double-check the action was done
