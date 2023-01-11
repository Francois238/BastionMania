import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';

import { of, asyncScheduler, scheduled } from 'rxjs';
import { tap, map } from 'rxjs/operators';

import { AuthService } from 'src/app/core/auth';
import { Router } from '@angular/router';
import { LocalStorage } from 'src/app/core/local-storage/local-storage.service';
import { environment } from '../../../../environments/environment';
import { UserService } from '../user/user.service';

@Injectable({
  providedIn: 'root',
})
export class AuthentificationService {
  constructor(
    protected http: HttpClient,
    protected auth: AuthService,
    protected router: Router,
    protected user: UserService,
    protected localStorage: LocalStorage
  ) {}

  login(body: any) {
    return this.http.post<any>(`${environment.apiUrl}/v1/login/`, body).pipe(
      tap((resData) => {
        if (resData) {
          this.setDataAndRedirect(resData);
        }
      })
    );
  }

  setDataAndRedirect(resData: any) {
    const userId = resData.user.id;
  }

  logout() {
    return scheduled([true], asyncScheduler).subscribe(() => {
      return this.auth.disconnect();
    });
  }
}
