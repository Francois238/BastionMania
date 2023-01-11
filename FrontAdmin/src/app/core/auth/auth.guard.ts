import { Injectable } from '@angular/core';
import {
  CanLoad,
  Route,
  Router,
  ActivatedRouteSnapshot,
  RouterStateSnapshot,
} from '@angular/router';
import { AuthService } from './auth.service';
import { first } from 'rxjs/operators';
import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class AuthGuard implements CanLoad {
  constructor(protected auth: AuthService, protected router: Router) {}

  canLoad(route: Route): Observable<boolean> | boolean {
    this.auth.isConnected.subscribe((con) => {
      if (!con) {
        this.router.navigate([this.auth.getLoginUrl()]);
        return false;
      }
      return this.auth.isConnected.pipe(first());
    });
    return this.auth.isConnected.pipe(first());
  }

  canActivate(
    route: ActivatedRouteSnapshot,
    state: RouterStateSnapshot
  ): Observable<boolean> | boolean {
    this.auth.isConnected.subscribe((con) => {
      if (!con) {
        this.router.navigate([this.auth.getLoginUrl()]);
        return false;
      }
      const usr = this.auth.getUserObject();
      if (usr === null || usr === undefined) {
        this.router.navigate([this.auth.getLoginUrl()]);
        return false;
      }
      /*const aRole = usr.Role;
      if (
        aRole === null ||
        aRole === undefined ||
        (aRole.level !== undefined && aRole.level < route.data['role'])
      ) {
        this.router.navigate(['/profile']);
        return false;
      }*/
      return this.auth.isConnected.pipe(first());
    });
    return this.auth.isConnected.pipe(first());
  }
}
