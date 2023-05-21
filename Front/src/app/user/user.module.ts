import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { UserRoutingModule } from './user-routing.module';
import { UserComponent } from './user.component';
import { MenuComponent } from './menu/menu.component';
import { ProfilComponent } from './profil/profil.component';
import { RessourcesComponent } from './ressources/ressources.component';
import { BastiondetailComponent } from './bastiondetail/bastiondetail.component';
import { BastionsComponent } from './bastions/bastions.component';
import {MatInputModule} from "@angular/material/input";
import {FormsModule} from "@angular/forms";


@NgModule({
  declarations: [
    UserComponent,
    MenuComponent,
    ProfilComponent,
    RessourcesComponent,
    BastiondetailComponent,
    BastionsComponent
  ],
  imports: [
    CommonModule,
    UserRoutingModule,
    MatInputModule,
    FormsModule
  ]
})
export class UserModule { }
