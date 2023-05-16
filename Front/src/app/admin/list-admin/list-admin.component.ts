import { Component, OnInit } from '@angular/core';
import { NewAdmin } from '../new-admin';
import { FormGroup, FormControl } from '@angular/forms';
import { AdminInfo } from '../admin-info';
import { AdminService } from '../admin.service';

@Component({
  selector: 'app-list-admin',
  templateUrl: './list-admin.component.html',
  styleUrls: ['./list-admin.component.scss']
})
export class ListAdminComponent implements OnInit {


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

  public listAdmins : Array<AdminInfo> = new Array<AdminInfo>();

  constructor(protected adminService : AdminService) { 
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

    this.getListAdmin()
  }

  ajoutAdmin(){
    this.message = '';

    this.name = this.nameCrtl.value.trim();
    this.last_name = this.last_nameCrtl.value.trim();
    this.mail = this.mailCrtl.value.trim();
    this.password = this.passwordCrtl.value.trim();

    this.admin = {
      name : this.name,
      last_name : this.last_name,
      mail : this.mail,
      password : this.password
    }

    this.adminService.add_admin(this.admin).subscribe({
      next: (data : AdminInfo) => {
        
        this.message="L'administrateur a bien été ajouté"
        this.getListAdmin()

        
      },
      error: (e) => {
        
        console.error(e)
        this.message = "Impossible d'ajouter l'administrateur"
      }
  })

  }

  getListAdmin(){

    this.adminService.get_list_admin().subscribe({

      next: (data : AdminInfo[]) => {
        
        this.listAdmins = data

        
      },
      error: (e) => {
        
        console.error(e)
      },
    })

    /*this.listAdmins = [{id:1, name : "bob", last_name:"bastion", mail:"bob.bastion@bastionmania.fr", change:false, otpactive:false},
                        {id:2, name : "francois", last_name:"benet", mail:"francois.benet@bastionmania.fr", change:false, otpactive:false}
                      ]*/
  }

  refreshList(data : string){

    this.getListAdmin()
  }

}
