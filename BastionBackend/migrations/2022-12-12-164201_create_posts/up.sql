CREATE TABLE "bastion"(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    subnet_cidr TEXT NOT NULL,
    agent_endpoint TEXT NOT NULL,
    pubkey TEXT NOT NULL,
    port INT NOT NULL,
    net_id INT NOT NULL
);

CREATE TABLE "users"(
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    bastion_id INT NOT NULL,
    wireguard BOOLEAN NOT NULL,
    net_id INT NOT NULL

);

CREATE TABLE "ressource"
(
    id           INT PRIMARY KEY,
    id_bastion   INT NOT NULL,
    name         TEXT NOT NULL,
    type         TEXT NOT NULL,
    id_wireguard TEXT NOT NULL,
    id_ssh       TEXT NOT NULL,
    id_k8s       TEXT NOT NULL
);




