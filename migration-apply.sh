PGPASSWORD=sqlx psql --host localhost --port 5432 -U sqlx -d sqlx -c 'ALTER TABLE coffee ADD COLUMN process varchar(255)'
