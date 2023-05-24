import { Component } from '@angular/core';
import { Router } from '@angular/router';
import { AuthenticationService } from '../authentication.service';
import { Url } from '../url';

@Component({
  selector: 'app-user-activate-otp',
  templateUrl: './user-activate-otp.component.html',
  styleUrls: ['./user-activate-otp.component.scss']
})
export class UserActivateOtpComponent {

  public code : string =''
  public error : string = ''

  constructor(protected router: Router, protected authenticationService : AuthenticationService) { }


  getOtp(){

    this.authenticationService.user_activate_otp().subscribe({
      next: (data : Url) => {
        
        this.code = data.url
        
      },
      error: (e) => {
        
        this.error = "Vous avez deja la double authentification activée"
      },
  })

  }

  nextPage(){

      this.router.navigate(['/login/otp']); // on va venir finir sa connexion
    }

}
