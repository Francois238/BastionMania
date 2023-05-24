import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { UserRoutingModule } from './user-routing.module';
import { UserComponent } from './user.component';
import { MenuComponent } from './menu/menu.component';
import { ProfilComponent } from './profil/profil.component';


@NgModule({
  declarations: [
    UserComponent,
    MenuComponent,
    ProfilComponent
  ],
  imports: [
    CommonModule,
    UserRoutingModule
  ]
})
export class UserModule { }
