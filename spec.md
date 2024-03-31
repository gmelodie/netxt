# Vision

1. Days:
    - Days start with a date e.g. a line containing [2024-12-31]
    - Days contain sections. Sections are optional.
    - Days are usually split by at least one empty line, althought that is not required
2. Sections:
    - Each section has a name which is a line that does not start with a dash
    - Each section has tasks which are all lines that start with a dash after the beginning of the section until the next section or day
    - Sections are usually split by an empty line, but that is not required
3. Tasks:
    - Every task must be inside a day, and may be inside a section (if not inside a section, an empty or anonymous section with name "" will be created)
    - Every task must start by a dash


E.g.:
```
[2024-12-30]
Meeting with natalie
- chose color for website
- need to deploy it
Groceries
- garlic bread
- tomatoes

[2024-12-29]
Meeting with natalie
- chose color for website
- need to deploy it
Groceries
- garlic bread
- tomatoes
```

Obs: before this was primarily a todo list, now it became something more flexible. You can still use it as a todo list, or simply as a marker for things you did, or a mix of both.
