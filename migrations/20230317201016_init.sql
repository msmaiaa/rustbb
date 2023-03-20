-- Add migration script here
CREATE TABLE main_forum (
	id SERIAL PRIMARY KEY,
	title TEXT NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX main_forum_unique ON main_forum ((true));

CREATE TABLE forum_user (
	id SERIAL PRIMARY KEY,
	username VARCHAR(50) NOT NULL UNIQUE,
	email VARCHAR(320) NOT NULL UNIQUE,
	password VARCHAR(255) NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE category (
	id SERIAL PRIMARY KEY,
	title VARCHAR(255) NOT NULL,
	description TEXT,
	creator_id INT NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY (creator_id) REFERENCES forum_user(id)
);

CREATE TABLE forum (
	id SERIAL PRIMARY KEY,
	title VARCHAR(255) NOT NULL,
	description TEXT,
	slug VARCHAR(255) NOT NULL,
	category_id INT NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY (category_id) REFERENCES category(id)
);

CREATE TABLE thread (
	id SERIAL PRIMARY KEY,
	title VARCHAR(255) NOT NULL,
	slug VARCHAR(255) NOT NULL,
	sticky BOOLEAN NOT NULL DEFAULT FALSE,
	forum_id INT NOT NULL,
	creator_id INT NOT NULL,

	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

	FOREIGN KEY (creator_id) REFERENCES forum_user(id),
	FOREIGN KEY (forum_id) REFERENCES forum(id)
);