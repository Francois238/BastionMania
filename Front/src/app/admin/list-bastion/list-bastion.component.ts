import { Component, OnInit } from '@angular/core';
import { BastionInfo } from '../bastion-info';
import { FormGroup, FormControl } from '@angular/forms';
import { AuthenticationService } from 'src/app/login/authentication.service';
import { AdminService } from '../admin.service';
import { NewBastion } from '../new-bastion';
import { throwToolbarMixedModesError } from '@angular/material/toolbar';

@Component({
  selector: 'app-list-bastion',
  templateUrl: './list-bastion.component.html',
  styleUrls: ['./list-bastion.component.scss']
})
export class ListBastionComponent implements OnInit {

  public name: string ='';
  public subnet_cidr: string ='';
  public message: string ='';
  public bastion! : NewBastion
  public ajoutForm: FormGroup;
  public nameCrtl: FormControl;
  public subnet_cidr_nameCrtl: FormControl;

  public agentForm: FormGroup;
  public tokenCrtl: FormControl;
  public public_keyCrtl: FormControl;
  public agent_hostCrtl: FormControl;

  public error='';

  public listBastions : Array<BastionInfo> = new Array<BastionInfo>();


  constructor(protected adminService : AdminService, protected serviceAuthentication: AuthenticationService) { 

    this.nameCrtl = new FormControl('')
    this.subnet_cidr_nameCrtl = new FormControl('')
    this.ajoutForm = new FormGroup({
        name: this.nameCrtl,
        subnet_cidr: this.subnet_cidr_nameCrtl,

    })

    this.tokenCrtl = new FormControl('')
    this.public_keyCrtl = new FormControl('')
    this.agent_hostCrtl = new FormControl('')
    this.agentForm = new FormGroup({
        token: this.tokenCrtl,
        public_key: this.public_keyCrtl,
        agent_host: this.agent_hostCrtl,

    })
  }

  ngOnInit(): void {

    this.nameCrtl = new FormControl('')
    this.subnet_cidr_nameCrtl = new FormControl('')
    this.ajoutForm = new FormGroup({
        name: this.nameCrtl,
        subnet_cidr: this.subnet_cidr_nameCrtl,

    })

    this.tokenCrtl = new FormControl('')
    this.public_keyCrtl = new FormControl('')
    this.agent_hostCrtl = new FormControl('')
    this.agentForm = new FormGroup({
        token: this.tokenCrtl,
        public_key: this.public_keyCrtl,
        agent_host: this.agent_hostCrtl,

    })

    this.getListBastion()
  }

  configureAgent(){

    this.message = '';

    let token = this.tokenCrtl.value.trim() as string;
    let public_key = this.public_keyCrtl.value.trim() as string;
    let agent_host = this.agent_hostCrtl.value.trim() as string;

    let agentSend = {
      token : token,
      public_key : public_key,
      agent_host : agent_host
  }

    

    this.adminService.configure_agent(agentSend).subscribe({
      next: (data : any) => {
          
          this.message="L'agent a bien été configuré"
          
        }
    })

}

  ajoutBastion(){
    this.message = '';
    this.error = '';

    this.name = this.nameCrtl.value.trim();
    this.subnet_cidr = this.subnet_cidr_nameCrtl.value.trim();

    this.bastion = {
      bastion_name : this.name,
      subnet_cidr : this.subnet_cidr
    }

    console.log("nom du bastion: " + this.bastion.bastion_name)

    this.adminService.add_bastion(this.bastion).subscribe({
      next: (data : any) => {
        
        this.message="Le bastion a bien été ajouté\n Voici le token: " + data.data.token
        this.getListBastion()

        
      },
      error: (e) => {
        
        console.error(e)
        this.error = "Impossible d'ajouter le bastion"
      }
  })

  }

  getListBastion(){

    this.adminService.get_bastions().subscribe({

      next: (data : BastionInfo[]) => {

        
        
        this.listBastions = data

        console.log("liste des bastions: " + this.listBastions)

        
      },
      error: (e) => {
        
        console.error(e)
      },
    })

  }

  refreshList(data : string){

    this.getListBastion()
  }

}

