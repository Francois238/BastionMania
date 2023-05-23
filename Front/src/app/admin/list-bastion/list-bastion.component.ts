import { Component, OnInit } from '@angular/core';
import { BastionInfo } from '../bastion-info';
import { FormGroup, FormControl } from '@angular/forms';
import { AdminInfo } from '../admin-info';
import { NewAdmin } from '../new-admin';
import { AuthenticationService } from 'src/app/login/authentication.service';
import { AdminService } from '../admin.service';

@Component({
  selector: 'app-list-bastion',
  templateUrl: './list-bastion.component.html',
  styleUrls: ['./list-bastion.component.scss']
})
export class ListBastionComponent implements OnInit {

  public name: string ='';
  public last_name: string ='';
  public mail: string ='';
  public password: string ='';
  public message: string ='';
  public admin! : NewAdmin
  public ajoutForm: FormGroup;
  public nameCrtl: FormControl;
  public last_nameCrtl: FormControl;
  public mailCrtl: FormControl;
  public passwordCrtl: FormControl;

  public listBastions : Array<BastionInfo> = new Array<BastionInfo>();


  constructor(protected adminService : AdminService, protected serviceAuthentication: AuthenticationService) { 

    this.nameCrtl = new FormControl('')
    this.last_nameCrtl = new FormControl('')
    this.mailCrtl = new FormControl('')
    this.passwordCrtl = new FormControl('')
    this.ajoutForm = new FormGroup({
        name: this.nameCrtl,
        last_name: this.last_nameCrtl,
        mail: this.mailCrtl,
        password: this.passwordCrtl

    })
  }

  ngOnInit(): void {

    this.nameCrtl = new FormControl('')
    this.last_nameCrtl = new FormControl('')
    this.mailCrtl = new FormControl('')
    this.passwordCrtl = new FormControl('')
    this.ajoutForm = new FormGroup({
        name: this.nameCrtl,
        last_name: this.last_nameCrtl,
        mail: this.mailCrtl,
        password: this.passwordCrtl

    })

    this.getListBastion()
  }

  ajoutBastion(){
    this.message = '';

    this.name = this.nameCrtl.value.trim();
    this.last_name = this.last_nameCrtl.value.trim();
    this.mail = this.mailCrtl.value.trim();
    this.password = this.passwordCrtl.value.trim();

    if( this.password.length< 2){

      this.message = "Le mot de passe doit contenir au moins 2 caractères"
      return
    }

    this.admin = {
      name : this.name,
      last_name : this.last_name,
      mail : this.mail,
      password : this.serviceAuthentication.get_hash_password(this.password)
    }

    console.log("mot de passe hashe admin : " + this.admin.password)

    this.adminService.add_admin(this.admin).subscribe({
      next: (data : AdminInfo) => {
        
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

    this.adminService.get_list_admin().subscribe({

      next: (data : AdminInfo[]) => {
        
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

