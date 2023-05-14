import { Component } from '@angular/core';
import { UntypedFormGroup, FormControl,FormGroup, FormBuilder } from '@angular/forms';
import { InfoLogin } from '../info-login';
import { Router } from '@angular/router';
import { Credentials } from '../credentials';

@Component({
  selector: 'app-admin',
  templateUrl: './admin.component.html',
  styleUrls: ['./admin.component.scss']
})
export class AdminComponent {


  public mail: string ='';
  public password: string ='';
  public message : string = ''
  public credentials! :Credentials
  public infoLogin! : InfoLogin
  public loginForm: FormGroup;

  constructor(private fb: FormBuilder, protected router: Router) {
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
      this.credentials = { mail : this.mail, password : this.password}

      console.log("Appuie sur le bouton envoie otp: " + this.credentials.mail + " " + this.credentials.password)

    }
  }
}
