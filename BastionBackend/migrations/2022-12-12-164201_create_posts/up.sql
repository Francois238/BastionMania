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
    id INT,
    bastion_id INT NOT NULL,
    wireguard BOOLEAN NOT NULL,
    net_id INT NOT NULL,
    PRIMARY KEY (id, bastion_id)
);
