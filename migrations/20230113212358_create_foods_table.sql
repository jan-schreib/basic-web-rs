CREATE TABLE foods (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
    kcal INTEGER,
    purine INTEGER,
    uric_acid REAL,
    gout_factor INTEGER
);

-- INSERT INTO foods(name, kcal, purine, uric_acid, gout_factor) values ("foo", 1, 2, 3, 4);
-- INSERT INTO foods(name, kcal, purine, uric_acid, gout_factor) values ("bar", 1, 2, 3, 4);