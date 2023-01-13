import { Injectable, Inject, PLATFORM_ID } from '@angular/core';
import { ReplaySubject } from 'rxjs';
import { isPlatformBrowser } from '@angular/common';
import { LocalStorage } from '../local-storage/local-storage.service';

@Injectable({
  providedIn: 'root',
})
export class AuthService {
  public static roles = {
    MEMBER: 20,
    ADMIN: 80,
  };
  public tokenValue: string | null = null;
  public isConnected = new ReplaySubject<boolean>(0);
  public userObject$ = new ReplaySubject<any>(0);

  private readonly tokenKey: string = 'access_token';
  private readonly userKey: string = 'user_id';
  private readonly expireTime: string = 'expire';
  private readonly profil = 'role';

  constructor(
    @Inject(PLATFORM_ID) protected plateformID: any,
    protected localSave: LocalStorage
  ) {
    if (isPlatformBrowser(this.plateformID)) {
      this.tokenValue = this.localSave.getItem(this.tokenKey);
    }

    if (!!this.getToken()) {
      this.userObject$.next(this.getUserObject());
    }
    this.isConnected.next(!!this.getToken());
  }

  getToken(): string | null {
    return this.tokenValue;
  }

  getUserObject(): any {
    return this.localSave.getItem(this.userKey);
  }

  setUserObject(user: any) {
    this.localSave.setItem(this.userKey, user);
  }

  connect(token: string, userId?: any, expires?: string) {
    this.localSave.setItem(this.tokenKey, token);
    this.tokenValue = token;
    if (userId !== undefined) {
      this.localSave.setItem(this.userKey, userId);
    }
    if (expires !== undefined) {
      this.localSave.setItem(this.expireTime, expires);
    }
    this.isConnected.next(true);
    this.userObject$.next(userId);
  }

  setProfil(profil: any) {
    this.localSave.setItem(this.profil, profil);
  }

  getProfil() {
    return this.localSave.getItem(this.profil);
  }

  getHomeUrl() {
    return '/';
  }

  getLoginUrl() {
    return '/login';
  }

  disconnect() {
    this.isConnected.next(false);
    this.userObject$.next(null);
    this.localSave.removeItem(this.tokenKey);
    this.localSave.removeItem(this.userKey);
    this.localSave.removeItem(this.profil);
    this.localSave.removeItem(this.expireTime);
    this.tokenValue = null;
  }
}
