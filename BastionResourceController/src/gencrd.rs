use bastion_kubernetes_ressource::controller::Bastion;
use kube::CustomResourceExt;

fn main() {
    println!("{}", serde_yaml::to_string(&Bastion::crd()).unwrap());
}
