import { Component } from '@angular/core';
import { Router } from '@angular/router';
import { AuthenticationService } from '../authentication.service';

@Component({
  selector: 'app-user-ext-next',
  templateUrl: './user-ext-next.component.html',
  styleUrls: ['./user-ext-next.component.scss']
})
export class UserExtNextComponent {

  constructor( protected router: Router, protected serviceAuthentication: AuthenticationService) {
    
    this.serviceAuthentication.login_user_extern_next().subscribe({
      next: data => {

        this.serviceAuthentication.set_info_login(data);

        this.router.navigate(['/user/menu']);

      },

      error: err => {

        this.router.navigate(['/login']);
      }
    })
  }


}
