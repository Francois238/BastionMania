export interface RessourceInfo {
    id: string;
    id_bastion: string;
    name: string;
    rtype: string;
    id_wireguard: number | null;
    id_ssh: number | null;
    id_k8s: number | null;
}
