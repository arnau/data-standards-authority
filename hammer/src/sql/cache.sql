-- Support set to prune the cache from unseen resources.
CREATE TABLE IF NOT EXISTS session_trail (
  checksum      text     NOT NULL,
  resource_type text     NOT NULL,
  timestamp     datetime NOT NULL,

  UNIQUE (checksum, resource_type, timestamp)
);


-- jpeg, png, ...
CREATE TABLE IF NOT EXISTS asset (
  id            text NOT NULL PRIMARY KEY,
  checksum      text NOT NULL,
  content_type  text NOT NULL,
  content       blob NOT NULL
);


CREATE TABLE IF NOT EXISTS theme (
  id          text    NOT NULL PRIMARY KEY,
  checksum    text    NOT NULL,
  name        text    NOT NULL,
  description text    NOT NULL,
  ordinal     integer NOT NULL
);

CREATE TABLE IF NOT EXISTS topic (
  id          text    NOT NULL PRIMARY KEY,
  checksum    text    NOT NULL,
  name        text    NOT NULL,
  description text    NOT NULL,
  theme_id    text    NOT NULL,
  ordinal     integer NOT NULL

  -- FOREIGN KEY (theme_id) REFERENCES theme (id)
);

CREATE TABLE IF NOT EXISTS licence (
  id       text NOT NULL PRIMARY KEY,
  checksum text NOT NULL,
  name     text NOT NULL,
  acronym  text,
  url      text NOT NULL
);

CREATE TABLE IF NOT EXISTS organisation (
  id       text NOT NULL PRIMARY KEY,
  checksum text NOT NULL,
  name     text NOT NULL,
  url      text NOT NULL
);

CREATE TABLE IF NOT EXISTS section (
  id            text NOT NULL PRIMARY KEY,
  checksum      text NOT NULL,
  resource_type text NOT NULL,
  content       text NOT NULL
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
  topic_id      text NOT NULL,
  specification text NOT NULL,
  licence_id    text,
  maintainer_id text NOT NULL,
  content       text NOT NULL

  -- FOREIGN KEY (topic_id) REFERENCES topic (id)
  -- FOREIGN KEY (licence_id) REFERENCES licence (id)
  -- FOREIGN KEY (maintainer_id) REFERENCES organisation (id)
);

CREATE TABLE IF NOT EXISTS guidance_standard (
  guidance_id text NOT NULL,
  standard_id text NOT NULL,

  UNIQUE (guidance_id, standard_id),
  FOREIGN KEY (guidance_id) REFERENCES guidance (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS guidance (
  id               text NOT NULL PRIMARY KEY,
  checksum         text NOT NULL,
  description      text,
  maintainer_id    text NOT NULL,
  status           text NOT NULL,
  creation_date    date NOT NULL,
  update_date      date NOT NULL,
  publication_date date,
  canonical_url    text,
  content          text NOT NULL

  -- FOREIGN KEY (maintainer_id) REFERENCES organisation (id)
);
