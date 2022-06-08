PGPASSWORD=sqlx psql --host localhost --port 5432 -U sqlx -d sqlx -c 'ALTER TABLE coffee DROP COLUMN process'
