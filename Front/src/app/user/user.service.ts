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
import { RessourceCredentialSsh } from './ressource-credential-ssh';
import { RessourceCredentialWireguard } from './ressource-credential-wireguard';

@Injectable({
  providedIn: 'root'
})
export class UserService {

  constructor(private http: HttpClient, protected router: Router, protected authenticationService: AuthenticationService) { }

  baseUrlUser = 'https://bastionmania.intra/api/user-management/';

  baseUrlBastion = 'https://bastionmania.intra/api/';

  ressource!: RessourceInfo;

  /***********Gestion Ressource*************/
    public set_ressource(ressource: RessourceInfo){
      sessionStorage.setItem('ressource', JSON.stringify(ressource));
    }
  
  
    public get_ressource(): RessourceInfo {
  
      return JSON.parse(sessionStorage.getItem('ressource') || '{}');
    }

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

public get_bastions() : Observable<any>{

  const token = this.authenticationService.get_token();

  const headers = {'Authorization': 'Bearer ' + token};

  const url = this.baseUrlBastion +`bastions`;
  return this.http.get<any>(url, {headers})

}

public get_a_bastion(bastion_id: string) : Observable<any>{

  const token = this.authenticationService.get_token();

  const headers = {'Authorization': 'Bearer ' + token};

  const url = this.baseUrlBastion +`bastions/${bastion_id}`;
  return this.http.get<any>(url, {headers})

}

public get_ressources(bastion_id: string) : Observable<any>{

  const token = this.authenticationService.get_token();

  const headers = {'Authorization': 'Bearer ' + token};

  const url = this.baseUrlBastion +`bastions/${bastion_id}/ressources`;
  return this.http.get<any>(url, {headers})

}


public generate_ssh_access(bastion_id: string, ressource_id: string, ssh: RessourceCredentialSsh) : Observable<any>{

  const token = this.authenticationService.get_token();

  const headers = {'Authorization': 'Bearer ' + token};

  const body=JSON.stringify(ssh);

  const url = this.baseUrlBastion +`bastions/${bastion_id}/ressources/${ressource_id}/getressourcecredentials/ssh`;
  return this.http.post<any>(url, body, {headers})

}

public generate_wireguard_access(bastion_id: string, ressource_id: string, wireguard: RessourceCredentialWireguard) : Observable<any>{

  const token = this.authenticationService.get_token();

  const headers = {'Authorization': 'Bearer ' + token};

  const body=JSON.stringify(wireguard);

  const url = this.baseUrlBastion +`bastions/${bastion_id}/ressources/${ressource_id}/getressourcecredentials/wireguard`;
  return this.http.post<any>(url, body, {headers})

}

public start_session(bastion_id: string, ressource_id: string): Observable<any> {

  const token = this.authenticationService.get_token();

  const headers = {'Authorization': 'Bearer ' + token};


  const url = this.baseUrlBastion +`bastions/${bastion_id}/ressources/${ressource_id}/startsession`;

  return this.http.post<any>(url,null, {headers});

}

public stop_session(bastion_id: string, ressource_id: string): Observable<any> {

  const token = this.authenticationService.get_token();

  const headers = {'Authorization': 'Bearer ' + token};


  const url = this.baseUrlBastion +`bastions/${bastion_id}/ressources/${ressource_id}/stopsession`;

  return this.http.post<any>(url,null, {headers});

}

}
