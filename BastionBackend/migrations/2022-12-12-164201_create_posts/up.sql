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
    rtype         TEXT NOT NULL,
    id_wireguard INT ,
    id_ssh       INT ,
    id_k8s       INT
);

CREATE TABLE wireguardressource(
    id INT PRIMARY KEY,
    name TEXT NOT NULL,
    subnet_cidr TEXT NOT NULL
);

CREATE TABLE sshressource(
    id INT PRIMARY KEY,
    name TEXT NOT NULL,
    ip_machine TEXT NOT NULL
);

CREATE TABLE k8sressource(
    id INT PRIMARY KEY,
    name TEXT NOT NULL,
    ip_cluster TEXT NOT NULL
);


