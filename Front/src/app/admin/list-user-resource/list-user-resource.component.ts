import { Component } from '@angular/core';
import { FormGroup, FormControl } from '@angular/forms';
import { AuthenticationService } from 'src/app/login/authentication.service';
import { AdminInfo } from '../admin-info';
import { AdminService } from '../admin.service';
import { NewAdmin } from '../new-admin';
import { ActivatedRoute, ParamMap } from '@angular/router';
import { UserBastionInfo } from '../user-bastion-info';
import { RessourceInfo } from '../ressource-info';

@Component({
  selector: 'app-list-user-resource',
  templateUrl: './list-user-resource.component.html',
  styleUrls: ['./list-user-resource.component.scss']
})
export class ListUserResourceComponent {

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

  public ressource!: RessourceInfo;

  public listUsers : Array<UserBastionInfo> = new Array<UserBastionInfo>();

  public bastion_id : string = '';

  public ressource_id : string = '';

  constructor(protected adminService : AdminService, protected serviceAuthentication: AuthenticationService,private activRoute: ActivatedRoute) { 

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

    this.activRoute.paramMap.subscribe((params: ParamMap) => {
      this.bastion_id = params.get('idBastion') || '';

      this.activRoute.paramMap.subscribe((params: ParamMap) => {
        this.ressource_id = params.get('idRessource') || '';

        this.adminService.get_a_ressource(this.bastion_id, this.ressource_id).subscribe({
          next: (data : RessourceInfo) => {
            this.ressource = data;
          }
        });

        this.getListUser()

      });
    });

  }

  ajoutUser(){
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
        this.getListUser()

        
      },
      error: (e) => {
        
        console.error(e)
        this.message = "Impossible d'ajouter l'administrateur"
      }
  })

  }

  getListUser(){

    this.adminService.get_users_on_ressource(this.bastion_id, this.ressource_id).subscribe({

      next: (data : UserBastionInfo[]) => {
        
        this.listUsers = data

        
      },
      error: (e) => {
        
        console.error(e)
      },
    })

  }

  refreshList(data : string){

    this.getListUser()
  }

}
