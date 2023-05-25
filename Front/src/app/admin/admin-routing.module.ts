import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { AdminComponent } from './admin.component';
import { MenuComponent } from './menu/menu.component';
import { ListAdminComponent } from './list-admin/list-admin.component';
import { ProfilComponent } from './profil/profil.component';
import { ListUserComponent } from './list-user/list-user.component';
import { ListBastionComponent } from './list-bastion/list-bastion.component';
import { BastiondetailComponent } from '../user/bastiondetail/bastiondetail.component';
import { ListResourcesComponent } from './list-resources/list-resources.component';
import { ResourceDetailComponent } from './resource-detail/resource-detail.component';
import { ListUserResourceComponent } from './list-user-resource/list-user-resource.component';

const routes: Routes = [
  {
    path: '',
    component: AdminComponent,
    children: [
      { path: '', redirectTo: 'menu', pathMatch: 'full' }, // Default route, redirect to 'menu'
      { path: 'menu', component: MenuComponent },
      { path: 'admins', component: ListAdminComponent },
      { path: 'profil', component: ProfilComponent },
      { path: 'users', component: ListUserComponent },
      {path: 'bastions', component: ListBastionComponent},
      {path: 'bastions/:idBastion', component: ListResourcesComponent},
      {path: 'bastions/:idBastion/:idRessource', component: ListUserResourceComponent}
    ]
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class AdminRoutingModule { }
