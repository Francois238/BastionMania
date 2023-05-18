import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { InfoLogin } from './info-login';
import { Observable, map } from 'rxjs';
import { Credentials } from './credentials';
import { Otp } from './otp';
import { sha3_512 } from 'js-sha3';
import { NewAdmin } from '../admin/new-admin';

@Injectable({
  providedIn: 'root'
})
export class AuthenticationService {

  urlBase = 'https://bastionmania.intra/api/authentication/';

  urlBaseAdmin = 'https://bastionmania.intra/api/admin-management/';

  urlBaseUser = 'https://bastionmania.intra/api/user-management/';

  constructor(private http: HttpClient) { }

  public get_hash_password(password: string): string {

    let salt = 'bastionmania.intra';

    let hash = sha3_512.create();

    let passwordSalt = password + salt;

    hash.update(passwordSalt);

    let newHash= hash.hex();

    return newHash;


  }

  public set_token(token: string): void {
    sessionStorage.setItem('token', token);
  }

  public set_info_login(infoLogin: InfoLogin){
    sessionStorage.setItem('infoLogin', JSON.stringify(infoLogin));
  }

  public get_token(): string {

    return sessionStorage.getItem('token') || '';
  }

  public get_info_login(): InfoLogin {

    return JSON.parse(sessionStorage.getItem('infoLogin') || '{}');
  }

/*************************************************************************/

/********************APPEL API POUR ADMIN ********************************/


public first_use(code: NewAdmin): Observable<any> { //envoyer le code au serveur

  const headers = { 'content-type': 'application/json'}
  const body=JSON.stringify(code);

  const url = this.urlBaseAdmin + 'premiere_utilisation';
  return this.http.post<any>(url, body ,{headers, observe: 'response'});
  }

  public login_admin(code: Credentials): Observable<any> { //envoyer le code au serveur

    const headers = { 'content-type': 'application/json'}
    const body=JSON.stringify(code);

    const url = this.urlBase + 'login/admin';
    return this.http.post<any>(url, body ,{headers, observe: 'response'})
    .pipe(
      map(response => {
        // Récupérer le header Authorization
        const authHeader = response.headers.get('Authorization');
        // Vérifier que le header est présent et contient un token
        if (authHeader) {
          const token = authHeader.split(' ')[1];

          // Stocker le token dans le session storage
          this.set_token( token);
        }

        // Retourner le corps de la réponse
        return response.body;
      })
    );
    }


  public admin_authentication_extern(){ //envoyer le code au serveur

    const url = this.urlBase + 'login/admin/extern';
    document.location=url 
    
  }

  public login_admin_extern_next(): Observable<any> { //envoyer le code au serveur


    const url = this.urlBase + 'login/admin/extern/next';
    return this.http.get<any>(url, {observe: 'response'})    
    .pipe(
      map(response => {
        // Récupérer le header Authorization
        const authHeader = response.headers.get('Authorization');
        // Vérifier que le header est présent et contient un token
        if (authHeader) {
          const token = authHeader.split(' ')[1];

          // Stocker le token dans le session storage
          this.set_token( token);
        }
        // Retourner le corps de la réponse
        return response.body;
      })
    );
    
  }

/***Activation OTP*******/

  public admin_activate_otp(): Observable<any> {

    let info = this.get_info_login();

    let id = info.id;

    const token = this.get_token();

    const headers = { 'Authorization': 'Bearer ' + token};

    const url = `${this.urlBaseAdmin}admins/${id}/otp`;

    return this.http.post<any>(url,null, {headers});

  }

  /*********Envoie OTP *****/

  public admin_send_otp(otp : Otp): Observable<any> {

    let token = this.get_token();
    const headers = { 'content-type': 'application/json',
    'Authorization': 'Bearer ' + token}
    const body=JSON.stringify(otp);

    const url = `${this.urlBase}login/admin/otp`;
    return this.http.post<any>(url, body ,{headers, observe: 'response'})
    .pipe(
      map(response => {
        // Récupérer le header Authorization
        const authHeader = response.headers.get('Authorization');
        // Vérifier que le header est présent et contient un token
        if (authHeader) {
          const token = authHeader.split(' ')[1];

          // Stocker le token dans le session storage
          this.set_token( token);
        }
        // Retourner le corps de la réponse
        return response.body;
      })
    );
    }

    
    /************************************************************************/

    /********************APPEL API POUR USER ********************************/



    public login_user(code: Credentials): Observable<any> { //envoyer le code au serveur

      const headers = { 'content-type': 'application/json'}
      const body=JSON.stringify(code);
  
      const url = this.urlBase + 'login';
      return this.http.post<any>(url, body ,{headers, observe: 'response'})
      .pipe(
        map(response => {
          // Récupérer le header Authorization
          const authHeader = response.headers.get('Authorization');
          // Vérifier que le header est présent et contient un token
          if (authHeader) {
            const token = authHeader.split(' ')[1];
  
            // Stocker le token dans le session storage
            this.set_token( token);
          }
  
          // Retourner le corps de la réponse
          return response.body;
        })
      );
      }
  
  
    public user_authentication_extern(){ //envoyer le code au serveur
  
      const url = this.urlBase + 'login/extern';
      document.location=url  
      
    }
  
    public login_user_extern_next(): Observable<any> { //envoyer le code au serveur
  
  
      const url = this.urlBase + 'login/extern/next';
      return this.http.get<any>(url, {observe: 'response'})    
      .pipe(
        map(response => {
          // Récupérer le header Authorization
          const authHeader = response.headers.get('Authorization');
          // Vérifier que le header est présent et contient un token
          if (authHeader) {
            const token = authHeader.split(' ')[1];
  
            // Stocker le token dans le session storage
            this.set_token( token);
          }
          // Retourner le corps de la réponse
          return response.body;
        })
      );
      
    }
  
  /***Activation OTP*******/
  
    public user_activate_otp(): Observable<any> {
  
      let info = this.get_info_login();
  
      let id = info.id;
  
      const token = this.get_token();
  
      const headers = { 'Authorization': 'Bearer ' + token};
  
      const url = `${this.urlBaseUser}users/${id}/otp`;
  
      return this.http.post<any>(url,null, {headers});
  
    }
  
    /*********Envoie OTP *****/
  
    public user_send_otp(otp : Otp): Observable<any> {
  
      let token = this.get_token();
      const headers = { 'content-type': 'application/json',
      'Authorization': 'Bearer ' + token}
      const body=JSON.stringify(otp);
  
      const url = `${this.urlBase}login/otp`;
      return this.http.post<any>(url, body ,{headers, observe: 'response'})
      .pipe(
        map(response => {
          // Récupérer le header Authorization
          const authHeader = response.headers.get('Authorization');
          // Vérifier que le header est présent et contient un token
          if (authHeader) {
            const token = authHeader.split(' ')[1];
  
            // Stocker le token dans le session storage
            this.set_token( token);
          }
          // Retourner le corps de la réponse
          return response.body;
        })
      );
      }
  

}
