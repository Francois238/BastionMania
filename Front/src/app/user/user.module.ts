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
import {ReactiveFormsModule} from "@angular/forms";
import { RessourceItemComponent } from './ressource-item/ressource-item.component';
import { HeaderComponent } from './header/header.component';
import { RessourcePageComponent } from './ressource-page/ressource-page.component';


@NgModule({
  declarations: [
    UserComponent,
    MenuComponent,
    ProfilComponent,
    RessourcesComponent,
    BastiondetailComponent,
    BastionsComponent,
    RessourceItemComponent,
    HeaderComponent,
    RessourcePageComponent
  ],
  imports: [
    CommonModule,
    UserRoutingModule,
    MatInputModule,
    ReactiveFormsModule
  ]
})
export class UserModule { }
