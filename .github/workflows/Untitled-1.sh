#!/bin/bash
labels=$(gh pr list --state merged --base main --json labels --jq '.[0].labels | map(.name)')
# echo "PR_LABELS=labels" >> "$GITHUB_OUTPUT"
echo "$labels"
echo $labels | jq -e '.[] | select(. == "type:feature" or . == "type:fix")'
if echo $labels | jq -e '.[] | select(. == "type:feature" or . == "type:fix")' > /dev/null; then
	# Determine the tag based on the label found
	VERSION_BUMP=$(echo "$PR_LABELS" | jq -e '.[] | select(. == "type:feature") | "version:major" // "version:minor"')
	# Output the Version Tag
#	echo "VERSION_BUMP=$VERSION_BUMP" >> "$GITHUB_OUTPUT"
    echo $VERSION_BUMP
    echo "Hello"
else
	echo "No 'type:feature' or 'type:fix' label found."
	echo "No Version Bump"
fi
