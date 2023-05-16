import { Component } from '@angular/core';
import { Router } from '@angular/router';
import { AuthenticationService } from '../authentication.service';

@Component({
  selector: 'app-admin-choice',
  templateUrl: './admin-choice.component.html',
  styleUrls: ['./admin-choice.component.scss']
})
export class AdminChoiceComponent {

  message='';

  constructor( protected router: Router, protected serviceAuthentication: AuthenticationService) {}

  goOTP(){
    this.router.navigate(['/login/admin/activate_otp']);
  }

  keycloak(){

    this.serviceAuthentication.admin_enable_keycloak().subscribe({
      next: data => {
          
          this.serviceAuthentication.admin_authentication_extern()
  
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
