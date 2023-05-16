import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { InfoLogin } from './info-login';
import { Observable, map } from 'rxjs';
import { Credentials } from './credentials';
import { Otp } from './otp';

@Injectable({
  providedIn: 'root'
})
export class AuthenticationService {

  urlBase = 'https://bastionmania.intra/api/authentication/';

  urlBaseAdmin = 'https://bastionmania.intra/api/admin-management/';

  constructor(private http: HttpClient) { }

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


  /*******Appel Api pour Admin *********/

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

          console.log("token " + token);
          // Stocker le token dans le session storage
          this.set_token( token);
        }

        else{
          console.log("erreur")
        }
        // Retourner le corps de la réponse
        return response.body;
      })
    );
    }


  public admin_authentication_extern(){ //envoyer le code au serveur

    const url = this.urlBase + 'login/admin/extern';
    document.location=url 
   // window.open(url, "_blank");

    
    
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

          console.log("token " + token);
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

    console.log("url " + url);

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

          console.log("token " + token);
          // Stocker le token dans le session storage
          this.set_token( token);
        }
        // Retourner le corps de la réponse
        return response.body;
      })
    );
    }

    /******Activation Keycloak *******/
    public admin_enable_keycloak(): Observable<any> {
      const token = this.get_token();
  
      console.log("token " + token)
  
      const headers = {'Authorization': 'Bearer ' + token};
  
      const url = `${this.urlBase}login/admin/enable_extern`;
  
      return this.http.patch<any>(url, {headers});
  
    }


}
