import { Component } from '@angular/core';

@Component({
  selector: 'app-header',
  templateUrl: './header.component.html',
  styleUrls: ['./header.component.scss']
})
export class HeaderComponent {

  activeAdmin = "";
  activeUser = "";
  activeBastion = "";
  activeProfil = "";

  constructor() {}

  on_click_admin() {
    this.activeAdmin = "active";
    this.activeUser = "";
    this.activeBastion = "";
    this.activeProfil = "";
  }

  on_click_user() {
    this.activeAdmin = "";
    this.activeUser = "active";
    this.activeBastion = "";
    this.activeProfil = "";
  }

  on_click_bastion() {
    this.activeAdmin = "";
    this.activeUser = "";
    this.activeBastion = "active";
    this.activeProfil = "";
  }

  on_click_profil() {
    this.activeAdmin = "";
    this.activeUser = "";
    this.activeBastion = "";
    this.activeProfil = "active";
  }
}
