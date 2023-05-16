import { Component } from '@angular/core';
import { Url } from '../url';
import { Router } from '@angular/router';
import { AuthenticationService } from '../authentication.service';

@Component({
  selector: 'app-admin-activate-otp',
  templateUrl: './admin-activate-otp.component.html',
  styleUrls: ['./admin-activate-otp.component.scss']
})
export class AdminActivateOtpComponent {

  public code : string =''
  public error : string = ''

  constructor(protected router: Router, protected authenticationService : AuthenticationService) { }


  getOtp(){

    this.authenticationService.admin_activate_otp().subscribe({
      next: (data : Url) => {
        
        this.code = data.url
        console.log("Voici les donnees de l admin : " + this.code)
        
      },
      error: (e) => {
        
        console.error(e)
        this.error = "Vous avez deja la double authentification activée"
      },
  })

  }

  nextPage(){

      this.router.navigate(['/login/admin/otp']); // on va venir finir sa connexion
    }

  }