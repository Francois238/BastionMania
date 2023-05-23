import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { AdminComponent } from './admin.component';
import { MenuComponent } from './menu/menu.component';
import { ListAdminComponent } from './list-admin/list-admin.component';
import { ProfilComponent } from './profil/profil.component';
import { ListUserComponent } from './list-user/list-user.component';
import { ListBastionComponent } from './list-bastion/list-bastion.component';

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
      {path: 'bastions', component: ListBastionComponent}
    ]
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class AdminRoutingModule { }
