import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root',
})
export class Utilities {
  protected lang: string;
  constructor() {
    this.lang = 'fr';
  }

  /**
   * Generates a random string from alphanumeric characters
   *
   * @param lengthCode - Numer of element the code generated should have
   *
   * @returns String of generated value
   */
  public generateRandomString(lengthCode = 2) {
    const options = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890';
    let text = '';
    for (let i = 0; i < lengthCode; i++) {
      text += options.charAt(Math.floor(Math.random() * options.length));
    }

    return text;
  }

  /**
   * Count the occurence of a value in an array
   *
   * @param array Array of elements to search in
   * @param what Element to look in the array
   *
   * @returns number of time what appears in array
   */
  public countInArray(array: any[], what: any) {
    return array.filter((item) => item === what).length;
  }

  public dateAgo(value: any, lang = 'fr') {
    let intervals: any;
    if (value) {
      const seconds = Math.floor((+new Date() - +new Date(value)) / 1000);
      if (seconds < 86400) {
        // less than 1 day ago will show as 'today'
        if (lang === 'fr') {
          return "aujourd'hui";
        }
        return 'today';
      }

      if (lang === 'fr') {
        intervals = {
          année: 31536000,
          mois: 2592000,
          semaine: 604800,
          jour: 86400,
          heure: 3600,
          minute: 60,
          seconde: 1,
        };
      } else {
        intervals = {
          year: 31536000,
          month: 2592000,
          week: 604800,
          day: 86400,
          hour: 3600,
          minute: 60,
          second: 1,
        };
      }

      let counter;
      for (const i in intervals) {
        if (intervals[i] > 0) {
          counter = Math.floor(seconds / intervals[i]);
          if (counter > 0) {
            if (counter === 1 || (counter > 1 && i === 'mois')) {
              return `${counter} ${i}${lang !== 'fr' ? ' ago' : ' auparavant'}`; // singular (1 day ago)
            } else {
              return `${counter} ${i}${
                lang !== 'fr' ? 's ago' : 's auparavant'
              }`; // plural (2 days ago)
            }
          }
        }
      }
    }
    return value;
  }
}
