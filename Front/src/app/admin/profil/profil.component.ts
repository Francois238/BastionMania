import { Component, OnInit } from '@angular/core';
import { AdminInfo } from '../admin-info';
import { AdminService } from '../admin.service';
import { FormGroup, FormControl } from '@angular/forms';
import { Password } from '../password';
import { InfoLogin } from 'src/app/login/info-login';

@Component({
  selector: 'app-profil',
  templateUrl: './profil.component.html',
  styleUrls: ['./profil.component.scss']
})
export class ProfilComponent implements OnInit {

  public password: string ='';
  public message: string ='';
  public admin! : AdminInfo
  public passwordForm: FormGroup;
  public passwordCrtl: FormControl;
  public passwordEnvoye! : Password
  public change : boolean = false
  public infoLogin! : InfoLogin
  public disabled=true

  constructor(protected adminService : AdminService) { 
    this.passwordCrtl = new FormControl('')
    this.passwordForm = new FormGroup({
        password: this.passwordCrtl,

    })

    this.infoLogin = this.adminService.get_info_login()

    if (this.infoLogin.change != null){ //si l utilisateur utilise SSO ou pas
      this.disabled = false
    }
    else{
      this.disabled = true
    }

  }

  ngOnInit(): void {

    this.passwordCrtl = new FormControl('')
    this.passwordForm = new FormGroup({
        password: this.passwordCrtl,

    })

    this.admin = this.adminService.get_info_login()

    //this.admin = {id : 1, name : "bob", last_name : "bastion", mail:"bob.bastion", change : false, otpactive : false}
  }

  changePwd(){

    this.password = this.passwordCrtl.value.trim()

    if(this.password.length >2 ){

      this.passwordEnvoye = { password : this.password}
      this.adminService.change_password(this.passwordEnvoye).subscribe({
        next: (data : AdminInfo) => {
  
          this.message = "Votre mot de passe a bien été changé"
          
          this.admin = data
          console.log("Voici les donnees de l admin : " + this.admin)
  
          this.passwordCrtl = new FormControl('')
          this.passwordForm = new FormGroup({
              password: this.passwordCrtl,
      
          })
  
          this.change = true
  
        },
        error: (e) => {
          
          console.error(e)
          this.message = "Une erreur est survenue, votre mot de passe n'a pas été modifié"
        },
    })
    }

    else{
      this.message = "Mot de passe trop court"
    }
    

  }

}
