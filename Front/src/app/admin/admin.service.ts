import { Injectable } from '@angular/core';
import { InfoLogin } from '../login/info-login';
import { HttpClient } from '@angular/common/http';
import { AdminModule } from './admin.module';
import { AdminInfo } from './admin-info';
import { Observable, map } from 'rxjs';
import { NewAdmin } from './new-admin';
import { Password } from './password';
import { UserInfo } from './user-info';
import { NewUser } from './new-user';
import jwt_decode from "jwt-decode";
import { Jwt } from '../login/jwt';
import { Router } from '@angular/router';
import { AuthenticationService } from '../login/authentication.service';

@Injectable({
  providedIn: 'root'
})
export class AdminService {

  baseUrlAdmin = 'https://bastionmania.intra/api/admin-management/';

  baseUrlUser = 'https://bastionmania.intra/api/user-management/';

  constructor(private http: HttpClient, protected router: Router, protected authenticationService: AuthenticationService) { }

/****gestion du token ********/
  public validate_token(): boolean{
      
      let token = this.authenticationService.get_token();
  
      if (token == '') {
        return false;
      }

      let data= jwt_decode(token) as Jwt;

      if (data.admin == false || data.complete_authentication == false) {
        //this.router.navigate(['/login']);
        return false;
        
      }  

      return true;
  
  }

  /********Gestion des Admins*******/

  public get_list_admin() : Observable<AdminInfo[]>{

    const token = this.authenticationService.get_token();

    const headers = {'Authorization': 'Bearer ' + token};

    const url = this.baseUrlAdmin +`admins`;
    return this.http.get<AdminInfo[]>(url, {headers})

  }

  public add_admin(admin : NewAdmin) : Observable<AdminInfo>{

    const token = this.authenticationService.get_token();

    const headers = { 'content-type': 'application/json',
    'Authorization': 'Bearer ' + token};

    const body=JSON.stringify(admin);


    const url = this.baseUrlAdmin + 'admins';
    return this.http.post<AdminInfo>(url, body ,{headers})

  }


  public delete_admin(id : string) : Observable<any>{

    const token = this.authenticationService.get_token();

    const headers = { 'Authorization': 'Bearer ' + token};

    const url = this.baseUrlAdmin + 'admins/' + id;
    return this.http.delete<any>(url, {headers})

  }


  /******gestion mot de passe de l admin************/

  public change_password(password : Password): Observable<any> {

    let infoLogin = this.authenticationService.get_info_login();

    let id = infoLogin.id;

    let token = this.authenticationService.get_token();
    const headers = { 'content-type': 'application/json',
    'Authorization': 'Bearer ' + token}
    const body=JSON.stringify(password);

    const url = `${this.baseUrlAdmin}admins/${id}`;
    return this.http.patch<any>(url, body ,{headers, observe: 'response'})
    .pipe(
      map(response => {
        // Récupérer le header Authorization
        const authHeader = response.headers.get('Authorization');
        // Vérifier que le header est présent et contient un token
        if (authHeader) {
          const token = authHeader.split(' ')[1];

          console.log("token " + token);
          // Stocker le token dans le session storage
          this.authenticationService.set_token( token);
        }
        // Retourner le corps de la réponse
        return response.body;
      })
    );
    }

    /*******Gestion utilisateurs**********/


    public get_list_user() : Observable<UserInfo[]>{

      const token = this.authenticationService.get_token();
  
      const headers = {'Authorization': 'Bearer ' + token};
  
      const url = this.baseUrlUser +`users`;
      return this.http.get<UserInfo[]>(url, {headers})
  
    }
  
    public add_user(admin : NewUser) : Observable<UserInfo>{
  
      const token = this.authenticationService.get_token();
  
      const headers = { 'content-type': 'application/json',
      'Authorization': 'Bearer ' + token};
  
      const body=JSON.stringify(admin);
  
  
      const url = this.baseUrlUser + 'users';
      return this.http.post<UserInfo>(url, body ,{headers})
  
    }


    public delete_user(id : string) : Observable<any>{

      const token = this.authenticationService.get_token();
  
      const headers = { 'Authorization': 'Bearer ' + token};
  
      const url = this.baseUrlUser + 'users/' + id;
      return this.http.delete<any>(url, {headers})
  
    }

}
