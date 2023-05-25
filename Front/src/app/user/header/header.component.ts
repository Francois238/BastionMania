import { Component } from '@angular/core';

@Component({
  selector: 'app-header',
  templateUrl: './header.component.html',
  styleUrls: ['./header.component.scss']
})
export class HeaderComponent {

  activeBastion = "";
  activeProfil = "";

  constructor() {}
  on_click_bastion() {

    this.activeBastion = "active";
    this.activeProfil = "";
  }

  on_click_profil() {

    this.activeBastion = "";
    this.activeProfil = "active";
  }
}