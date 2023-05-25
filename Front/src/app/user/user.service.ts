import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Router } from '@angular/router';
import { AuthenticationService } from '../login/authentication.service';
import jwt_decode from "jwt-decode";
import { Jwt } from '../login/jwt';
import { Observable, map } from 'rxjs';
import { Password } from './password';
import { BastionInfo } from './bastion-info';
import { RessourceInfo } from './ressource-info';

@Injectable({
  providedIn: 'root'
})
export class UserService {

  constructor(private http: HttpClient, protected router: Router, protected authenticationService: AuthenticationService) { }

  baseUrlUser = 'https://bastionmania.intra/api/user-management/';

  baseUrlBastion = 'https://bastionmania.intra/api/bastion-management/';

  /****gestion du token ********/
  public validate_token(): boolean{

    let token = this.authenticationService.get_token();

    if (token == '') {
      return false;
    }

    let data= jwt_decode(token) as Jwt;

    if (data.admin == false) {
      //this.router.navigate(['/login']);
      return false;

    }

    return true;

}


public change_password(password : Password): Observable<any> {

  let infoLogin = this.authenticationService.get_info_login();

  let id = infoLogin.id;

  let token = this.authenticationService.get_token();
  const headers = { 'content-type': 'application/json',
  'Authorization': 'Bearer ' + token}
  const body=JSON.stringify(password);

  const url = `${this.baseUrlUser}users/${id}`;
  return this.http.patch<any>(url, body ,{headers, observe: 'response'})
  .pipe(
    map(response => {
      // Récupérer le header Authorization
      const authHeader = response.headers.get('Authorization');
      // Vérifier que le header est présent et contient un token
      if (authHeader) {
        const token = authHeader.split(' ')[1];

        // Stocker le token dans le session storage
        this.authenticationService.set_token( token);
      }
      // Retourner le corps de la réponse
      return response.body;
    })
  );
  }



/***************************************/
/***********Gestion Bastion*************/
/***************************************/

public get_bastions() : Observable<BastionInfo[]>{

  const token = this.authenticationService.get_token();

  const headers = {'Authorization': 'Bearer ' + token};

  const url = this.baseUrlBastion +`bastions`;
  return this.http.get<BastionInfo[]>(url, {headers})

}

public get_a_bastion(bastion_id: string) : Observable<BastionInfo>{

  const token = this.authenticationService.get_token();

  const headers = {'Authorization': 'Bearer ' + token};

  const url = this.baseUrlBastion +`bastions/${bastion_id}`;
  return this.http.get<BastionInfo>(url, {headers})

}

public get_ressources(bastion_id: string) : Observable<RessourceInfo[]>{

  const token = this.authenticationService.get_token();

  const headers = {'Authorization': 'Bearer ' + token};

  const url = this.baseUrlBastion +`bastions/${bastion_id}/resources`;
  return this.http.get<RessourceInfo[]>(url, {headers})

}

public get_a_ressource(bastion_id: string, ressource_id: string) : Observable<RessourceInfo>{

  const token = this.authenticationService.get_token();

  const headers = {'Authorization': 'Bearer ' + token};

  const url = this.baseUrlBastion +`bastions/${bastion_id}/resources${ressource_id}`;
  return this.http.get<RessourceInfo>(url, {headers})

}



}
