CREATE TABLE instance (
	id INTEGER NOT NULL,
	name TEXT NOT NULL,
	instance_json TEXT NOT NULL,

	PRIMARY KEY (id),

	UNIQUE (name)
);
