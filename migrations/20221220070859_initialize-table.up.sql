CREATE TABLE cluster_key (
    id INT AUTO_INCREMENT,
    data VARCHAR(500) NOT NULL,
    algorithm VARCHAR(30),
    identity VARCHAR(40) UNIQUE,
    create_at DATETIME,
    expire_at DATETIME,
    PRIMARY KEY(id)
);
