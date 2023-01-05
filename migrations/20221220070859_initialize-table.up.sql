CREATE TABLE cluster_key (
    id INT AUTO_INCREMENT,
    data VARCHAR(255) NOT NULL,
    description VARCHAR(255),
    create_at DATETIME,
    expire_at DATETIME,
    PRIMARY KEY(id)
);
