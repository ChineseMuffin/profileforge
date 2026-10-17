# Project Guidelines

## Work at a human-understandable pace

- Respond in the language the user uses.
- When a checklist is present, work on one item at a time. Do not bundle multiple items or start later items ahead of the user's pace.
- Before each item, briefly explain its purpose, the scope of the proposed change, and how it will be verified.
- For decisions that affect design or behavior, briefly explain the options and wait for the user's choice. Do not make significant assumptions unilaterally.
- After each item, summarize what changed and the verification results. Wait for the user before starting the next item.
- When the user asks a question or requests an explanation, answer clearly before making code changes.

## Making changes

- Inspect the existing structure and relevant files before changing only the requested scope.
- Keep changes small and verifiable; run the smallest relevant tests or checks.
- Do not report checks as successful if they could not be run; explain why.
- Do not add real profile data, secrets, or generated images to the repository.

## Git commits

- Do not disable commit signing. If signing fails, stop and report the issue rather than bypassing it.
