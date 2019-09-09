CREATE TABLE books (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  user_id INTEGER NOT NULL,
  librarything_id TEXT,
  title TEXT NOT NULL,
  author_lf TEXT NOT NULL,
  author_code TEXT NOT NULL,
  isbn TEXT NOT NULL,
  publicationdate TEXT NOT NULL,
  rating INTEGER,
  language_main TEXT NOT NULL,
  language_secondary TEXT,
  language_original TEXT NOT NULL,
  review TEXT,
  cover TEXT NOT NULL,
  -- tags VARCHAR,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  dateacquired_stamp DATETIME,
  started_stamp DATETIME,
  finished_stamp DATETIME
);