import { Component } from '@angular/core';
import { Router } from '@angular/router';
import { NgbAlertModule } from '@ng-bootstrap/ng-bootstrap';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.scss'],
})
export class LoginComponent {
  public login: string;
  public pwd: string;
  public error: string;
  public success: string;

  constructor(protected router: Router) {
    this.login = '';
    this.pwd = '';
    this.error = '';
    this.success = '';
  }

  authenticate() {
    this.error = '';
    if (this.login == '' || this.pwd == '') {
      this.error = "L'indentifiant et le mot de passe de doivent pas etre vide";
      return;
    }

    if (this.login == 'hanae' && this.pwd == 'test') {
      this.success = 'Authentification réussi';
      this.router.navigate(['/bastionlist']);
    } else {
      this.error = 'Indentifiant ou mot de passe invalide !!';
    }
  }
}
