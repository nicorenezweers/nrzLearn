"""Per-exercise checks for verify.py, keyed by exercise id.

A check is a dict (or a list of dicts). Keys:
  sql             query to run (default: the exercise code; for gap, blocks,
                  order and bug: the completed or fixed code)
  mode, headers   sqlite3 output mode, "list" (default, no headers) or "table"
  expect          exact output
  first           exact first line of the output
  contains_line   one line the output must contain
  rows            number of output lines
  error           the query must fail with this text in the error message
  measure         for input exercises: "output" (default), "rows" or "cols";
                  the first check's value is compared with the first answer
  answer_contains text that only the right choice option contains
  fix             for bug exercises: the corrected line
"""

CHECKS = {
    # ───────────── select ─────────────
    "select-1": {"sql": "SELECT * FROM books LIMIT 1;", "headers": True, "first": "id|title|author_id|genre|year|price"},
    "select-2": {"measure": "rows"},
    "select-3": {"rows": 7},
    "select-4": {"rows": 7, "first": "Emma|novel"},
    "select-5": {"sql": "select title from books;", "rows": 7},
    "select-6": {"fix": "  price"},
    "select-7": {"sql": "SELECT year, title FROM books LIMIT 1;", "headers": True, "first": "year|title"},
    "select-8": {"sql": "SELECT * FROM books LIMIT 1;", "measure": "cols"},
    "select-10": {"rows": 7, "first": "Emma|8.5"},
    "select-12": [
        {"sql": "SELECT title FROM books;", "rows": 7, "first": "Emma"},
        {"sql": "SELECT books FROM title;", "error": "no such table: title"},
        {"sql": "FROM books SELECT title;", "error": "syntax error"},
    ],
    # ───────────── where ─────────────
    "where-1": {"measure": "rows"},
    "where-2": [
        {"sql": "SELECT 1 <> 2, 1 <> 1;", "expect": "1|0"},
        {"sql": "SELECT 1 =! 2;", "error": "unrecognized token"},
        {"sql": "SELECT 1 >< 2;", "error": "syntax error"},
        {"sql": "SELECT 1 =/= 2;", "error": "syntax error"},
    ],
    "where-3": {"expect": "Emma\nPersuasion"},
    "where-4": {"expect": "Dracula\nFrankenstein"},
    "where-5": {"sql": "SELECT title FROM books WHERE year BETWEEN 1815 AND 1818 AND year IN (1815, 1818);",
                "expect": "Emma\nFrankenstein"},
    "where-6": [
        {"expect": "Dracula\nFrankenstein"},
        {"sql": "SELECT title FROM books WHERE (genre = 'horror' OR genre = 'novel') AND year > 1850;",
         "expect": "Dracula"},
    ],
    "where-7": {"fix": "WHERE year BETWEEN 1815 AND 1818;"},
    "where-10": {"expect": "The Time Machine\nMoby Dick"},
    "where-11": {"sql": "SELECT title FROM books WHERE title LIKE 'the%';",
                 "expect": "The Time Machine\nThe War of the Worlds"},
    "where-12": {"measure": "rows"},
    # ───────────── sorting ─────────────
    "sorting-2": {"sql": "SELECT year FROM books ORDER BY year DESC LIMIT 1;", "expect": "1898"},
    "sorting-3": {"expect": "The War of the Worlds\nDracula"},
    "sorting-4": {"expect": "adventure\nhorror\nnovel\nsci-fi"},
    "sorting-5": {"sql": "SELECT price FROM books WHERE price IS NOT NULL ORDER BY price;",
                  "expect": "5.5\n6.25\n7.5\n7.75\n8.5\n11.75"},
    "sorting-6": {"first": "The Time Machine|sci-fi", "answer_contains": "The Time Machine"},
    "sorting-7": {"fix": "ORDER BY year DESC"},
    "sorting-8": {"measure": "rows"},
    "sorting-10": {"expect": "Persuasion"},
    # ───────────── functions ─────────────
    "functions-3": {"sql": "SELECT (SELECT COUNT(*) FROM books WHERE price IS NULL),"
                           " (SELECT COUNT(*) FROM books WHERE price = NULL),"
                           " (SELECT COUNT(*) FROM books WHERE price = 0),"
                           " (SELECT COUNT(*) FROM books WHERE price = '');",
                    "expect": "1|0|0|0"},
    "functions-4": {"rows": 6},
    "functions-5": {"sql": "SELECT NULL + 5 IS NULL;", "expect": "1"},
    "functions-6": {"expect": "11.8", "answer_contains": "11.8"},
    "functions-7": {"rows": 7, "first": "EMMA"},
    "functions-8": {"fix": "WHERE price IS NULL;"},
    "functions-10": {"rows": 6, "contains_line": "Moby Dick|14.1"},
    "functions-12": [
        {"rows": 6},
        {"sql": "SELECT COUNT(*) FROM books WHERE (price < 6 OR price >= 6) AND title = 'The War of the Worlds';",
         "expect": "0"},
    ],
    # ───────────── aggregates ─────────────
    "aggregates-3": [
        {"sql": "SELECT AVG(year) FROM books WHERE genre = 'novel';", "expect": "1816.0"},
        {"sql": "SELECT MEAN(year) FROM books;", "error": "no such function"},
        {"sql": "SELECT AVERAGE(year) FROM books;", "error": "no such function"},
        {"sql": "SELECT MID(year) FROM books;", "error": "no such function"},
    ],
    "aggregates-4": {"expect": "6", "answer_contains": "6"},
    "aggregates-5": {"expect": "16.25"},
    "aggregates-6": {"sql": "SELECT AVG(price) = SUM(price) / 6.0, AVG(price) = SUM(price) / 7.0 FROM books;",
                     "expect": "1|0"},
    "aggregates-7": {"expect": "5.5"},
    "aggregates-8": [{"fix": "  MAX(price) AS most_expensive"}, {"expect": "11.75"}],
    "aggregates-10": {"expect": "3|1815"},
    "aggregates-12": {"expect": "13.75", "rows": 1},
    # ───────────── grouping ─────────────
    "grouping-1": {"measure": "rows"},
    "grouping-2": [
        {"sql": "SELECT genre FROM books GROUP BY genre HAVING COUNT(*) > 1 ORDER BY genre;",
         "expect": "horror\nnovel\nsci-fi"},
        {"sql": "SELECT genre FROM books WHERE COUNT(*) > 1 GROUP BY genre;", "error": "misuse of aggregate"},
    ],
    "grouping-3": {"expect": "adventure|1\nhorror|2\nnovel|2\nsci-fi|2"},
    "grouping-4": [
        {"expect": "horror\nnovel\nsci-fi", "answer_contains": "adventure"},
        {"sql": "SELECT genre FROM books GROUP BY genre ORDER BY genre LIMIT 1;", "expect": "adventure"},
    ],
    "grouping-5": {"sql": "SELECT genre FROM books WHERE COUNT(*) > 1 GROUP BY genre;", "error": "misuse of aggregate"},
    "grouping-6": {"rows": 4, "contains_line": "horror|1897"},
    "grouping-8": [{"fix": "HAVING AVG(price) > 7"}, {"expect": "adventure|11.75\nnovel|8.125"}],
    "grouping-10": {"expect": "sci-fi|2"},
    "grouping-11": {"sql": "SELECT (SELECT COUNT(*) FROM (SELECT genre FROM books GROUP BY genre)),"
                           " (SELECT COUNT(DISTINCT genre) FROM books);",
                    "expect": "4|4"},
    "grouping-12": [
        {"sql": "SELECT SUM(price) FROM books WHERE genre = 'horror';"},
        {"contains_line": "horror|13.75"},
    ],
    # ───────────── joins ─────────────
    "joins-1": {"measure": "rows"},
    "joins-3": {"rows": 7},
    "joins-4": {"rows": 8, "answer_contains": "8"},
    "joins-5": {"sql": "SELECT COUNT(*) FROM authors AS a JOIN books AS b ON b.author_id = a.id"
                       " WHERE a.name = 'Jules Verne';",
                "expect": "0"},
    "joins-6": {"rows": 7, "contains_line": "Dracula|Bram Stoker"},
    "joins-7": [{"fix": "LEFT JOIN books AS b"}, {"rows": 8, "contains_line": "Jules Verne|"}],
    "joins-10": {"rows": 7, "first": "Dracula|Bram Stoker"},
    "joins-12": [
        {"rows": 5},
        {"sql": "SELECT COUNT(*) FROM authors WHERE country = 'UK';", "expect": "3"},
    ],
    # ───────────── changing ─────────────
    "changing-3": {"sql": "INSERT INTO books (id, title) VALUES (8, 'Kipps');\nSELECT COUNT(*) FROM books;",
                   "expect": "8"},
    "changing-4": {"sql": "UPDATE books SET price = 9.99 WHERE id = 1;\nSELECT price FROM books WHERE id = 1;",
                   "expect": "9.99"},
    "changing-5": {"sql": "DELETE FROM books;\nSELECT COUNT(*) FROM books;", "expect": "0"},
    "changing-6": {"error": "NOT NULL constraint failed: members.name", "answer_contains": "error"},
    "changing-7": {"sql": "DELETE FROM books WHERE id = 7;\nSELECT COUNT(*) FROM books WHERE id = 7;",
                   "expect": "0"},
    "changing-8": [
        {"fix": "VALUES (1, 'horror');"},
        {"sql": "CREATE TABLE genres (id INTEGER PRIMARY KEY, name TEXT NOT NULL);\n"
                "INSERT INTO genres (id, name)\nVALUES (1, horror);", "error": "no such column: horror"},
    ],
    "changing-10": {"expect": "1|A"},
    "changing-11": {"expect": "5.0"},
}
