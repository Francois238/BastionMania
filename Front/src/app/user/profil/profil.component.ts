import { Component, OnInit } from '@angular/core';
import { UserInfo } from '../user-info';
import { UserService } from '../user.service';
import { Password } from '../password';
import { FormGroup, FormControl } from '@angular/forms';
import { AuthenticationService } from 'src/app/login/authentication.service';
import { InfoLogin } from 'src/app/login/info-login';

@Component({
  selector: 'app-profil',
  templateUrl: './profil.component.html',
  styleUrls: ['./profil.component.scss']
})
export class ProfilComponent implements OnInit {

  public password: string ='';
  public message: string ='';
  public user! : UserInfo
  public passwordForm: FormGroup;
  public passwordCrtl: FormControl;
  public passwordEnvoye! : Password
  public change : boolean = false
  public infoLogin! : InfoLogin

  constructor(protected userService : UserService, protected authenticationService: AuthenticationService) { 
    this.passwordCrtl = new FormControl('')
    this.passwordForm = new FormGroup({
        password: this.passwordCrtl,

    })

    this.infoLogin = this.authenticationService.get_info_login()

    if (this.infoLogin.change != null){ //si l utilisateur utilise SSO ou pas
      this.change = true
    }
    else{
      this.change = false
    }

  }

  ngOnInit(): void {

    this.passwordCrtl = new FormControl('')
    this.passwordForm = new FormGroup({
        password: this.passwordCrtl,

    })

    this.user = this.authenticationService.get_info_login()

  }

  changePwd(){

    this.password = this.passwordCrtl.value.trim()

    if(this.password.length >2 ){

      this.passwordEnvoye = { password : this.authenticationService.get_hash_password(this.password)}
      this.userService.change_password(this.passwordEnvoye).subscribe({
        next: (_data : UserInfo) => {
  
          this.message = "Votre mot de passe a bien été changé"
          
  
          this.passwordCrtl = new FormControl('')
          this.passwordForm = new FormGroup({
              password: this.passwordCrtl,
      
          })

  
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