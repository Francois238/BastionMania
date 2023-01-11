import { Injectable, Inject, PLATFORM_ID } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';

@Injectable({ providedIn: 'root' })
export class LocalStorage {
  constructor(@Inject(PLATFORM_ID) protected platformId: any) {}

  getItem<T = any>(index: string): null | T {
    let value = null;

    if (isPlatformBrowser(this.platformId)) {
      try {
        let unparsedValue = null;
        unparsedValue = localStorage.getItem(index);
        value = unparsedValue ? (JSON.parse(unparsedValue) as T) : null;
      } catch (error) {
        value = null;
      }
    }

    return value;
  }

  setItem(index: string, value: any): void {
    if (isPlatformBrowser(this.platformId)) {
      localStorage.setItem(index, JSON.stringify(value));
    }
  }

  removeItem(index: string): void {
    if (isPlatformBrowser(this.platformId)) {
      localStorage.removeItem(index);
    }
  }

  clear(): void {
    if (isPlatformBrowser(this.platformId)) {
      localStorage.clear();
    }
  }
}
