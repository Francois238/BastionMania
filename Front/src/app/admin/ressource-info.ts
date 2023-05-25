export interface RessourceInfo {
    id: string;
    bastion_id: string;
    name: string;
    rtype: string;
    id_wiresguard: number | null;
    id_ssh: number | null;
    id_k8s: number | null;
}
