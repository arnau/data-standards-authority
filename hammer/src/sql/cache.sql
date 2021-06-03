-- Support set to prune the cache from unseen resources.
CREATE TABLE IF NOT EXISTS session_trail (
  checksum      text     NOT NULL,
  resource_type text     NOT NULL,
  timestamp     datetime NOT NULL,

  UNIQUE (checksum, resource_type, timestamp)
);


CREATE TABLE IF NOT EXISTS topic (
  id          text NOT NULL,
  name        text NOT NULL,
  description text
);


CREATE TABLE IF NOT EXISTS endorsement_state (
  standard_id text NOT NULL PRIMARY KEY,
  status      text NOT NULL,
  start_date  date NOT NULL,
  review_date date NOT NULL,
  end_date    date,

  FOREIGN KEY (standard_id) REFERENCES standard (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS related_standard (
  standard_id text NOT NULL,
  related_standard_id text NOT NULL,

  UNIQUE (standard_id, related_standard_id),
  FOREIGN KEY (standard_id) REFERENCES standard (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS standard (
  id            text NOT NULL PRIMARY KEY,
  checksum      text NOT NULL,
  name          text NOT NULL,
  acronym       text,
  topic         text NOT NULL,
  specification text NOT NULL,
  licence       text,
  maintainer    text NOT NULL,
  content       text NOT NULL
);
