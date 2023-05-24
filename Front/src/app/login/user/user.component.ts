import { Component } from '@angular/core';
import { FormGroup, FormBuilder } from '@angular/forms';
import { Router } from '@angular/router';
import { Credentials } from '../credentials';
import { InfoLogin } from '../info-login';
import { AuthenticationService } from '../authentication.service';

@Component({
  selector: 'app-user',
  templateUrl: './user.component.html',
  styleUrls: ['./user.component.scss']
})
export class UserComponent {

  public mail: string ='';
  public password: string ='';
  public message : string = ''
  public credentials! :Credentials
  public infoLogin! : InfoLogin
  public loginForm: FormGroup;

  constructor(private fb: FormBuilder, protected router: Router, protected serviceAuthentication: AuthenticationService) {
    this.loginForm = this.fb.group({
      mail: [''],
      password: ['']
    });
  }


  login() {
    this.message = '';

    this.mail = this.loginForm.value.mail as string

    this.password = this.loginForm.value.password as string

    this.mail = this.mail.trim();
    this.password = this.password.trim();

    if (this.mail == '' || this.password =='') {
      this.message = "Mail ou mot de passe invalide";

    }

    else{
      this.credentials = { mail : this.mail, password : this.serviceAuthentication.get_hash_password(this.password)}

      this.serviceAuthentication.login_user(this.credentials).subscribe({
        next: (data: InfoLogin)=> {
  
          this.serviceAuthentication.set_info_login(data);

          if (!data.otpactive) { //1ere connexion
            this.router.navigate(['/login/activate_otp']);
          }

          else{
            this.router.navigate(['/login/otp']);
          }
  
        },
  
        error: err => {
  
          if(err.status <500){
            this.message = err.error.message;
          }
  
          else{
            this.message = 'Erreur interne';
          }
        }
      })

    }
  }


  loginExtern() {

    this.serviceAuthentication.user_authentication_extern()
      
  }
}
