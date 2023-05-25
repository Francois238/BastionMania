import { Component, OnInit } from '@angular/core';
import { BastionInfo } from '../bastion-info';
import { FormGroup, FormControl } from '@angular/forms';
import { AuthenticationService } from 'src/app/login/authentication.service';
import { AdminService } from '../admin.service';
import { NewBastion } from '../new-bastion';

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

  public listBastions : Array<BastionInfo> = new Array<BastionInfo>();


  constructor(protected adminService : AdminService, protected serviceAuthentication: AuthenticationService) { 

    this.nameCrtl = new FormControl('')
    this.subnet_cidr_nameCrtl = new FormControl('')
    this.ajoutForm = new FormGroup({
        name: this.nameCrtl,
        subnet_cidr: this.subnet_cidr_nameCrtl,

    })
  }

  ngOnInit(): void {

    this.nameCrtl = new FormControl('')
    this.subnet_cidr_nameCrtl = new FormControl('')
    this.ajoutForm = new FormGroup({
        name: this.nameCrtl,
        subnet_cidr: this.subnet_cidr_nameCrtl,

    })

    this.getListBastion()
  }

  ajoutBastion(){
    this.message = '';

    this.name = this.nameCrtl.value.trim();
    this.subnet_cidr = this.subnet_cidr_nameCrtl.value.trim();

    this.bastion = {
      bastion_name : this.name,
      subnet_cidr : this.subnet_cidr
    }

    console.log("mot de passe hashe admin : " + this.bastion.bastion_name)

    this.adminService.add_bastion(this.bastion).subscribe({
      next: (data : BastionInfo) => {
        
        this.message="L'administrateur a bien été ajouté"
        this.getListBastion()

        
      },
      error: (e) => {
        
        console.error(e)
        this.message = "Impossible d'ajouter l'administrateur"
      }
  })

  }

  getListBastion(){

    this.adminService.get_bastions().subscribe({

      next: (data : BastionInfo[]) => {
        
        this.listBastions = data

        
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

