CREATE SCHEMA IF NOT EXISTS slep;
-- AWS RDS support this: https://docs.aws.amazon.com/AmazonRDS/latest/PostgreSQLReleaseNotes/postgresql-extensions.html
CREATE EXTENSION IF NOT EXISTS btree_gist;