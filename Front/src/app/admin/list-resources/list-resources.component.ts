import { Component, OnInit } from '@angular/core';
import { FormGroup, FormControl } from '@angular/forms';
import { AuthenticationService } from 'src/app/login/authentication.service';
import { AdminInfo } from '../admin-info';
import { AdminService } from '../admin.service';
import { BastionInfo } from '../bastion-info';
import { NewAdmin } from '../new-admin';
import { ActivatedRoute, ParamMap } from '@angular/router';
import { RessourceInfo } from '../ressource-info';
import { NewRessourceSshCreation } from '../new-ressource-ssh-creation';
import { NewRessourceWireguardCreation } from '../new-ressource-wireguard-creation';

@Component({
  selector: 'app-list-resources',
  templateUrl: './list-resources.component.html',
  styleUrls: ['./list-resources.component.scss']
})
export class ListResourcesComponent implements OnInit {


  public messageSSH: string ='';

  public messageWiresguard: string ='';

  public newSSH!: NewRessourceSshCreation

  public newWireguard!: NewRessourceWireguardCreation;

  public ajoutSSHForm: FormGroup;
  public nameSSHCrtl: FormControl;
  public ip_machineCrtl: FormControl;
  public portCrtl: FormControl;

  public ajoutWireguardForm: FormGroup;
  public nameWireguardCrtl: FormControl;
  public target_ipCrtl: FormControl;

  public bastion! : BastionInfo;

  public listRessources : Array<RessourceInfo> = new Array<RessourceInfo>();

  public bastion_id : string = '';


  constructor(protected adminService : AdminService, protected serviceAuthentication: AuthenticationService,     private activRoute: ActivatedRoute) { 

    this.ip_machineCrtl = new FormControl('')
    this.nameSSHCrtl = new FormControl('')
    this.portCrtl = new FormControl('')

    this.ajoutSSHForm = new FormGroup({
        nameSSH: this.nameSSHCrtl,
        ip_machine: this.ip_machineCrtl,
        port: this.portCrtl,

    })

    this.nameWireguardCrtl = new FormControl('')
    this.target_ipCrtl = new FormControl('')

    this.ajoutWireguardForm = new FormGroup({
        nameWireguard: this.nameWireguardCrtl,
        target_ip: this.target_ipCrtl,
    })
  }

  ngOnInit(): void {

    this.ip_machineCrtl = new FormControl('')
    this.nameSSHCrtl = new FormControl('')
    this.portCrtl = new FormControl('')

    this.ajoutSSHForm = new FormGroup({
        nameSSH: this.nameSSHCrtl,
        ip_machine: this.ip_machineCrtl,
        port: this.portCrtl,

    })

    this.nameWireguardCrtl = new FormControl('')
    this.target_ipCrtl = new FormControl('')

    this.ajoutWireguardForm = new FormGroup({
      nameWireguard: this.nameWireguardCrtl,
      target_ip: this.target_ipCrtl,
  })

    this.activRoute.paramMap.subscribe((params: ParamMap) => {
      this.bastion_id = params.get('idBastion') || '';

      this.adminService.get_a_bastion(this.bastion_id).subscribe({

        next: (data : any) => {
          this.bastion = data.data;
        }

      });

      this.getListlistRessources()
    });

   
  }

  ajoutRessourceSSH(){
    this.messageSSH = '';

    let nameSSH = this.nameSSHCrtl.value.trim() as string;
    let ip_machine = this.ip_machineCrtl.value.trim() as string;
    let port = this.portCrtl.value.trim() as number;

    this.newSSH = {
      name : nameSSH,
      rtype : "ssh",
      ip_machine : ip_machine,
      port : port
    }

    console.log("nom ressource ssh : " + this.newSSH.name)

    this.adminService.create_ssh_ressource(this.bastion_id,this.newSSH).subscribe({
      next: (data : AdminInfo) => {
        
        this.messageSSH="La ressource a bien été ajouté"
        this.getListlistRessources()

        
      },
      error: (e) => {
        
        console.error(e)
        this.messageSSH = "Impossible d'ajouter la ressource"
      }
  })

  }


  ajoutRessourceWireguard(){
    this.messageWiresguard = '';

    let nameWireguard = this.nameWireguardCrtl.value.trim() as string;
    let target_ip = this.target_ipCrtl.value.trim() as string;

    this.newWireguard = {
      name : nameWireguard,
      rtype : "wireguard",
      target_ip : target_ip,

    }

    console.log("nom ressource wireguard : " + this.newWireguard.name)

    this.adminService.create_wireguard_ressource(this.bastion_id,this.newWireguard).subscribe({
      next: (data : any) => {
        
        this.messageWiresguard="La ressource a bien été ajouté"
        this.getListlistRessources()

        
      },
      error: (e) => {
        
        console.error(e)
        this.messageWiresguard = "Impossible d'ajouter la ressource"
      }
  })

  }

  getListlistRessources(){

    this.adminService.get_ressources(this.bastion_id).subscribe({

      next: (data : any) => {
        
        this.listRessources = data.data

        
      },
      error: (e) => {
        
        console.error(e)
      },
    })

  }

  refreshList(data : string){

    this.getListlistRessources()
  }
}