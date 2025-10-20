CREATE TABLE divisions (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(25) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

INSERT INTO divisions (name) VALUES ('friend');
INSERT INTO divisions (name) VALUES ('medinfo');
INSERT INTO divisions (name) VALUES ('food');
INSERT INTO divisions (name) VALUES ('it');
INSERT INTO divisions (name) VALUES ('fundraise');