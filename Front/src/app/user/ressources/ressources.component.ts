import { Component } from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Router} from "@angular/router";

@Component({
  selector: 'app-ressources',
  templateUrl: './ressources.component.html',
  styleUrls: ['./ressources.component.scss']
})
export class RessourcesComponent {
  constructor(private http: HttpClient, private router: Router) {
  }
  submitForm() {
    const uploadRes = this.http.post('http://localhost:8740/user/bastions/:bastion_id/ressources/:ressource_id', {observe: 'response'});
    uploadRes.subscribe((res: any) => {
      console.log(res);
      if (res.status === 200) {
        this.loadurl()
      }
    });


  }

  loadurl() {
    this.router.navigate(['/ressources/:ressource_id'])
  }

  //pas fonctionnel ne pas chercher la logique
}
