export interface BastionInfo {
    bastion_id: string;
    name: string;
    subnet_cidr: string;
    ssh_port: number;
    wireguard_port: number;
    net_id: number;
}
