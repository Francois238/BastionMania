import { Injectable } from '@angular/core';
import { InfoLogin } from '../login/info-login';
import { HttpClient } from '@angular/common/http';
import { AdminModule } from './admin.module';
import { AdminInfo } from './admin-info';
import { Observable, map } from 'rxjs';
import { NewAdmin } from './new-admin';
import { Password } from './password';

@Injectable({
  providedIn: 'root'
})
export class AdminService {

  baseUrl = 'https://bastionmania.intra/api/admin-management/';

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


  public get_list_admin() : Observable<AdminInfo[]>{

    const token = this.get_token();

    const headers = {'Authorization': 'Bearer ' + token};

    const url = this.baseUrl +`admins`;
    return this.http.get<AdminInfo[]>(url, {headers})

  }

  public add_admin(admin : NewAdmin) : Observable<AdminInfo>{

    const token = this.get_token();

    const headers = { 'content-type': 'application/json',
    'Authorization': 'Bearer ' + token};

    const body=JSON.stringify(admin);


    const url = this.baseUrl + 'admins';
    return this.http.post<AdminInfo>(url, body ,{headers})

  }


  public delete_admin(id : string) : Observable<any>{

    const token = this.get_token();

    const headers = { 'Authorization': 'Bearer ' + token};

    const url = this.baseUrl + 'admins/' + id;
    return this.http.delete<any>(url, {headers})

  }

  public change_password(password : Password): Observable<any> {

    let infoLogin = this.get_info_login();

    let id = infoLogin.id;

    let token = this.get_token();
    const headers = { 'content-type': 'application/json',
    'Authorization': 'Bearer ' + token}
    const body=JSON.stringify(password);

    const url = `${this.baseUrl}admins/${id}`;
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
          this.set_token( token);
        }
        // Retourner le corps de la réponse
        return response.body;
      })
    );
    }


}
