/*
File: init.sql
Date: 5/5/25
Written by: Jake 
About: Template of the database layout
*/

-- accounts keeps track of the accounts
-- only needs the ID and email because we are using oauth
CREATE TABLE IF NOT EXISTS accounts(
	id SERIAL PRIMARY KEY,
	email TEXT NOT NULL
);

/*
sessions:

they need to keep track of the name and elapsed time

time_elpased = the last time we have updated the entry it should be used to manage time calculations
Ex:
Fetch-Time: 10s

Then say we fetch-time at 20s, we would say time_now - time_current = change_in_time
*/
CREATE TABLE IF NOT EXISTS sessions (
	id SERIAL PRIMARY KEY, 
	name TEXT NOT NULL,
	time_elapsed BIGINT DEFAULT 0,
	time_current BIGINT NOT NULL,
	time_ended BIGINT DEFAULT 0,
	active BOOLEAN DEFAULT FALSE,
	account_id INT references accounts(id)
);

/*
active projects we are working on

same idea from sessions appiles to time_current here
*/
CREATE TABLE IF NOT EXISTS tasks (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL,
	time_elapsed BIGINT DEFAULT 0 NOT NULL,
	time_current BIGINT NOT NULL,
	active BOOLEAN DEFAULT FALSE,
	completed BOOLEAN DEFAULT FALSE,
	account_id INT references accounts(id)
);

/*
notes are tide to a session or a task
*/
CREATE TABLE IF NOT EXISTS notes (
	timestamp BIGINT NOT NULL,
	message TEXT NOT NULL,
	foreign_id INT NOT NULL,
	foreign_type INT CHECK (foreign_type IN (0,1)) NOT NULL, -- 0 = session | 1 - tasks
	account_id INT references accounts(id)
);
