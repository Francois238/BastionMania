import { Component } from '@angular/core';
import { FormGroup, FormBuilder } from '@angular/forms';
import { AuthenticationService } from '../authentication.service';
import { Router } from '@angular/router';
import { Credentials } from '../credentials';
import { InfoLogin } from '../info-login';
import { NewAdmin } from 'src/app/admin/new-admin';

@Component({
  selector: 'app-installation',
  templateUrl: './installation.component.html',
  styleUrls: ['./installation.component.scss']
})
export class InstallationComponent {

  public name : string ='';
  public last_name : string ='';
  public mail: string ='';
  public password: string ='';
  public message : string = ''
  public credentials! :NewAdmin
  public infoLogin! : InfoLogin
  public loginForm: FormGroup;

  constructor(private fb: FormBuilder, protected router: Router, protected serviceAuthentication: AuthenticationService) {
    this.loginForm = this.fb.group({
      last_name: [''],
      name: [''],
      mail: [''],
      password: ['']
    });
  }


  add() {
    this.message = '';

    this.name = this.loginForm.value.name as string

    this.last_name = this.loginForm.value.last_name as string

    this.mail = this.loginForm.value.mail as string

    this.password = this.loginForm.value.password as string

    this.name = this.name.trim();
    this.last_name = this.last_name.trim();
    this.mail = this.mail.trim();
    this.password = this.password.trim();

    if (this.last_name== '' || this.name=='' || this.mail == '' || this.password =='') {
      this.message = "Il y a un champ vide";

    }

    else{
      this.credentials = {name: this.name, last_name: this.last_name , mail : this.mail, password : this.serviceAuthentication.get_hash_password(this.password)}

      this.serviceAuthentication.first_use(this.credentials).subscribe({
        next: (data: any)=> {

          this.router.navigate(['/login/admin']);
        },
  
        error: err => {
  
          if(err.status <500){
            this.message = "Vous n'avez pas les droits";
          }
  
          else{
            this.message = 'Erreur interne';
          }
        }
      })

    }
  }

}