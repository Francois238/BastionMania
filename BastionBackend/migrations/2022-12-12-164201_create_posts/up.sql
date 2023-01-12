-- Your SQL goes 

CREATE TABLE "to_agent_config"(
    id SERIAL PRIMARY KEY,
    privatekey TEXT NOT NULL,
    publickey TEXT NOT NULL,
    address TEXT NOT NULL, -- bloqué à 10.10.1.2
    peerallowedips TEXT NOT NULL, --ip protégé vers l'agent
    peerendpoint TEXT NOT NULL --ip/port de l'agent
);

CREATE TABLE "to_user_config"(
    id INT PRIMARY KEY,
    publickey TEXT NOT NULL,
    privatekey TEXT NOT NULL,
    ip TEXT NOT NULL
);

CREATE TABLE "agent"(
    id SERIAL PRIMARY KEY,
    privatekey TEXT NOT NULL,
    publickey TEXT NOT NULL,
    address TEXT NOT NULL --bloqué à 10.10.1.1
);
/*
CREATE TABLE "user"(
    id_user INT PRIMARY KEY,
    id_bastion INT PRIMARY KEY,
    peerpublickey TEXT NOT NULL
);*/

CREATE TABLE "bastion"(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    protocols TEXT NOT NULL,
    subnet_CIDR TEXT NOT NULL,
    endpoint_ip TEXT NOT NULL,
    endpoint_port TEXT NOT NULL,
    serveur_config_id TEXT NOT NULL,
    client_config_id TEXT NOT NULL
);