use bastion_ressource_controller::controller::Bastion;
use kube::CustomResourceExt;

fn main() {
    println!("{}", serde_yaml::to_string(&Bastion::crd()).unwrap());
}
