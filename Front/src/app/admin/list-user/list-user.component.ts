import { Component, OnInit } from '@angular/core';
import { NewUser } from '../new-user';
import { FormGroup, FormControl } from '@angular/forms';
import { UserInfo } from '../user-info';
import { AdminService } from '../admin.service';
import { AuthenticationService } from 'src/app/login/authentication.service';

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
  public userCrtl: FormControl;
  public searchForm: FormGroup;

  public listUsers : Array<UserInfo> = new Array<UserInfo>();

  constructor(protected adminService : AdminService, protected serviceAuthentication: AuthenticationService) { 
    this.adminService.validate_token();

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
    this.userCrtl = new FormControl('')
    this.searchForm = new FormGroup({
      mailSearch: this.userCrtl,
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
    this.userCrtl = new FormControl('')
    this.searchForm = new FormGroup({
      mailSearch: this.userCrtl,
    })

    this.getListUser()
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

    this.user = {
      name : this.name,
      last_name : this.last_name,
      mail : this.mail,
      password : this.serviceAuthentication.get_hash_password(this.password)
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

  }

  refreshList(data : string){

    this.getListUser()
  }

  searchUser(){

    let mailForm = this.userCrtl.value as string

    let mail = mailForm.trim();

    this.adminService.get_user_mail(mail).subscribe({

      next: (data : UserInfo[]) => {
          
          this.listUsers = data
      
        },
         error: (e) => {
        
          console.error(e)
        }
      })


  }

}
