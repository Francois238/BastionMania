import { Component } from '@angular/core';
import { FormGroup, FormControl } from '@angular/forms';
import { AuthenticationService } from 'src/app/login/authentication.service';
import { AdminInfo } from '../admin-info';
import { AdminService } from '../admin.service';
import { BastionInfo } from '../bastion-info';
import { NewAdmin } from '../new-admin';
import { ActivatedRoute, ParamMap } from '@angular/router';
import { RessourceInfo } from '../ressource-info';

@Component({
  selector: 'app-list-resources',
  templateUrl: './list-resources.component.html',
  styleUrls: ['./list-resources.component.scss']
})
export class ListResourcesComponent {

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

  public bastion! : BastionInfo;

  public listRessources : Array<RessourceInfo> = new Array<RessourceInfo>();

  public bastion_id : string = '';


  constructor(protected adminService : AdminService, protected serviceAuthentication: AuthenticationService,     private activRoute: ActivatedRoute) { 

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

      this.adminService.get_a_bastion(this.bastion_id).subscribe({

        next: (data : BastionInfo) => {
          this.bastion = data;
        }

      });

      this.getListlistRessources()
    });

   
  }

  ajoutRessource(){
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
        this.getListlistRessources()

        
      },
      error: (e) => {
        
        console.error(e)
        this.message = "Impossible d'ajouter l'administrateur"
      }
  })

  }

  getListlistRessources(){

    this.adminService.get_ressources(this.bastion_id).subscribe({

      next: (data : RessourceInfo[]) => {
        
        this.listRessources = data

        
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