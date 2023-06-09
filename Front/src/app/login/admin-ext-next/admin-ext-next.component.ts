import { Component } from '@angular/core';
import { Router } from '@angular/router';
import { AuthenticationService } from '../authentication.service';
import { timer } from 'rxjs';

@Component({
  selector: 'app-admin-ext-next',
  templateUrl: './admin-ext-next.component.html',
  styleUrls: ['./admin-ext-next.component.scss']
})
export class AdminExtNextComponent {

  constructor( protected router: Router, protected serviceAuthentication: AuthenticationService) {
    
    this.serviceAuthentication.login_admin_extern_next().subscribe({
      next: data => {

        this.serviceAuthentication.set_info_login(data);

        this.router.navigate(['/admin/menu']);

      },

      error: err => {

        timer(3000).subscribe({ next: () => { this.router.navigate(['/login']);}
          
        });
      }
    })
  }


  

}
