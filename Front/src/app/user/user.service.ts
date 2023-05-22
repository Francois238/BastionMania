import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Router } from '@angular/router';
import { AuthenticationService } from '../login/authentication.service';
import jwt_decode from "jwt-decode";
import { Jwt } from '../login/jwt';

@Injectable({
  providedIn: 'root'
})
export class UserService {

  constructor(private http: HttpClient, protected router: Router, protected authenticationService: AuthenticationService) { }

  baseUrlUser = 'https://bastionmania.intra/api/user-management/';

  /****gestion du token ********/
  public validate_token(): boolean{
      
    let token = this.authenticationService.get_token();

    if (token == '') {
      return false;
    }

    let data= jwt_decode(token) as Jwt;

    if (data.complete_authentication == false) {
      //this.router.navigate(['/login']);
      return false;
      
    }  

    return true;

}
}
