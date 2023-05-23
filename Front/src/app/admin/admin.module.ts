import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { AdminRoutingModule } from './admin-routing.module';
import { AdminComponent } from './admin.component';
import { MenuComponent } from './menu/menu.component';
import { ListAdminComponent } from './list-admin/list-admin.component';
import { ReactiveFormsModule } from '@angular/forms';
import { AdminDetailComponent } from './admin-detail/admin-detail.component';
import { ProfilComponent } from './profil/profil.component';
import { HeaderComponent } from './header/header.component';
import { ListUserComponent } from './list-user/list-user.component';
import { UserDetailComponent } from './user-detail/user-detail.component';
import { ListBastionComponent } from './list-bastion/list-bastion.component';
import { BastionDetailComponent } from './bastion-detail/bastion-detail.component';
import { ListResourcesComponent } from './list-resources/list-resources.component';
import { ResourceDetailComponent } from './resource-detail/resource-detail.component';
import { ListUserResourceComponent } from './list-user-resource/list-user-resource.component';
import { ListUserResourceDetailComponent } from './list-user-resource-detail/list-user-resource-detail.component';


@NgModule({
  declarations: [
    AdminComponent,
    MenuComponent,
    ListAdminComponent,
    AdminDetailComponent,
    ProfilComponent,
    HeaderComponent,
    ListUserComponent,
    UserDetailComponent,
    ListBastionComponent,
    BastionDetailComponent,
    ListResourcesComponent,
    ResourceDetailComponent,
    ListUserResourceComponent,
    ListUserResourceDetailComponent,
  ],
  imports: [
    CommonModule,
    AdminRoutingModule,
    ReactiveFormsModule
  ]
})
export class AdminModule { }
