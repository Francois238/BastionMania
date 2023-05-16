import { Component, OnInit } from '@angular/core';
import { NewUser } from '../new-user';
import { FormGroup, FormControl } from '@angular/forms';
import { UserInfo } from '../user-info';
import { AdminService } from '../admin.service';

@Component({
  selector: 'app-list-user',
  templateUrl: './list-user.component.html',
  styleUrls: ['./list-user.component.scss']
})
export class ListUserComponent implements OnInit {


  public name: string ='';
  public last_name: string ='';
  public mail: string ='';
  public password: string ='';
  public message: string ='';
  public user! : NewUser
  public ajoutForm: FormGroup;
  public nameCrtl: FormControl;
  public last_nameCrtl: FormControl;
  public mailCrtl: FormControl;
  public passwordCrtl: FormControl;

  public listUsers : Array<UserInfo> = new Array<UserInfo>();

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

    this.getListUser()
  }

  ajoutUser(){
    this.message = '';

    this.name = this.nameCrtl.value.trim();
    this.last_name = this.last_nameCrtl.value.trim();
    this.mail = this.mailCrtl.value.trim();
    this.password = this.passwordCrtl.value.trim();

    this.user = {
      name : this.name,
      last_name : this.last_name,
      mail : this.mail,
      password : this.password
    }

    this.adminService.add_user(this.user).subscribe({
      next: (data : UserInfo) => {
        
        this.message="L'utilisateur a bien été ajouté"
        this.getListUser()

        
      },
      error: (e) => {
        
        console.error(e)
        this.message = "Impossible d'ajouter l'utilisateur"
      }
  })

  }

  getListUser(){

    this.adminService.get_list_user().subscribe({

      next: (data : UserInfo[]) => {
        
        this.listUsers = data

        
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

    this.getListUser()
  }

}
