CREATE TABLE IF NOT EXISTS accounts(
	id SERIAL PRIMARY KEY,
	email TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions (
	id SERIAL PRIMARY KEY, 
	name TEXT NOT NULL,
	time_elapsed BIGINT DEFAULT 0,
	time_current BIGINT NOT NULL,
	time_ended BIGINT DEFAULT 0,
	active BOOLEAN DEFAULT FALSE,
	account_id INT references accounts(id)
);

CREATE TABLE IF NOT EXISTS tasks (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL,
	time_elapsed BIGINT DEFAULT 0 NOT NULL,
	time_current BIGINT NOT NULL,
	active BOOLEAN DEFAULT FALSE,
	completed BOOLEAN DEFAULT FALSE,
	account_id INT references accounts(id)
);

CREATE TABLE IF NOT EXISTS notes (
	timestamp BIGINT NOT NULL,
	message TEXT NOT NULL,
	foreign_id INT NOT NULL,
	foreign_type INT CHECK (foreign_type IN (0,1)) NOT NULL, -- 0 = session | 1 - tasks
	account_id INT references accounts(id)
);
