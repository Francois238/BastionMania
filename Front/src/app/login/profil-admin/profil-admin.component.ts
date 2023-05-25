import { Component, OnInit } from '@angular/core';
import { FormGroup, FormControl } from '@angular/forms';
import { AdminInfo } from 'src/app/admin/admin-info';
import { AdminService } from 'src/app/admin/admin.service';
import { Password } from 'src/app/admin/password';
import { AuthenticationService } from '../authentication.service';
import { InfoLogin } from '../info-login';
import { Router } from '@angular/router';

@Component({
  selector: 'app-profil-admin',
  templateUrl: './profil-admin.component.html',
  styleUrls: ['./profil-admin.component.scss']
})
export class ProfilAdminComponent implements OnInit {

  public password: string ='';
  public message: string ='';
  public admin! : AdminInfo
  public passwordForm: FormGroup;
  public passwordCrtl: FormControl;
  public passwordEnvoye! : Password
  public change : boolean = false
  public infoLogin! : InfoLogin

  constructor(protected adminService : AdminService, protected authenticationService: AuthenticationService, protected router: Router) { 
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

    this.admin = this.authenticationService.get_info_login()

  }

  changePwd(){

    this.password = this.passwordCrtl.value.trim()

    if(this.password.length >2 ){

      this.passwordEnvoye = { password : this.authenticationService.get_hash_password(this.password)}
      this.adminService.change_password(this.passwordEnvoye).subscribe({
        next: (_data : AdminInfo) => {
  
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

  menu(){
    this.router.navigate(['/admin/menu'])
  }

}
