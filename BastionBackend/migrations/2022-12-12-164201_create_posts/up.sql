-- Your SQL goes 

CREATE TABLE "client_config"(
    privatekey TEXT NOT NULL PRIMARY KEY,
    address TEXT NOT NULL,
    peerpublickey TEXT NOT NULL,
    peerallowedips TEXT NOT NULL,
    peerendpoint TEXT NOT NULL
);

CREATE TABLE "serveur_config"(
    id INT PRIMARY KEY,
    publikey TEXT NOT NULL,
    presharedkey TEXT NOT NULL,
    ip TEXT NOT NULL
);

CREATE TABLE "bastion"(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    protocols TEXT NOT NULL,
    wireguard_id INT ,
    CONSTRAINT fk_wireguard_serv
      FOREIGN KEY(wireguard_id) 
	  REFERENCES serveur_config(id)
);