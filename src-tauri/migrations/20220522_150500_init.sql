CREATE TABLE sites (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "key" TEXT NOT NULL,
  "url" TEXT NOT NULL,
  "title" TEXT,
  "analysis_count" INTEGER NOT NULL DEFAULT 0,
  "download_count" INTEGER NOT NULL DEFAULT 0,
  "created_at" TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "updated_at" TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE site_queries (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "site_id" INTEGER NOT NULL,
  "key" TEXT NOT NULL,
  "url_pattern" TEXT NOT NULL,
  "processor" TEXT NOT NULL,
  "dom_selector" TEXT,
  "url_filter" TEXT NOT NULL,
  "filename" TEXT,
  "is_persist" INTEGER NOT NULL DEFAULT 0,
  "priority" INTEGER NOT NULL DEFAULT 0,
  "created_at" TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "updated_at" TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- parentは親ページ要素
CREATE TABLE pages (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "site_id" INTEGER NOT NULL,
  "parent_page_id" INTEGER,
  "url" TEXT NOT NULL,
  "title" TEXT,
  "is_persist" INTEGER NOT NULL DEFAULT 0,
  "created_at" TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "updated_at" TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE queues (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "site_id" INTEGER NOT NULL,
  "page_id" INTEGER NOT NULL,
  "priority" INTEGER NOT NULL DEFAULT 0,
  "created_at" TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "updated_at" TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
