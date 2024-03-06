# Vision

1. Days:
    - Days start with a date e.g. a line containing [2024-12-31]
    - Days contain sections. Sections are optional, but the Done section is mandatory
    - Days are usually split by two empty lines, but that is not required
2. Sections:
    - Each section has a name which is a line that does not start with a dash
    - Each section has tasks which are all lines that start with a dash after the beginning of the section until the next section or day
    - Sections are usually split by an empty line, but that is not required
3. Tasks:
    - Every task must be inside a day, and may be inside a section
    - Every task must start by a dash
    - When a new day begins, all tasks in the previous day are copied to the current day, except the ones that are under the Done section


E.g. for this day:
```
[2024-12-29]

Shopping
- garlic bread
- tomatoes
- screw driver

Serious
- install router
- bake garlic bread

Done
- pick up clothes
- some other task that is done
```

The next day will start like this:
```
[2024-12-30]

Shopping
- garlic bread
- tomatoes
- screw driver

Serious
- install router
- bake garlic bread

Done

```
