CREATE TABLE cluster_key (
    id INT AUTO_INCREMENT,
    data VARCHAR(500) NOT NULL,
    algorithm VARCHAR(30),
    identity VARCHAR(40) UNIQUE,
    create_at DATETIME,
    expire_at DATETIME,
    PRIMARY KEY(id)
);

CREATE TABLE data_key (
     id INT AUTO_INCREMENT,
     name VARCHAR(100) NOT NULL,
     description VARCHAR(200),
     user VARCHAR(40) NOT NULL,
     email VARCHAR(40) NOT NULL,
     attributes VARCHAR(200),
     key_type VARCHAR(10) NOT NULL,
     private_key VARCHAR(5000),
     public_key VARCHAR(5000),
     certificate VARCHAR(5000),
     create_at DATETIME,
     expire_at DATETIME,
     PRIMARY KEY(id)
);
