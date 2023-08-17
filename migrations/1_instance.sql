CREATE TABLE instance (
	id INTEGER NOT NULL,
	name TEXT NOT NULL,
	instance_json TEXT NOT NULL,

	PRIMARY KEY (id),

	UNIQUE (name)
);

CREATE TABLE adapter (
	python_package_name TEXT NOT NULL,
	module_name TEXT NOT NULL,
	data_json TEXT NOT NULL,

	PRIMARY KEY (python_package_name, module_name)
);
