CREATE TABLE users (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    pass VARCHAR(255) NOT NULL,
    role VARCHAR(5) NOT NULL,
    division_id INTEGER NOT NULL REFERENCES divisions(id),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

INSERT INTO users (name, email, pass, role, division_id) VALUES ('Admin', 'haidarbahzi07@gmail.com', 'a6cf6e2e6c4525456b2e05c7fa977638f6bd8e2f1949665fb8fd3457ee084920', 'admin', 4);