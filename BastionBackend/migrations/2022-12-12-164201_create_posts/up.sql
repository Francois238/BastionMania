CREATE TABLE "bastion"(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    subnet_cidr TEXT NOT NULL,
    pubkey TEXT NOT NULL,
    port INT NOT NULL,
    net_id INT NOT NULL
);
